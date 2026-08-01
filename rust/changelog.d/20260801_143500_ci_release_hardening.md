---
bump: patch
---

### Fixed
- Delegated npm publication to the JavaScript trusted-publishing workflow, preventing the Rust release from failing with `ENEEDAUTH` and avoiding duplicate release ownership.
- Hardened CI cancellation, permissions, workflow inputs, dependency installation, lockfile checks, secret scanning, fresh-merge validation, rustdoc warnings, Codecov uploads, and file-size annotations.
- Rebased release commits before modifying files, tagged only after a successful push, removed consumed changelog fragments, and pinned release-note documentation badges to the released version.
