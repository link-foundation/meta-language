---
bump: patch
---

Keep the Python Tree-sitter parser as a development oracle dependency and remove
its production grammar dispatcher entry. Ordinary Python parsing uses the
shipped generated Links grammar. Check the package manifest boundary in
JavaScript and verify the production grammar dispatcher has no Python oracle.
