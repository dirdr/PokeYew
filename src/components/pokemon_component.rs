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
            <div class="row justify-content-center" >
                <div class="col-8 text-center" style="border: 1px solid black;">
                    <PokemonInputForm {get_pokemon}/> // here inside brace is the Property we pass the
                    if let Some(exist) = &self.pokemon {
                        <img src={exist.clone().sprites.other.official_artwork.front_default} alt="sprite" style="transform: scale(1)"/>
                        <ul style="list-style-type: none;">
                            {
                                exist.clone()
                                    .types
                                    .iter()
                                    .map(|t| html! {
                                        <li>
                                            <img src={format!("resources/types/{}.png", t.type_field.name.clone())} alt="type" style="transform: scale(1.5)"/>
                                        </li>
                                    })
                                    .collect::<Html>()
                            }
                        </ul>
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
                if let Err(_) = fetched {
                    self.error_message = Some(String::from("This pokemon doesn't exist"));
                    self.pokemon = None;
                } else {
                    self.error_message = None;
                    self.pokemon = fetched.as_ref().ok().cloned();
                }
                true
            }
        }
    }
}
