---
bump: patch
---

### Added

- JavaScript arrow functions and function expressions bound to top-level constants (`const inc = x => x + 1;`, `const f = function (n) { … };`) translate as functions in both runtimes, with an expression or block body, before the first top-level statement; a corpus program of them runs natively in Rust, Lean, Rocq and JavaScript with Node's output.
