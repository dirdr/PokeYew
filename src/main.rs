mod pokemon;
mod pokemon_component;

use crate::pokemon::Pokemon;
use yew::prelude::*;
use yew::Properties;
use web_sys::HtmlInputElement;


#[derive(Properties, PartialEq)]
struct LinkProps {
    pub emit: Callback<String>
}

#[function_component(Form)]
fn form(props: &LinkProps) -> Html {
    let pokemon_handle = use_state(|| "".to_string());
    let handle_input = Callback::from(move |event: InputEvent| {
        if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
            pokemon_handle.set(input.value());
        }
    });
    //let handle_click = Calback::from(move |event: MouseEvent| {
                 
    //})
    html! {
        <div>
            <input type="text" name="pokemon_name_request" oninput={handle_input}/>
            <button>{"Submit"}</button>
        </div>
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
