use pokeyew::structs::Pokemon;
use gloo_net::http::Request; 
use wasm_bindgen::JsValue;
use yew::prelude::*;
use pokeyew::components::PokemonInputForm; 

pub struct PokemonComponent {
    pub pokemon: Option<Pokemon>,
    pub error_message: Option<String>,
}

pub enum MsgPokemonComponent {
    GetPokemon(String),
    Received(Option<Pokemon>),
}

impl Component for PokemonComponent {
    type Message = MsgPokemonComponent;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            pokemon: None,
            error_message: None,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let get_pokemon: Callback<String> = Callback::from(move |p_name: String| {
            link.send_message(MsgPokemonComponent::GetPokemon(p_name.clone()))
        });
        html! {
            <div class="container">
                if let Some(pokemon) = &self.pokemon {
                    <h2>{pokemon.name.clone()}</h2> 
                }
                if let Some(t) = &self.error_message {
                    <h2>{t}</h2>        
                }
                <div>
                    if let Some(exist) = self.pokemon.clone() {
                        <img src={exist.sprites.front_default.unwrap()} alt="sprite"/>
                    }
                </div>
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
                    //web_sys::console::log_1(&JsValue::from(&endpoint));
                    let fetched: Result<Pokemon, gloo_net::Error> = Request::get(&endpoint)
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await;
                    link.send_message(MsgPokemonComponent::Received(fetched.ok()));
                });
                false
            }
            MsgPokemonComponent::Received(fetched) => {
                self.pokemon = fetched.clone();
                if let None = fetched {self.error_message = Some(String::from("Pokemon Not found!"))} else {self.error_message = None}
                true
            }
        } 
    }
}
