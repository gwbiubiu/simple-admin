use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::protected::{NotFound, Welcome, user, role, api};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/app/welcome")]
    Welcome,
    #[at("/app/users")]
    User,
    #[at("/app/roles")]
    Roles,
    #[at("/app/apis")]
    Apis,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Welcome => html! { <Welcome /> },
        Route::User => html! {<user::InternalPage/>},
        Route::Roles => html!{<role::InternalPage/>},
        Route::Apis => html!{<api::InternalPage/>},
        Route::NotFound => html! {<NotFound />},
    }
}