use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="row justify-content-center text-center" style="border: 1px solid white">
            <img src="resources/logo.png" alt="logo" style="width: 190px; height: 260px;" class="my-4"/>
            <p class="lead">{"PokeAPI wrapper built with Rust and Yew"}</p>
        </div>
    }
}
