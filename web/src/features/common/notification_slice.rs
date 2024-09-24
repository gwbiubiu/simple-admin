use yewdux::prelude::*;
use yew::prelude::*;

#[derive(Clone, PartialEq, Default)]
pub enum NofiticationType {
    Error,
    Info,
    Success,
    Warn,
    Alert,
    #[default]
    None,
}

#[derive(Clone, PartialEq, Store, Default)]
pub struct NotificationState {
    pub alert_type: NofiticationType,
    pub message: String,
}

#[hook]
pub fn use_notification_success() -> impl Fn(String) {
    let (_, dispatch) = use_store::<NotificationState>();
    move |message: String| {
        dispatch.reduce_mut(|state| {
            state.alert_type = NofiticationType::Success;
            state.message = message;
        });
    }
}
