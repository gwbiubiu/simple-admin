use web_sys::console;
use yew::prelude::*;
use yew_hooks::*;
use yewdux::use_store;
use crate::apis::role::{get_role_apis_group, RoleApiCategory};
use crate::components::error::{ErrorAction, ErrorState};


#[derive(Properties, PartialEq)]
struct CheckboxProps {
    label: &'static str,
    path: &'static str,
    checked: bool,
    on_toggle: Callback<()>,
}

#[derive(Properties, PartialEq)]
pub struct ApiProps {
    pub role_id: u32,
}

#[function_component(Checkbox)]
fn checkbox(CheckboxProps { label, path, checked, on_toggle }: &CheckboxProps) -> Html {
    let label_id = format!("checkbox-{}", label);
    let checked_state = *checked;

    let onclick = {
        let on_toggle = on_toggle.clone();
        Callback::from(move |_| {
            on_toggle.emit(());
        })
    };

    html! {
        <div class="form-check">
            <input
                class="form-check-input"
                type="checkbox"
                id={label_id.clone()}
                checked={checked_state}
                {onclick}
            />
            <label class="form-check-label" for={label_id.clone()}>
                { label }
            </label>
            <span class="text-muted">{ path }</span>
        </div>
    }
}

#[function_component(Api)]
pub fn component(api_prop: &ApiProps) -> Html {
    let role_api_group = use_state(|| vec![]);
    
    let role_api_data = {
        let role_id = api_prop.role_id;
        use_async(async move { get_role_apis_group(role_id).await })
    };    
    {
        let role_id = api_prop.role_id;
        let role_api_data = role_api_data.clone();   
        use_effect_with(role_id, move |_| {
            role_api_data.run();
        });
    }
    {
        let role_api_group = role_api_group.clone();
        use_effect_with(role_api_data, move |role_api_data| {
            if let Some(data) = &role_api_data.data {
                role_api_group.set(data.clone());
            }
        });    
    }

   
    html! {
        <div class="container mt-5">
            <p1>{"API组"}</p1>
            {(*role_api_group).iter().map(|api_group| html!{
                <div>
                <p1>{&api_group.api_group}</p1>
                </div>
            }).collect::<Html>()
        }
        </div>
    }
}
