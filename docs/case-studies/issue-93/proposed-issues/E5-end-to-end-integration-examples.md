# E5 — End-to-end integration tests + `examples/`

> **Epic:** E — Tooling, integration, benchmarking · **Blocked by:** [`C4`](./C4-rust-parser-codegen.md), [`C5`](./C5-javascript-parser-codegen.md), [`D5`](./D5-blackbox-cfg-inference.md), [`E1`](./E1-cli-grammar-subcommands.md) · **Blocks:** —
> **Requirements:** P-7 · **Milestone:** M5
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic E (E5), §4 DAG (`C4,C5 → E5`),
> [`requirements.md`](../requirements.md) P-7.

## Context

Requirement **P-7** is "a **working Rust implementation**"
([`requirements.md`](../requirements.md) P-7). Every other issue ships a unit-tested
slice — the IR ([`A1`](./A1-grammar-ir.md)), importers ([`B1`](./B1-bnf-importer.md)–B7),
emitters ([`C1`](./C1-bnf-ebnf-abnf-emitters.md)–[`C3`](./C3-gbnf-emitter.md)), codegen
([`C4`](./C4-rust-parser-codegen.md), [`C5`](./C5-javascript-parser-codegen.md)), the
inference engine ([`D5`](./D5-blackbox-cfg-inference.md)), and the CLI
([`E1`](./E1-cli-grammar-subcommands.md)). What proves the *whole* is working is the
full pipeline run as a single artefact: **positive examples → infer a grammar
([`D5`](./D5-blackbox-cfg-inference.md)) → emit a parser ([`C4`](./C4-rust-parser-codegen.md)/[`C5`](./C5-javascript-parser-codegen.md))
or a GBNF grammar ([`C3`](./C3-gbnf-emitter.md)) → re-parse the original examples and
confirm they still parse.** That is the end-to-end loop P-7 demands, and nothing
exercises it until the pieces it depends on land — hence E5 is the integration
capstone of Epic E, blocked by exactly the four issues that complete the pipeline.

The repo already keeps runnable, compiled-checked demos under
[`./examples`](../../../../examples) (six today, e.g.
`examples/document_formatting_round_trip.rs`,
`examples/custom_parser_registry.rs`) and end-to-end integration tests under
`tests/integration/` (`tests/integration/cli.rs` drives the built binary via
`env!("CARGO_BIN_EXE_meta-language")`). E5 adds the grammar-pipeline members of
both, following those exact conventions.

## Goal

Deliver **runnable `examples/` programs** and **`tests/integration/` tests** that
execute the complete grammar pipeline end-to-end on small fixed corpora and assert
each stage's output, so a reader can `cargo run --example …` to watch
examples-in → grammar-out → parser-out → re-parse-ok, and CI proves the integrated
system works (P-7).

## Scope

**In scope**
- Three example programs under `./examples` (see Design):
  - `grammar_infer_and_emit_rust.rs` — examples → [`D5`](./D5-blackbox-cfg-inference.md)
    → [`C4`](./C4-rust-parser-codegen.md) Rust parser source → compile+run that parser
    over the originals.
  - `grammar_infer_and_emit_js.rs` — examples → [`D5`](./D5-blackbox-cfg-inference.md)
    → [`C5`](./C5-javascript-parser-codegen.md) JS parser source → run it under Node
    (skip gracefully if `node` is absent).
  - `grammar_infer_and_emit_gbnf.rs` — examples → [`D5`](./D5-blackbox-cfg-inference.md)
    → [`C3`](./C3-gbnf-emitter.md) GBNF text → re-parse via the
    [`B7`](./B7-lark-gbnf-importer.md) GBNF importer and confirm the round-trip.
- Integration tests under `tests/integration/` (registered in
  `tests/integration/mod.rs`) asserting each pipeline end-to-end without printing,
  including one that drives the [`E1`](./E1-cli-grammar-subcommands.md) CLI as a
  subprocess (the `infer` → `emit-grammar` path) the way `tests/integration/cli.rs`
  drives `describe`/`verify`.
