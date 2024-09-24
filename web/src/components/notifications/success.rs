use yew::prelude::*;
use super::AlertProps;

#[function_component(SuccessAlert)]
pub fn success_alert(props: &AlertProps) -> Html {
    html! {
        <div class="absolute top-20	left-1/2 items-center justify-center alert z-30 w-auto transform -translate-x-1/2 bg-green-300">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span>{props.message.clone()}</span>
        </div>
    }
}
