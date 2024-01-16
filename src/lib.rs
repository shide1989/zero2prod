use actix_web::dev::Server;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

/// health check
/// curl -i http://localhost:8080/health_check
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

pub fn run(tcp_listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/healthz", web::get().to(health_check)))
        .listen(tcp_listener)?
        .run();

    Ok(server)
}
