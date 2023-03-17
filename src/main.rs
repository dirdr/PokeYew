extern crate self as pokeyew;

mod components;
mod props;
mod structs;

use components::{Header, PokemonComponent};
use yew::prelude::*;

/*
top level component for our application
*/
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
