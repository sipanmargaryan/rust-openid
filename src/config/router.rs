use actix_web::web;

use crate::routes::*;



pub fn configure_router(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
        .service(
            web::scope("/auth")
            .service(web::resource("/signup").route(web::get().to(auth::sign_up)))
        )
    );
}