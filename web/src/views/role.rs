use web_sys::console;
use yew::prelude::*;
use yew_hooks::use_async;
use yewdux::use_store;
use crate::apis::role::{get_role_list, QueryRoleParams};
use crate::components::pagination::{PaginationState, Pagination, PaginationAction};


#[function_component(Role)]
pub fn component() -> Html {
    let (state, dispatch) = use_store::<PaginationState>();
    dispatch.apply(PaginationAction::ResetPage);
    let total_pages = use_state(|| 0);
    let name = use_state(|| None);
    let roles = use_state(|| Vec::new());

    let query_params = use_memo(
        (state.current_page, state.page_size, name.clone()),
        |(page, page_size, name)| {
            QueryRoleParams {
                page: *page,
                page_size: *page_size,
                name: (**name).clone(),
            }
        },
    );

    let user_data = {
        let query_params = query_params.clone();
        use_async(async move { get_role_list((*query_params).clone()).await })
    };

    {
        let user_data = user_data.clone();
        use_effect_with(query_params.clone(), move |_| {
            user_data.run();
        });
    }
    {
        let users = roles.clone();
        let total_pages = total_pages.clone();
        use_effect_with(user_data.clone(), move |user_data| {
            if let Some(data) = &user_data.data {
                let new_total_pages = (data.page.total as f64 / state.page_size as f64).ceil() as u32;
                users.set(data.items.clone());
                total_pages.set(new_total_pages);
                let json_string = serde_json::to_string_pretty(data).unwrap_or_else(|_| "Failed to serialize data".to_string());
                //console::log_1(&json_string.into());
            }
        });
    }

    html! {
        <div>
            <h2>{"角色管理"}</h2>
            <table class="table table-striped">
                <thead>
                    <tr>
                        <th>{"ID"}</th>
                        <th>{"用户名"}</th>
                        <th>{"创建时间"}</th>
                        <th>{"操作"}</th>
                    </tr>
                </thead>
                <tbody>
                    {(*roles).iter().map(|role| html! {
                        <tr key={role.id}>
                            <td>{role.id}</td>
                            <td>{&role.name}</td>
                            <td>{&role.create_time.format("%Y-%m-%d %H:%M:%S").to_string()}</td>
                            <td>
                                <button class="btn btn-sm btn-info me-2">{"编辑"}</button>
                                <button class="btn btn-sm btn-danger">{"删除"}</button>
                            </td>
                        </tr>
                    }).collect::<Html>()}
                </tbody>
            </table>
            <Pagination
                total_pages={*total_pages}
            />
        </div>
    }
}