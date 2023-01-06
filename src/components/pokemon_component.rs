use pokeyew::structs::Pokemon;
use gloo_net::http::Request; 
use wasm_bindgen::JsValue;
use yew::prelude::*;
use pokeyew::components::PokemonInputForm; 

pub struct PokemonComponent {
    pub pokemon: Option<Pokemon>,
}

pub enum MsgPokemonComponent {
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
        let link = ctx.link().clone();
        let get_pokemon: Callback<String> = Callback::from(move |p_name: String| {
            link.send_message(MsgPokemonComponent::GetPokemon(p_name.clone()))
        });
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
                <PokemonInputForm {get_pokemon}/> // here inside brace is the Property we pass the
            // the component, only one field in our case, the callback, should have the same name
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgPokemonComponent::GetPokemon(requested_pokemon_name) => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let endpoint = format!("https://pokeapi.co/api/v2/pokemon/{}", requested_pokemon_name);
                    web_sys::console::log_1(&JsValue::from(&endpoint));
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