- A tiny shared corpus of positive examples per pipeline (under
  `tests/fixtures/grammar/corpora/`), small enough to infer deterministically.

**Out of scope** (owned elsewhere)
- The inference engine itself, its metrics, and its determinism guarantees →
  [`D5`](./D5-blackbox-cfg-inference.md), [`D1`](./D1-inference-evaluation-harness.md).
- The emitters / codegen → [`C3`](./C3-gbnf-emitter.md),
  [`C4`](./C4-rust-parser-codegen.md), [`C5`](./C5-javascript-parser-codegen.md).
- The CLI argument surface → [`E1`](./E1-cli-grammar-subcommands.md) (E5 only *drives*
  it).
- Competitor corpora and the P-12 benchmark gate → [`E3`](./E3-competitor-benchmark-suite.md)
  (E5 uses *toy* corpora to prove the wiring, not to win metrics).
- Round-trip *fidelity matrices* for grammar formats → [`F2`](./F2-grammar-format-fidelity-matrix.md).

## Design / specification

### Pipeline shape (shared by all three examples)

Each example is a `fn main()` that runs the same five stages and `assert!`s at each
boundary, modeled structurally on `examples/document_formatting_round_trip.rs`
(seed → transform → assert round-trip, with `println!` narration):

1. **Corpus.** A fixed `const EXAMPLES: &[&str]` of ~5–10 positive strings in one
   tiny language (see corpora below) — inlined in the example so it is
   self-contained and reproducible.
2. **Infer.** Call the [`D5`](./D5-blackbox-cfg-inference.md) entry point
   (`infer_grammar(&EXAMPLES, …) -> Grammar`) to get an [`A1`](./A1-grammar-ir.md)
   `Grammar` with `source_format = Some(GrammarFormat::Inferred)`.
3. **Validate.** Run [`E4`](./E4-grammar-authoring-ergonomics.md) `validate(&grammar)`
   and assert no `Error` diagnostics (the inferred grammar is well-formed).
4. **Emit.** Produce the target artefact:
   - Rust: `emit_rust_parser(&grammar) -> String` ([`C4`](./C4-rust-parser-codegen.md));
   - JS: `emit_js_parser(&grammar) -> String` ([`C5`](./C5-javascript-parser-codegen.md));
   - GBNF: `emit_gbnf(&grammar) -> String` ([`C3`](./C3-gbnf-emitter.md)).
5. **Re-parse / verify.** Feed every original example back through the emitted
   artefact and assert it parses (the *closing the loop* step that proves P-7):
   - **Rust:** write the emitted source to a temp crate/file, compile it (or, if
     [`C4`](./C4-rust-parser-codegen.md) targets an in-process runtime such as a
     `pest_meta`/`winnow` form, instantiate it directly per C4's documented entry
     point), and assert each `EXAMPLES[i]` parses without error.
   - **JS:** write the emitted parser to a temp `.js`, run it under `node` over each
     example; if `node` is not on `PATH`, the example prints a skip notice and exits
     0 (CI without Node still builds the example via `--all-targets`).
   - **GBNF:** re-import the emitted GBNF with [`B7`](./B7-lark-gbnf-importer.md)
     `import_gbnf(&text)` back into an [`A1`](./A1-grammar-ir.md) `Grammar` and assert
     it is structurally equivalent to the inferred grammar (the source-of-truth
     in-process check that does not require llama.cpp).

Each example ends by printing the inferred grammar (as
[`A2`](./A2-grammar-surface-syntax.md) surface via `write_grammar_surface`) and the
emitted artefact, so `cargo run --example grammar_infer_and_emit_rust` is a readable
demonstration.

### Corpora (toy, deterministic)

Small enough that [`D5`](./D5-blackbox-cfg-inference.md) infers a stable grammar:

| Corpus | Examples (positive only) | Exercises |
|---|---|---|
| `arith` | `"1+2"`, `"3*4"`, `"1+2*3"`, `"(1+2)*3"`, `"7"` | recursion, precedence, the meta-notation bracket prior ([`D6`](./D6-delimiter-structural-prior.md)) |
| `csv-row` | `"a,b,c"`, `"x"`, `"1,2"`, `",,"` | repetition / separators |
| `json-ish` | `"{}"`, `"{\"a\":1}"`, `"[1,2]"`, `"true"` | nesting, the bracket/quote skeleton |

