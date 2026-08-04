---
bump: patch
---

### Fixed
- Restored npm publishing: the placeholder `_authToken` that `actions/setup-node` writes made npm skip the OIDC trusted-publishing exchange, so every release since 0.55.0 failed with a misleading `E404`. The publish job now sanitizes the npm user config, supports an optional `NPM_TOKEN` bootstrap, and fails with actionable guidance when no credential is available.
