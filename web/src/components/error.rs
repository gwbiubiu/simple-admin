use std::rc::Rc;
use yew::prelude::*;
use gloo::timers::callback::Timeout;
use yewdux::prelude::*;


#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct ErrorState {
    message: Option<String>,
}

pub enum ErrorAction {
    SetError(String),
    ClearError,
}

impl Reducer<ErrorState> for ErrorAction {
    fn apply(self, mut state: Rc<ErrorState>) -> Rc<ErrorState> {
        let mut state = Rc::make_mut(&mut state);
        match self {
            ErrorAction::SetError(message) => {
                state.message = Some(message);
            }
            ErrorAction::ClearError => {
                state.message = None;
            }
        }
        Rc::new(state.clone())
    }
}


#[function_component(ErrorComponent)]
pub fn error_component() -> Html {
    let (state, dispatch) = use_store::<ErrorState>();

    {
        let dispatch = dispatch.clone();
        let state = state.clone();
        use_effect(
            move || {
                if state.message.is_some() {
                    let timeout = Timeout::new(5000, move || {
                        dispatch.apply(ErrorAction::ClearError);
                    });
                    timeout.forget();
                }
                || {}
            },
        );  
    }
    

    let close = {
        let dispatch = dispatch.clone();
        Callback::from(move |_| dispatch.apply(ErrorAction::ClearError))
    };

    if let Some(error_message) = &state.message {
        html! {
            <div class="container mt-3">
                <div class="alert alert-danger alert-dismissible fade show" role="alert">
                    <strong>{"错误："}</strong> {error_message}
                    <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close" onclick={close}></button>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}