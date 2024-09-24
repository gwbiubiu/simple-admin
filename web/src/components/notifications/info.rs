use yew::prelude::*;
use super::AlertProps;

#[function_component(InfoAlert)]
pub fn info_alert(props: &AlertProps) -> Html {
    html! {
        <div class="absolute top-20	left-1/2 items-center justify-center alert alert-info z-30 w-auto transform -translate-x-1/2 bg-blue-300">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span>{props.message.clone()}</span>
        </div>
    }
}
