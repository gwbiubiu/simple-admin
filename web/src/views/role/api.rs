use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct CheckboxProps {
    label: &'static str,
    path: &'static str,
    checked: bool,
    on_toggle: Callback<()>,
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
pub fn app() -> Html {
    let checkboxes = use_state(|| vec![
        ("获取公告列表", "/info/getInfoList", true),
        ("根据ID获取公告", "/info/findInfo", true),
        ("更新公告", "/info/updateInfo", true),
        ("批量删除公告", "/info/deleteInfoByIds", true),
        ("删除公告", "/info/deleteInfo", true),
        ("新建公告", "/info/createInfo", true),
    ]);

    let toggle_checkbox = {
        let checkboxes = checkboxes.clone();
        Callback::from(move |index: usize| {
            checkboxes.set(
                checkboxes
                    .iter()
                    .enumerate()
                    .map(|(i, item)| {
                        if i == index {
                            (item.0, item.1, !item.2)
                        } else {
                            *item
                        }
                    })
                    .collect::<Vec<_>>(),
            );
        })
    };

    html! {
        <div class="container mt-5">
            <div class="card">
                <div class="card-header">
                    { "公告组" }
                </div>
                <div class="card-body">
                    <div class="form-check">
                        <input class="form-check-input" type="checkbox" id="groupCheck" checked=true />
                        <label class="form-check-label" for="groupCheck">
                            { "公告组" }
                        </label>
                    </div>
                    <div class="ms-4">
                        {
                            for checkboxes.iter().enumerate().map(|(index, (label, path, checked))| {
                                html! {
                                    <Checkbox
                                        label={label}
                                        path={path}
                                        checked={*checked}
                                        on_toggle={toggle_checkbox.reform(move |_| index)}
                                    />
                                }
                            })
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
