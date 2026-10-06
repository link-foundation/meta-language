---
bump: patch
---

### Fixed
- CMake parses with a vendored tree-sitter-cmake 0.7.5 whose external scanner starts with no bracket open. Upstream read uninitialised memory, so recovery trees differed between macOS/Windows and Linux/WebAssembly; both runtimes now compile the same patched scanner.
