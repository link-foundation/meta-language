---
bump: patch
---

### Fixed
- `LinkNetwork::apply_edit` keeps the order of siblings that share a span, as two zero-width MISSING leaves at one point, so the edited network equals a fresh parse.
