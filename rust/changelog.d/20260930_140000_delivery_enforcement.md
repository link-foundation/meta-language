---
bump: patch
---

### Changed

- Add read-only live inspection of the default-branch aggregate merge rule, its bypass settings, and a failed check on the current PR revision. Preserve the GitHub responses with acceptance evidence; stale or incomplete probes remain unverified.
- Reject dirty or mislabeled evidence checkouts before executing suites or packing candidates, verify the checkout again afterwards, and allow the scope comparison to read the complete ledger beyond Node's default subprocess buffer.
- Add `npm run check:dependencies:delivery`, which rejects stale retained dependencies and generated descendants even when their compatibility reasons pass inventory validation. Outstanding upgrades keep their delivery assertions unverified.
- Upgrade six npm build resolutions, including Peggy's compiler utilities and PDF compression and TypeScript helpers, with executable parser, source-map, CLI and PDF interoperability regressions. Clean consumers still require separate verification because repository overrides do not propagate to installed packages.
- Run JavaScript test files sequentially to bound simultaneous grammar-runtime allocations during local verification.
