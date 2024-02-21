use actix_web::HttpResponse;
use crate::helpers::{
        response::ResponseBody,
        error:: ServiceError
};



pub async fn sign_up() -> Result<HttpResponse, ServiceError> {
    // Ok(HttpResponse::Ok().json(ResponseBody::new("Hi",  "")))
    Err(ServiceError::BadRequest {
        error_message: "messages".to_string(),
    })
}