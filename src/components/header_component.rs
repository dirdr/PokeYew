use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="row justify-content-center text-center my-4" style="border: 1px solid black">
            <img src="resources/logo.png" alt="logo" style="width: 190px; height: 260px;"/>
            <p>{"A yew based pokeapi visualizer"}</p>
        </div>
    }
}
