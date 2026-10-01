---
bump: patch
---

### Fixed
- Run the Full Requirements Aggregate even when delivery candidates are missing and fail it explicitly, because GitHub counts a skipped required check as passing; live merge evidence now rejects an acceptance workflow that can skip the aggregate.
- Name the missing repository Administration read permission when the workflow token cannot see ruleset bypass actors, and read rulesets with an optional `ISSUE_195_RULESET_TOKEN`.
- Recognize killed-session log uploads as automation output and register the maintainer working-process directive, so the source register matches the live discussion again.
- Refresh the atomic manifest's pinned hash of the source register.
- Compile generated Rust parsers against the most recently built `pest`/`pest_derive` artifacts, so a target cache restored from an older rustc no longer breaks the generated-parser tests, and satisfy the Clippy lints added in Rust 1.99, including `assert_is_empty` in the test suites, whose assertions now report the unexpected contents on failure.
