
use yew::prelude::*;
use yewdux::prelude::*;
use crate::features::common::modal_slice::{ModalState,ModalType, ModalSize};
use crate::features::common::components::confirmation_modal_body::ConfirmationModalBody;


#[function_component(ModalLayout)]
pub fn modal_layout() -> Html {
    let (state, modal_dispatch) = use_store::<ModalState>();
    let close_modal: Callback<MouseEvent> = modal_dispatch.reduce_mut_callback(|state| state.is_open = false);
    let modal_body  = match(state.modal_type){
        ModalType::CONFIRMATION => {
            html!{
                <ConfirmationModalBody/>
            }
        },
        ModalType::DEFAULT=>{
            html!{}
        }
    };
    html! {
        <div class={format!("modal {}", if state.is_open { "modal-open" } else { "" })}>
            <div class={format!("modal-box {}", if state.size == ModalSize::Large {"max-w-5xl"}else{""})}>
                <button className="btn btn-sm btn-circle absolute right-2 top-2" onclick={close_modal}>{"\u{2715}"}</button>
                <h3 className="font-semibold text-2xl pb-6 text-center">{state.title.clone()}</h3>
                <div>
                 {modal_body}
                </div>
            
            </div>
        </div>
    }
}