use actix_web::{App, HttpServer};


mod config;
mod routes;
mod helpers;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config::router::configure_router)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
