mod error;
mod info;
mod success;
mod warn;
mod alert;
use yew::prelude::*;
use error::ErrorAlert;
use info::InfoAlert;
use success::SuccessAlert;
use warn::WarnAlert;
use alert::Alert;
use crate::features::common::notification_slice::{NotificationState,NofiticationType}; 
use yewdux::prelude::*; 
use gloo::timers::callback::Timeout;


#[derive(Properties, Clone, PartialEq)]
pub struct AlertProps {
    pub message: String,
}




#[function_component(Nofitication)]
pub fn notification() -> Html{
    let (state, dispatch) = use_store::<NotificationState>();
        {
            let dispatch = dispatch.clone();
            let state = state.clone();
            use_effect(
                move || {
                    if state.alert_type != NofiticationType::None {
                        let timeout = Timeout::new(3000, move || {
                            dispatch.reduce_mut(|state| state.alert_type = NofiticationType::None);
                        });
                        timeout.forget();
                    }
                    || {}
                },
            );
        }
    match state.alert_type {
        NofiticationType::Error => html!{<ErrorAlert message={state.message.clone()}/>},
        NofiticationType::Info => html!{<InfoAlert message={state.message.clone()}/>},
        NofiticationType::Success => html!{<SuccessAlert message={state.message.clone()}/>},
        NofiticationType::Warn => html!{<WarnAlert message={state.message.clone()}/>},
        NofiticationType::Alert => html!{<Alert message={state.message.clone()}/>},
        _ => html!{}
    }
}


