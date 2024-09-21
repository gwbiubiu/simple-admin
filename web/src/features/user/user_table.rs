use crate::apis::user::{get_user_list, QueryUserParams};
use crate::components::cards::title_card::TitleCard;
use yew::prelude::*;
use yew_hooks::*;

#[function_component(TopSideButtons)]
fn top_side_buttons() -> Html {
    html! {
        <div class="inline-block float-right">
            <button class="btn px-6 btn-sm normal-case btn-primary">{"Add User"}</button>
        </div>
    }
}

#[function_component(UserManagement)]
pub fn user_management() -> Html {
    let users = use_state(|| Vec::new());
    let user_data = {
        let query = QueryUserParams::new();
        use_async(async move { get_user_list(query).await })
    };

    {
        let user_data = user_data.clone();
        use_effect_once(move || {
            user_data.run();
            || {}
        });
    } 

    

    {
        let users = users.clone();
        use_effect_with(user_data.clone(), move |user_data| {
            if let Some(data) = &user_data.data {
                users.set(data.items.clone());
            }
        });
    }
    html! {
        <>
            <TitleCard title="User Management" top_margin="mt-2" top_side_buttons={Some(html!{<TopSideButtons/>})}>
                <div class="overflow-x-auto w-full">
                    <table class="table w-full">
                        <thead>
                            <tr>
                                <th>{"Name"}</th>
                                <th>{"Email Id"}</th>
                                <th>{"Joined On"}</th>
                                <th>{"Role"}</th>
                                <th>{"Last Active"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {(*users).iter().map(|user| html! {
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
            </TitleCard>

        </>
    }
}
