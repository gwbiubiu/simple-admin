use yew::prelude::*;
use yew::{Properties};

#[derive(Properties, PartialEq, Clone)]
pub struct InputProps {
    pub value: String,
    #[prop_or(false)]
    pub disabled: bool,
    #[prop_or(String::from("请输入内容"))]
    pub placeholder: String,
}

pub struct ElInput {
    text: String,
}

pub enum ElInputMsg {}

impl Component for ElInput {
    type Message = ElInputMsg;
    type Properties = InputProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            text: ctx.props().value.clone()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
           <div class="el-input">
            <input disabled={ctx.props().disabled} 
            value={self.text.clone()}
            class="el-input__inner"
            placeholder={ctx.props().placeholder.clone()} />
            </div>
        }
    }
}