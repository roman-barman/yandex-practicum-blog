use crate::api::http::errors::create_error_response;
use crate::infrastructure::JwtService;
use actix_web::body::BoxBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::http::StatusCode;
use actix_web::middleware::Next;
use actix_web::{Error, HttpMessage, web};
use std::sync::Arc;

pub(crate) async fn auth_middleware(
    req: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, Error> {
    match req.headers().get("Authorization") {
        Some(value) => {
            let header_value = value.to_str();
            match header_value {
                Err(err) => {
                    tracing::error!("Error parsing authorization header: {}", err);
                    Ok(req.into_response(create_unauthorized_response()))
                }
                Ok(header_value) => {
                    if header_value.starts_with("Bearer ") {
                        let token = header_value.split(" ").collect::<Vec<&str>>()[1];
                        let jwt_service = req.app_data::<web::Data<Arc<JwtService>>>();
                        match jwt_service {
                            None => {
                                tracing::error!("JwtService not found in request data");
                                Ok(req.into_response(create_internal_server_error_response()))
                            }
                            Some(jwt_service) => {
                                let claims = jwt_service.decode_jwt(token);
                                match claims {
                                    Err(_) => Ok(req.into_response(create_unauthorized_response())),
                                    Ok(claims) => {
                                        req.extensions_mut().insert(claims.sub());
                                        next.call(req).await
                                    }
                                }
                            }
                        }
                    } else {
                        Ok(req.into_response(create_unauthorized_response()))
                    }
                }
            }
        }
        None => Ok(req.into_response(create_unauthorized_response())),
    }
}

fn create_unauthorized_response() -> actix_web::HttpResponse {
    create_error_response(StatusCode::UNAUTHORIZED, "unauthorized".to_string())
}

fn create_internal_server_error_response() -> actix_web::HttpResponse {
    create_error_response(
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal server error".to_string(),
    )
}
