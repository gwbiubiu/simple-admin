use yew::prelude::*;

pub struct UserList {}


struct User {
    id: u32,
    username: String,
    email: String,
    register_date: String,
}


impl Component for UserList {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self{}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let users = vec![
            User { id: 1, username: "张三".to_string(), email: "zhangsan@example.com".to_string(), register_date: "2023-01-01".to_string() },
            User { id: 2, username: "李四".to_string(), email: "lisi@example.com".to_string(), register_date: "2023-02-15".to_string() },
        ];
        html! {
        <div>
            <h2>{"用户管理"}</h2>
            <table class="table table-striped">
                <thead>
                    <tr>
                        <th>{"ID"}</th>
                        <th>{"用户名"}</th>
                        <th>{"邮箱"}</th>
                        <th>{"注册日期"}</th>
                        <th>{"操作"}</th>
                    </tr>
                </thead>
                <tbody>
                    {users.iter().map(|user| html! {
                        <tr key={user.id}>
                            <td>{user.id}</td>
                            <td>{&user.username}</td>
                            <td>{&user.email}</td>
                            <td>{&user.register_date}</td>
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