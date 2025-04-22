pub mod routers {
    use axum::{Router, routing::post};

    use crate::{
        handlers::{auth, users},
        models::app::AppState,
    };

    pub fn user_router(state: AppState) -> Router {
        let router = Router::new().route("/", post(users::create));
        Router::new().nest("/users", router).with_state(state)
    }
    pub fn auth_router(state: AppState) -> Router {
        let router = Router::new().route("/login", post(auth::login));
        Router::new().nest("/auth", router).with_state(state)
    }
}
