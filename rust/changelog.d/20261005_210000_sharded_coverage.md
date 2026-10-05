---
bump: patch
---

### Fixed

- Code coverage runs in the five suites of the test job, each under its own timeout, and a coverage report job merges their lcov reports with `scripts/merge-lcov.mjs`, enforces the 84.30% line coverage floor on the merged report and uploads it to Codecov: the whole suite under instrumentation in one job outgrew its 15-minute timeout.
