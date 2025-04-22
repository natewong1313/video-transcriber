use axum_login::AuthManagerLayerBuilder;
use time::Duration;

use axum::Router;
use tokio::{signal, task::AbortHandle};
use tower_http::trace::{self, TraceLayer};
use tower_sessions::{ExpiredDeletion, Expiry, SessionManagerLayer, cookie::Key};
use tracing::Level;

use crate::{
    common::{db::connect, store::SqliteStore},
    handlers::auth::Backend,
    models::app::AppState,
};

use super::routers::routers;

pub async fn serve() {
    let pool = match connect().await {
        Ok(pool) => pool,
        Err(err) => panic!("Error connecting to db: {}", err),
    };

    let app_state = AppState { pool };

    let session_store = SqliteStore::new(app_state.clone().pool);
    if let Err(err) = session_store.migrate().await {
        panic!("Error migrating session store: {}", err);
    }
    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(tokio::time::Duration::from_secs(60)),
    );
    let key = Key::generate();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)))
        .with_signed(key);
    let backend = Backend::new(app_state.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let app = Router::new()
        .with_state(app_state.clone())
        .merge(routers::user_router(app_state.clone()))
        .layer(auth_layer)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(deletion_task.abort_handle()))
        .await;
    deletion_task.await;
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
