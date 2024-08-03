use actix_web::{get, Responder, web};


pub fn ping_router(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}


#[get("/ping")]
async fn ping() -> impl Responder {
    format!("{}", "Pong!")
}
