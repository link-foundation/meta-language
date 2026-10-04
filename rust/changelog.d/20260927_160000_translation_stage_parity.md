---
bump: patch
---

### Fixed
- Report an empty or invalid radix BigInt literal such as `0xn` as a syntax diagnostic at the literal in both packages, instead of a raw `SyntaxError` crash in JavaScript and a span-less error in Rust.
- Run the JavaScript/Rust translation stage parity test over all 1770 recorded programs in ordinary CI; it was previously ignored.
