---
bump: patch
---

### Changed
- Every pull request check now depends only on state that exists before merge, and every one of them can pass. The live default-branch rule inspection of `I195-ACCEPTANCE-REQUIRED-MERGE-CHECK` moves to a new `post-merge` checkpoint. It runs on `main` as a non-blocking Post-merge Report with the workflow token. The `ISSUE_195_RULESET_TOKEN` secret is no longer used anywhere.
- Pull requests compare delivered dependencies offline against the committed audit. `check-dependencies.mjs` goes live only with `--live`, which runs on `main` as a non-blocking report. The `--online` source-register and documentation comparisons also run only on `main`.
- A scheduled Dependency Refresh workflow on `main` updates the lockfiles, refreshes the dependency audit and opens its own pull request.

### Added
- A gate self-test (`I195-ACCEPTANCE-PR-CHECKS-PASSABLE`) checks that the pre-merge aggregate is green on a fixture in which every pre-merge row passes, and that no pre-merge row needs its own aggregate to fail.
