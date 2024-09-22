use std::ops::Div;

use serde::Serialize;
use web_sys::HtmlSelectElement;
use yew::prelude::*;

#[derive(Serialize)]
pub struct PageInfo {
    pub page: u32,
    pub page_size: u32,
}

#[derive(Properties, PartialEq, Clone)]
pub struct PaginationProps {
    pub page_change: Callback<PageInfo>,
    pub total: u32,
}
#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let current_page = use_state(|| 1);
    let page_size = use_state(|| 10);
    let page_sizes = vec![10, 20, 50];
    let total_pages = props.total.div(*page_size);

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
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<HtmlSelectElement>();
            let input_page_size = input.unwrap().value().parse::<u32>().unwrap_or(10);
            page_size.set(input_page_size);
            props.page_change.emit(PageInfo {
                page: *current_page,
                page_size: input_page_size,
            });
        })
    };
    let create_page_buttons = || {
        let mut pages = vec![];
        let max_display_pages = 5; 

        let start_page = if *current_page > 3 {
            *current_page - 2
        } else {
            1
        };

        let end_page = if start_page + max_display_pages - 1 > total_pages {
            total_pages
        } else {
            start_page + max_display_pages - 1
        };

        if start_page > 1 {
            pages.push(html! {
                <>
                    <button class="join-item btn" onclick={on_page_change.reform(|_| 1)}>{"1"}</button>
                    <span class="join-item btn btn-disabled">{"..."}</span>
                </>
            });
        }

        for page in start_page..=end_page {
            pages.push(html! {
                <button class={if page == *current_page { "join-item btn btn-primary" } else { "join-item btn" }}
                        onclick={on_page_change.reform(move |_| page)}>{ page }</button>
            });
        }

        if end_page < total_pages {
            pages.push(html! {
                <>
                    <span class="join-item btn btn-disabled">{"..."}</span>
                    <button class="join-item btn" onclick={on_page_change.reform(move |_| total_pages)}>{ total_pages }</button>
                </>
            });
        }

        pages
    };

    

    html! {
        <div class="join float-right">
            <div>
                <button class="join-item btn" disabled={*current_page == 1} onclick={
                    let current_page = current_page.clone();
                    Callback::from(move |_|{
                        current_page.set(*current_page-1);
                    })
                }>{"«"}</button>
                { for create_page_buttons() }
                <button class="join-item btn" disabled={*current_page == total_pages} onclick={
                    let current_page = current_page.clone();
                    Callback::from(move |_|{
                        current_page.set(*current_page+1);
                    })
                }>{"»"}</button>
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
