---
bump: patch
---

### Fixed
- Run the Full Requirements Aggregate even when delivery candidates are missing and fail it explicitly, because GitHub counts a skipped required check as passing; live merge evidence now rejects an acceptance workflow that can skip the aggregate.
- Name the missing repository Administration read permission when the workflow token cannot see ruleset bypass actors, and read rulesets with an optional `ISSUE_195_RULESET_TOKEN`.
- Recognize killed-session log uploads as automation output and register the maintainer working-process directive, so the source register matches the live discussion again.
