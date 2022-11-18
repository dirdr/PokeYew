use yew::prelude::*;
use serde::Deserialize;
use gloo_net::http::Request;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Move {
    pub name: String,
    pub url: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Moves {
    pub moves: Vec<Move>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Ability {
    pub name: String,
    pub url: String
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Pokemon {
    pub name: String,
    pub id: i32,
}

struct PokemonComponent {
    pub pokemon: Option<Pokemon>,
}

enum MsgPokemonComponent {
    GetPokemon,
    ReceivedPokemon(Pokemon),
}

impl Component for PokemonComponent {
    type Message = MsgPokemonComponent;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            pokemon: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| MsgPokemonComponent::GetPokemon);
        //let oninput = ctx.link().callback(|e: InputEvent| MsgPokemonComponent::UserInput(e.data().unwrap()));
        html! {
            <div>
                <form>
                    <input type="text" id="wantedPokemon"/>
                    <input onclick={onclick} type="submit"/>
                </form>
                <h1>
                    {
                        match self.pokemon.clone() {
                            Some(pok) => pok.name,   
                            None => String::from("No pokemon yet.."),
                        }
                    }
                </h1>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgPokemonComponent::GetPokemon => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_pokemon: Pokemon = Request::get("https://pokeapi.co/api/v2/pokemon/ditto")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    link.send_message(MsgPokemonComponent::ReceivedPokemon(fetched_pokemon));
                });
                false
            }
            MsgPokemonComponent::ReceivedPokemon(pokemon) => {
                self.pokemon = Some(pokemon);
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
