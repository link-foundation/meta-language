---
bump: patch
---

### Added

- The JavaScript frontend translates `let` bindings, assignments (`=`, the portable compound assignments, `++` and `--`), `while`, `do … while` and `for` loops, and `break` and `continue`, which it used to reject. An assignment binds a new variable, each loop is lifted to a generated tail-recursive function of the variables it uses, and statements that continue at several places are joined by a generated data type. The Rust and JavaScript emitters run a lifted loop as a loop, so a hundred thousand iterations need no hundred thousand stack frames. The Rust frontend mirrors the JavaScript one stage by stage, and `var`, labels, `for…of`, uninitialised `let` and assignments in the temporal dead zone are rejected with the construct and its span.
