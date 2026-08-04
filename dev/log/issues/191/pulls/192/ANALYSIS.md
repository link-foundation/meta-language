# Issue #191 — CI/CD false positives, false negatives, warnings and errors

Deep analysis for [issue #191](https://github.com/link-foundation/meta-language/issues/191),
delivered in [PR #192](https://github.com/link-foundation/meta-language/pull/192).

All evidence in this folder was collected from the repository's own Actions runs:

| File | Run | Workflow | Conclusion |
| --- | --- | --- | --- |
| `ci-logs/run-30735100706.log` | 30735100706 | JavaScript | failure — `E404` on publish |
| `ci-logs/run-30733403030.log` | 30733403030 | JavaScript | failure — `E404` on publish |
| `ci-logs/run-30708127649.log` | 30708127649 | JavaScript | failure — parity manifest gate |
| `ci-logs/run-30240588949.log` | 30240588949 | Rust | failure — `ENEEDAUTH` on npm publish |
| `ci-logs/run-30166422548.log` | 30166422548 | Rust | failure — `ENEEDAUTH` on npm publish |
| `ci-logs/run-30734654534-js-success.log` | 30734654534 | JavaScript | success (baseline) |
| `ci-logs/run-30734654535-js-success.log` | 30734654535 | Rust | success (baseline) |
| `ci-logs/run-30732901520-rust-success.log` | 30732901520 | Rust | success (baseline) |

`CI-CD-BEST-PRACTICES.md` is a verbatim copy of the document the issue points at
(`link-assistant/hive-mind`), kept here so the review is reproducible even if the
upstream file moves. `templates/{js,rust,python}` are shallow clones of the three
pipeline templates the issue asks us to compare against; they are intentionally
untracked (see `.gitignore`) because they are third-party working copies.

## 1. Timeline

1. **v0.46.0** — last version of `meta-language` that actually reached npm
   (`npm view meta-language version` → `0.46.0`, maintainer `konard`).
2. **Before #177** — the Rust release workflow published the npm package itself,
   using `NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}`. The repository has no
   `NPM_TOKEN` secret (org level exposes only `CARGO_TOKEN`, `DOCKERHUB_TOKEN`,
   `NUGET_API_KEY`), so the variable expanded to the empty string and npm failed
   with `ENEEDAUTH` — runs 30166422548 and 30240588949, both for
   `meta-language@0.55.0`.
3. **Issue #177 / its PR** — npm publishing was moved out of `rust.yml` into the
   canonical `js.yml` and switched to OIDC trusted publishing: `id-token: write`,
   `npm install -g npm@11`, `npm publish --provenance`, and the `NODE_AUTH_TOKEN`
   environment variable was deleted. A regression test
   (`js/tests/package-release.test.js`) was added asserting
   `assert.doesNotMatch(workflow, /NODE_AUTH_TOKEN/)`.
4. **After #177** — the failure mode changed but did not go away: every publish now
   ends with
   `npm error code E404 / npm error 404 Not Found - PUT https://registry.npmjs.org/meta-language`
   (runs 30733403030, 30735100706). The `test` job passes, so the release looks
   green until the publish job runs.
5. **Run 30708127649** — an unrelated, already-fixed failure: the parity manifest
   gate rejected `javascript-parser-codegen` because
   `rust/changelog.d/20260619_233000_javascript_parser_codegen.md` had been consumed
   by a release. `npm run check:parity` passes on `main` today.
6. **Issue #191** — asks for a full sweep of false positives, false negatives,
   warnings and errors across CI/CD, compared against the three templates.

## 2. Root cause of the release failures

`actions/setup-node@v6`, whenever `registry-url` is set, unconditionally writes

```
//registry.npmjs.org/:_authToken=${NODE_AUTH_TOKEN}
```

into the npm user config (`$NPM_CONFIG_USERCONFIG`) and exports the literal
placeholder `NODE_AUTH_TOKEN=XXXXX-XXXXX-XXXXX-XXXXX` when the workflow does not
supply a token.

This is directly visible in our own failing run — `ci-logs/run-30735100706.log`
shows, for every step of the publish job:

```
NPM_CONFIG_USERCONFIG: /home/runner/work/_temp/.npmrc
NODE_AUTH_TOKEN: XXXXX-XXXXX-XXXXX-XXXXX
```

even though `js.yml` at that commit set no token at all. The run used
`actions/setup-node@v6` (SHA `249970729cb0ef3589644e2896645e5dc5ba9c38`), node
24.18.0 and npm 11.19.0 — so neither an outdated npm nor a missing
`id-token: write` can explain the failure.

Upstream state, verified rather than assumed:

* [actions/setup-node#1551](https://github.com/actions/setup-node/issues/1551)
  ("registry-url writes _authToken line that breaks npm Trusted Publisher OIDC
  when no NODE_AUTH_TOKEN is set") was **closed as a duplicate** of
  [#1440](https://github.com/actions/setup-node/issues/1440), with a maintainer
  stating OIDC publishing worked for them even with the dummy token present.
* On #1440, however, multiple users report that removing the dummy token is the
  only thing that fixes the `E404`, and the maintainers ultimately agreed:
  [PR #1558](https://github.com/actions/setup-node/pull/1558) —
  "remove the dummy/default `NODE_AUTH_TOKEN` export" — was merged on 2026-05-28.
* That fix shipped **only in the v7 line**. Checked directly against the built
  bundles: `dist/setup/index.js` on `v7` contains no `XXXXX-XXXXX` placeholder,
  while `v6` (which `@v6` currently resolves to, v6.5.0) still does. The
  `v6.5.0` tag and the #1558 merge commit are on diverged branches — the fix was
  never backported.

So the behaviour is real, is present in the exact action version this repository
pins, and is fixed upstream only by moving to `actions/setup-node@v7`.

The consequence is precisely the two failures above:

* **With no token configured (post-#177):** npm 11 sees a configured
  `_authToken` for the registry, concludes it is already authenticated, and
  therefore **never performs the OIDC token exchange**. It sends the tarball with
  the bogus placeholder credential; the registry treats the request as anonymous
  and answers `404 Not Found` for `PUT /meta-language` — npm's standard way of
  hiding "you may not write to this package" from unauthenticated clients. The
  `E404` is a *false* error message: the package exists, the real problem is that
  trusted publishing was silently skipped.
* **With an empty token (pre-#177):** the same line expands to an empty value and
  npm reports `ENEEDAUTH`.

So both historical failures share one root cause: **the npm user config written by
`setup-node` decides the auth mode before npm ever looks at OIDC.** Removing
`NODE_AUTH_TOKEN` from the workflow (what #177 did) does not remove the npmrc line,
which is why the fix did not take effect.

A second, independent precondition also has to hold: the `meta-language` package
must have a trusted publisher registered on npmjs.com pointing at
`link-foundation/meta-language` and the workflow file `.github/workflows/js.yml`.
That is repository-owner configuration and cannot be done from a PR — so the fix
must both unblock OIDC *and* fail loudly with instructions when neither credential
path is available, instead of surfacing a misleading `E404`.

## 3. Requirements from the issue, and how each is addressed

| # | Requirement (from the issue) | Status | Where |
| --- | --- | --- | --- |
| R1 | Find false positives in CI/CD | done | §4.1 |
| R2 | Find false negatives in CI/CD | done | §4.2 |
| R3 | Find warnings in CI/CD | done | §4.3 |
| R4 | Find errors in CI/CD | done | §4.4 |
| R5 | Fix all of them | done | PR #192 commits |
| R6 | Compare every CI/CD file against the three pipeline templates | done | §5 |
| R7 | Report the same problem upstream in the templates when it exists there | done | §6 |
| R8 | Follow `hive-mind/docs/CI-CD-BEST-PRACTICES.md` | done | §5 |
| R9 | Plan and execute everything in a single pull request | done | PR #192 |

## 4. Findings

### 4.1 False positives (CI reports a problem that is not real)

* **`E404 Not Found - PUT https://registry.npmjs.org/meta-language`** — the package
  exists and the version is new; the real fault is a skipped OIDC exchange. Fixed by
  `js/scripts/prepare-npm-auth.mjs` plus the `Explain npm publish failure` step,
  which replaces the misleading 404 with the actual cause and the two remediation
  paths.
* **`ENEEDAUTH` attributed to npm** — the same npmrc line, with an empty expansion.
  Already removed from `rust.yml`; the sanitizer prevents it from returning.

### 4.2 False negatives (CI stays green while something is broken)

* **A release could complete with the npm package never published.** The `test` job
  is green, the GitHub Release is created by `rust.yml`, and only the separate
  dispatched publish job fails — so the headline status of the release commit does
  not reflect the missing artifact. `prepare-npm-auth.mjs` now **exits non-zero**
  when it resolves auth mode `none`, before `npm publish` is attempted, so the
  absence of any credential is an explicit failure rather than a late, disguised one.
* **The regression test locked in the broken state.**
  `assert.doesNotMatch(workflow, /NODE_AUTH_TOKEN/)` asserted the exact
  configuration that cannot publish, so the test suite actively defended the bug.
  Replaced with assertions describing the intended state: the sanitizer step runs,
  and `NODE_AUTH_TOKEN` appears exactly once, as the optional `secrets.NPM_TOKEN`
  bootstrap fallback.

### 4.3 Warnings

* **Deprecated `always-auth` key.** `setup-node` still writes `always-auth=false`;
  npm 11 warns about it. `sanitizeNpmrc` removes it in every mode. This matches what
  the JS template does in `scripts/sanitize-npm-userconfig.mjs`.

### 4.4 Errors

Covered by §2 and §4.1 — the `E404` and `ENEEDAUTH` publish errors are the only
hard errors in the collected logs that are not already fixed on `main` (the parity
gate failure of run 30708127649 is).

## 5. Template and best-practices comparison

Checked `.github/workflows/js.yml` and `.github/workflows/rust.yml` against the
three `link-foundation/*-ai-driven-development-pipeline-template` repositories and
against `CI-CD-BEST-PRACTICES.md`.

Already conformant in this repository:

* `permissions: contents: read` at workflow level, widened per job only where needed
  (`id-token: write` on publish, `actions: write` on the release jobs).
* Repository-scoped serialised writer group
  `release-${{ github.repository }}-main-write` with `cancel-in-progress: false`,
  and cancellable read-only `check-*` groups.
* `!cancelled()` rather than `always()` in job conditions, so a cancelled run does
  not resurrect downstream jobs.
* `timeout-minutes` on every job.
* Version indirection through `env:` (`REQUESTED_VERSION`, `PACKAGE_VERSION`) instead
  of interpolating `${{ github.event.inputs.* }}` directly into `run:` scripts —
  the script-injection guard the best-practices document requires.
* Idempotent publish: `npm view` pre-check plus a post-publish verification.
* Skip-with-a-`::notice::` rather than fail when an optional secret is absent
  (the coverage job's `CODECOV_TOKEN` handling).

Gap found and closed by this PR:

* The templates' `sanitize-npm-userconfig.mjs` strips only `always-auth` and leaves
  the placeholder `_authToken` line in place, so the templates carry the same latent
  bug. This repository now strips both (`prepare-npm-auth.mjs`), and the divergence
  is reported upstream — see §6.

## 6. Upstream reports

The defect is not local to this repository; it is shared by every project generated
from the JS template and, at its origin, by `actions/setup-node` itself. See
`UPSTREAM-REPORTS.md` in this folder for the reproducible example, the workaround,
and the suggested code fix that accompany each report.

## 7. Existing components surveyed

* [`actions/setup-node#1551`](https://github.com/actions/setup-node/issues/1551) /
  [`#1440`](https://github.com/actions/setup-node/issues/1440) /
  [`PR #1558`](https://github.com/actions/setup-node/pull/1558) — the upstream
  reports and the merged fix. Two remedies exist: remove the `_authToken` line
  before publishing (works on any action version), or move to
  `actions/setup-node@v7`, which no longer exports the placeholder. This PR
  implements the first because it is version-agnostic and does not couple the
  release path to a major action bump; the v7 upgrade is recorded as a
  follow-up rather than smuggled into a release fix.
* [npm trusted publishing docs](https://docs.npmjs.com/trusted-publishers) —
  requirements: npm >= 11.5.1, `id-token: write`, and a trusted publisher whose
  workflow filename matches exactly.
* JS template `scripts/sanitize-npm-userconfig.mjs` — closest existing component;
  adopted its shape and extended it to cover `_authToken` (this is the upstream gap).
* JS template `scripts/publish-failure-classifier.mjs` — the pattern behind the
  `Explain npm publish failure` step.
* [Changesets](https://github.com/changesets/changesets) / [Scriv](https://scriv.readthedocs.io/) —
  the prior art the existing `changelog.d` fragment mechanism already follows; no
  change needed.

No third-party library replaces the fix: the problem is a five-line npmrc
sanitisation that must run between `setup-node` and `npm publish`, and the only
existing implementation of it (the template script) is the one missing the relevant
case.

## 8. Debug output and verbose mode

Per the issue's requirement that unresolved causes be traceable on the next
iteration, `prepare-npm-auth.mjs` supports a verbose mode that is **off by default**:

* enable with `--verbose`, or by setting the repository variable `NPM_AUTH_DEBUG=1`
  (already wired into the publish job's `env:`);
* it prints the resolved auth mode, the npm user config path, and the *key names*
  present in that file before sanitisation;
* it never prints values, so a real token cannot leak into a workflow log — this is
  covered by a test.
