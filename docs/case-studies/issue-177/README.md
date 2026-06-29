# Issue 177 CI/CD False Positive Case Study

## Scope

Issue: https://github.com/link-foundation/meta-language/issues/177

Investigated runs:

- Rust run `28398677445`: `cancelled`, branch `main`, SHA `63ab0c52135420fa26131484e215faca0c6f7deb`, created `2026-06-29T19:54:19Z`.
- JavaScript run `28398677404`: `success`, branch `main`, SHA `63ab0c52135420fa26131484e215faca0c6f7deb`, created `2026-06-29T19:54:19Z`.
- Rust PR verification run `28401016525`: `failure`, event `pull_request`, SHA `da68db415a152a74f2214ddd48854e33f39f6b85`, created `2026-06-29T20:36:27Z`.

Preserved artifacts:

- `ci-logs/rust-28398677445.log`
- `ci-logs/javascript-28398677404.log`
- `ci-logs/rust-28401016525.log`
- `rust-run-28398677445.json`
- `javascript-run-28398677404.json`
- `rust-run-28401016525.json`
- `template-data/*`

## Findings

### Rust Windows tests were cancelled during post-job cache save

The Windows test job completed `cargo test --all-features --verbose` and doc tests successfully, then hit the job timeout while `actions/cache` was saving the cache:

- `ci-logs/rust-28398677445.log:5174`: `The operation was canceled.`
- Run metadata shows `Test (windows-latest)` started at `2026-06-29T19:55:39Z` and the post-cache cancellation happened at `2026-06-29T20:05:39Z`, exactly ten minutes later.

Root cause:

- The `test` job timeout was `10` minutes.
- The Windows cache archived `rust/target`, which is large for this crate because it builds many native parser and TLS dependencies.
- The timeout applied to the whole job lifetime, including post-job cache work, so successful tests could still produce a cancelled job.

Fix:

- Increase the Rust `test` job timeout to `20` minutes.
- Split the test cache by platform.
- On Windows, cache only `~/.cargo/registry` and `~/.cargo/git`; do not upload `rust/target`.
- Keep `rust/target` caching on Unix runners where the observed post-cache duration was short.

### Cargo cache keys were shared across jobs

The Ubuntu test job reported:

- `ci-logs/rust-28398677445.log:9368`: `Unable to reserve cache with key Linux-cargo-... another job may be creating this cache.`

Root cause:

- `lint` and `test` both used the same key: `${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}`.
- GitHub cache entries are immutable once created, so parallel jobs racing to save the same exact key create noisy but non-fatal save failures.

Fix:

- Scope cache keys by job:
  - `cargo-lint`
  - `cargo-test`
  - `cargo-test-registry`
  - existing `cargo-coverage`
  - existing `cargo-build`

### Codecov upload failed while the job stayed green

The coverage job generated the coverage file successfully, then Codecov reported warnings and an upload error:

- `ci-logs/rust-28398677445.log:2544`: `xcrun is not installed or can't be found.`
- `ci-logs/rust-28398677445.log:2545`: `No gcov data found.`
- `ci-logs/rust-28398677445.log:2550`: `Upload queued for processing failed: {"message":"Token required - not valid tokenless upload"}`

The step and job still reported success because the workflow used `fail_ci_if_error: false`.

Root cause:

- The workflow attempted a Codecov upload without `CODECOV_TOKEN`.
- `fail_ci_if_error: false` converted a real upload failure into a false green step.
- Codecov also searched for extra coverage formats, creating unrelated warnings for a workflow that already passes an explicit `lcov.info`.

Fix:

- Set `CODECOV_TOKEN` in the coverage job environment.
- Run the Codecov action only when `env.CODECOV_TOKEN != ''`.
- Emit an explicit GitHub Actions notice when the token is missing.
- Pass `token: ${{ env.CODECOV_TOKEN }}` and `fail_ci_if_error: true` when the upload runs.
- Set `disable_search: true` so the action uploads the requested `rust/lcov.info` file without searching for unrelated report formats.

### Crate-size guard treated a transient registry error as final

