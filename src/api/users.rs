
use axum::{
    extract::{Path, State, Json},
    routing::{get, post, put, delete},
    Router,
    response::IntoResponse
};
use crate::{services::UserService, models::User};

pub fn routes(user_service: UserService<impl UserRepository>) -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(list_users))
        .route("/:id", get(get_user))
        .route("/:id", put(update_user))
        .route("/:id", delete(delete_user))
        .with_state(user_service)
}

// POST /api/users
async fn create_user(
    State(service): State<UserService<impl UserRepository>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let user = service.register_user(payload.email, payload.name).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

// GET /api/users/:id
async fn get_user(
    Path(user_id): Path<String>,
    State(service): State<UserService<impl UserRepository>>,
) -> Result<Json<User>, ApiError> {
    let user = service.get_user(&user_id).await?;
    Ok(Json(user))
}

#[derive(serde::Deserialize)]
struct CreateUserRequest {
    email: String,
    name: String,
}
