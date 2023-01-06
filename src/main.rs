extern crate self as pokeyew;

mod components;
mod structs;
mod props;

use yew::prelude::*;
use components::PokemonComponent;

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
