use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <div class="navbar sticky top-0 bg-base-100 z-10 shadow-md">
            <div class="flex-1">
                <label for="left-sidebar-drawer" class="btn btn-primary drawer-button lg:hidden">
                    <i class="fas fa-bars"></i>
                </label>
                <h1 class="text-2xl font-semibold ml-2">{"Header"}</h1>
            </div>
        </div>
    }
}