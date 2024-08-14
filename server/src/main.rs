use std::env::set_var;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, middleware, web, Error};
use actix_web::middleware::TrailingSlash;
use anyhow::Result;
use log::info;
use sea_orm::{Database};
use server::{Config, AppState, routers};
use actix_cors::Cors;

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
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8081")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);


        App::new()
            .app_data(state.clone())
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            //.wrap(JwtMiddleware)
            .default_service(web::route().to(not_found))
            .configure(routers::router)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;
    info!("Server stop successfully");
    Ok(())
}


async fn not_found(_data: web::Data<AppState>, _request: HttpRequest) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound().body("Not Found 404"))
}
