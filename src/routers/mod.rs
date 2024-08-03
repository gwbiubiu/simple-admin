mod ping;
mod user;

use actix_web::{web};

pub fn router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(user::user_router)
            .configure(ping::ping_router)
    );
}
