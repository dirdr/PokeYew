extern crate self as pokeyew;

mod components;
mod structs;
mod props;

use yew::prelude::*;
use components::{PokemonComponent, Header};

#[function_component(App)] 
fn app() -> Html {
    html! {
        <main class="container-fluid min-vh-100">
            <Header/>
            <PokemonComponent/> 
        </main>
    }
}

fn main() {
    yew::start_app::<App>();
}
