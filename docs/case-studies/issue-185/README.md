# Issue 185 CI/CD Audit Case Study

## Scope

Issue: https://github.com/link-foundation/meta-language/issues/185

The audit compared this repository's complete Rust and JavaScript workflow and
script trees with the current Rust, JavaScript, and Python pipeline templates.
It also inspected recent main-branch runs and preserved failed Rust run metadata
and logs under the ignored local `ci-logs/` directory.

## Observed CI Failure

Rust runs `30240588949`, `30166422548`, `29215150911`, and `28402713927` all
completed lint, tests, coverage, build, crate publication, documentation, and
GitHub release work successfully. Each run failed only in the Rust workflow's
duplicate JavaScript publication step:

```text
npm error code ENEEDAUTH
npm error need auth This command requires you to be logged in to https://registry.npmjs.org/
```

The npm registry contained only `meta-language@0.46.0`, while the failed runs
attempted versions `0.52.0` through `0.55.0`. No `NPM_TOKEN` repository secret
was configured. The separate JavaScript workflow already used npm trusted
publishing with provenance and an OIDC token.

## Root Causes and Fixes

### Duplicate npm release ownership

The Rust workflow copied the JavaScript test, pack, registry, and publish steps.
That made `.github/workflows/rust.yml` a second npm publisher without the trusted
publisher identity configured for `.github/workflows/js.yml`.

The Rust release now creates the GitHub release and explicitly dispatches
`js.yml` with the synchronized version. The JavaScript workflow is the only npm
publisher and verifies that a dispatched version matches `js/package.json`.
Rust release completeness deliberately excludes npm so an independently
retryable npm failure cannot cause an unrelated Rust version bump.

### Release-loop and concurrency defects

The version script staged modified files before rebasing and tagged the commit
before a retrying push. It also collected changelog fragments without deleting
them, leaving every historical fragment eligible to trigger another release.

The script now rebases a clean tree only when the remote is ahead, removes and
stages consumed fragments, retries the commit push, and creates the tag only
after the release commit is on the remote.

Read-only jobs now have independent cancellable concurrency groups. Every job
that can mutate main, a registry, a release, or Pages shares the same non-
cancelling write group. The invalid template key `queue: max` was removed after
actionlint proved GitHub's concurrency schema does not accept it.

### False warnings and hidden failures

- File-size warnings are limited to changed Rust files; the hard
  repository-wide limit remains enforced.
- Rust documentation builds with warnings denied before release.
- Codecov uses the current Node 24 action and an explicit token gate.
- `rust-script` installation is locked and retried.
- Secret scanning, fresh-merge simulation, and a committed `Cargo.lock` guard
  run before release-producing checks.
- Manual workflow input is transferred through environment variables instead
  of being interpolated into shell commands.
- Release-note docs.rs badges are static and version-specific rather than
  reflecting the status of a later docs.rs build.

## Template Findings Reported Upstream

- Rust unsupported concurrency key: https://github.com/link-foundation/rust-ai-driven-development-pipeline-template/issues/113
- JavaScript unsupported concurrency key: https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/117
- Python unsafe dispatch inputs and mixed concurrency: https://github.com/link-foundation/python-ai-driven-development-pipeline-template/issues/42

The remaining matching Rust and JavaScript hardening changes were already
present in their current templates and were ported into this repository.

## Regression Coverage

Focused tests cover:

- exclusive npm ownership and trusted-publishing dispatch;
- least-privilege workflow permissions and safe manual inputs;
- cancellable check jobs and serialized writer jobs;
- lockfile, secret-scan, fresh-merge, rustdoc, Codecov, and file-size gates;
- clean rebase/tag ordering and deletion of consumed fragments;
- version-specific documentation badges.

The workflow files are also validated with actionlint before submission.
