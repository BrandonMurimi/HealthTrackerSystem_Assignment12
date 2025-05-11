# 🏃 Health Fitness Tracker – Assignment 13: CI/CD Pipeline

This README outlines how continuous integration and delivery (CI/CD) are implemented for the Health Fitness Tracker system using GitHub Actions.

---

## 🧪 Run Tests Locally

To verify the system locally:

```bash
cargo build
cargo test
```

---

## 🚀 CI/CD Pipeline

Our `.github/workflows/ci.yml` file automates testing and artifact delivery:

### ✅ Continuous Integration (CI)
- **Trigger**: On push and PR to `main`
- **Steps**:
  - Checkout code
  - Set up stable Rust
  - Build project
  - Run all unit/integration tests

### 📦 Continuous Delivery (CD)
- **Trigger**: Push directly to `main`
- **Steps**:
  - Archive code as `health-fitness-tracker.zip`
  - Upload as GitHub Action artifact

---

## 🔐 Branch Protection

Enabled on the `main` branch to enforce code quality:

- ✅ Require pull request reviews
- ✅ Require status checks to pass
- ✅ Block direct pushes to `main`

See [`PROTECTION.md`](./PROTECTION.md) for details.

---

## 📁 Key Files

```
.github/workflows/ci.yml        # CI/CD automation
PROTECTION.md                   # Branch protection policy
```

---

## 📸 Required Screenshots (not included in repo)

- Branch protection rules UI
- Green checkmarks on CI job
- PR blocked by test failure
- Artifact uploaded to Actions tab

---

📅 **Submitted:** May 2025
