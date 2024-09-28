use yew::prelude::*;
use yewdux::prelude::*;

use super::super::modal_slice::ModalState;
#[function_component(ConfirmationModalBody)]
pub fn confirmation_modal_body() -> Html {
    let (state, modal_dispatch) = use_store::<ModalState>();

    let on_cancel_confirm = modal_dispatch.reduce_mut_callback(|state| {
        state.is_open = false;
        state.modal_type = crate::features::common::modal_slice::ModalType::CONFIRMATION;
    });

    html! {
        <>
            <p class="text-xl mt-8 text-center">
                {"Are you sure you want to delete this item?"}
            </p>
            <div class="modal-action mt-12">
                <button class="btn btn-outline" onclick={on_cancel_confirm}>{"Cancel"}</button>
                <button class="btn btn-primary w-36" onclick={state.callback.clone()}>{"Yes"}</button>
            </div>
        </>
    }
}