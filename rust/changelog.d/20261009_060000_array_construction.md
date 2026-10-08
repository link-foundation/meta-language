---
bump: minor
---

Translate dense immutable array copying with `slice()` and single-array
`Array.from()`, and homogeneous `Array.of()` construction, through the shared
checked array representation. Preserve order, nested and empty arrays and single
evaluation of effectful inputs in both runtimes. Generate the method and
argument decisions into Rust. Retain explicit diagnostics for indexed slices,
non-array iterators, array-like objects and mapping callbacks.
