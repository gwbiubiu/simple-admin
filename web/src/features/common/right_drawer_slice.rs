use std::rc::Rc;
use yewdux::prelude::*;


#[derive(Default, Clone, PartialEq, Store)]
pub struct RightDrawerState{
    pub header: String,
    pub is_open: bool,
    pub body_type: String,
    pub extra_object: Rc<ExtraObject>,
}

#[derive(Clone, PartialEq,Default, Debug)]
struct ExtraObject{

}
