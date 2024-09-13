use yew::prelude::*;


#[function_component(NotificationBodyRightDrawer)]
pub fn notification_body_right_drawer() -> Html {
    let notifications: Vec<String> = (0..15).map(|i| {
        if i % 2 == 0 {
            "Your sales has increased by 20% yester day".to_string()
        }else{
            "Total likes for instagram post - New launch this week,  has crossed 100k ".to_string()
        }
    }).collect();
    html! {
        <>
            {
                notifications.iter().enumerate().map(|(i, notification)| {
                html! {
                    <div key={i} class={format!("grid mt-3 card bg-base-200 rounded-box p-3 {}",
                            if i < 5 { "bg-blue-100" } else { "" })}>
                            {notification}
                    </div>
                    }}).collect::<Html>()
            }
            
        </>
    }
}