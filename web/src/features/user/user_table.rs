use crate::apis::user::{get_user_list, QueryUserParams};
use crate::components::cards::title_card::TitleCard;
use crate::components::page::{Pagination,PageInfo};
use yew::prelude::*;
use yew_hooks::*;
use gloo::console::log;


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
    let page = use_state(|| 1);
    let page_size = use_state(|| 10);
    let total = use_state(|| 0);
    let query_params = use_memo(((*page).clone(), (*page_size).clone()), |(page, page_size)|{
        QueryUserParams {
            page: *page-1,
            page_size: *page_size,
            username: None,
        }
    });
    let user_data = {
        let query = (*query_params).clone();
        use_async(async move { get_user_list(query).await })
    };

    {
        let user_data = user_data.clone();
        use_effect_with(query_params.clone(), move |_| {
            user_data.run();
        });
    } 


    {
        let users = users.clone();
        let total = total.clone();
        use_effect_with(user_data.clone(), move |user_data| {
            if let Some(data) = &user_data.data {
                users.set(data.items.clone());
                total.set(data.page.total);
            }
        });
    }
    
    let page_change = {
        Callback::from(move |page_info: PageInfo| {
            let page_info_string = serde_json::to_string(&page_info).unwrap();
            log!("call page_change: {:?}", page_info_string);
            page.set(page_info.page);
            page_size.set(page_info.page_size);
        })
    };
    
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
                <Pagination page_change={page_change} total={*total}/>
            </TitleCard>

        </>
    }
}
