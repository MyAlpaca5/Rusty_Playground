use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
struct SubscribeForm {
    name: String,
    email: String,
}

async fn subscribe(form: web::Form<SubscribeForm>) -> HttpResponse {
    HttpResponse::Ok().finish()
}