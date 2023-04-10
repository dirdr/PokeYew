use serde_json::Value;
use serde::{Deserialize};

#[derive(Deserialize, Debug, Clone)]
pub struct Species {
    pub base_happiness: i64,
    pub capture_rate: i64,
    pub color: Color,
    pub egg_groups: Vec<EggGroup>,
    pub evolution_chain: EvolutionChain,
    pub evolves_from_species: Value,
    pub flavor_text_entries: Vec<FlavorTextEntry>,
    pub form_descriptions: Vec<Value>,
    pub forms_switchable: bool,
    pub gender_rate: i64,
    pub genera: Vec<Genera>,
    pub generation: Generation,
    pub growth_rate: GrowthRate,
    pub habitat: Habitat,
    pub has_gender_differences: bool,
    pub hatch_counter: i64,
    pub id: i64,
    pub is_baby: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub name: String,
    pub names: Vec<Name>,
    pub order: i64,
    pub pal_park_encounters: Vec<PalParkEncounter>,
    pub pokedex_numbers: Vec<PokedexNumber>,
    pub shape: Shape,
    pub varieties: Vec<Variety>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Color {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EggGroup {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EvolutionChain {
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FlavorTextEntry {
    pub flavor_text: String,
    pub language: Language,
    pub version: Version,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Language {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Version {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Genera {
    pub genus: String,
    pub language: Language2,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Language2 {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Generation {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GrowthRate {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Habitat {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Name {
    pub language: Language3,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Language3 {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PalParkEncounter {
    pub area: Area,
    pub base_score: i64,
    pub rate: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Area {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PokedexNumber {
    pub entry_number: i64,
    pub pokedex: Pokedex,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pokedex {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Shape {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Variety {
    pub is_default: bool,
    pub pokemon: Pokemon,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pokemon {
    pub name: String,
    pub url: String,
}
