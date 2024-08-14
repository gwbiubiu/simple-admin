use yew::prelude::*;
use web_sys::{console, window};
use crate::apis::login::{login, Msg};

pub struct Login {
    username: String,
    password: String,
    remember_me: bool,
}


impl Component for Login {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let storage = window().unwrap().local_storage().unwrap().unwrap();
        let saved_username = storage.get_item("remembered_username").unwrap().unwrap_or_default();
        let tag = !saved_username.is_empty();
        Self {
            username: saved_username,
            password: String::new(),
            remember_me: tag,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateUsername(username) => {
                self.username = username;
                console::log_1(&"Username updated".into());
                true
            }
            Msg::UpdatePassword(password) => {
                self.password = password;
                true
            }
            Msg::ToggleRememberMe => {
                self.remember_me = !self.remember_me;
                true
            }
            Msg::Submit => {
                // 处理登录逻辑
                console::log_2(&"Username:".into(), &self.username.clone().into());
                console::log_2(&"Password:".into(), &self.password.clone().into());
                let storage = window().unwrap().local_storage().unwrap().unwrap();
                if self.remember_me {
                    storage.set_item("remembered_username", &self.username).unwrap();
                } else {
                    storage.remove_item("remembered_username").unwrap();
                }
                _ctx.link().send_future(login(self.username.clone(), self.password.clone()));
                true
            }
            Msg::LoginSuccess(msg) => {
                console::log_1(&"Login success".into());
                console::log_2(&"Token:".into(), &msg.into());
                false
            }
            Msg::LoginFailed(msg) => {
                console::log_1(&"Login failed".into());
                console::log_2(&"Message:".into(), &msg.into());
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onsubmit = ctx.link().callback(|e: SubmitEvent| {
            e.prevent_default();
            Msg::Submit
        });


        html! {
            <div class="container">
                <div class="row justify-content-center mt-5">
                    <div class="col-md-4">
                        <div class="card">
                            <div class="card-header">
                                <h3 class="text-center">{"登录"}</h3>
                            </div>
                            <div class="card-body">
                                <form {onsubmit}>
                                    <div class="mb-3">
                                        <label for="username" class="form-label">{"用户名"}</label>
                                        <input
                                            type="text"
                                            class="form-control"
                                            id="username"
                                            placeholder="请输入用户名"
                                            value={self.username.clone()}
                                            onchange={ctx.link().callback(|e: Event| Msg::UpdateUsername(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                                        />
                                    </div>
                                    <div class="mb-3">
                                        <label for="password" class="form-label">{"密码"}</label>
                                        <input
                                            type="password"
                                            class="form-control"
                                            id="password"
                                            placeholder="请输入密码"
                                            value={self.password.clone()}
                                            onchange={ctx.link().callback(|e: Event| Msg::UpdatePassword(e.target_unchecked_into::<web_sys::HtmlInputElement>().value()))}
                                        />
                                    </div>
                                    <div class="mb-3 form-check">
                                        <input type="checkbox" class="form-check-input" id="rememberMe" checked={self.remember_me} onchange={ctx.link().callback(|_| Msg::ToggleRememberMe)}/>
                                        <label class="form-check-label" for="rememberMe">{"记住我"}</label>
                                    </div>
                                    <div class="d-grid">
                                        <button type="submit" class="btn btn-primary">{"登录"}</button>
                                    </div>
                                </form>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}