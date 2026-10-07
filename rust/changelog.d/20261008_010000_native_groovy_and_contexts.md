---
bump: patch
---

### Added

Native Groovy parsing now uses the pinned grammar imported into shared Links
Notation, with its Tree-sitter parser moved to development-only oracles.
Focused parity fixtures cover literals, declarations, control flow, closures,
collections, comments, input boundaries and lossless invalid-source recovery.
Reviewed grammar transformations also reconcile CSS selector spans and
arguments, PowerShell argument attachments, and Erlang attributes, sigils and
macro replacement contexts. Preserve the legitimate empty symbol name of
an upstream end sentinel instead of the Web Tree-sitter wrapper's fallback
error name.
