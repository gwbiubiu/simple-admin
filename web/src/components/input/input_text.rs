use yew::prelude::*;


#[derive(PartialEq, Eq,Clone)]
pub enum TextType {
    Text,
    Password,
}

#[derive(Properties, PartialEq, Clone)]
pub struct InputTextProps {
    pub container_style: String,
    pub label_title: String,
    pub value: String,
    pub text_type: TextType,
    pub id: String,
}

#[function_component(InputText)]
pub fn input_text(props: &InputTextProps) -> Html {
    let input_text_type = match props.text_type {
        TextType::Text => "text",
        TextType::Password=> "password"
    };
    html! {
        <div class={format!("w-full {}", props.container_style)}>
            <label class="label mb-2">
                <span class="label-text text-base-content">{props.label_title.clone()}</span>
            </label>
            <input id={props.id.clone()} type={input_text_type} value={props.value.clone()} class="input input-bordered w-full"/>
        </div>
    }
}
