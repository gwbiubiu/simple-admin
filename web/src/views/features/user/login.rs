use yew::prelude::*;
use super::landing_intro::LandingIntro;
use crate::components::input::InputText;



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
                    <h2 class="text-2xl font-semibold mb-2 text-center">{"Login"}</h2>
                    <form>
                        <div class="mb-4">
                            <InputText container_style="mt-4" label_title="username" />
                            <InputText container_style="mt-4" label_title="password"/>
                        </div>
                    </form>
                </div>
            </div>
        </div>

        </div>
    }
}