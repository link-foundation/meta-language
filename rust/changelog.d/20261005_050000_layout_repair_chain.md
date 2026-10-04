---
bump: patch
---

### Fixed

- Error recovery no longer repairs again and again at one offset after a
  MISSING leaf that tokens the external scanner scanned of no width follow, as
  a layout token opening an indented block does: each repaired block opened
  another up to the scanner's deepest indentation, so recovering a Lean source
  ran out of steps. The default step budget of a parse with error recovery is
  twice that of one without, since its rounds parse each repaired alternative
  too.
