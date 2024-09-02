use yew::prelude::*;

#[function_component(TemplatePointers)]
pub fn template_pointers() -> Html {
    html! {
        <>
            <h1 class="text-2xl mt-8 font-bold">{"Admin Dashboard Starter Kit"}</h1>
            <p class="py-2 mt-4">{"✓ "}<span class="font-semibold">{"Light/dark"}</span>{" mode toggle"}</p>
            <p class="py-2">{"✓ "}<span class="font-semibold">{"Redux toolkit"}</span>{" and other utility libraries configured"}</p>
            <p class="py-2">{"✓ "}<span class="font-semibold">{"Calendar, Modal, Sidebar"}</span>{" components"}</p>
            <p class="py-2">{"✓ User-friendly "}<span class="font-semibold">{"documentation"}</span></p>
            <p class="py-2 mb-4">{"✓ "}<span class="font-semibold">{"Daisy UI"}</span>{" components, "}<span class="font-semibold">{"Tailwind CSS"}</span>{" support"}</p>
        </>
    }
}
