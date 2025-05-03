
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum ApiError {
    Service(ServiceError),
    NotFound,
    Unauthorized,
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::NotFound => ApiError::NotFound,
            _ => ApiError::Service(err),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Resource not found"),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            ApiError::Service(ServiceError::Validation(msg))) => 
                (StatusCode::BAD_REQUEST, msg.as_str()),
            ApiError::Service(ServiceError::BusinessRuleViolation(msg))) => 
                (StatusCode::CONFLICT, msg.as_str()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
