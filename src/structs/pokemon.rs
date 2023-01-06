use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Move {
    pub name: String,
    pub url: String
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Moves {
    pub moves: Vec<Move>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Ability {
    pub name: String,
    pub url: String
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Abilities {
    pub abilities: Vec<Ability>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sprites {
    pub back_default: String,
    pub back_female: String,
    pub back_shiny: String,
    pub back_shiny_famale: String,
    pub front_default: String,
    pub front_female: String,
    pub front_shiny: String,
    pub front_shiny_famale: String
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon {
    pub name: String,
    pub id: i32,
}
