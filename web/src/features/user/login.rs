use gloo::console::log;
use yew::prelude::*;
use yew_hooks::use_async;
use super::landing_intro::LandingIntro;
use crate::components::input::{InputText,TextType};
use crate::components::typography::ErrorText;
use crate::apis::login::{login as user_login, LoginParam};
use gloo::utils::document;
use web_sys::{HtmlInputElement,window};
use wasm_bindgen::JsCast;



#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    // Async hook for login action
    let login_data = use_async(async move{
        let username_input = document()
            .get_element_by_id("username_input")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let password_input = document()
            .get_element_by_id("password_input")
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        
        let param = LoginParam {
            username: username_input.value(),
            password: password_input.value(),
        };
        user_login(param).await
    });

    let on_login_click = {
        let login_data = login_data.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            login_data.run();
        })
    };

    if let Some(_) = &login_data.data {
        window().unwrap().location().set_href("/app/dashboard").unwrap();
    }


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
                            <InputText id={"username_input"} text_type={TextType::Text} container_style="mt-4" label_title="用户名" value={(*username).clone()}/>
                            <InputText id={"password_input"} text_type={TextType::Password} container_style="mt-4" label_title="密码" value={(*password).clone()}/>
                        </div>
                        <div class="text-right text-primary"><span class="text-sm inline-block hover:text-primary hover:underline hover:cursor-pointer transtion duration-200">{"Forgot Password?"}</span></div>
                        <ErrorText style_class= "mt-8" error_message = ""/> 
                        <button type="submit" class="btn mt-2 w-full btn-primary" onclick={on_login_click}>{"Login"}</button>
                    </form>
                </div>
            </div>
        </div>

        </div>
    }
}