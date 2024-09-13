use yew::prelude::*;



#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center h-full">
            <h1 class="text-4xl font-bold">{"404"}</h1>
            <p class="text-lg">{"Page not found"}</p>
        </div>
    }
}