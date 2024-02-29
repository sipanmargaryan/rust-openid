use actix_web::{App, HttpServer, web};
use std::env;
use dotenv::dotenv;


mod config;
mod routes;
mod helpers;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().expect("Can not load env variables");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not found.");

    let pool = config::db::init_db_pool(&db_url);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(config::router::configure_router)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
