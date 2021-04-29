use actix_web::{
    error::{BlockingError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DbError},
};
use uuid::Error as ParseError;

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum ApiError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    NotFound(String),
    ParseError(String),
    PoolError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
    InternalServerError(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => {
                HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
            }
            ApiError::NotFound(message) => HttpResponse::NotFound()
                .header("Content-Type", "application/json")
                .json::<ErrorResponse>(message.into()),
            ApiError::ValidationError(errors) => HttpResponse::UnprocessableEntity()
                .header("Content-Type", "application/json")
                .json::<ErrorResponse>(errors.to_vec().into()),
            ApiError::Unauthorized(error) => {
                HttpResponse::Unauthorized().json::<ErrorResponse>(error.into())
            }
            ApiError::InternalServerError(error) => {
                HttpResponse::InternalServerError().json::<ErrorResponse>(error.into())
            }
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

impl From<DbError> for ApiError {
    fn from(error: DbError) -> ApiError {
        match error {
            DbError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ApiError::BadRequest(message);
                }
                ApiError::InternalServerError("Unknown database error".into())
            }
            _ => ApiError::InternalServerError("Unknown database error".into()),
        }
    }
}

impl From<PoolError> for ApiError {
    fn from(error: PoolError) -> ApiError {
        ApiError::PoolError(error.to_string())
    }
}

impl From<ParseError> for ApiError {
    fn from(error: ParseError) -> ApiError {
        ApiError::ParseError(error.to_string())
    }
}

impl From<BlockingError<ApiError>> for ApiError {
    fn from(error: BlockingError<ApiError>) -> ApiError {
        match error {
            BlockingError::Error(api_error) => api_error,
            BlockingError::Canceled => ApiError::BlockingError("Thread blocking error".into()),
        }
    }
}
