use yew::prelude::*;

use crate::element::base::ElInput;
pub struct Login {
    username: String,
    password: String,
}

pub enum Msg {}

impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            username: "".to_string(),
            password: "".to_string(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                // <ElInput value="abc"></ElInput>
                <table class="table">
                    <caption>{"基本的表格布局"}</caption>
                   <thead>
                      <tr>
                         <th>{"名称"}</th>
                         <th>{"城市"}</th>
                      </tr>
                   </thead>
                   <tbody>
                      <tr>
                         <td>{"Tanmay"}</td>
                         <td>{"Bangalore"}</td>
                      </tr>
                      <tr>
                         <td>{"Sachin"}</td>
                         <td>{"Mumbai"}</td>
                      </tr>
                   </tbody>
                </table>
            </div>
        }
    }
}