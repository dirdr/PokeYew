use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PokemonSpecies {
    pub names: Vec<Name>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Name {
    pub language: Language,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Language {
    pub name: String,
    pub url: String,
}
