---
bump: minor
---

### Added

- Self-translation of meta-language's own modules between JavaScript, TypeScript and Rust through links: `self_translate(source, from, to)` and the `meta-language translate --to <language> [--from <language>] [--items] <file>` command. Within one language the source is written back byte for byte. Into the other language, each top-level item the portable core expresses is translated and every other item is carried verbatim in a marked comment. Both keep their source as provenance, so translating an unedited translation back restores the source byte for byte, and an edited translated item is translated again. The shared cases in `parity/self-translation` are checked in both runtimes.

### Changed

- The Rust emitter compares operands in place (`a == b`, and `s == "text"` for a string literal) instead of borrowing both sides, so clippy accepts the comparisons it writes.
