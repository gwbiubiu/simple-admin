
use yewdux::prelude::*;
use yew::prelude::*;
use std::rc::Rc;
#[derive(Default,Clone, PartialEq)]
pub enum ModalType {
    CONFIRMATION,
    #[default]
    DEFAULT
}

#[derive(Clone, PartialEq, Store, Default)]
pub struct ModalState {
    pub is_open: bool,
    pub size: ModalSize,
    pub title: String,
    pub modal_type: ModalType,
    pub callback: Option<Rc<Callback<MouseEvent>>>,
}

#[derive(Clone, PartialEq,Default)]
pub enum ModalSize {
    #[default]
    Small,
    Large,
}

