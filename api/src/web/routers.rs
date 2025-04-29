pub mod routers {
    use axum::{
        Router,
        routing::{get, post},
    };

    use crate::{
        handlers::{
            auth, transcribe,
            users::{self},
        },
        models::app::AppState,
    };

    pub fn user_router(state: AppState) -> Router {
        let router = Router::new()
            .route("/", post(users::create))
            .route("/", get(users::get));
        Router::new().nest("/users", router).with_state(state)
    }
    pub fn auth_router(state: AppState) -> Router {
        let router = Router::new().route("/login", post(auth::login));
        Router::new().nest("/auth", router).with_state(state)
    }
    pub fn transcribe_router(state: AppState) -> Router {
        let router = Router::new().route("/new", post(transcribe::upload));
        Router::new().nest("/transcribe", router).with_state(state)
    }
}
