use yew::{function_component, Html, html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="container">
            <div class="row">
                <div class="col-md-12">
                    <div class="error-template">
                        <h1 class="display-1">
                            <i class="bi bi-exclamation-triangle text-warning"></i> {"哎呀！"}
                        </h1>
                        <h2 class="display-4">{"404 未找到"}</h2>
                        <div class="error-details mb-4">
                            {"抱歉，您请求的页面不存在！"}
                        </div>
                        <div class="error-actions">
                            <a href="/" class="btn btn-primary btn-lg">
                                <i class="bi bi-house"></i> {" 返回首页"}
                            </a>
                            <a href="/contact" class="btn btn-secondary btn-lg">
                                <i class="bi bi-envelope"></i> {" 联系支持"}
                            </a>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}