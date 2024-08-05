use std::env::set_var;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, web, Error};
use actix_web::middleware::TrailingSlash;
use actix_web::web::ReqData;
use simple_admin::{AppState, Config, JwtMiddleware, routers};
use anyhow::Result;
use log::info;
use sea_orm::{Database, DatabaseConnection};

#[tokio::main] // or
async fn main() -> Result<()> {
    let config = Config::default();
    unsafe {
        set_var("RUST_LOG", "debug");
    }
    tracing_subscriber::fmt::init();

    let db_url = config.get_db_url();

    let conn = Database::connect(&db_url).await?;

    let state = web::Data::new(AppState { conn });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            .wrap(JwtMiddleware)
            .default_service(web::route().to(not_found))
            .configure(routers::router)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;
    info!("Server stop successfully");
    Ok(())
}


async fn not_found(data: web::Data<AppState>, request: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound().body("Not Found 404"))
}
