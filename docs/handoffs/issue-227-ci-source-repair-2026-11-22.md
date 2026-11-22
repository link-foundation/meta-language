# Issue #227 CI source repairs, November 22, 2026

[PR #228](https://github.com/link-foundation/meta-language/pull/228) continues
[issue #227](https://github.com/link-foundation/meta-language/issues/227) and
[grammar discovery issue #224](https://github.com/link-foundation/meta-language/issues/224).
The [remaining-work plan](issue-227-remaining-work-plan-2026-10-31.md) and its
two requirement appendices retain the full scope.

## Published CI baseline

[Run 42129692629](https://github.com/link-foundation/meta-language/actions/runs/42129692629)
was triggered for head `3b77cb71a42c091c89d952f24acfe652ee95a1a1` and checked
out merge candidate `7db1c94b77f90523cdd989c4749f02c415fc9034`. GitHub reported
77 failures, 19 skipped jobs and two passes. The branch already contains
default-branch commit `ec9f4efb0e9ef936114e09823981352f6db13700`. Complete log
archives for the latest five runs and selected job logs are retained locally
in `ci-logs/`.

| Failure | Draft repair and reproduction |
| --- | --- |
| Duplicate `error` declaration prevents the Rust frontend from loading | Give the callback-specific diagnostic its own binding. The existing borrowed iterator test imports the affected frontend. [Report job](https://github.com/link-foundation/meta-language/actions/runs/42129692629/job/127602873465) |
| All JavaScript group jobs encounter 42 unassigned files | Register each exact filename in an existing group. The existing test-group check verifies that every test file has one owner. Refresh the classifier history with the existing author. [Group job](https://github.com/link-foundation/meta-language/actions/runs/42129692629/job/127608680399) |
| The Rust source version job lacks a cache teardown event | Supply `--event ci-teardown` to the existing project-scoped command. The workflow cache policy check requires the event. [Group job](https://github.com/link-foundation/meta-language/actions/runs/42129692629/job/127608680399) |
| Package contracts detect stale inventories, an unknown callables source pin and missing case study metadata | Refresh the source register, requirement manifest, declaration inventory and compact naming history with their authors. Restore the historical index. [Contracts job](https://github.com/link-foundation/meta-language/actions/runs/42129692629/job/127602873881) |
| TypeScript 7.0.2 reports `TS2820` for the unchanged unknown const signature literal | Select the exact observed diagnostic by compiler version, retain the existing `TS2322` expectation for 5.9.2 and require uncovered versions to report the missing policy. The public declaration control exercises all three selections. [Generated package job](https://github.com/link-foundation/meta-language/actions/runs/42129692629/job/127602873736) |

## Open work

- A source syntax scan found an extra parenthesis in the ordinary class
  closure control. After that repair, static linking reaches the missing
  `experiments/javascript-properties/error-class-source-codecs.mjs`.
  The existing `draft-javascript-error-source-kernels.mjs --draft` recipe on
  supported Node 24.14.0 fails in `JavaScriptParser.typedefs` with
  `expected : in object type but found "?" at 264..265`. Complete optional
  field translation through the source checker, native emitter, and wire
  transport before generating class, collection and module-initialization
  codecs from their actual checked declarations. The initializer's separate
  author hits the same translator error at `1589..1590`.
- The compact naming author refreshed the current callables pin with both
  historical inventories intact. The additional naming author hit its
  existing subprocess timeout during capture. It emitted no successor
  inventory. The complete naming check remains open.
- Raw archive descriptor restoration, the binary buffer native refresh and
  the earlier report and translation deadline failures still need exact
  source repairs and CI evidence. The current module loading failure masked
  those behaviors in the published run.
- The register refresh matched 67 live requirement sources and 14 status
  reports. The authored manifest has 473 requirements, 494 fixture identities
  and 1,017 verification cells. Eighty requirements lack implementation entry
  points for at least one required runtime, including twelve in grammar
  inference. The exact-commit tests, native compilation, package installation,
  downstream workloads and acceptance observations remain outstanding.
