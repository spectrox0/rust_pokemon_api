use reqwest::get;
use std::error::Error;
use crate::models::pokemon::{Pokemon, PokemonSpecies, EvolutionChain};
// Get Pokemon service by name
//In your get_pokemon_by_name function, it's more idiomatic to accept a &str rather than a &String. This allows more flexibility and avoids unnecessary references.
pub async fn get_pokemon_by_name(name: &str) -> Result<Pokemon, Box<dyn Error>> {
    // Obtener datos del Pokémon
    let pokemon_url = format!("https://pokeapi.co/api/v2/pokemon/{}", name);
    let mut pokemon = get(&pokemon_url).await?.json::<Pokemon>().await?;
    println!("Pokemon: {:?}", pokemon);

    // let species_url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", pokemon.id);
    let evolution_chain_url = get_pokemon_species(pokemon.id).await?;

    let evolution_chain = get_evolution_chain(&evolution_chain_url).await?;
    pokemon.evolution_chain = Some(evolution_chain);
    Ok(pokemon)
}


pub async fn get_pokemon_species(id: u32) -> Result<String, Box<dyn Error>> {
    let url = format!("https://pokeapi.co/api/v2/pokemon-species/{}", id);
    let res = get(&url).await?.json::<PokemonSpecies>().await?;
    Ok(res.evolution_chain.url)
}


pub async fn get_evolution_chain(url: &str) -> Result<EvolutionChain, Box<dyn Error>> {
    let res = get(url).await?.json::<EvolutionChain>().await?;
    Ok(res)
}
