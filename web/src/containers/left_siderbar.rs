use crate::routers::SIDERBAR_ROUTERS;
use yew::prelude::*;
use yew_icons::Icon;
use yew_router::prelude::*;

#[derive(Clone, PartialEq, Routable)]
enum AppRoutes {
    #[at("/app/welcome")]
    Welcome,
    #[at("/app/:path")]
    Other { path: String },
}

#[function_component(LeftSidebar)]
pub fn left_sidebar() -> Html {
    let location = use_location().unwrap();
    let sidebar_ref = use_node_ref();
    let close_sidebar = {
        let sidebar_ref = sidebar_ref.clone();
        Callback::from(move |_| {
            if let Some(sidebar) = sidebar_ref.cast::<web_sys::HtmlElement>() {
                sidebar.click();
            }
        })
    };

    html! {
        <div class= "drawer-side z-30">
            <label for="left-sidebar-drawer" class="drawer-overlay" ref={sidebar_ref}></label>
            <ul class="menu pt-2 w-80 bg-base-100 min-h-full text-base-content">
                <div class="flex items-center justify-between mb-2">
                    <li class="font-semibold text-xl">
                        <Link<AppRoutes> to={AppRoutes::Welcome}>
                            <img class="mask mask-squircle w-10" src="/logo192.png" alt="DashWind Logo"/>{"DashWind"}
                        </Link<AppRoutes>>
                    </li>
                    <button class="btn btn-ghost bg-base-300 btn-circle z-50 top-0 right-0 lg:hidden" onclick={close_sidebar.clone()}>
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            stroke="currentColor">
                            <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>
                    {
                        for SIDERBAR_ROUTERS.iter().map(|router| {
                                if router.sub_routers.is_some() {
                                    html!{}
                                }else{
                                    let is_active = location.path() == router.path;
                                    html!{
                                        <li>
                                            <Link<AppRoutes>
                                                to={AppRoutes::Other { path: router.path.to_string() }}
                                                classes={classes!(if is_active { "font-semibold bg-base-200 items-center" } else { "font-normal items-center" })}>
                                                <Icon icon_id={router.icon_id} />{router.name}
                                                if is_active {
                                                        <span class="absolute inset-y-0 left-0 w-1 rounded-tr-md rounded-br-md bg-primary" aria-hidden="true"></span>
                                                }
                                            </Link<AppRoutes>>
                                        </li>
                                    }
                                }
                        })
                    }
            </ul>
        </div>
    }
}
