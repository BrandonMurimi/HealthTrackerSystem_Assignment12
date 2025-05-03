
#[derive(Debug)]
pub enum ServiceError {
    Validation(String),
    BusinessRuleViolation(String),
    NotFound,
    RepositoryError(Box<dyn std::error::Error>),
}

impl From<Box<dyn std::error::Error>> for ServiceError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        ServiceError::RepositoryError(err)
    }
}
