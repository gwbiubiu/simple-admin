use yew::prelude::*;
use crate::features::api::ApiManagement;


#[function_component(InternalPage)]
pub fn internal_page() -> Html {
    html! {
        <div>
            <ApiManagement/>
        </div>
    }
}