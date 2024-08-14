use yew::prelude::*;
use yew_router::prelude::*;

pub mod login;
pub mod not_found;

use login::Login;
use not_found::NotFound;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{"Welcome to Home Page"}</h1> },
        Route::Login => html! { <Login /> },
        Route::NotFound => html! { <NotFound /> },
    }
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}