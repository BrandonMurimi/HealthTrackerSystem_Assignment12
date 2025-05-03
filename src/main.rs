
// src/main.rs
use axum::Router;
use crate::{api::routes, repositories::{InMemoryUserRepository, InMemoryActivityRepository}};

#[tokio::main]
async fn main() {
    let user_repo = InMemoryUserRepository::new();
    let activity_repo = InMemoryActivityRepository::new();
    
    let app = routes(
        UserService::new(user_repo),
        ActivityService::new(activity_repo)
    );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
