use yew::{Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct InputPokemonCallbackProps {
    pub emit: Callback<String>
}
