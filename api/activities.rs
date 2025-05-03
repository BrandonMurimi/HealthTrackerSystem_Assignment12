
// src/api/activities.rs
use axum::{
    extract::{Path, State, Json},
    routing::{post, delete},
    Router
};
use crate::{services::ActivityService, models::{Activity, ActivityType}};

pub fn routes(activity_service: ActivityService<impl ActivityRepository>) -> Router {
    Router::new()
        .route("/", post(start_activity))
        .route("/:id/complete", post(complete_activity))
        .with_state(activity_service)
}

// POST /api/activities
async fn start_activity(
    State(service): State<ActivityService<impl ActivityRepository>>,
    Json(payload): Json<StartActivityRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let activity = service
        .start_activity(payload.user_id, payload.activity_type)
        .await?;
    Ok((StatusCode::CREATED, Json(activity)))
}

// POST /api/activities/:id/complete
async fn complete_activity(
    Path(activity_id): Path<String>,
    State(service): State<ActivityService<impl ActivityRepository>>,
) -> Result<Json<Activity>, ApiError> {
    let activity = service.complete_activity(activity_id).await?;
    Ok(Json(activity))
}

#[derive(serde::Deserialize)]
struct StartActivityRequest {
    user_id: String,
    activity_type: ActivityType,
}
