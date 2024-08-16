use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct PaginationProps {
    pub current_page: u32,
    pub total_pages: u32,
    pub page_size: u32,
    pub on_page_change: Callback<u32>,
    pub on_page_size_change: Callback<Event>,
}

#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let PaginationProps {
        current_page,
        total_pages,
        page_size,
        on_page_change,
        on_page_size_change,
    } = props;

    let on_page_sub = {
        let on_page_change = on_page_change.clone();
        let current = *current_page;
        Callback::from(move |_| on_page_change.emit(current.saturating_sub(1)))
    };

    let on_page_add = {
        let on_page_change = on_page_change.clone();
        let current = *current_page;
        Callback::from(move |_| on_page_change.emit(current.saturating_add(1)))
    };

    html! {
        <div class="d-flex justify-content-end align-items-center mt-3">
            <nav aria-label="Page navigation example" class="me-3">
              <ul class="pagination pagination-sm mb-0">
                <li class={classes!("page-item", (*current_page == 0).then(|| "disabled"))}>
                    <a class="page-link" href="#" onclick={on_page_sub}>
                        {"\u{00AB}"}
                    </a>
                </li>
                {(1..=*total_pages).map(|p| html! {
                    <li class={classes!("page-item", (p-1 == *current_page).then(|| "active"))}>
                        <a class="page-link" href="#" onclick={on_page_change.reform(move |_| p)}>
                            {p}
                        </a>
                    </li>
                }).collect::<Html>()}
                <li class={classes!("page-item", (*current_page + 1 == *total_pages).then(|| "disabled"))}>
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
    }
}