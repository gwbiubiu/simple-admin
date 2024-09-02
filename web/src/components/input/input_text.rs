use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct InputTextProps {
    pub container_style: String,
    pub label_title: String,
}

#[function_component(InputText)]
pub fn input_text(props: &InputTextProps) -> Html {
    html! {
        <div class={format!("form-control w-full {}", props.container_style)}>
            <label class="label">
                <span class="label-text text-base-content">{props.label_title.clone()}</span>
            </label>
            <input type="text" class="input  input-bordered w-full"/>
        </div>
    }
}
