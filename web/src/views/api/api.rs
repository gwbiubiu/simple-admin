use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once};
use yewdux::use_store;
use crate::apis::api::{get_api_list, QueryApiParams};
use crate::components::error::{ErrorAction, ErrorState};
use crate::components::pagination::{PaginationState, Pagination, PaginationAction};


#[function_component(Api)]
pub fn component() -> Html {
    let (state, dispatch) = use_store::<PaginationState>();
    let (_, dispatch_err) = use_store::<ErrorState>();
    {
        let dispatch = dispatch.clone();
        use_effect_once(move || {
            dispatch.apply(PaginationAction::ResetPage);
            || {}
        });
    }
    let total_pages = use_state(|| 0);
    let name = use_state(|| None);
    let apis = use_state(|| Vec::new());
    let query_params = use_memo(
        (state.current_page, state.page_size, name.clone()),
        |(page, page_size, name)| {
            QueryApiParams {
                page: *page,
                page_size: *page_size,
                name: (**name).clone(),
            }
        },
    );
    
    let api_data = {
        let query_params = query_params.clone();
        use_async(async move { get_api_list((*query_params).clone()).await })
    };
    
    {
        let api_data = api_data.clone();
        use_effect_with(query_params.clone(), move |_| {
            api_data.run();
        });
    }
    
    
    {
        let apis = apis.clone();
        let total_pages = total_pages.clone();
        use_effect_with(api_data.clone(), move |api_data| {
            if let Some(data) = &api_data.data {
                let new_total_pages = (data.page.total as f64 / state.page_size as f64).ceil() as u32;
                apis.set(data.items.clone());
                total_pages.set(new_total_pages);
            } else {
                let err_msg = api_data.error.as_ref().map(|e| e.clone());
                dispatch_err.apply(ErrorAction::SetError(err_msg));
            }
        });
    }
    
    html! {
        <div>
            <h2>{"接口管理"}</h2>
            <table class="table table-striped">
                <thead>
                    <tr>
                        <th>{"ID"}</th>
                        <th>{"接口名称"}</th>
                        <th>{"创建时间"}</th>
                        <th>{"操作"}</th>
                    </tr>
                </thead>
                <tbody>
                    {(*apis).iter().map(|api| html! {
                        <tr key={api.id}>
                            <td>{api.id}</td>
                            <td>{&api.name}</td>
                            <td>{&api.create_time.format("%Y-%m-%d %H:%M:%S").to_string()}</td>
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