Corpora are stored under `tests/fixtures/grammar/corpora/<name>.txt` (one example
per line) **and** inlined in the example program (so the example needs no fixture
path). The integration tests load the fixture files.

### Integration tests (`tests/integration/`)

One test module per pipeline plus one CLI driver, each a plain `#[test]` that runs
the pipeline in-process and asserts — no `println!`, no manual inspection:

- `grammar_pipeline_rust.rs::infer_emit_rust_reparses_examples` — runs stages 1–5
  for the `arith` corpus and asserts every example re-parses through the
  [`C4`](./C4-rust-parser-codegen.md) output.
- `grammar_pipeline_js.rs::infer_emit_js_reparses_examples` — same for JS; **`#[test]`
  is `#[ignore]`d unless `node` is detected** (so default `cargo test` is
  hermetic), with a documented `cargo test -- --ignored` opt-in.
- `grammar_pipeline_gbnf.rs::infer_emit_gbnf_round_trips` — emit GBNF, re-import via
  [`B7`](./B7-lark-gbnf-importer.md), assert structural equality with the inferred
  grammar (fully in-process).
- `grammar_cli_pipeline.rs::cli_infer_then_emit_grammar` — drives the
  [`E1`](./E1-cli-grammar-subcommands.md) binary as a subprocess
  (`env!("CARGO_BIN_EXE_meta-language")`, exactly as `tests/integration/cli.rs:7`):
  `infer --examples <fixture>` piped/handed to `emit-grammar --format gbnf`, asserting
  the process exits 0 and stdout is a non-empty GBNF grammar that
  [`B7`](./B7-lark-gbnf-importer.md) re-imports.

`tests/integration/mod.rs` (currently `mod cli;`) gains the four new module
declarations.

## File-level plan

| File | Change |
|---|---|
| `examples/grammar_infer_and_emit_rust.rs` | New. examples → D5 → C4 → compile/run → re-parse; narrated with `println!`. |
| `examples/grammar_infer_and_emit_js.rs` | New. examples → D5 → C5 → Node run (skip if `node` absent). |
| `examples/grammar_infer_and_emit_gbnf.rs` | New. examples → D5 → C3 → B7 re-import round-trip. |
| `tests/integration/grammar_pipeline_rust.rs` | New. In-process Rust-pipeline assertion. |
| `tests/integration/grammar_pipeline_js.rs` | New. JS pipeline, `#[ignore]` unless `node` present. |
| `tests/integration/grammar_pipeline_gbnf.rs` | New. GBNF emit→re-import equality. |
| `tests/integration/grammar_cli_pipeline.rs` | New. Drives the E1 CLI subprocess. |
| `tests/integration/mod.rs` | Add `mod grammar_pipeline_rust; mod grammar_pipeline_js; mod grammar_pipeline_gbnf; mod grammar_cli_pipeline;`. |
| `tests/fixtures/grammar/corpora/` | New. `arith.txt`, `csv-row.txt`, `json-ish.txt` (one positive example per line). |
| `changelog.d/` | Add a fragment (see `scripts/check-changelog-fragment.rs`). |

## Reuse

- Existing `examples/` convention — `examples/document_formatting_round_trip.rs`
  (round-trip-assert-and-narrate shape) and `examples/custom_parser_registry.rs`;
  examples are compiled by `cargo clippy --all-targets` and built/run-checked by
  `cargo test --all-features` (CI `release.yml` clippy step at line 173, test job at
  line 180). See [`README.md`](../../../../README.md) §Development.
- Existing integration-test convention — `tests/integration/cli.rs` (subprocess via
  `env!("CARGO_BIN_EXE_meta-language")`) and `tests/integration/mod.rs`.
