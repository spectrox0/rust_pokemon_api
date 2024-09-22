use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use crate::services::pokemon::get_pokemon_by_name;
use crate::models::error::{ErrorResponse, NotFoundError};
use tracing::{error, info};

pub async fn get_pokemon_handler(Path(name): Path<String>) -> impl IntoResponse {
    info!("Fetching data for Pokémon: {}", name);
    match get_pokemon_by_name(&name).await {
        Ok(pokemon) => {
            info!("Successfully fetched Pokémon: {}", pokemon.name);
            (StatusCode::OK, Json(pokemon)).into_response()},
        Err(e) => {
            error!("Error fetching Pokémon data: {:?}", e);
            if e.is::<NotFoundError>() {
                (
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error: e.to_string(),
                    }),
                )
            } else {

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "Internal server error".to_string(),
                    }),
                )
            }
            .into_response()
        }
    }
}