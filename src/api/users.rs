
// src/api/users.rs
use utoipa::ToSchema;

#[derive(serde::Deserialize, ToSchema)]
pub struct CreateUserRequest {
    /// User's email address
    #[schema(example = "user@example.com")]
    pub email: String,
    
    /// User's full name
    #[schema(example = "John Doe")]
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = CreateUserRequest,
    responses(
        (status = 201, description = "User created successfully", body = User),
        (status = 400, description = "Validation error", 
            body = ApiError,
            example = json!({"error": "Invalid email format"})),
        (status = 500, description = "Internal server error")
    ),
    tag = "Users"
)]
async fn create_user(/* ... */) { /* ... */ }

#[utoipa::path(
    get,
    path = "/api/users/{id}",
    params(
        ("id" = String, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details", body = User),
        (status = 404, description = "User not found",
            body = ApiError,
            example = json!({"error": "User not found"}))
    ),
    tag = "Users"
)]
async fn get_user(/* ... */) { /* ... */ }
