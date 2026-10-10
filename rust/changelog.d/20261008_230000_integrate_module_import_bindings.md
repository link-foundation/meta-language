---
bump: patch
---

### Fixed

- Integrate main's sibling-item and relative-import binder with declaration-scope retries. Expose eligible exported function signatures from successful retries and preserve source-error restoration. Keep nonliteral constant captures unbound until eager initialization semantics are represented, using the generated shared eligibility rule.
