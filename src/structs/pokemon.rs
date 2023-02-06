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
pub struct Sprites {
    pub front_default: Option<String>,
    pub front_shiny: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Type {
    pub name: String,
    pub url: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Types {
    pub slot: i32,
    pub types: Vec<Type>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pokemon {
    pub name: String,
    pub id: i32,
    pub sprites: Sprites,
    pub types: Vec<Types>,
}
