use web_sys::console;
use yew::prelude::*;
use crate::apis::user::{get_user_list, Msg, QueryUserParams, UserListResp};

pub struct User {
    user_list: Option<UserListResp>,
}


impl Component for User {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        _ctx.link().send_future(get_user_list(QueryUserParams::new()));
        Self {
            user_list: None,
        }
    }


    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UserList(resp) => {
                self.user_list = Some(resp);
                true
            }
            Msg::Error(msg) => {
                console::log_2(&"get user list error".into(), &msg.into());
                false
            }
        }
    }


    fn view(&self, _ctx: &Context<Self>) -> Html {
        let users = match &self.user_list {
            Some(resp) => &resp.items,
            None => &vec![],
        };
        html! {
        <div>
            <h2>{"用户管理"}</h2>
            <table class="table table-striped">
                <thead>
                    <tr>
                        <th>{"ID"}</th>
                        <th>{"用户名"}</th>
                        <th>{"邮箱"}</th>
                        <th>{"创建时间"}</th>
                        <th>{"操作"}</th>
                    </tr>
                </thead>
                <tbody>
                    {users.iter().map(|user| html! {
                        <tr key={user.id}>
                            <td>{user.id}</td>
                            <td>{&user.username}</td>
                            <td>{&user.email}</td>
                            <td>{&user.create_time.format("%Y-%m-%d %H:%M:%S").to_string()}</td>
                            <td>
                                <button class="btn btn-sm btn-info me-2">{"编辑"}</button>
                                <button class="btn btn-sm btn-danger">{"删除"}</button>
                            </td>
                        </tr>
                    }).collect::<Html>()}
                </tbody>
            </table>
        </div>
    }
    }
}