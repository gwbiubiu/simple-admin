use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SubTitleProps{
    pub children: Children,
}


#[function_component(SubTitle)]
pub fn sub_title(props: &SubTitleProps) -> Html {
    html! {
        <div class="text-xl font-semibold">
            {for props.children.iter()}
        </div>
    }
}