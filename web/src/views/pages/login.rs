use yew::prelude::*;
use crate::views::features::user;



#[function_component(Login)]
pub fn external_page()-> Html{
    html!{
        <div>
         <user::Login/>
        </div> 
    }
}