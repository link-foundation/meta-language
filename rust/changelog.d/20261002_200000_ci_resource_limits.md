---
bump: patch
---

### Changed
- The issue #195 evidence runs as separate CI stage jobs: the JavaScript suite, the Rust suite, runtime parity, one job per native translation target (JavaScript, Rust, Lean, Rocq), delivery and merge enforcement. Each stage uploads its own record and logs. The Full Requirements Aggregate only merges the stage outputs and evaluates them. A stage that fails or never reports is one gate error naming the stage, not a failure of every row it feeds.
- A local evidence run executes its stages one after another with `CARGO_BUILD_JOBS=2`, `RUST_TEST_THREADS=2` and `CARGO_INCREMENTAL=0` unless the caller set them. It builds only the default Rust features. A native validation stage deletes the compiler outputs no cell cites.
- The Rust runtime probe (`examples/issue_195_runtime_probe.rs`) streams its observation as NDJSON to the file given as its argument. The parity check reads that file without a large output buffer, compares the runtimes entry by entry, and keeps one digest per entry plus the full entries only where the runtimes differ. Without an argument, the probe still prints the whole observation.
- `docs/vision.md` has a Resource limits section. The ledger has one row each for the compile gate, the test matrices, the evidence stages, workflow concurrency, the agent resource rules, the bounded sequential evidence run, lazy grammars, the grammar tiering budget, inline-parser reuse, source-boundary checkpoints, parity digests, the cleanup wrapper and the native output cleanup.
