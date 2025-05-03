
pub mod users;
pub mod activities;

use axum::{Router, routing::get};
use crate::services::{UserService, ActivityService};

pub fn routes(
    user_service: UserService<impl UserRepository + Send + Sync + 'static>,
    activity_service: ActivityService<impl ActivityRepository + Send + Sync + 'static>
) -> Router {
    Router::new()
        .nest("/api/users", users::routes(user_service))
        .nest("/api/activities", activities::routes(activity_service))
        .route("/api/health", get(|| async { "OK" }))
}
