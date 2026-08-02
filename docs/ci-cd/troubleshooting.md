# CI/CD Troubleshooting Guide

This guide covers common CI/CD issues and their solutions for Rust projects using this template.

## Table of Contents

1. [Release Jobs Skipped](#release-jobs-skipped)
2. [Version Already Released (False Positive)](#version-already-released-false-positive)
3. [Crates.io Publishing Fails](#cratesio-publishing-fails)
4. [Crate Package Too Large (HTTP 413)](#crate-package-too-large-http-413)
5. [Docker Hub Publishing Fails](#docker-hub-publishing-fails)
6. [npm Publishing Fails with E404 or ENEEDAUTH](#npm-publishing-fails-with-e404-or-eneedauth)
7. [Secret Configuration Issues](#secret-configuration-issues)
8. [Multi-Language Repository Issues](#multi-language-repository-issues)

---

## Release Jobs Skipped

### Symptom
Release jobs (auto-release or manual-release) are skipped even though you expected them to run.

### Common Causes

#### 1. Upstream job was skipped
When a job like `detect-changes` is skipped (e.g., on `workflow_dispatch`), all dependent jobs are also skipped by default.

**Solution:** Ensure dependent jobs use `always() && !cancelled()` in their conditions:
```yaml
if: |
  always() && !cancelled() && (
    github.event_name == 'push' ||
    github.event_name == 'workflow_dispatch' ||
    needs.detect-changes.outputs.rs-changed == 'true'
  )
```

#### 2. Build or test failed
Release jobs depend on `build` which depends on `lint` and `test`. If any of these fail, release jobs won't run.

**Solution:** Check the logs for lint, test, and build jobs. Fix any failures before releasing.

#### 3. Wrong trigger condition
The job condition may not match your trigger event.

**Solution:** Verify the job's `if` condition matches your trigger:
- `github.event_name == 'push'` for automatic releases on merge
- `github.event_name == 'workflow_dispatch'` for manual triggers

### Reference
- [GitHub Actions Runner Issue #491](https://github.com/actions/runner/issues/491)

---

## Version Already Released (False Positive)

### Symptom
The release workflow says "version already released" but the package is not actually on crates.io.

### Root Cause
The workflow was checking git tags instead of crates.io. Git tags can exist without the package being published (e.g., from previous GitHub-only releases).

### Solution
This template now checks crates.io directly using the API:
```javascript
const response = await fetch(
  `https://crates.io/api/v1/crates/${crateName}/${version}`
);
const isPublished = response.ok && (await response.json()).version;
```

### Verification
Check if your package exists on crates.io:
```bash
curl -s "https://crates.io/api/v1/crates/YOUR_CRATE_NAME" | jq
```

### Reference
- [browser-commander Issue #29](https://github.com/link-foundation/browser-commander/issues/29)

---

## Crates.io Publishing Fails

### Symptom
The "Publish to Crates.io" step fails with an error.

### Common Errors

#### "please provide a non-empty token"
**Cause:** The `CARGO_REGISTRY_TOKEN` environment variable is empty or not set.

**Solution:**
1. Ensure you have a secret configured (either `CARGO_REGISTRY_TOKEN` or `CARGO_TOKEN`)
2. Map the secret correctly in your workflow:
```yaml
- name: Publish to Crates.io
  env:
    CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_TOKEN }}
  run: node scripts/publish-crate.mjs
```

#### "already uploaded" or "already exists"
**Cause:** This version was already published to crates.io.

**Note:** This is handled gracefully by the script and is not a failure.

#### "unauthorized" or authentication errors
**Cause:** Invalid or expired token.

**Solution:**
1. Generate a new token at https://crates.io/settings/tokens
2. Update the secret in your repository or organization settings

### Reference
- [browser-commander Issue #33](https://github.com/link-foundation/browser-commander/issues/33)
- [Cargo Publishing Documentation](https://doc.rust-lang.org/cargo/reference/publishing.html)

---

## Crate Package Too Large (HTTP 413)

### Symptom
`cargo publish` is rejected by crates.io with:

```
error: failed to publish to registry
the remote server responded with an error (status 413 Payload Too Large):
max upload size is 10485760
```

### Root Cause
The generated `.crate` archive exceeds the crates.io upload limit of **10 MiB
(10485760 bytes)**. This usually happens when documentation, case studies,
generated CI artifacts, datasets, or experiment files are silently bundled into
the package.

### How This Template Prevents It

#### 1. Pre-publish size guard
`scripts/check-crate-size.rs` builds the `.crate` archive and fails the workflow
**before** publishing when the archive is over the limit. It runs in the `build`
job (early PR feedback) and again right before the publish step in both the
`auto-release` and `manual-release` jobs.

Run it locally before pushing:
```bash
rust-script scripts/check-crate-size.rs
```

#### 2. Narrow `include` allowlist
`Cargo.toml` declares an `include` list so only the crate sources and a few
documentation files ship in the release archive:
```toml
include = [
    "src/**/*.rs",
    "examples/**/*.rs",
    "README.md",
    "LICENSE",
    "CHANGELOG.md",
]
```

### Solution When the Guard Fails
1. Inspect what is being packaged:
   ```bash
   cargo package --list --allow-dirty
   ```
2. Tighten the `include` allowlist in `Cargo.toml` (or add an `exclude` list) to
   drop large files such as docs, datasets, generated logs, and experiments.
3. Re-run the size guard to confirm the archive is under 10 MiB.

### Reference
- [Cargo `include`/`exclude` fields](https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields)
- [Cargo packaging documentation](https://doc.rust-lang.org/cargo/reference/publishing.html#packaging-a-crate)

---

## Docker Hub Publishing Fails

### Symptom
The crates.io publish succeeds, but the release workflow fails before or during Docker Hub publishing.

### Required Configuration

Docker Hub publishing is optional. It runs only when all of these are true:

- A root `Dockerfile` exists
- Repository variable `DOCKERHUB_IMAGE` is set to `namespace/repository`
- `DOCKERHUB_USERNAME` is set as a repository variable or secret
- Repository secret `DOCKERHUB_TOKEN` is set

### Common Errors

#### "Docker Hub publishing requires DOCKERHUB_USERNAME and DOCKERHUB_TOKEN"
**Cause:** `DOCKERHUB_IMAGE` and `Dockerfile` enabled Docker publishing, but credentials are incomplete.

**Solution:** Set `DOCKERHUB_USERNAME` and create a Docker Hub access token stored as `DOCKERHUB_TOKEN`.

#### Docker tag is missing after crates.io already published
**Cause:** A previous release run published the crate, then failed before Docker Hub or GitHub Release completed.

**Solution:** Re-run the release workflow after fixing the Docker Hub configuration. The release check treats the version as incomplete and recreates missing artifacts without bumping the Cargo version again.

### Verification
Check whether a Docker Hub tag exists:

```bash
curl -fsSL "https://hub.docker.com/v2/repositories/NAMESPACE/REPOSITORY/tags/VERSION"
```

### Reference
- [Docker GitHub Actions guide](https://docs.docker.com/build/ci/github-actions/)

---

## npm Publishing Fails with E404 or ENEEDAUTH

### Symptom

The `Publish npm Package` job of `.github/workflows/js.yml` fails with either:

```
npm error code E404
npm error 404 Not Found - PUT https://registry.npmjs.org/meta-language
```

or, on older configurations:

```
npm error code ENEEDAUTH
```

The `E404` is misleading: the package exists on the registry and the version is new.
npm returns 404 rather than 403 for writes it considers unauthenticated.

### Root Cause

`actions/setup-node` with `registry-url` unconditionally writes

```
//registry.npmjs.org/:_authToken=${NODE_AUTH_TOKEN}
```

into `$NPM_CONFIG_USERCONFIG` and exports the placeholder
`NODE_AUTH_TOKEN=XXXXX-XXXXX-XXXXX-XXXXX` when the workflow supplies no token.
npm then treats itself as already authenticated, **skips the OIDC token exchange**
required by trusted publishing, and the registry rejects the anonymous upload.
With an *empty* token the same line produces `ENEEDAUTH` instead.

See [actions/setup-node#1440](https://github.com/actions/setup-node/issues/1440)
and [#1551](https://github.com/actions/setup-node/issues/1551). The upstream fix
([PR #1558](https://github.com/actions/setup-node/pull/1558)) removed the
placeholder, but it shipped only on the **v7** line — `@v6` still exports it.

### How This Repository Prevents It

The publish job runs `js/scripts/prepare-npm-auth.mjs` between `setup-node` and
`npm publish`. The script:

1. resolves the auth mode — `token` (a real `NPM_TOKEN`/`NODE_AUTH_TOKEN`),
   `oidc` (`ACTIONS_ID_TOKEN_REQUEST_URL` + `ACTIONS_ID_TOKEN_REQUEST_TOKEN`),
   or `none`;
2. strips the placeholder `_authToken` line unless a real token is in play, so the
   OIDC handshake actually happens, and always strips the deprecated `always-auth`
   key that npm 11 warns about;
3. **exits non-zero with remediation instructions** when the mode is `none`, so a
   missing credential fails loudly instead of surfacing as a bare `E404`.

### Solution

If the job reports auth mode `oidc` and the publish still fails, the trusted
publisher is not configured. On npmjs.com open the package settings and add a
trusted publisher for repository `link-foundation/meta-language` with workflow file
`.github/workflows/js.yml` — the filename must match exactly. Keep
`permissions: id-token: write` on the publish job and npm >= 11.5.1.

As a bootstrap, before trusted publishing is configured, add an `NPM_TOKEN`
repository secret; the workflow passes it as `NODE_AUTH_TOKEN` and the script keeps
the credential line in place. The secret can be removed afterwards.

### Debugging

Verbose credential tracing is off by default. Enable it by setting the repository
variable `NPM_AUTH_DEBUG` to `1` (or by running the script with `--verbose`):

```bash
NPM_AUTH_DEBUG=1 node js/scripts/prepare-npm-auth.mjs
```

It prints the resolved auth mode, the npm user config path, and the *key names* in
that file. Values are never printed, so a real token cannot leak into the log.

### Reference

- [npm trusted publishers](https://docs.npmjs.com/trusted-publishers)
- [Issue #191](https://github.com/link-foundation/meta-language/issues/191) and its
  evidence folder `dev/log/issues/191/pulls/192/`

---

## Secret Configuration Issues

### Required Secrets

| Secret Name | Purpose | Where to Get |
|------------|---------|--------------|
| `CARGO_REGISTRY_TOKEN` or `CARGO_TOKEN` | Publish to crates.io | https://crates.io/settings/tokens |
| `DOCKERHUB_TOKEN` | Publish to Docker Hub when `DOCKERHUB_IMAGE` is configured | https://app.docker.com/settings/personal-access-tokens |
| `GITHUB_TOKEN` | Create GitHub releases | Automatic (provided by GitHub) |
| `NPM_TOKEN` | Optional bootstrap for npm publishing until a trusted publisher is configured | https://www.npmjs.com/settings/~/tokens |

### Organization vs Repository Secrets

If using organization secrets with different names, map them in your workflow:
```yaml
env:
  # Map organization secret to the expected variable name
  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_TOKEN }}
```

### Checking Secret Values

Secrets are masked in logs, but you can verify they're set:
```yaml
- name: Debug secrets
  run: |
    if [ -n "$CARGO_REGISTRY_TOKEN" ]; then
      echo "CARGO_REGISTRY_TOKEN is set (value masked)"
    else
      echo "WARNING: CARGO_REGISTRY_TOKEN is NOT set"
    fi
  env:
    CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_TOKEN }}
```

### Reference
- [GitHub Actions Secrets Documentation](https://docs.github.com/actions/security-guides/using-secrets-in-github-actions)

---

## Multi-Language Repository Issues

### Symptom
Scripts fail to find `Cargo.toml` or run in the wrong directory.

### Solution
This template auto-detects the repository structure:
- **Single-language:** `Cargo.toml` in repository root
- **Multi-language:** `Cargo.toml` in `rust/` subfolder

If auto-detection fails, you can explicitly configure the Rust root:
```bash
# Via environment variable
RUST_ROOT=rust node scripts/publish-crate.mjs

# Via CLI argument
node scripts/publish-crate.mjs --rust-root rust
```

### Workflow Configuration
For multi-language repos, ensure your workflow has the correct `working-directory`:
```yaml
defaults:
  run:
    working-directory: rust

steps:
  - name: Publish to Crates.io
    working-directory: .  # Override for scripts that handle paths themselves
    run: node rust/scripts/publish-crate.mjs
```

### Reference
- [browser-commander Issue #31](https://github.com/link-foundation/browser-commander/issues/31)

---

## General Debugging Tips

### 1. Check Job Dependencies
View the workflow graph in GitHub Actions to see which jobs depend on which.

### 2. Download Full Logs
```bash
gh run view <run-id> --repo owner/repo --log > ci-logs.txt
```

### 3. Enable Debug Logging
Add this secret to enable debug logging:
- Name: `ACTIONS_STEP_DEBUG`
- Value: `true`

### 4. Check crates.io Status
Sometimes crates.io has issues. Check: https://status.crates.io/

### 5. Verify Package Locally
Before pushing, verify your package builds and passes checks:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features
cargo test --all-features
cargo package --list
rust-script scripts/check-crate-size.rs
```
