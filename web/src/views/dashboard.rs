use yew::prelude::*;
use yew_router::prelude::*;
use super::Route;
use super::{UserList, RoleList};

pub struct Dashboard;


#[derive(Clone, Routable, PartialEq)]
pub enum DashboardRoute {
    #[at("/dashboard")]
    Home,
    #[at("/dashboard/users")]
    Users,
    #[at("/dashboard/roles")]
    Roles,
}

impl Component for Dashboard {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="container-fluid">
                <div class="row">
                    <nav class="col-md-2 d-none d-md-block bg-light sidebar">
                        <div class="position-sticky pt-3">
                            <ul class="nav flex-column">
                                <li class="nav-item">
                                    <Link<DashboardRoute> classes="nav-link" to={DashboardRoute::Users}>
                                        { "用户管理" }
                                    </Link<DashboardRoute>>
                                </li>
                                <li class="nav-item">
                                    <Link<DashboardRoute> classes="nav-link" to={DashboardRoute::Roles}>
                                        { "角色管理" }
                                    </Link<DashboardRoute>>
                                </li>
                            </ul>
                        </div>
                    </nav>
                    <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
                        <Switch<DashboardRoute> render={Self::switch} />
                    </main>
                </div>
            </div>
        }
    }
}

impl Dashboard {
    fn switch(routes: DashboardRoute) -> Html {
        match routes {
            DashboardRoute::Home => html! { <h1>{ "欢迎来到仪表板" }</h1> },
            DashboardRoute::Users => html! { <UserList /> },
            DashboardRoute::Roles => html! { <RoleList /> },
        }
    }
}