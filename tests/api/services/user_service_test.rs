
// tests/services/user_service_test.rs
#[tokio::test]
async fn test_register_user_validation() {
    let mut repo = InMemoryUserRepository::new();
    let mut service = UserService::new(repo);

    // Test invalid email
    let result = service.register_user("bad-email".into(), "Alice".into()).await;
    assert!(matches!(result, Err(ServiceError::Validation(_)));

    // Test valid registration
    let user = service.register_user("alice@fit.com".into(), "Alice".into())
        .await
        .unwrap();
    assert_eq!(user.email, "alice@fit.com");
}

// tests/services/activity_service_test.rs
#[tokio::test]
async fn test_concurrent_activity_limit() {
    let mut repo = InMemoryActivityRepository::new();
    let mut service = ActivityService::new(repo);

    let user_id = "user1".to_string();
    
    // Create 3 activities (limit)
    for _ in 0..3 {
        service.start_activity(user_id.clone(), ActivityType::Running).await.unwrap();
    }

    // 4th should fail
    let result = service.start_activity(user_id.clone(), ActivityType::Cycling).await;
    assert!(matches!(result, Err(ServiceError::BusinessRuleViolation(_)));
}
