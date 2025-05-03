
// tests/api/users.rs
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn test_user_creation() {
    let repo = InMemoryUserRepository::new();
    let service = UserService::new(repo);
    let app = api::routes(service, ActivityService::new(InMemoryActivityRepository::new()));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    r#"{"email": "test@fit.com", "name": "Alice"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_invalid_user_email() {
    let repo = InMemoryUserRepository::new();
    let service = UserService::new(repo);
    let app = api::routes(service, ActivityService::new(InMemoryActivityRepository::new()));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/users")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    r#"{"email": "bad-email", "name": "Alice"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
