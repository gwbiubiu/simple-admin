use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store, Default)]
pub struct ModalState {
    pub is_open: bool,
    pub size: ModalSize,
}

#[derive(Clone, PartialEq,Default)]
pub enum ModalSize {
    #[default]
    Small,
    Large,
}

