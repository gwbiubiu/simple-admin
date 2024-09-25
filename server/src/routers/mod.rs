mod ping;
mod user;
mod auth;
mod api;
mod role;
mod menu;

use actix_web::web;

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(user::user_router)
            .configure(ping::ping_router)
            .configure(auth::auth_router)
            .configure(api::api_router)
            .configure(role::role_router)
    );
}




