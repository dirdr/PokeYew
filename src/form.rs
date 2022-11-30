use yew::prelude::*;

pub struct FormComponent;

enum Msg {
        
}

impl FormComponent {
}

impl Component for FormComponent {
    type Message = Msg;    
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
            </div>
        } 
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true 
    }
}
