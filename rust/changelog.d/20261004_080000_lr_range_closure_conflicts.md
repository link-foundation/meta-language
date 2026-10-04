---
bump: patch
---

### Fixed
- The native executor settles the shifts and reductions an LR parser resolves by precedence the way tree-sitter does: a closure `|a| b` and an or-pattern `|a|b` that start together and end their leftmost chains apart keep the result the parser would shift into, a node that nests the other from the same offset keeps its tokens aligned, and a bare right operand such as the `..` of `a ..= ..` stands only where no reduction of the same precedence expression ends just before it. Rust sources with closures inside or-patterns and runs of bare, prefix and postfix ranges now give the tree-sitter trees, and the JS and Rust runtimes agree on them.
