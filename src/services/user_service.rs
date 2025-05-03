
// src/services/user_service.rs
use crate::{
    models::User,
    repositories::UserRepository,
    error::ServiceError,
};

pub struct UserService<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Register user with validation
    pub async fn register_user(
        &mut self,
        email: String,
        name: String,
    ) -> Result<User, ServiceError> {
        // Validate input
        if email.is_empty() || !email.contains('@') {
            return Err(ServiceError::Validation("Invalid email format".into()));
        }

        // Business rule: Name must be at least 2 characters
        if name.len() < 2 {
            return Err(ServiceError::Validation("Name too short".into()));
        }

        let user = User::new(email, name);
        self.repo.save(user.clone()).await?;
        Ok(user)
    }

    /// Get user by ID with error handling
    pub async fn get_user(&self, user_id: &str) -> Result<User, ServiceError> {
        self.repo
            .find_by_id(user_id.to_string())
            .await?
            .ok_or(ServiceError::NotFound)
    }
}
