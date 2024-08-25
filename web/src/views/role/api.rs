use std::collections::HashSet;

use crate::apis::role::{get_role_apis_group, role_api_update};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::*;

#[derive(Properties, PartialEq)]
pub struct ApiProps {
    pub role_id: u32,
}

#[function_component(Api)]
pub fn component(api_prop: &ApiProps) -> Html {
    let role_api_group = use_state(|| vec![]);
    let selected_apis = use_state(|| HashSet::new());

    let on_check = {
        let selected_apis = selected_apis.clone();
        Callback::from(move |(api_id, checked): (u32, bool)| {
            let mut selected_apis_set = (*selected_apis).clone();
            if checked {
                selected_apis_set.insert(api_id);
            } else {
                selected_apis_set.remove(&api_id);
            }
            selected_apis.set(selected_apis_set);
        })
    };
    let role_api_data = {
        let role_id = api_prop.role_id;
        use_async(async move { get_role_apis_group(role_id).await })
    };
    let update_role_api = {
        let selected_apis = selected_apis.clone();
        let role_id = api_prop.role_id;
        let selected_apis = selected_apis.iter().map(|id| *id).collect();
        use_async(async move { role_api_update(role_id, selected_apis).await })
    };

    {
        let role_id = api_prop.role_id;
        let role_api_data = role_api_data.clone();
        use_effect_with(role_id, move |_| {
            role_api_data.run();
        });
    }
    let on_save = {
        Callback::from(move |_| {
            update_role_api.run();
        })
    };
    {
        let role_api_group = role_api_group.clone();
        let selected_apis = selected_apis.clone();
        use_effect_with(role_api_data, move |role_api_data| {
            if let Some(data) = &role_api_data.data {
                role_api_group.set(data.clone());
                let all_selected_apis: HashSet<u32> = data
                    .iter()
                    .flat_map(|group| group.items.iter().filter(|api| api.has).map(|api| api.id))
                    .collect();

                selected_apis.set(all_selected_apis);
            }
        });
    }

    html! {
        <div class="container mt-5">
            <p1>{"API组"}</p1>
            {(*role_api_group).iter().cloned().map(|data| html!{

                <div class="card-body">
                    <div class="form-check">
                        <input class="form-check-input" type="checkbox" id="groupCheck" checked=true />
                        <label class="form-check-label" for="groupCheck">
                            { &data.api_group }
                        </label>
                    </div>
                    <div class="ms-4">
                        {
                            for data.items.iter().cloned().enumerate().map(|(_,api)| {
                                let is_checked = selected_apis.contains(&api.id);
                                html! {
                                    <div class="form-check">
                                        <input
                                            class="form-check-input"
                                            type="checkbox"
                                            checked={is_checked}
                                            onchange={
                                                let on_check = on_check.clone();
                                                Callback::from(move |e: Event| {
                                                    let input: HtmlInputElement = e.target_unchecked_into();
                                                    on_check.emit((api.id, input.checked()));
                                                })
                                            }
                                        />
                                        <span class="text-muted">{ &api.name }</span>
                                        <span class="text-muted ms-5">{ &api.path }</span>
                                    </div>
                                }
                            })
                        }
                    </div>
            </div>
            }).collect::<Html>()}
            <button type="button" class="mt-3 btn btn-primary" onclick={on_save} >{"保存"}</button>
        </div>
    }
}
