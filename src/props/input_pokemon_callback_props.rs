use yew::{Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct InputPokemonCallbackProps {
    pub get_pokemon: Callback<String>
}
