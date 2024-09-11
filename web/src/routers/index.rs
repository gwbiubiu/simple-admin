use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/*")]
    NotFound,
}


pub fn switch(routes: Route) -> Html {
    match routes {
        Route::NotFound => html! { <h1>{"hello 404"}</h1> },
    }
}