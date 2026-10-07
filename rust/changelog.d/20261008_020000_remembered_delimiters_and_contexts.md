---
bump: patch
---

### Added

An exact text delimiter scanner family generates shared grammar data for
remembered opening labels, matching closing labels, Unicode content and
state reset between parses. JavaScript and Rust use the same generated
fixtures and existing grammar actions and predicates.

### Fixed

Preserve source lexer token identities when context guards reuse word
patterns. Input boundary preferences now handle trailing grammar extras
without adding a CST leaf. Reviewed context decisions reconcile Groovy
methods, functions, conditional bodies, labels, switches and newline
terminators, and Erlang declarations with leading whitespace. Shared
concept fixtures include the newly shared Groovy constructs.
