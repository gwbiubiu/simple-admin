use yew::prelude::*;

#[function_component(LandingIntro)]
pub fn landing_intro() -> Html {
    html! {
        <div class="hero min-h-full rounded-l-xl bg-base-200">
            <div class="hero-content py-12">
                <div class="max-w-md">
                    <h1 class="text-3xl text-center font-bold">
                        <img src="/logo192.png" class="w-12 inline-block mr-2 mask mask-circle" alt="dashwind-logo" />
                        {"DashWind"}
                    </h1>
                    <div class="text-center mt-12">
                        <img src="./intro.png" alt="Dashwind Admin Template" class="w-48 inline-block"/>
                    </div>
                </div>

            </div>
        </div>
    }
}
