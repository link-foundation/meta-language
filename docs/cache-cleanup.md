# Cache cleanup

Builds, tests, coverage, benchmarks, packaging and the issue #195 acceptance
run write caches that can be regenerated. They grow quickly: one acceptance
run leaves several gigabytes of candidate packages, clean consumers and their
Rust targets, and `rust/target` alone passes 15 GiB after a few test,
coverage and release builds. This document describes the single entry point
that cleans them, what it may and may not delete, and when it runs.

The requirements are recorded in the issue #195 ledger as
`I195-CACHE-CLEANUP-ENTRY-POINT`, `-SAFETY`, `-HOOK`, `-EVENTS`, `-BUDGET`,
`-POLICY` and `-MEASURED` (see
[issue-195-requirement-ledger.md](issue-195-requirement-ledger.md)).

## Entry point

```bash
node scripts/clean-caches.mjs                  # prune to the disk budget
node scripts/clean-caches.mjs --full           # remove every cache class
node scripts/clean-caches.mjs --dry-run --json cache-report.json
node scripts/clean-caches.mjs --list-classes
node scripts/clean-caches.mjs --help
```

The script works on the git worktree that contains the current directory.
Outside a git worktree, or without git, it reports `skipped` and exits 0.
Every run prints the bytes before, after and reclaimed. Real runs also write
the full report to `.git/meta-language-cache/last-run.json` and append a
summary to `.git/meta-language-cache/history.jsonl`, which keeps the last 200
runs.

| Option | Meaning |
| --- | --- |
| `--mode prune\|full`, `--full` | prune to the budget (default), or remove every class |
| `--budget-mb N` | aggregate cache budget, default `$META_LANGUAGE_CACHE_BUDGET_MB` or 4096 |
| `--min-free-mb N` | low-disk preflight floor, default `$META_LANGUAGE_MIN_FREE_MB` or 2048 |
| `--no-preflight` | never escalate to a full clean |
| `--event NAME` | the event that triggered the run, recorded in the report |
| `--target-dir DIR` | an extra Cargo target directory (repeatable); `CARGO_TARGET_DIR` and `CARGO_BUILD_TARGET_DIR` are added automatically |
| `--results-dir DIR` | an extra evidence directory whose `work/` holds scratch (repeatable) |
| `--protect DIR` | an extra directory that must never be removed (repeatable) |
| `--class ID` | clean only this class (repeatable) |
| `--no-docker` | skip container resources |
| `--dry-run` | report without removing anything |
| `--json FILE` | also write the full report to `FILE` |

The exit status is 0 when the cleanup ran, found another cleanup running
(`busy`) or was skipped; 2 for invalid arguments; 1 for an internal error.

## Cache classes

Each class is defined in `scripts/lib/cache-classes.mjs`. Its `covers` list
names the categories it owns, and `scripts/check-cache-policy.mjs` fails when a
required category has no class.

| Class | Covers | What it removes |
| --- | --- | --- |
| `rust-target` | Rust debug, release, test, coverage, incremental, linked examples, doc, package and benchmark output; custom target directories | In every Cargo target directory (`rust/target`, `target`, `rust/web/target`, ignored nested `target` directories, `CARGO_TARGET_DIR` and `--target-dir`): incremental state, linked examples, `llvm-cov-target`, `tmp`, `package`, `criterion`, `nextest` and `*.profraw` are transient. `doc` and the profile directories are the warm cache. The whole target directory goes only in full mode. |
| `javascript` | JavaScript build caches, test coverage, temporary package consumers | `node_modules/.cache`, `coverage`, `.nyc_output`, and `npm pack` tarballs in the repository root and `js/` |
| `generated-intermediates` | generated parser and compiler intermediates | `rust/web/pkg`, `_site`, and ignored `build`, `.build`, `node_modules`, `target` directories and object files under the vendored grammars |
| `proof-build` | Lean and Rocq build output | `.lake` directories, `build` next to a `lakefile`, and `.vo`, `.vok`, `.vos`, `.glob`, `.aux`, `Makefile.coq` and `.lia.cache` files |
| `acceptance-scratch` | acceptance scratch, nested consumer targets, nested clones | every child of `issue-195-results/work` (candidate packages, clean consumers with their Rust targets, downstream clones) except the evidence below, and the legacy `.issue-195-work` |
| `temporary-clones` | nested temporary clones | directories in the OS temporary directory that a repository script created with `scripts/lib/scratch.mjs`, whose marker names this worktree and whose creator has exited |
| `containers` | container and BuildKit caches | `docker container/image/volume prune --filter label=org.link-foundation.meta-language=cache`, and `docker buildx prune --builder meta-language`. `js/scripts/build-web-tree-sitter-runtime.mjs` labels its containers. |

Transient caches are removed on every run. The warm cache (Cargo profile
directories and rustdoc output) is kept while the total stays within the
budget. When it does not fit, it is removed in priority order until it does:
rustdoc first, then optimized builds, then the debug and test build.

## Safety

A candidate is only a proposal: `scripts/lib/cache-cleanup.mjs` removes it
only when all of the following hold.

- It is not a symlink and does not resolve through one, so nothing outside
  escapes through a link. `--target-dir`, `--results-dir` and `--protect`
  reject `..` segments.
