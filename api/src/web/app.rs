use axum::Router;

use crate::{common::db::connect, models::app::AppState};

use super::routers::routers;

pub async fn serve() {
    let pool = match connect().await {
        Ok(pool) => pool,
        Err(err) => panic!("Error connecting to db: {}", err),
    };

    let app_state = AppState { pool };

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .with_state(app_state.clone())
        .merge(routers::user_router(app_state.clone()));
    // .merge(auth_router(app_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
