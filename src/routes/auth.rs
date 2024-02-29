use actix_web::{web, HttpResponse};

use crate::helpers::{
        response::ResponseBody,
        error:: ServiceError
};
use crate::config::db::Pool;



pub async fn sign_up(
    pool: web::Data<Pool>,
) -> Result<HttpResponse, ServiceError> {
    let response: Result<HttpResponse, ()> = Ok(HttpResponse::Ok().json(ResponseBody::new("Hi",  "")));
    Err(ServiceError::BadRequest {
        error_message: "messages".to_string(),
    })
}