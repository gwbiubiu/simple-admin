use std::rc::Rc;
use yew::prelude::*;
use yewdux::prelude::*;
use web_sys::HtmlSelectElement;

#[derive(Properties, PartialEq, Store, Clone)]
pub struct PaginationState {
    pub current_page: u32,
    pub page_size: u32,
}

pub enum PaginationAction {
    ResetPage,

}

impl Default for PaginationState {
    fn default() -> Self {
        Self {
            current_page: 0,
            page_size: 10,
        }
    }
}


impl Reducer<PaginationState> for PaginationAction {
    fn apply(self, mut state: Rc<PaginationState>) -> Rc<PaginationState> {
        let state = Rc::make_mut(&mut state);
        match self {
            PaginationAction::ResetPage => {
                state.current_page = 0;
                state.page_size = 10;
            }
        };
        Rc::new(state.clone())
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct PaginationProps {
    pub total_pages: u32,
}


#[function_component(Pagination)]
pub fn pagination(props: &PaginationProps) -> Html {
    let (state, dispatch) = use_store::<PaginationState>();
    let on_page_sub = {
        dispatch.reduce_mut_callback(|state| {
            state.current_page = state.current_page.saturating_sub(1);
        })
    };

    let on_page_add = {
        dispatch.reduce_mut_callback(|state| {
            state.current_page = state.current_page.saturating_add(1);
        })
    };

    let on_page_change = {
        let dispatch = dispatch.clone();
        Callback::from(move |p: u32| {
            let dispatch = dispatch.clone();
            dispatch.reduce_mut(move |state| {
                state.current_page = p - 1;
            });
        })
    };

    let on_page_size_change = {
        let dispatch = dispatch.clone();
        Callback::from(move |event: Event| {
            let dispatch = dispatch.clone();
            dispatch.reduce_mut(|state| {
                if let Some(input) = event.target_dyn_into::<HtmlSelectElement>() {
                    if let Ok(value) = input.value().parse() {
                        state.page_size = value;
                        state.current_page = 0;  // 重置当前页
                    }
                }
            });
        })
    };

    html! {
        <div class="d-flex justify-content-end align-items-center mt-3">
            <nav aria-label="Page navigation example" class="me-3">
              <ul class="pagination pagination-sm mb-0">
                <li class={classes!("page-item", (state.current_page == 0).then(|| "disabled"))}>
                    <a class="page-link" href="#" onclick={on_page_sub}>
                        {"\u{00AB}"}
                    </a>
                </li>
                {(1..=props.total_pages).map(|p| html! {
                    <li class={classes!("page-item", (p-1 == state.current_page).then(|| "active"))}>
                        <a class="page-link" href="#" onclick={on_page_change.reform(move |_| p)}>
                            {p}
                        </a>
                    </li>
                }).collect::<Html>()}
                <li class={classes!("page-item", (state.current_page + 1 == props.total_pages).then(|| "disabled"))}>
                    <a class="page-link" href="#" onclick={on_page_add}>
                        {"\u{00BB}"}
                    </a>
                </li>
              </ul>
            </nav>
            <div class="d-flex align-items-center">
                <span class="me-2">{"每页显示："}</span>
                <select class="form-select form-select-sm" style="width: auto;" aria-label="Select page size" onchange = {on_page_size_change}>
                    <option selected={state.page_size == 5} value="5">{"5"}</option>
                    <option selected={state.page_size == 10} value="10">{"10"}</option>
                    <option selected={state.page_size == 20} value="20">{"20"}</option>
                </select>
                <span class="ms-2">{"条"}</span>
            </div>
        </div>
    }
}