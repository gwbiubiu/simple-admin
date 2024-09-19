use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::protected::{NotFound, Welcome};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/app/welcome")]
    Welcome,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Welcome => html! { <Welcome /> },
        Route::NotFound => html! {<NotFound />},
    }
}