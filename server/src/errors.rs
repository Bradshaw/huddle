use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, From};

#[derive(Display, From, Debug)]
pub enum HuddleError {
    NotFound,
    MissingConfigurationError(String),
}
impl std::error::Error for HuddleError {}

impl ResponseError for HuddleError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            HuddleError::NotFound => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
