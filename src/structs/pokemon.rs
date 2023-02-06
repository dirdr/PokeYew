use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Move {
    pub name: String,
    pub url: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct Moves {
    pub moves: Vec<Move>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Ability {
    pub name: String,
    pub url: String
}

#[derive(Deserialize, Debug, Clone)]
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
pub struct TypeInfo {
    pub slot: i32,
    #[serde(rename = "type")]
    pub type_field: Type,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TypeArray {
    pub types: Vec<Type>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pokemon {
    pub name: String,
    pub id: i32,
    pub sprites: Sprites,
    pub types: Vec<TypeInfo>
}
