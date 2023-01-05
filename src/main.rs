mod pokemon;

use crate::pokemon::Pokemon;
use yew::prelude::*;
use gloo_net::http::Request;
use web_sys::HtmlInputElement;

struct PokemonComponent {
    pub pokemon: Option<Pokemon>,
}

enum MsgPokemonComponent {
    GetPokemon(String),
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
        html! {
            <div>
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
            MsgPokemonComponent::GetPokemon(requested_pokemon_name) => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let endpoint = format!("https://pokeapi.co/api/v2/pokemon/{}", requested_pokemon_name);
                    let fetched_pokemon: Pokemon = Request::get(&endpoint)
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


#[function_component(Form)]
fn form() -> Html {
    let pokemon_handle = use_state(|| "".to_string());
    let handle_input = Callback::from(move |event: InputEvent| {
        if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
            pokemon_handle.set(input.value());
        }
    });
    html! {
        <div>
            <input type="text" name="pokemon_name_request" oninput={handle_input}/>
        </div>
    }
}

#[function_component(App)] 
fn app() -> Html {
    html! {
        <div>
            <PokemonComponent/> 
            <Form/>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
