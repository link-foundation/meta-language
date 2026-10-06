---
bump: patch
---

### Added
- The issue 195 acceptance workflow has a "Formal AI Workloads" job. It checks out link-assistant/formal-ai at the pinned revision and verifies every inventoried file. It patches formal-ai's Rust crate to the unpacked candidate crate, after refreshing formal-ai's `Cargo.lock` for meta-language alone. It then runs formal-ai's own meta-language test groups. A clean consumer of the npm candidate runs formal-ai's Links Notation data through the public JavaScript API. Both runtimes write the same outputs, and the job compares them. The JavaScript and Rust suites validate the report and observe `I195-DOWNSTREAM-FORMAL-AI-WORKLOADS` only when it holds.
