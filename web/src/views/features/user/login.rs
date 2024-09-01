use yew::prelude::*;
use super::landing_intro::LandingIntro;



#[function_component(Login)]
pub fn login() -> Html {
    html!{
        <div class="min-h-screen bg-base-100 flex items-center">
        <div class="card mx-auto w-full max-w-5xl shadow-xl">
            <div class="grid  md:grid-cols-2 grid-cols-1  bg-base-100 rounded-xl">
                <div>
                    <LandingIntro/>
                </div>
                <div class="py-24 px-10">
                    <p>{"this is login page"}</p>
                </div>
            </div>
        </div>

        </div>
    }
}