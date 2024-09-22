use yew::prelude::*;
use crate::features::role::RoleManagement;


#[function_component(InternalPage)]
pub fn internal_page() -> Html {
    html! {
        <div>
            <RoleManagement/>
        </div>
    }
}