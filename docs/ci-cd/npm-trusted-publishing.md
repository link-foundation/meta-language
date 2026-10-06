# npm Trusted Publishing

`main` publishes the crate and the npm package from the same merged commit:

1. `auto-release` (or `manual-release`) in `.github/workflows/rust.yml` bumps the
   version, tags the release commit `v<version>`, publishes the crate to crates.io
   and creates the GitHub release.
2. It then dispatches `.github/workflows/js.yml` on that tag
   (`gh workflow run js.yml --ref "v$RELEASE_VERSION"`). The tagged commit is the
   one the crate came from, even when `main` has moved on in the meantime.
3. The `Publish npm Package` job of `js.yml` publishes `meta-language` with
   `npm publish --provenance` using OIDC trusted publishing. No npm token is
   needed once the trusted publisher below is configured.

## Trusted publisher settings on npmjs.com

On npmjs.com open **meta-language → Settings → Trusted Publisher**, choose
**GitHub Actions** and enter exactly:

| Field | Value |
|-------|-------|
| Organization or user | `link-foundation` |
| Repository | `meta-language` |
| Workflow filename | `js.yml` |
| Environment name | *(leave empty)* |

- The workflow filename is the bare filename. Do not enter the path
  `.github/workflows/js.yml`. npm compares the field with the workflow in the
  `workflow_ref` OIDC claim of the run, which here is
  `link-foundation/meta-language/.github/workflows/js.yml@refs/tags/v<version>`.
- Leave the environment empty, because the publish job declares no `environment:`.
  If an environment is added to the job later, enter the same name here.
- The dispatching workflow (`rust.yml`) is not the publisher. npm only sees
  `js.yml`, which is the workflow that runs `npm publish`.
- After the first trusted publish succeeds, **Publishing access** can be set to
  *Require two-factor authentication and disallow tokens*. The `NPM_TOKEN`
  secret is then no longer used.

The workflow side already meets npm's requirements, and
`js/tests/prepare-npm-auth.test.js` and `js/tests/package-release.test.js`
check it:

- The publish job has `permissions: id-token: write` and runs on a
  GitHub-hosted runner.
- npm 12 is installed before publishing. Trusted publishing needs npm 11.5.1 or
  later and Node.js 22.14.0 or later; the job uses Node.js 24.
- The `repository.url` in `js/package.json` is
  `git+https://github.com/link-foundation/meta-language.git`, the repository that
  provenance names.
- `js/scripts/prepare-npm-auth.mjs` removes the placeholder `_authToken` that
  `actions/setup-node` writes. Otherwise npm would skip the OIDC exchange
  ([troubleshooting](troubleshooting.md#npm-publishing-fails-with-e404-or-eneedauth)).

## Verifying the exchange

npm treats a refused exchange as "no credential" and reports only `ENEEDAUTH`.
The step **Verify the trusted-publisher exchange** therefore runs
`node scripts/prepare-npm-auth.mjs --verify-exchange` before `npm publish`, and
the step:

1. requests a GitHub OIDC token with audience `npm:registry.npmjs.org`;
2. prints the claims npm matches (`repository`, `workflow_ref`, `ref`,
   `event_name`, `environment`). These claims contain no credential;
3. posts the token to
   `https://registry.npmjs.org/-/npm/v1/oidc/token/exchange/package/meta-language`,
   which is the exchange `npm publish` performs, and discards the resulting
   token;
4. fails with the registry's own answer and the settings table above when the
   registry refuses the exchange.

## Publishing versions that were missed

The registry served only `0.46.0` while the trusted publisher was missing. Once
the settings above are saved, publish the latest release from its tag:

```bash
gh workflow run js.yml --ref v0.58.2 -f release_version=0.58.2
```

Each release after that is published by the dispatch in step 2.

## Reference

- [npm trusted publishers](https://docs.npmjs.com/trusted-publishers)
- [npm trusted publishing with OIDC is generally available](https://github.blog/changelog/2025-07-31-npm-trusted-publishing-with-oidc-is-generally-available/)
- [Troubleshooting: npm publishing fails with E404 or ENEEDAUTH](troubleshooting.md#npm-publishing-fails-with-e404-or-eneedauth)
