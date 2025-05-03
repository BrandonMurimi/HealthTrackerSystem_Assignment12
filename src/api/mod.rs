
// src/api/mod.rs
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        users::create_user,
        users::get_user,
        activities::start_activity
    ),
    components(
        schemas(User, CreateUserRequest, Activity, ApiError)
    ),
    tags(
        (name = "Users", description = "User management"),
        (name = "Activities", description = "Fitness activities")
    )
)]
pub struct ApiDoc;

pub fn routes(/* ... */) -> Router {
    let swagger = SwaggerUi::new("/docs")
        .url("/api-docs/openapi.json", ApiDoc::openapi());
    
    Router::new()
        .merge(swagger)
        // ... existing routes
}
