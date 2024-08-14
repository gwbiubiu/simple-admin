use yew::prelude::*;
use gloo::timers::callback::Timeout;

pub struct ErrorComponent {
    close_timeout: Option<Timeout>,
}


#[derive(Properties, PartialEq)]
pub struct ErrorComponentProps {
    pub error_message: String,
    pub show: bool,
    pub on_close: Callback<()>,
}

pub enum Msg {
    Close
}

impl Component for ErrorComponent {
    type Message = Msg;
    type Properties = ErrorComponentProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            close_timeout: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Close => {
                _ctx.props().on_close.emit(());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();

        if !props.show {
            return html! {};
        }

        html! {
            <div class="container mt-3">
                <div class="alert alert-danger alert-dismissible fade show" role="alert">
                    <strong>{"错误："}</strong> {&props.error_message}
                    <button type="button" class="btn-close" data-bs-dismiss="alert" aria-label="Close" onclick={ctx.props().on_close.reform(|_| ())}></button>
                </div>
            </div>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if ctx.props().show {
            let link = ctx.link().clone();
            self.close_timeout = Some(Timeout::new(5000, move || {
                link.send_message(Msg::Close);
            }));
        }
        true
    }

    fn destroy(&mut self, _ctx: &Context<Self>) {
        self.close_timeout = None;
    }
}