After the first fix, Rust PR verification run `28401016525` confirmed that the original failure areas passed: Code Coverage, Lint and Format Check, and all three matrix Test jobs completed successfully. The run then failed in `Build Package / Check crate package size` while running `rust-script scripts/check-crate-size.rs`:

- `ci-logs/rust-28401016525.log:12248`: `Updating crates.io index`
- `ci-logs/rust-28401016525.log:12252`: `download of se/rd/serde_yaml_ng failed`
- `ci-logs/rust-28401016525.log:12255`: `curl failed`
- `ci-logs/rust-28401016525.log:12258`: `[16] Error in the HTTP2 framing layer`
- `ci-logs/rust-28401016525.log:12259`: `cargo package failed; cannot determine crate archive size`

Root cause:

- The crate-size guard runs `cargo package` to produce the `.crate` archive before checking the archive size.
- `cargo package` may still contact the crates.io index while preparing package metadata.
- The guard executed `cargo package` once, so a transient registry or network failure surfaced as a deterministic package-size failure.

Fix:

- Retry `cargo package` up to three times before reporting `cargo package failed`.
- Keep the existing final failure message and all archive-size checks unchanged.
- Add unit tests that simulate one transient failure before success and permanent failure after all retries.

### JavaScript workflow did not require changes

The JavaScript run completed successfully. The only text matches for warnings/errors were normal runner grouping lines, a git checkout hint, and `# cancelled 0` from the test runner output. No JavaScript workflow false-positive failure was found in run `28398677404`.

## Template Comparison

Template snapshots were captured in `template-data/` at these commits:

- Rust: `67598fb14cff5326f0a971fc5b3004fb852250ee`
- JavaScript: `482f6528f980ace473ca44a8c998a9c66b044235`
- Python: `e5c9b689510fb7de16825a22465611b18ca6cf4b`
- C#: `b76374de356148e9903cbea1f724e0b3bbfe0c25`

The Rust template has the same relevant patterns as this repository had before the fix:

- Generic Cargo cache keys for both lint and test jobs.
- Windows matrix tests caching `target`.
- `timeout-minutes: 10` on the test job.
- Codecov upload with `fail_ci_if_error: false` and no token gate.

The Python and C# templates also contain Codecov uploads with `fail_ci_if_error: false`. The JavaScript template did not contain the Rust-specific cache or Codecov problem.

Upstream reports filed:

- Rust template: https://github.com/link-foundation/rust-ai-driven-development-pipeline-template/issues/87
- Rust template crate-size retry: https://github.com/link-foundation/rust-ai-driven-development-pipeline-template/issues/88
- Python template: https://github.com/link-foundation/python-ai-driven-development-pipeline-template/issues/27
- C# template: https://github.com/link-foundation/csharp-ai-driven-development-pipeline-template/issues/34

## External References Checked

- GitHub Actions workflow syntax for `jobs.<job_id>.timeout-minutes`: https://docs.github.com/actions/using-workflows/workflow-syntax-for-github-actions
- GitHub Actions secret handling in conditionals through environment variables: https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions
- `actions/cache` behavior and immutable cache keys: https://github.com/actions/cache
- `codecov/codecov-action` inputs including `token`, `disable_search`, and `fail_ci_if_error`: https://github.com/codecov/codecov-action

## Verification

Added workflow-policy tests in `rust/tests/unit/ci-cd/workflow_release.rs` covering:

- `test` job timeout is `20` minutes.
- Cargo cache keys are scoped by job.
- Windows test caching excludes `rust/target`.
- Codecov upload is token-gated, disables broad coverage search, and fails when an attempted upload fails.
- Missing Codecov token is reported with an explicit notice instead of a hidden upload error.

Focused check:

```bash
cargo test --test unit ci_cd::workflow_release -- --nocapture
```

Result after the fix: `15 passed; 0 failed`.

Added crate-size guard tests in `rust/scripts/check-crate-size.rs` covering:

- `cargo package` succeeds after one transient failure.
- `cargo package` reports failure only after all retry attempts fail.

Focused check:

```bash
cargo test --test unit ci_cd::check_crate_size -- --nocapture
```

Result after the fix: `8 passed; 0 failed`.
