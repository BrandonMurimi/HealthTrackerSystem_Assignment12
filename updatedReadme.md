🌦️ Assignment 13 – CI/CD Pipeline for Weather Tracking System

This README outlines the CI/CD automation and protection rules implemented for the Weather Tracking System project using **GitHub Actions**.

---

 ✅ Local Development

To build and test the project locally:

```bash
cargo build
cargo test
```

---

 🚀 CI/CD Pipeline Overview

The CI/CD pipeline is configured in `.github/workflows/ci.yml`. It performs the following:

 🔁 Continuous Integration (CI)
- **Trigger**: On `push` and `pull_request` to `main`
- **Steps**:
  - Checkout code
  - Set up stable Rust toolchain
  - Build the project
  - Run unit & integration tests

### 📦 Continuous Delivery (CD)
- **Trigger**: On direct `push` to `main`
- **Steps**:
  - Archive the entire Weather Tracking System as `weather-tracking-system.zip`
  - Upload the archive as a GitHub Actions artifact
  - (Optional) Can be extended to publish to a registry or deploy via Docker in future

Example snippet from `ci.yml`:

```yaml
release-artifact:
  if: github.ref == 'refs/heads/main' && github.event_name == 'push'
  runs-on: ubuntu-latest
  needs: build-and-test
  steps:
    - uses: actions/checkout@v3
    - run: zip -r weather-tracking-system.zip .
    - uses: actions/upload-artifact@v3
      with:
        name: weather-artifact
        path: weather-tracking-system.zip
```

---

 🔐 Branch Protection

To ensure high code quality and production stability, the following rules are enforced on the `main` branch:

- ✅ Require pull request reviews (at least 1)
- ✅ Require status checks to pass (CI must succeed)
- ✅ Block direct pushes

More details in [`PROTECTION.md`](./PROTECTION.md)

---

 🤝 Getting Started for Contributors

```bash
git clone https://github.com/your-username/health-fitness-tracker.git
cd health-fitness-tracker
cargo build && cargo test
```

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) for contribution instructions.

---

🔮 Roadmap Preview

Check out the [`ROADMAP.md`](./ROADMAP.md) for upcoming features, DevOps improvements, and community collaboration opportunities.

---

 🌟 Contribution Highlights

| Label              | Description                        |
|-------------------|------------------------------------|
| good-first-issue  | Ideal for beginners                |
| feature-request   | Suggested enhancements or modules |

---

 📁 Key Files

```
.github/workflows/ci.yml        # CI/CD workflow
PROTECTION.md                   # Branch protection policy
CONTRIBUTING.md                 # How to contribute
ROADMAP.md                      # Upcoming features
LICENSE                         # Open-source license
openapi.yaml                    # API docs
```

---

✅ Example Screenshots (not included in markdown)

- Passing test badge
- Branch protection rule screen
- Pull request blocked by failed test
- Release artifact generated after successful merge

---

📅 **Submitted:** May 2025
