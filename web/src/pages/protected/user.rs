use yew::prelude::*;
use crate::features::user::UserManagement;


#[function_component(InternalPage)]
pub fn internal_page() -> Html {
    html! {
        <div>
            <UserManagement/>
        </div>
    }
}