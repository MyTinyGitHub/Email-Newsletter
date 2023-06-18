use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscription(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(||  { 
              App::new()
                .route("/health_check", web::get().to(health_check))
                .route("/subscription", web::post().to(subscription))
         })
        .listen(listener)?
        .run();

    Ok(server)
}
