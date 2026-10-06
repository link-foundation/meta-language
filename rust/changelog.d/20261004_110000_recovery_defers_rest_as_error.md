---
bump: patch
---

### Fixed
- Automatic recovery no longer takes all the input after an early error as one ERROR when a later error could be repaired where it is. When the cheapest complete result of a repair round takes the rest of the input as ERROR at a repair point while the result that reaches farthest ends past that point and could still complete for less, the round asks for that end as the next repair point; the complete result stands when the rounds end. `struct S { a: u8,, }` followed by an impl with a stray `]` is now repaired with one ERROR at the comma and one at the bracket, as tree-sitter repairs it, and twenty Rust conformance and generative recoveries become local. The recorded repairs of three native Racket and Rust fixtures follow; the JS and Rust executors defer alike.
