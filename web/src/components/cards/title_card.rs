use super::super::typography::SubTitle;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TitleCardProps {
    pub title: String,
    pub top_margin: String,
    pub top_side_buttons: Option<Html>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(TitleCard)]
pub fn title_card(props: &TitleCardProps) -> Html {
    html! {
        <div class="card w-full p-6 bg-base-100 shadow-xl mt-6">
            <div class="card-body">
                <SubTitle>
                    {props.title.clone()}
                    {
                        if let Some(buttons) = &props.top_side_buttons {
                            html!{<div class="inline-block float-right">{buttons.clone()}</div>}
                        } else {
                            html!{}
                        }
                    }
                </SubTitle>
                <div class="divider mt-2"></div>
                <div class="h-full w-full pb-6 bg-base-100-100">
                    {for props.children.iter()}
                </div>
                
            </div>
        </div>
    }
}
