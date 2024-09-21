use yew::prelude::*;

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

    let on_page_click = {
        let current_page = current_page.clone();
        let page_size = page_size.clone();
        let props = props.clone();
        Callback::from(move |page: u32| {
            current_page.set(page);
            props.page_change.emit(PageInfo {
                page: *current_page,
                page_size: *page_size,
            });
        })
    };

    html! {
        <div class="join float-right">
            <div>
                <button class="join-item btn">{"«"}</button>
                <button class="join-item btn" onclick={on_page_click.reform(|_|1)}>{"1"}</button>
                <button class="join-item btn" onclick={on_page_click.reform(|_|2)}>{"2"}</button>
                <button class="join-item btn btn-disabled">{"..."}</button>
                <button class="join-item btn">{"99"}</button>
                <button class="join-item btn">{"100"}</button>
                <button class="join-item btn">{"»"}</button>
            </div>
            <select class="ml-3 select select-bordered w-20 max-w-24">
                <option disabled={true} selected={true}>{"10"}</option>
                <option>{"20"}</option>
                <option>{"50"}</option>
            </select>
        </div>
    }
}