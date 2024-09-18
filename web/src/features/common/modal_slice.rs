use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store)]
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


impl Default for ModalState {
    fn default() -> Self {
        Self {
            is_open: false,
            size: ModalSize::Small,
        }
    }
}