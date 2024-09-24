use yew::prelude::*;
use crate::features::user::login;
use crate::components::notifications::Nofitication;

#[function_component(Login)]
pub fn external_page()-> Html{
    html!{
        <div>
        <Nofitication/>
         <login::Login/>
        </div> 
    }
}