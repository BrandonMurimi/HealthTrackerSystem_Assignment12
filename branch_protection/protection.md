🔐 Branch Protection Rules – PROTECTION.md

To maintain quality and prevent regressions in the Health Fitness Tracker System, we enforce branch protection rules on the `main` branch.

 ✅ Enforced Rules

| Rule                          | Description |
|------------------------------|-------------|
| ✅ Require pull request reviews | All changes to `main` must be reviewed by at least one other contributor. |
| ✅ Require status checks to pass | GitHub Actions must verify the code builds and passes all tests. |
| ✅ Block direct pushes         | No developer can push directly to `main`—all changes go through PRs. |

🔍 Why It Matters

- **Code Integrity**: Prevents unreviewed or broken code from reaching production.
- **Collaboration**: Encourages team peer review and shared ownership.
- **Quality Assurance**: CI runs catch issues before merge.
- **Compliance**: Aligns with modern DevOps workflows and version control best practices.

 ⚙️ How to Configure

On GitHub:
- Go to `Settings → Branches → Add Rule → main`
- Check:
  - ✅ Require pull request reviews
  - ✅ Require status checks to pass
  - ✅ Include administrators (optional)

---

> These rules help ensure that only tested and reviewed code enters the main release branch, supporting a reliable health data platform.


