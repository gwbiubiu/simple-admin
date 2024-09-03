use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct  ErrorTextProps{
    pub error_message: String,
    pub style_class: String,
}

#[function_component(ErrorText)]
pub fn error_text(props: &ErrorTextProps)-> Html{
    html!{
        <p class={format!("text-center text-error {}",props.style_class )}> {props.error_message.clone()}</p>
    }
}