# GBNF importer and emitter parity

Checks that `js/src/grammar-importers/gbnf.js` and `js/src/grammar-emitters/gbnf.js` match the Rust `grammar::import::gbnf` and `grammar::emit::gbnf` modules. Every case is imported or emitted by both runtimes, and the results are compared: rule renderings, docs, start rule, emitted text, lossy notes and error messages.

Run it from the repository root with `node experiments/gbnf-parity/compare.mjs`. The Rust side (`render.rs`) is built by `experiments/run-rust-experiment.mjs` in a scratch crate under the system temporary directory, so no Cargo package is kept in `experiments/`, and `--verbose` prints the Rust output.
