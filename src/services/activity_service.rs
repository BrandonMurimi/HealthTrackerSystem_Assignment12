
// src/services/activity_service.rs
use crate::{
    models::{Activity, ActivityType},
    repositories::ActivityRepository,
    error::ServiceError,
};

pub struct ActivityService<R: ActivityRepository> {
    repo: R,
    max_concurrent_activities: usize,
}

impl<R: ActivityRepository> ActivityService<R> {
    pub fn new(repo: R) -> Self {
        Self {
            repo,
            max_concurrent_activities: 3, // Business rule
        }
    }

    /// Start new activity with validation
    pub async fn start_activity(
        &mut self,
        user_id: String,
        activity_type: ActivityType,
    ) -> Result<Activity, ServiceError> {
        // Check existing active activities
        let active_activities = self.repo
            .find_by_user(user_id.clone())
            .await?
            .into_iter()
            .filter(|a| a.end_time.is_none())
            .count();

        // Business rule enforcement
        if active_activities >= self.max_concurrent_activities {
            return Err(ServiceError::BusinessRuleViolation(
                "Maximum 3 concurrent activities allowed".into(),
            ));
        }

        let activity = Activity::new(user_id, activity_type);
        self.repo.save(activity.clone()).await?;
        Ok(activity)
    }
}
