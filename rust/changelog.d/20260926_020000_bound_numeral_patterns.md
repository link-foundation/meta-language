---
bump: patch
---

### Fixed
- Compile natural-number numeral patterns above 16 as equality tests and reject `n + k` patterns with offsets above 16, so a large numeral no longer unfolds into a successor chain with one node per unit. Mixed numeral and successor matches keep their meaning in both packages.
