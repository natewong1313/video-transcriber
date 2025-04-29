use std::env;

use aws_config::BehaviorVersion;

use axum_login::AuthManagerLayerBuilder;
use time::Duration;

use axum::{Router, extract::DefaultBodyLimit};
use tokio::{signal, task::AbortHandle};
use tower_http::trace::{self, TraceLayer};
use tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer};
use tracing::Level;

use crate::{
    common::{auth_backend::Backend, db::connect, store::SqliteStore},
    models::app::AppState,
};

use super::routers::routers;

pub async fn serve() {
    if let Err(err) = dotenvy::dotenv() {
        panic!("Error parsing .env: {}", err);
    };
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(err) => panic!("Failed to parse db url: {}", err),
    };

    let pool = match connect(&db_url).await {
        Ok(pool) => pool,
        Err(err) => panic!("Error connecting to db: {}", err),
    };
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .region("auto")
        .endpoint_url("https://738ca889bda1e4ef05316cd872c0c38b.r2.cloudflarestorage.com")
        .load()
        .await;
    let s3_client = aws_sdk_s3::Client::new(&sdk_config);

    let app_state = AppState { pool, s3_client };

    let session_store = SqliteStore::new(app_state.clone().pool);
    if let Err(err) = session_store.migrate().await {
        panic!("Error migrating session store: {}", err);
    }
    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );
    // let key = Key::generate();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));
    // .with_signed(key);
    let backend = Backend::new(app_state.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = Router::new()
        .with_state(app_state.clone())
        .merge(routers::user_router(app_state.clone()))
        .merge(routers::auth_router(app_state.clone()))
        .merge(routers::transcribe_router(app_state))
        .layer(auth_layer)
        .layer(DefaultBodyLimit::disable())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await;
    let _ = deletion_task.await;
}

async fn shutdown_signal(deletion_task_abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
