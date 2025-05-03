
// src/api/error.rs
#[derive(utoipa::ToSchema)]
pub enum ApiError {
    #[schema(example = "Resource not found")]
    NotFound,
    #[schema(example = "Invalid email format")]
    Validation(String),
    #[schema(example = "Maximum 3 concurrent activities allowed")]
    BusinessRuleViolation(String),
}
