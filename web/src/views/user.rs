use yew::prelude::*;
use crate::apis::user::{get_user_list, QueryUserParams};
use yew_hooks::prelude::*;
use crate::components::error::{ErrorState, ErrorAction};
use yewdux::prelude::*;

#[function_component(User)]
pub fn component() -> Html {
    let (_, dispatch) = use_store::<ErrorState>();

    let user_data = use_async(async move { get_user_list(QueryUserParams::new()).await });
    {
        let user_data = user_data.clone();
        use_effect_with( (), move |_| {
            user_data.run();
        });
    }

    let mut users = vec![];
    if let Some(data) = &user_data.data {
        users = data.items.clone();
    } else {
        if let Some(err) = &user_data.error {
            dispatch.apply(ErrorAction::SetError(err.to_string()));
        }
    }

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
