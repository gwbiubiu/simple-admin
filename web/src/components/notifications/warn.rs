use yew::prelude::*;
use super::AlertProps;


#[function_component(WarnAlert)]
pub fn warn_alert(props: &AlertProps) -> Html {
    html! {
        <div class="absolute top-20	left-1/2 items-center justify-center alert alert-warn z-30 w-auto">
            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 shrink-0 stroke-current" fill="none" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
            <span>{props.message.clone()}</span>
        </div>
    }
}
