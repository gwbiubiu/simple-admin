use yew::prelude::*;

use crate::element::base::ElInput;
pub struct Login {
    username: String,
    password: String,
}

pub enum Msg {}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <ElInput value="abc"></ElInput>
            </div>
        }
    }
}