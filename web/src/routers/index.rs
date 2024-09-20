use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::protected::{NotFound, Welcome, user};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/app/welcome")]
    Welcome,
    #[at("/app/users")]
    User,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Welcome => html! { <Welcome /> },
        Route::NotFound => html! {<NotFound />},
        Route::User => html! {<user::InternalPage/>},
    }
}