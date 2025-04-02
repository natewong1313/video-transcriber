mod db;
mod models;
mod user;

use axum::{Router, routing::get};
use models::AppState;
use user::user_router;

#[tokio::main]
async fn main() {
    let pool = match db::connect_db().await {
        Ok(pool) => pool,
        Err(err) => panic!("Error connecting to db: {}", err),
    };

    let app_state = AppState { pool };

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .with_state(app_state.clone())
        .merge(user_router(app_state));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
