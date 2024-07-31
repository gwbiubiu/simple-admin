use actix_web::{App, HttpServer};
use simple_admin::routers;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(routers::router)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}