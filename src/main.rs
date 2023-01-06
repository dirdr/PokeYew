mod components;
mod structs;
mod props;

extern crate self as pokeyew;

use pokeyew::props::InputPokemonCallbackProps;
use yew::{prelude::*};
use web_sys::HtmlInputElement;
use pokeyew::components::PokemonComponent;


#[function_component(Form)]
fn form(props: &InputPokemonCallbackProps) -> Html {
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
