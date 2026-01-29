use crate::domain::value_objects::Identification;
use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use std::future::{Ready, ready};
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct AuthenticatedUser {
    id: Identification,
}

impl AuthenticatedUser {
    pub fn id(&self) -> &Identification {
        &self.id
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(id) = req.extensions().get::<Uuid>() {
            return ready(Ok(AuthenticatedUser {
                id: Identification::from(*id),
            }));
        }
        ready(Err(ErrorUnauthorized("no claims")))
    }
}
