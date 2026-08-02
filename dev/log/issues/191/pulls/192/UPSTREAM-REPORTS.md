# Upstream reports for issue #191

The npm publishing defect analysed in `ANALYSIS.md` is not specific to
`meta-language`. This file records where it was reported and the exact content of
each report, so the reports stay reproducible from this repository.

## 1. `actions/setup-node` — already reported and fixed upstream

No new report is warranted; the bug is known and the fix is merged:

* [#1551](https://github.com/actions/setup-node/issues/1551) — closed as a
  duplicate of [#1440](https://github.com/actions/setup-node/issues/1440).
* [PR #1558](https://github.com/actions/setup-node/pull/1558) — removes the
  dummy `NODE_AUTH_TOKEN` export; merged 2026-05-28.

What is *not* obvious from those threads, and is worth adding as a data point, is
that the fix shipped only on the v7 line: `dist/setup/index.js` on `v7` no longer
contains the `XXXXX-XXXXX-XXXXX-XXXXX` placeholder, while `v6` (v6.5.0, what the
floating `@v6` tag resolves to) still does. Projects pinned to `@v6` therefore
remain affected even though the issue is closed.

## 2. `link-foundation/js-ai-driven-development-pipeline-template` — affected

The template ships `scripts/sanitize-npm-userconfig.mjs`, which removes only the
deprecated `always-auth` key and deliberately leaves the `_authToken` line intact:

```js
const ALWAYS_AUTH_LINE = /^[^\S\r\n]*always-auth[^\S\r\n]*=.*(?:\r?\n|$)/gim;
```

Its release workflow uses `actions/setup-node` with `registry-url` and relies on
OIDC trusted publishing, so every project generated from it inherits the same
failure whenever no `NPM_TOKEN` bootstrap secret is configured.

The Rust and Python templates are **not** affected: neither contains `npm publish`
nor `registry-url` in any workflow (verified by grep over
`.github/workflows/` in both clones).

### Report content

**Title:** `sanitize-npm-userconfig.mjs leaves the setup-node placeholder _authToken, breaking OIDC trusted publishing`

**Body:**

> `scripts/sanitize-npm-userconfig.mjs` removes the deprecated `always-auth` key but
> keeps the `_authToken` line that `actions/setup-node` writes. When a project has no
> `NPM_TOKEN` secret and relies on OIDC trusted publishing, that leftover line makes
> npm believe it is already authenticated, so it never performs the OIDC exchange and
> the registry rejects the anonymous upload with a misleading `E404`.
>
> **Reproduction**
>
> 1. Generate a project from this template; do not set an `NPM_TOKEN` secret.
> 2. Configure a trusted publisher for the package on npmjs.com.
> 3. Trigger the release workflow.
>
> The publish step fails with:
>
> ```
> npm error code E404
> npm error 404 Not Found - PUT https://registry.npmjs.org/<package>
> ```
>
> and every step of the job shows `NODE_AUTH_TOKEN: XXXXX-XXXXX-XXXXX-XXXXX` in its
> environment, even though the workflow sets no token. The npm user config at
> `$NPM_CONFIG_USERCONFIG` contains
> `//registry.npmjs.org/:_authToken=${NODE_AUTH_TOKEN}`.
>
> **Root cause**
>
> `actions/setup-node` with `registry-url` unconditionally writes that line and
> exports the placeholder — see actions/setup-node#1440 and #1551. The fix
> (actions/setup-node#1558) shipped only on the v7 line; `@v6` still contains the
> placeholder in `dist/setup/index.js`.
>
> **Workaround**
>
> Strip the line after `setup-node` and before `npm publish`:
>
> ```bash
> sed -i '/_authToken/d' "$NPM_CONFIG_USERCONFIG"
> ```
>
> **Suggested fix in code**
>
> Extend `sanitize-npm-userconfig.mjs` to remove the `_authToken` line as well,
> unless a real credential is present. A token is "real" only when it is non-empty
> and is neither `XXXXX-XXXXX-XXXXX-XXXXX` nor an unexpanded `${NODE_AUTH_TOKEN}`:
>
> ```js
> const AUTH_TOKEN_LINE = /^[^\S\r\n]*(?:\/\/[^\s=]*:)?_authToken[^\S\r\n]*=.*(?:\r?\n|$)/gim;
>
> export function sanitizeNpmrc(content, { keepAuthToken = false } = {}) {
>   const withoutAuthToken = keepAuthToken ? content : content.replace(AUTH_TOKEN_LINE, '');
>   return withoutAuthToken.replace(ALWAYS_AUTH_LINE, '');
> }
> ```
>
> It is also worth failing fast with actionable guidance when neither a usable token
> nor OIDC request variables (`ACTIONS_ID_TOKEN_REQUEST_URL` /
> `ACTIONS_ID_TOKEN_REQUEST_TOKEN`) are available, so the failure names the missing
> configuration instead of surfacing a bare `E404`. Moving the template to
> `actions/setup-node@v7` fixes the root cause too, but the sanitiser change is
> version-agnostic and protects projects that are still pinned to `@v6`.
>
> A working implementation, with tests, is in
> `js/scripts/prepare-npm-auth.mjs` of link-foundation/meta-language (PR #192).

**Filed as:** https://github.com/link-foundation/js-ai-driven-development-pipeline-template/issues/120
