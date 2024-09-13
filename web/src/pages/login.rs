use yew::prelude::*;
use crate::features::user::login;

#[function_component(Login)]
pub fn external_page()-> Html{
    html!{
        <div>
         <login::Login/>
        </div> 
    }
}