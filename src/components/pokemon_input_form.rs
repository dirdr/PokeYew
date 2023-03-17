use web_sys::HtmlInputElement;
use yew::prelude::*;
use pokeyew::props::InputPokemonCallbackProps;

#[function_component(PokemonInputForm)]
pub fn pokemon_input_form(props: &InputPokemonCallbackProps) -> Html {
    let pokemon_handle = use_state(|| "".to_string());
    let pokemon_value = (*pokemon_handle).clone();
    let handle_input = Callback::from(move |event: InputEvent| {
        if let Some(input) = event.target_dyn_into::<HtmlInputElement>() {
            pokemon_handle.set(input.value());
        }
    });
    let get_pokemon = props.get_pokemon.clone();
    html! {
        <div class="w-50 mx-auto">
            <div class="row">
                <div class="input-group mb-3">
                    <input class="form-control" type="text" name="pokemon_name_request" placeholder="search a pokemon..." oninput={handle_input}/>
                    <div class="input-group-append">
                        <button type="button" class="btn btn-primary" onclick = {move |_| 
                            get_pokemon.emit(pokemon_value.clone())
                        }>{"Submit"}</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
