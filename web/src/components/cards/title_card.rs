use yew::prelude::*;
use super::super::typography::SubTitle;

#[derive(Properties, PartialEq, Clone)]
pub struct TitleCardProps{
    pub title: String,
    pub top_margin: String,
    pub top_side_buttons: Option<Html>,
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
            </div>
        </div>
    }
}