---
bump: patch
---

### Fixed
- The release pipeline dispatches the npm publisher on the release tag, so npm receives the merged commit the crate was published from.
- The npm publish job performs the trusted-publisher exchange before `npm publish` and reports the registry's answer and the matched OIDC claims instead of a bare `ENEEDAUTH`.

### Added
- `docs/ci-cd/npm-trusted-publishing.md` lists the exact trusted-publisher settings on npmjs.com (organization `link-foundation`, repository `meta-language`, workflow filename `js.yml`, no environment).
