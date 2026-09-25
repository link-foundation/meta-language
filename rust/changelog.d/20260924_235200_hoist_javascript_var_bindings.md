---
bump: patch
---

### Fixed
- Resolve JavaScript `var` declarations in function scope, including references before the declaration, so binding-aware rename updates the intended references in both packages.
