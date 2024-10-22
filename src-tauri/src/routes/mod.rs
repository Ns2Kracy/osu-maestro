use axum::Router;

pub mod api;
pub mod ws;

pub fn mount() -> Router {
    Router::new().merge(api::mount()).merge(ws::mount())
}
