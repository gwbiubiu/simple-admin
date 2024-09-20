use yew::prelude::*;
use yew_router::prelude::*;
use crate::routers::AppRoutes;
use crate::features::user::components::TemplatePointers;
#[function_component(Welcome)]
pub fn welcome() -> Html {
    html! {
        <div class="hero h-4/5 bg-base-200">
            <div class="hero-content">
                <div class="max-w-md">
                    <TemplatePointers/>
                    <Link<AppRoutes>  to={AppRoutes::Other{ path: "dashboard".to_string() }}>
                        <button class="btn bg-base-100 btn-outline">{"Get Started"}</button>
                    </Link<AppRoutes>>
                </div>
            </div>
        </div>
    }
}
