use super::left_siderbar::LeftSidebar;
use super::page_content::PageContent;
use super::right_siderbar::RightSidebar;
use super::modal_layout::ModalLayout;
use crate::components::notifications::Nofitication;
use yew::prelude::*;

#[function_component(Layout)]
pub fn layout() -> Html {
    html! {
    <>
        <div class="drawer lg:drawer-open">
            <input id="left-sidebar-drawer" type="checkbox" class="drawer-toggle"/>
            <LeftSidebar/>
            <PageContent/>
        </div>
        <RightSidebar />
        <Nofitication/>
        <ModalLayout/>
        </>
    }
}
