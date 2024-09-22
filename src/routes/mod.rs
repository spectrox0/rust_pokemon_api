use axum::Router;
mod pokemon;
use pokemon::create_pokemon_routes;

pub fn create_routes() -> Router {
    Router::new().merge(create_pokemon_routes())
}