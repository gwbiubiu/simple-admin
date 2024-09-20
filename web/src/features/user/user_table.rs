use yew::prelude::*;
use crate::components::cards::title_card::{TitleCard, _TitleCardProps::top_side_buttons};



#[function_component(TopSideButtons)]
fn top_side_buttons() -> Html {
    html! {
        <div class="inline-block float-right">
            <button class="btn px-6 btn-sm normal-case btn-primary">{"Add User"}</button>
        </div>
    }
}

#[function_component(UserManagement)]
pub fn user_management() -> Html {
    html! {
        <div>
            <TitleCard title="User Management" top_margin="mt-2" top_side_buttons={Some(html!{<TopSideButtons/>})}/>
        </div>
    }
}