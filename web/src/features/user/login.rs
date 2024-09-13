use yew::prelude::*;
use super::landing_intro::LandingIntro;
use crate::components::input::InputText;
use crate::components::typography::ErrorText;



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
                            <InputText container_style="mt-4" label_title="用户名" />
                            <InputText container_style="mt-4" label_title="密码"/>
                        </div>
                        //TODO: link forget password
                        <div class="text-right text-primary"><span class="text-sm inline-block hover:text-primary hover:underline hover:cursor-pointer transtion duration-200">{"Forgot Password?"}</span></div>
                        <ErrorText style_class= "mt-8" error_message = ""/> 
                        <button type="submit" class="btn mt-2 w-full btn-primary">{"Login"}</button>
                    </form>
                </div>
            </div>
        </div>

        </div>
    }
}