use actix_web::{Responder, web};


pub fn ping_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("ping", web::get().to(ping))
    );
}


pub async fn ping() -> impl Responder {
    format!("{}", "Pong!")
}
