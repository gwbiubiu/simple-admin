use yew::prelude::*;
use yew_icons::{Icon, IconId};



#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="hero h-4/5 bg-base-200">
            <div class="hero-content text-accent text-center">
                <div class="max-w-md">
                <Icon icon_id={IconId::HeroiconsOutlineFaceFrown} class="h-48 w-48 inline-block" />
                <h1 class="text-5xl font-bold" >{"404 - Not Found"}</h1>
                </div>
            </div>
        </div>
    }
}