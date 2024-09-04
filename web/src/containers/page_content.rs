use yew::prelude::*;

use super::header::Header;

#[function_component(PageContent)]
pub fn page_content() -> Html {
    html! {
        <div class="drawer-content flex flex-col ">
            <Header/>
        </div>
    }
}