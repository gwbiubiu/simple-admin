use crate::features::common::{right_drawer_slice::RightDrawerState, modal_slice::ModalState};
use crate::apis::login::logout;
use yew::prelude::*;
use yew_icons::{Icon, IconId};
use yewdux::prelude::*;
use yew_hooks::use_async;
use web_sys::window;
use std::rc::Rc;
#[function_component(Header)]
pub fn header() -> Html {
    let (_, dispatch) = use_store::<RightDrawerState>();
    let (_, modal_dispatch) = use_store::<ModalState>();

    let open_notification = {
        dispatch.reduce_mut_callback(|state| {
            state.is_open = true;
        })
    };


    let logout_api = use_async(async move{
        logout().await
    });

    let on_logout_click = {
        let logout_api = logout_api.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            logout_api.run();
        })
    };

    let on_logout_confirm = {
        modal_dispatch.reduce_mut_callback(move|state| {
        state.is_open = true;
        state.modal_type = crate::features::common::modal_slice::ModalType::CONFIRMATION;
        state.title = "Logout".to_string();
        state.callback = on_logout_click.clone();
    })};

    {
        let logout_api = logout_api.clone();
        use_effect_with(logout_api, move|logout_api| {
            
            if let Some(data) = &logout_api.data {
                if data.status == crate::apis::Status::SUCCESS {
                    window().unwrap().location().set_href("/login").unwrap();
                }
            }
        })
    }

    html! {
        <div class="navbar sticky top-0 bg-base-100 z-10 shadow-md">
            <div class="flex-1">
                <label for="left-sidebar-drawer" class="btn btn-primary drawer-button lg:hidden">
                    <Icon icon_id={IconId::BootstrapJustify} class="h-5 inline-block w-5"/>
                </label>
                <h1 class="text-2xl font-semibold ml-2">{"Header"}</h1>
            </div>
            <div class="flex-none">
                <label class="swap">
                    <input type="checkbox"/>
                    <Icon icon_id={IconId::BootstrapSun} class="fill-current w-6 h-6"/>
                    //<Icon icon_id={IconId::BootstrapMoon} class="h-5 inline-block w-5"/>
                </label>
                <button class="btn btn-ghost ml-4  btn-circle" onclick={open_notification}>
                    <div class="indicator">
                        <Icon icon_id={IconId::BootstrapBell} class="h-6 w-6"/>
                        <span class="indicator-item badge-sm badge-secondary rounded-full">{"3"}</span>
                    </div>
                </button>
                <div class="dropdown dropdown-end ml-4">
                    <label tabindex="0" class="btn btn-ghost btn-circle avatar">
                        <div class="=w-10 rounded-full">
                            <img src="https://avatars.githubusercontent.com/u/57338234?v=4" alt="avatar"/>
                        </div>
                    </label>
                    <ul tabindex="0" class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52">
                        <li class="">
                        <a>{"Profile Settings"} <span class="badge-sm badge badge-accent">{"New"}</span> </a>
                        </li>
                        <li class=""><a>{"Bill History"}</a></li>
                        <div class="divider mt-0 mb-0"></div>
                        <li><a onclick={on_logout_confirm}>{"Logout"}</a></li>
                    </ul>
                </div>
            </div>
        </div>
    }
}
