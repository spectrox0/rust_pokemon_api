use axum::{routing::get, Router};
use crate::controllers::pokemon::get_pokemon_handler;

pub fn create_pokemon_routes() -> Router {
    Router::new().route("/pokemon/:name", get(get_pokemon_handler))
    //.route("/", get(|| async { "Hello, World!" }))
}