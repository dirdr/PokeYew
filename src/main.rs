use yew::prelude::*;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct Move {

}

#[derive(Deserialize, Debug)]
struct Moves {
    pub moves: Vec<Move>,
}

#[derive(Deserialize, Debug)]
struct Ability {

}

#[derive(Deserialize, Debug)]
struct Abilities {
    pub abilities: Vec<Ability>,
}

#[derive(Deserialize, Debug)]
struct Pokemon {
    pub id: i32,
    pub abilities: Abilities,
    pub moves: Moves,
    pub name: String
}

struct PokemonComponent {
    pub pokemon: Option<Pokemon>,
}

enum Msg {
    GetPokemon,
    Response(String),
}

impl PokemonComponent {
    fn deserialize(text: &str) -> serde_json::Result<()> {
        let result: Pokemon = serde_json::from_str(&text)?;
        Ok(())
    }
}

impl Component for PokemonComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            pokemon: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{"Je suis un Pokemon"}</h1>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GetPokemon => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let request = reqwest::get("https://pokeapi.co/api/v2/pokemon/ditto").await.unwrap();
                    let text = request.text().await.unwrap(); 
                    link.send_message(Msg::Response(text));
                });
                false
            }
            Msg::Response(text) => {
                true
            }
        } 
    }
}


#[function_component(App)] 
fn app() -> Html {
    html! {
        <div>
        <PokemonComponent/> 
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
