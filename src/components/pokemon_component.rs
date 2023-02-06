use gloo_net::http::Request;
use pokeyew::components::PokemonInputForm;
use pokeyew::structs::Pokemon;
use wasm_bindgen::JsValue;
use yew::prelude::*;

pub struct PokemonComponent {
    pub pokemon: Option<Pokemon>,
    pub error_message: Option<String>,
}

pub enum MsgPokemonComponent {
    GetPokemon(String),
    Received(Result<Pokemon, gloo_net::Error>),
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
            <div class="row min-vh-100 justify-content-center">
                <div class="col-8 text-center">
                    <PokemonInputForm {get_pokemon}/> // here inside brace is the Property we pass the
                    if let Some(exist) = self.pokemon.clone() {
                        <img src={exist.sprites.front_default.unwrap()} alt="sprite"/>
                    }
                    if let Some(pokemon) = &self.pokemon {
                        <h2>{pokemon.name.clone()}</h2>
                    }
                    if let Some(t) = &self.error_message {
                        <h2>{t}</h2>
                    }
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgPokemonComponent::GetPokemon(requested_pokemon_name) => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let endpoint = format!(
                        "https://pokeapi.co/api/v2/pokemon/{}",
                        requested_pokemon_name
                    );
                    web_sys::console::log_1(&JsValue::from(&endpoint));
                    let fetched: Result<Pokemon, gloo_net::Error> =
                        Request::get(&endpoint).send().await.unwrap().json().await;
                    web_sys::console::log_1(&JsValue::from(format!("{:?}", fetched)));
                    link.send_message(MsgPokemonComponent::Received(fetched));
                });
                false
            }
            MsgPokemonComponent::Received(fetched) => {
                web_sys::console::log_1(&JsValue::from(format!("{:?}", fetched)));
                self.pokemon = fetched.as_ref().ok().cloned();
                if let Err(e) = fetched {
                    self.error_message = Some(e.to_string())
                } else {
                    self.error_message = None
                }
                true
            }
        }
    }
}