- [`D5`](./D5-blackbox-cfg-inference.md) `infer_grammar`, [`C3`](./C3-gbnf-emitter.md)
  `emit_gbnf`, [`C4`](./C4-rust-parser-codegen.md) `emit_rust_parser`,
  [`C5`](./C5-javascript-parser-codegen.md) `emit_js_parser`,
  [`B7`](./B7-lark-gbnf-importer.md) `import_gbnf`, [`E1`](./E1-cli-grammar-subcommands.md)
  CLI, [`E4`](./E4-grammar-authoring-ergonomics.md) `validate`,
  [`A2`](./A2-grammar-surface-syntax.md) `write_grammar_surface` — E5 only *wires*
  these; it implements no new pipeline logic.
- `std::process::Command` for the JS / CLI subprocesses (as in
  `tests/integration/cli.rs:1`); `std::env::temp_dir` for emitted-source files; no
  new dependency.

## Acceptance criteria

- [ ] `cargo run --example grammar_infer_and_emit_rust` prints the inferred grammar
      and emitted Rust, and exits 0 after re-parsing every `arith` example.
- [ ] `cargo run --example grammar_infer_and_emit_gbnf` exits 0; the emitted GBNF
      re-imports via [`B7`](./B7-lark-gbnf-importer.md) to a grammar structurally
      equal to the inferred one.
- [ ] `cargo run --example grammar_infer_and_emit_js` exits 0 whether or not `node`
      is installed (runs the parser when present, prints a skip notice otherwise).
- [ ] All four integration tests pass under `cargo test --all-features`; the JS test
      is `#[ignore]`d unless `node` is detected, and is documented as opt-in via
      `cargo test -- --ignored`.
- [ ] `grammar_cli_pipeline` drives the real [`E1`](./E1-cli-grammar-subcommands.md)
      binary end-to-end (infer → emit-grammar) and asserts a non-empty, re-importable
      GBNF on stdout.
- [ ] For each corpus, **every** positive example re-parses through the emitted
      artefact (the closing-the-loop P-7 assertion).
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy
      pedantic/nursery are `warn` per `Cargo.toml:103-106`, and `--all-targets`
      lints the new examples), and `cargo test --all-features` all pass;
      `rust-script scripts/check-no-src-tests.rs` passes (tests live under `tests/`,
      examples under `examples/`, none under `src/`).

## Tests

The deliverable *is* tests + examples; the integration tests above are the test
plan. Additional assertions:

- Determinism: running each pipeline twice over the same corpus yields the same
  inferred grammar (relies on [`D5`](./D5-blackbox-cfg-inference.md) determinism) — a
  test asserts `infer(corpus) == infer(corpus)`.
- Negative guard: a string *not* in the language (e.g. `"1++"`) is parsed by the
  emitted artefact and asserted to **fail** (the grammar is not trivially
  permissive) — included where [`D5`](./D5-blackbox-cfg-inference.md)'s precision makes
  it stable; otherwise documented as a soft check.
- Each example fixture corpus is also covered by an in-process integration test (no
  reliance on `cargo run` succeeding in CI for correctness — the examples are for
  humans, the integration tests are the gate).

## References

- [`requirements.md`](../requirements.md) P-7; [`solution-plans.md`](../solution-plans.md)
  §Epic E (E5), §3 issue table (E5 row), §4 DAG (`C4,C5 → E5`), §5 milestone M5.
- Pipeline members: [`D5`](./D5-blackbox-cfg-inference.md),
  [`C3`](./C3-gbnf-emitter.md), [`C4`](./C4-rust-parser-codegen.md),
  [`C5`](./C5-javascript-parser-codegen.md), [`B7`](./B7-lark-gbnf-importer.md),
  [`E1`](./E1-cli-grammar-subcommands.md), [`E4`](./E4-grammar-authoring-ergonomics.md),
  [`A1`](./A1-grammar-ir.md), [`A2`](./A2-grammar-surface-syntax.md).
- Convention models: `examples/document_formatting_round_trip.rs`,
  `examples/custom_parser_registry.rs`, `tests/integration/cli.rs`,
  `tests/integration/mod.rs`; CI `release.yml` (clippy line 173, test line 180);
  [`README.md`](../../../../README.md) §Development.
