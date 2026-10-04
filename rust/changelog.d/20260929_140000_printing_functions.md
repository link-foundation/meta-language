---
bump: patch
---

### Added

- Translate `console.log` inside JavaScript functions, in both runtimes: Rust and JavaScript print where the source prints, and the Lean and Rocq targets thread the lines printed through every function that prints, directly or through a function it calls, as a value paired with its result in a generated data type, with a corpus program whose native Rust, Lean, Rocq and JavaScript runs match Node.
- Lean checks an assertion over a mutually recursive or general-recursive function when `main` runs, since the kernel cannot unfold a partial def.
