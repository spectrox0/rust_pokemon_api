use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PokemonType {
    Normal,
    Fire,
    Water,
    Grass,
    Electric,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dark,
    Dragon,
    Steel,
    Fairy,
    Unknown,
}
impl FromStr for PokemonType {
    type Err = ();

    fn from_str(s: &str) -> Result<PokemonType, ()> {
        match s {
            "normal" => Ok(PokemonType::Normal),
            "fire" => Ok(PokemonType::Fire),
            "water" => Ok(PokemonType::Water),
            "grass" => Ok(PokemonType::Grass),
            "electric" => Ok(PokemonType::Electric),
            "ice" => Ok(PokemonType::Ice),
            "fighting" => Ok(PokemonType::Fighting),
            "poison" => Ok(PokemonType::Poison),
            "ground" => Ok(PokemonType::Ground),
            "flying" => Ok(PokemonType::Flying),
            "psychic" => Ok(PokemonType::Psychic),
            "bug" => Ok(PokemonType::Bug),
            "rock" => Ok(PokemonType::Rock),
            "ghost" => Ok(PokemonType::Ghost),
            "dark" => Ok(PokemonType::Dark),
            "dragon" => Ok(PokemonType::Dragon),
            "steel" => Ok(PokemonType::Steel),
            "fairy" => Ok(PokemonType::Fairy),
            _ => Ok(PokemonType::Unknown),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub id: u32,
    pub height: u32,
    pub base_experience: u32,
    pub abilities: Vec<AbilitySlot>,
    pub held_items: Vec<HeldItem>,
    pub sprites: Sprites,
    pub evolution_chain: Option<EvolutionChain>,
    pub types: Vec<PokemonTypeSlot>,  // Tipos del Pokémon
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonTypeSlot {
    pub slot: u32,
    pub r#type: PokemonTypeData,  // Aquí cambiamos de `PokemonType` a `PokemonTypeData`
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonTypeData {
    pub name: PokemonType,  // `name` contendrá el `enum PokemonType`
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AbilitySlot {
    pub ability: Ability,
    pub is_hidden: bool,
    pub slot: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ability {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeldItem {
    pub item: Item,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sprites {
    pub front_default: Option<String>,
    pub back_default: Option<String>,
    pub front_shiny: Option<String>,
    pub back_shiny: Option<String>,
    pub front_female: Option<String>,
    pub back_female: Option<String>,
    pub front_shiny_female: Option<String>,
    pub back_shiny_female: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct EvolutionChain {
    pub chain: EvolutionDetail,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvolutionDetail {
    pub species: EvolutionSpecies,
    pub evolves_to: Vec<EvolutionDetail>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvolutionSpecies {
    pub name: String,
    pub url: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonSpecies {
    pub evolution_chain: EvolutionChainUrl,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EvolutionChainUrl {
    pub url: String,
}