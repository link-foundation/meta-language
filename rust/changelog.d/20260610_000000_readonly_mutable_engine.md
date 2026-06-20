---
bump: minor
---

### Added
- `AccessMode { Mutable, ReadOnly }` setting on `ParseConfiguration`
  (`with_access_mode` / `access_mode`), defaulting to `Mutable` so existing
  callers are unaffected.
- `LinkNetwork::freeze` / `as_read_only` yielding a `ReadOnlyNetwork` view that
  exposes only `&self` operations (query, project, reconstruct, verify,
  serialize); mutators are unreachable at compile time because the view never
  hands out `&mut LinkNetwork`.
- `LinkNetwork::parse_engine`, returning an `EngineNetwork` handle that honours
  the configured access mode: read-only parsing returns the frozen form and
  `EngineNetwork::as_mutable` rejects mutation with a `ReadOnlyViolation`
  diagnostic.
- Snapshot interop: `NetworkSnapshot::as_read_only` / `from_read_only` reuse the
  snapshot's `Arc<LinkNetwork>`, so the frozen form composes with snapshot
  versioning instead of duplicating it.