- It lies inside this worktree. The exceptions are a Cargo target directory
  named explicitly (recognized by `CACHEDIR.TAG` or `.rustc_info.json`) and a
  marked scratch directory of this worktree in the temporary directory.
- `git check-ignore` reports it ignored, and no tracked file lies under it.
  Source, tracked vendored grammars, fixtures, proofs, corpora and uncommitted
  work therefore stay.
- It is not evidence: `issue-195-results` outside `work/`, the execution
  records and logs in `work/`, the native validation evidence in
  `work/native`, `issue-195-artifacts`, `ci-logs`, `logs`, `dev/log`, `docs`,
  `parity` and `--protect` directories all stay.
- It contains no other worktree and lies in none.
- No build is active on it. While
  `scripts/with-cache-cleanup.mjs` or the evidence runner works, it holds a
  lease in `.git/meta-language-cache/leases`, and the build-sensitive classes
  are then left alone. A Cargo target whose `.cargo-lock` a running Cargo
  holds, which is probed with `flock`, is kept as well.

Concurrent cleanups of one worktree are serialized by
`.git/meta-language-cache/cleanup.lock`. A second run reports `busy` and
removes nothing. Stale leases and locks of exited processes are dropped.
Nothing global is ever purged: no `cargo cache`, no `npm cache clean`, no
`docker system prune`. Missing git, docker or flock is tolerated, and the
report says what was skipped.

## Events

| Event | How the cleanup runs |
| --- | --- |
| every commit | `.githooks/pre-commit`, installed by `node scripts/install-dev-hooks.mjs`, runs `clean-caches.mjs --event pre-commit` on every commit, including documentation-only commits. The `clean-caches` hook of `.pre-commit-config.yaml` does the same for `pre-commit run`. |
| build, test, coverage, benchmark, package | `node scripts/with-cache-cleanup.mjs --event <event> -- <command>` runs the command and cleans afterwards, also when it fails or is interrupted. |
| end of the acceptance run | `js/scripts/run-issue-195-evidence.mjs` cleans the worktree for the `acceptance` event and records the measurement in `issue-195-results/artifacts/cache-cleanup.json` (`I195-CACHE-CLEANUP-MEASURED`). |
| CI teardown | Every CI job that runs cargo, npm or the acceptance scripts ends with an `if: ${{ !cancelled() }}` step running `clean-caches.mjs --event ci-teardown`, so it runs after failures. The workflows avoid `always()` so a cancelled run stops promptly; a cancelled step's wrapper receives the runner's signal and cleans up before it exits. |

The wrapper holds a lease while the command runs. The first `SIGINT`,
`SIGTERM` or `SIGHUP` is forwarded to the command, and the wrapper waits for
it to exit; a second signal kills it. Only after that does the cleanup run.
The wrapper then exits with the command's status: its exit code, 128 plus the
signal number, 127 when the command is missing, or 126 when it cannot be run.
A failing cleanup is reported but never changes that status. Evidence and logs
are never cleaned, so they are archived before any cleanup.

### Hook installation

```bash
node scripts/install-dev-hooks.mjs          # install and verify
node scripts/install-dev-hooks.mjs --check  # verify only; exit 1 when inactive
```

The script sets `core.hooksPath` to `.githooks` with `git config --local`, in
this clone only. When another `core.hooksPath` is configured, it refuses to
replace it unless `--force` is given. `.githooks/pre-commit` first runs the
hook that `core.hooksPath` would hide (`<git-common-dir>/hooks/pre-commit`, for
example the one `pre-commit install` writes) and keeps its exit status. Then it
cleans. No npm lifecycle script, Cargo build script or package consumer
installs hooks or changes any git configuration.

## Growth limits

- `rust/Cargo.toml` sets `[profile.dev] debug = "line-tables-only"` and
  `incremental = false`. Test builds inherit these settings.
- The wrapper sets bounded defaults when the caller has not: `CARGO_BUILD_JOBS`
  and `RUST_TEST_THREADS` to three quarters of the CPUs (at most 8, or
  `META_LANGUAGE_JOBS`), `CARGO_INCREMENTAL=0`, and `SCCACHE_CACHE_SIZE=2G`.
- CI checks examples with `cargo check --examples` instead of linking them,
  and runs `cargo test --tests` and `cargo test --doc`.

## Policy check

`node scripts/check-cache-policy.mjs` runs in CI and fails when any of the
following holds:

- the hook is missing, not executable or does not clean;
- the `clean-caches` pre-commit entry is missing, restricted to some files or
  not `always_run`;
- a pre-commit hook or workflow step runs a cache-producing command (`cargo
  test/build/bench/llvm-cov/package`, `npm test/pack` or `wasm-pack build`)
  without the wrapper. The acceptance runners are not wrapped: the evidence
  runner holds its own lease and cleans at its end, and a wrapper's lease
  would hold that cleanup back;
- a job that runs cargo or npm does not end with the teardown that runs after failures;
- a required category has no class;
- the dev or test profile is not lean;
- the bootstrap is undocumented, or a lifecycle script touches git hooks.

## Measured disk use

The measured requirement records the real cleanup at the end of the pre-merge
evidence run. `issue-195-results/artifacts/cache-cleanup.json` holds the
report (`beforeBytes`, `afterBytes`, `reclaimedBytes`, `withinBudget`, the
removed and kept paths), the number of evidence files, and the checks that no
evidence file changed and that `git status` is unchanged.
