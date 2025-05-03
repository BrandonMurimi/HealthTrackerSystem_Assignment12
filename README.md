# HealthTrackerSystem_Assignment12

# Fitness Tracker API 
**Assignment 12: Service Layer & REST API Implementation**  
![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange) 
![Axum](https://img.shields.io/badge/Axum-0.7-blue)
![OpenAPI](https://img.shields.io/badge/OpenAPI-3.0-green)

## 🚀 Quick Start
```bash
# Clone and run
git clone https://github.com/your-username/fitness-tracker-rust.git
cd fitness-tracker-rust
cargo run

# Test endpoints
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{"email":"test@fit.com","name":"Alice"}'


Project Structure

.
├── src/
│   ├── services/          # Business logic (User/Activity services)
│   ├── api/               # REST controllers
│   ├── repositories/      # Data persistence (A11)
├── tests/
│   ├── services/          # Unit tests
│   └── api/               # Integration tests
├── docs/                  # OpenAPI specs
└── Cargo.toml


API Endpoints

Users
Method	Endpoint	Description
POST	/api/users	Register new user
GET	/api/users/{id}	Get user profile
Activities
Method	Endpoint	Description
POST	/api/activities	Start new activity
POST	/api/activities/{id}/complete	Complete activity

Example Request:

curl -X POST http://localhost:3000/api/activities \
  -H "Content-Type: application/json" \
  -d '{"user_id":"user123","activity_type":"Running"}


Testing

# Run unit tests (services)
cargo test --test services

# Run integration tests (API)
cargo test --test api

# Test coverage (requires grcov)
cargo tarpaulin --ignore-tests

Documentation

Interactive API docs available at:
http://localhost:3000/docs

Swagger UI

🛠️ Development







