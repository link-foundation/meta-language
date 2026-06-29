---
bump: patch
---

### Fixed
- Removed Rust CI false-positive errors by scoping Cargo cache keys per job, avoiding Windows target cache uploads, and making Codecov uploads token-gated instead of silently successful on upload failure.
- Retried the crate-size `cargo package` step so transient crates.io index or download failures do not immediately fail the release guard.
