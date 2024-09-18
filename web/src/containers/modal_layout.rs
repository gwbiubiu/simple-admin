
use yew::prelude::*;
use yewdux::prelude::*;
use crate::features::common::modal_slice::{ModalState,ModalSize};


#[function_component(ModalLayout)]
pub fn modal_layout() -> Html {
    let (state, _) = use_store::<ModalState>();
    html! {
        <div class={format!("modal {}", if state.is_open { "modal-open" } else { "" })}>
            <div class={format!("modal-box {}", if state.size == ModalSize::Large {"max-w-5xl"}else{""})}>
                <button className="btn btn-sm btn-circle absolute right-2 top-2">{"\u{2715}"}</button>
                <h3 className="font-semibold text-2xl pb-6 text-center">{"this is modal"}</h3>
            </div>
        </div>
    }
}