use yew::prelude::*;
use yew_router::prelude::*;
use crate::routers::index::{Route,switch};


use super::header::Header;

#[function_component(PageContent)]
pub fn page_content() -> Html {
    html! {
        <div class="drawer-content flex flex-col ">
            <Header/>
            <main class="flex-1 overflow-y-auto md:pt-4 pt-4 px-6  bg-base-200">
                <BrowserRouter>
                    <Switch<Route> render={switch} />
                </BrowserRouter>
                <div class="h-16"></div>
            </main>
        </div>
    }
}