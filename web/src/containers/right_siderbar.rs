use yew::prelude::*;
use yew_icons::{Icon, IconId};
use crate::features::common::components::notification_body_right_drawer::NotificationBodyRightDrawer;
#[derive(Properties, Clone, PartialEq)]
pub struct RightSidebarProps {
    #[prop_or_default]
    pub is_open: bool,
}

#[function_component(RightSidebar)]
pub fn right_sidebar(props: &RightSidebarProps) -> Html {


    html! {
            <>
            <div class={format!("fixed overflow-hidden z-20 bg-gray-900 bg-opacity-25 inset-0 transform ease-in-out {}",
                if props.is_open {
                    "transition-opacity opacity-100 duration-500 translate-x-0"
                } else {
                    "transition-all delay-500 opacity-0 translate-x-full"
                }
            )}>                    
                <section class={format!("w-80 md:w-96 right-0 absolute bg-base-100 h-full shadow-xl delay-400 duration-500 ease-in-out transition-all transform {}",
                                if props.is_open {
                                    "translate-x-0"
                                } else {
                                    "translate-x-full"
                                }
                            )}>                       
                        <div class="relative  pb-5 flex flex-col  h-full">
                            //header
                            <div class="navbar flex pl-4 pr-4 shadow-md">
                                <button class="float-left btn btn-circle btn-outline btn-sm">
                                <Icon icon_id={IconId::OcticonsX16} class="h-5 w-5" />
                                </button>
                                <span class="ml-2 font-bold text-xl">{ "Notifications" }</span>
                            </div>

                            <div class="overflow-y-scroll pl-4 pr-4 flex flex-col w-full">
                                <div className="flex flex-col w-full">
                                    <NotificationBodyRightDrawer/>
                                </div>
                            </div>
                        </div>
                    </section>
                    <section class="w-screen h-full cursor-pointer"></section>
                </div>

            </>
        }
}
