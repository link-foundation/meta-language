---
bump: patch
---

Exclude the pinned Lean and Rocq foreign parsers from production Rust compilation
and crate archives, and their WebAssembly assets from the npm package. Retain
both independent parsers in development scope, with their original source bytes,
MSVC UTF-8 handling and closed decompression streams. Ordinary parsing continues
to execute the shipped native Links grammars.
