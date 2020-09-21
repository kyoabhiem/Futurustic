use actix_web::{dev::HttpResponseBuilder, http::header, http::StatusCode, HttpResponse, ResponseError};
use derive_more::{Display};

#[allow(dead_code)]
#[derive(Debug, Display)]
pub enum Error {
    // 400
    #[display(fmt = "Bad Request")]
    BadRequest,

    // 401
    #[display(fmt = "Unauthorized")]
    Unauthorized,

    // 403
    #[display(fmt = "Forbidden")]
    Forbidden,

    // 404
    #[display(fmt = "Not Found")]
    NotFound,

    // 405
    #[display(fmt = "Method Not Allowed")]
    MethodNotAllowed,

    // 408
    #[display(fmt = "Request Timeout")]
    GatewayTimeout,

    // 422
    #[display(fmt = "Unprocessable Entity")]
    UnprocessableEntity,

    // 500
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            Error::Forbidden => StatusCode::FORBIDDEN,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::UnprocessableEntity => StatusCode::UNPROCESSABLE_ENTITY,
            Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            Error::BadRequest => StatusCode::BAD_REQUEST,
            Error::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/html; charset=utf-8")
            .body(self.to_string())
    }
}