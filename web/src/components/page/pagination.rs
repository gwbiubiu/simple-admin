use serde::Serialize;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Serialize)]
pub struct PageInfo {
    pub page: u32,
    pub page_size: u32,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PaginationProps{
    pub page_change: Callback<PageInfo> 
}
#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let current_page = use_state(|| 0);
    let page_size = use_state(|| 10);
    let page_sizes = vec![10, 20, 50];
    let on_page_change = {
        let current_page = current_page.clone();
        let page_size = page_size.clone();
        let props = props.clone();
        Callback::from(move |page: u32| {
            current_page.set(page);
            props.page_change.emit(PageInfo {
                page: page,
                page_size: *page_size,
            });
        })
    };

    let on_page_size_change = {
        let current_page = current_page.clone();
        let page_size = page_size.clone();
        let props = props.clone();
        Callback::from(move |e: Event|{
            let input = e.target_dyn_into::<HtmlSelectElement>();
            let input_page_size = input.unwrap().value().parse::<u32>().unwrap_or(10);
            page_size.set(input_page_size);
            props.page_change.emit(PageInfo {
                page: *current_page,
                page_size: input_page_size,
            });
        })
    };



    html! {
        <div class="join float-right">
            <div>
                <button class="join-item btn" disabled={*current_page == 1}>{"«"}</button>
                <button class="join-item btn btn-primary" onclick={on_page_change.reform(|_|1)}>{"1"}</button>
                <button class="join-item btn" onclick={on_page_change.reform(|_|2)}>{"2"}</button>
                <button class="join-item btn btn-disabled">{"..."}</button>
                <button class="join-item btn">{"99"}</button>
                <button class="join-item btn">{"100"}</button>
                <button class="join-item btn">{"»"}</button>
            </div>
            <select class="ml-3 select select-bordered w-20 max-w-24" onchange={on_page_size_change}>
                {
                    for page_sizes.iter().map(|size|{
                        html!{
                            <option value={size.to_string()} selected={*size == *page_size} disabled={*size == *page_size}>{size}</option>
                        }
                    })
                }
            </select>
        </div>
    }
}