use yew::prelude::*;
use yew_router::prelude::*;

pub mod login;
pub mod not_found;
pub mod user;
mod role;
mod dashboard;
mod api;

use login::Login;
use not_found::NotFound;
use dashboard::Dashboard;

use crate::components::error::ErrorComponent;


#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/dashboard")]
    Dashboard,
    #[at("/dashboard/*")]
    DashboardChild,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{"Welcome to Home Page"}</h1> },
        Route::Login => html! { <Login /> },
        Route::Dashboard | Route::DashboardChild => html! { <Dashboard /> },
        Route::NotFound => html! { <NotFound /> },
    }
}


#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <ErrorComponent />
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}