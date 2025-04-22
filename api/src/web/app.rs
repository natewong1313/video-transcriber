use axum::Router;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::{common::db::connect, models::app::AppState};

use super::routers::routers;

pub async fn serve() {
    let pool = match connect().await {
        Ok(pool) => pool,
        Err(err) => panic!("Error connecting to db: {}", err),
    };

    let app_state = AppState { pool };

    let app = Router::new()
        .with_state(app_state.clone())
        .merge(routers::user_router(app_state.clone()))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
