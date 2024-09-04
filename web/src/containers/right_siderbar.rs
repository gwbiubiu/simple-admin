use yew::prelude::*;
#[derive(Properties, Clone, PartialEq)]
pub struct RightSidebarProps {
    #[prop_or_default]
   pub  is_open: bool,
}

#[function_component(RightSidebar)]
pub fn right_sidebar(props: &RightSidebarProps) -> Html {
    let drawer_classes = if props.is_open {
        "transition-opacity opacity-100 duration-500 translate-x-0"
    } else {
        "transition-all delay-500 opacity-0 translate-x-full"
    };
    html! {
        <>
            <div class= {format!("fixed overflow-hidden z-20 bg-gray-900 bg-opacity-25 inset-0 transform ease-in-out  {}", drawer_classes)}>
            <p>{"Admin Dashboard Starter Kit"}</p>
            </div>

        </>
    }
}
