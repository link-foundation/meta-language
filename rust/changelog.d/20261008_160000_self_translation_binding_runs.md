---
bump: patch
---

### Fixed
- Retry contiguous JavaScript declarations in one checked binding scope when an isolated item fails type resolution, enabling sibling, forward and arrow-bound calls during self-translation.
- Preserve byte-exact provenance for successful runs and retain individual carried decisions when their combined scope cannot translate.
- Refresh the Dart default grammar digest and canonical concept ordering to match reproducible generation.
