use yew::prelude::*;
use yew_router::prelude::*;
use super::{UserList, RoleList};

pub struct Dashboard {
    active_route: DashboardRoute,
}

#[derive(Clone, Routable, PartialEq)]
pub enum DashboardRoute {
    #[at("/dashboard")]
    Home,
    #[at("/dashboard/users")]
    Users,
    #[at("/dashboard/roles")]
    Roles,
}

pub enum Msg {
    SetActiveRoute(DashboardRoute),
}

impl Component for Dashboard {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            active_route: DashboardRoute::Home,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetActiveRoute(route) => {
                self.active_route = route;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="container-fluid">
                <div class="row">
                    <nav class="col-md-2 d-none d-md-block bg-light sidebar min-vh-100">
                        <div class="position-sticky pt-3">
                            <ul class="nav flex-column">
                                { self.render_nav_item(ctx, DashboardRoute::Home, "仪表盘") }
                                { self.render_nav_item(ctx, DashboardRoute::Users, "用户管理") }
                                { self.render_nav_item(ctx, DashboardRoute::Roles, "角色管理") }
                            </ul>
                        </div>
                    </nav>
                    <main class="col-md-9 ms-sm-auto col-lg-10 px-md-4">
                        <div class="container mt-5 pt-4">
                            <Switch<DashboardRoute> render={Self::switch} />
                        </div>
                    </main>
                </div>
            </div>
        }
    }
}

impl Dashboard {
    fn render_nav_item(&self, ctx: &Context<Self>, route: DashboardRoute, label: &str) -> Html {
        let is_active = self.active_route == route;
        let link_class = if is_active {
            "nav-link active bg-info text-dark fw-bold"
        } else {
            "nav-link text-dark"
        };
        let route_clone = route.clone();

        let onclick = ctx.link().callback(move |_| Msg::SetActiveRoute(route_clone.clone()));
        html! {
            <li class="nav-item" onclick={onclick}>
                <Link<DashboardRoute>
                    classes={link_class}
                    to={route}
                >
                    { label }
                </Link<DashboardRoute>>
            </li>
        }
    }

    fn switch(routes: DashboardRoute) -> Html {
        match routes {
            DashboardRoute::Home => html! { <h1>{ "欢迎来到仪表盘" }</h1> },
            DashboardRoute::Users => html! { <UserList /> },
            DashboardRoute::Roles => html! { <RoleList /> },
        }
    }
}