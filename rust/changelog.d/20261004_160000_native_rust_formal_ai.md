---
bump: patch
---

### Fixed
- The native Rust grammar now builds the tree-sitter oracle rows for the formal-ai sources the downstream workload parses. Both runtimes made four fixes:
  - A keyword makes a span keyword-only only where it matched in the tree's parse state. Before, `m!('"') //"` followed by `type A = _;` was rejected.
  - Where two parses part deep in shared nodes, the shift order compares the pair of nodes that ends apart (`g(|| a, |p| p)`, `a + b..*c`).
  - A node that ends a silent rule keeps the rule's reduction precedence, as a token does (`if let A = b && !c && d {}`).
- The native Rust fixture keeps these sources as matches.
