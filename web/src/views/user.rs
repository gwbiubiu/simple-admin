use yew::prelude::*;
use crate::apis::user::{get_user_list, QueryUserParams};
use yew_hooks::prelude::*;
use crate::components::error::{ErrorState, ErrorAction};
use yewdux::prelude::*;
use web_sys::{HtmlSelectElement, Event, console};
use web_sys::wasm_bindgen::JsValue;

#[function_component(User)]
pub fn component() -> Html {
    let (_, dispatch) = use_store::<ErrorState>();
    let page = use_state(|| 0);
    let total_pages = use_state(|| 0);
    let page_size = use_state(|| 10);
    let username = use_state(|| None);
    let users = use_state(|| Vec::new());


    let query_params = use_memo(
        (*page.clone(), *page_size.clone(), username.clone()),
        |(page, page_size, username)| {
            QueryUserParams {
                page: *page,
                page_size: *page_size,
                username: (**username).clone(),
            }
        },
    );


    let on_page_change = {
        let page = page.clone();
        Callback::from(move |new_page: u32| page.set(new_page - 1))
    };


    let on_page_sub = {
        let page = page.clone();
        let current = *page;
        Callback::from(move |_| page.set(current.saturating_sub(1)))
    };

    let on_page_add = {
        let page = page.clone();
        let current = *page;
        Callback::from(move |_| page.set(current.saturating_add(1)))
    };

    let user_data = {
        let query_params = query_params.clone();
        use_async(async move { get_user_list((*query_params).clone()).await })
    };

    {
        let user_data = user_data.clone();
        use_effect_with(query_params.clone(), move |_| {
            user_data.run();
        });
    }


    {
        let page_size = page_size.clone();
        let users = users.clone();
        let total_pages = total_pages.clone();
        use_effect_with(user_data.clone(), move |user_data| {
            if let Some(data) = &user_data.data {
                let new_total_pages = (data.page.total as f64 / *page_size as f64).ceil() as u32;
                users.set(data.items.clone());
                total_pages.set(new_total_pages);
                let json_string = serde_json::to_string_pretty(data).unwrap_or_else(|_| "Failed to serialize data".to_string());
                console::log_1(&json_string.into());
            }
        });
    }


    let on_page_size_change = {
        let page_size = page_size.clone();
        let page = page.clone();
        Callback::from(move |e: Event| {
            let target: HtmlSelectElement = e.target_unchecked_into();
            let value = target.value().parse::<u32>().unwrap_or(10);
            page_size.set(value);
            page.set(0);
        })
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
                <div class="d-flex justify-content-end align-items-center mt-3">
                    <nav aria-label="Page navigation example" class="me-3">
                      <ul class="pagination pagination-sm mb-0">
                        <li class={classes!("page-item", (*page==0).then(|| "disabled"))}>
                            <a class="page-link" href="#" onclick={on_page_sub}>
                                {"\u{00AB}"}
                            </a>
                        </li>
                        {(1..=*total_pages).map(|p| html! {
                            <li class={classes!("page-item", (p-1 == *page).then(|| "active"))}>
                                <a class="page-link" href="#" onclick={on_page_change.reform(move |_| p)}>
                                    {p}
                                </a>
                            </li>
                        }).collect::<Html>()}
                        <li class={classes!("page-item", (*page + 1 == *total_pages).then(|| "disabled"))}>
                            <a class="page-link" href="#" onclick={on_page_add}>
                                {"\u{00BB}"}
                            </a>
                        </li>
                      </ul>
                    </nav>
                    <div class="d-flex align-items-center">
                        <span class="me-2">{"每页显示："}</span>
                        <select class="form-select form-select-sm" style="width: auto;" aria-label="Select page size" onchange={on_page_size_change}>
                            <option selected={*page_size == 5} value="5">{"5"}</option>
                            <option selected={*page_size == 10} value="10">{"10"}</option>
                            <option selected={*page_size == 20} value="20">{"20"}</option>
                        </select>
                        <span class="ms-2">{"条"}</span>
                    </div>
            </div>
        </div>
    }
}
