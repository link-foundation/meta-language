# E1 — CLI: infer / import / emit / translate-grammar

> **Epic:** E — Tooling, integration, benchmarking · **Blocked by:** [`A1`](./A1-grammar-ir.md), [`B1`](./B1-bnf-importer.md), [`C1`](./C1-bnf-ebnf-abnf-emitters.md), [`D5`](./D5-blackbox-cfg-inference.md)
> **Requirements:** P-1, P-7 · **Milestone:** M5
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic E,
> [`existing-capabilities.md`](../existing-capabilities.md) §1 (the CLI scaffold).

## Context

The whole grammar subsystem — IR (A1), importers (B1–B7), emitters (C1–C3, C7),
inference (D5) — is library-only until it is reachable from the command line.
P-1 ("easy to develop new grammars") and P-7 ("working Rust implementation")
require a usable tool, not just a crate API. Today `src/main.rs` exposes exactly
two `clap` subcommands — `Describe` and `Verify { language, text }`
(`src/main.rs:15-37`) — wired through `clap::{Parser, Subcommand}`
(`src/main.rs:1, 5-13`) and documented in `README.md` §CLI
(`cargo run -- describe`, `cargo run -- verify …`). There is **no CLI for
inference or grammar translation** (gap recorded in
[`existing-capabilities.md`](./../existing-capabilities.md) §3, row E1).

This issue adds four subcommands on that same scaffold — `infer`,
`import-grammar`, `emit-grammar`, `translate-grammar` — each a thin shell over an
existing library entry point. It follows the existing `describe`/`verify` style
exactly: a `Command` enum variant with `#[arg(long)]` flags, a small free
function per command, dispatched from `match cli.command`.

## Goal

Add four `clap` subcommands to `src/main.rs` so a user can, from the shell:
infer a grammar from example texts (D5), import a grammar from any supported
notation into the A1 IR / LiNo (B1–B7), emit an A1 grammar to any supported
notation (C1–C3, C7), and translate a grammar's human-facing surface across
natural languages (C6) — each with explicit flags, defaults, examples, and a
documented output format, matching the established `describe`/`verify` style.

## Scope

**In scope**
- Four new `Command` variants in `src/main.rs` and their handler functions.
- Argument parsing/validation via `clap` derive (`--format`, `--strategy`,
  `--metrics`, `--out`, `--start`, `--key`, `--from-language`/`--to-language`,
  positional input paths) with sensible defaults and clear `--help` text.
- Reading example files/dirs for `infer`; reading a grammar file for the others.
- Printing the result to stdout (or `--out <path>`) and a non-zero exit on error
  (mirroring `verify`'s `std::process::exit(1)`, `src/main.rs:53`).
- README §CLI updated with the new subcommands and examples.

**Out of scope** (owned elsewhere — E1 only *calls* them)
- The inference engine itself → **D5** (and D1–D9). E1 calls `infer_grammar(...)`.
- The importers → **B1–B7**. E1 calls `import_bnf(...)` etc.
- The emitters → **C1–C3, C7**. E1 calls `emit_bnf(...)` etc.
- Cross-language translation logic → **C6**. E1 calls the C6 entry point.
- Registering an inferred grammar as a live parser → **E2** (E1 may print a hint
  that `parse_with_registry` is available, but does not own that path).
- End-to-end example programs under `examples/` → **E5**.

## Design / specification

Extend the existing enum (do not restructure `Cli`; keep `Describe`/`Verify`):

```rust
#[derive(Subcommand, Debug)]
enum Command {
    /// Print the built-in self-description roots.            (existing)
    Describe,
    /// Parse text into a lossless token network and verify.  (existing)
    Verify { /* language, text — src/main.rs:20-27 */ },

    /// Infer a grammar from example texts (D5).
    Infer {
        /// One or more files or directories of example texts.
        #[arg(required = true)]
        examples: Vec<std::path::PathBuf>,
        /// Output grammar notation.
        #[arg(long, value_enum, default_value_t = GrammarFormatArg::Lino)]
        format: GrammarFormatArg,
        /// Inference strategy / pipeline depth.
        #[arg(long, value_enum, default_value_t = StrategyArg::Auto)]
        strategy: StrategyArg,
        /// Also print precision/recall/F1/runtime (D1 metrics) to stderr.
        #[arg(long)]
        metrics: bool,
        /// Write the grammar here instead of stdout.
        #[arg(long)]
        out: Option<std::path::PathBuf>,
    },

    /// Import a grammar in another notation into the IR / LiNo (B1–B7).
    ImportGrammar {
        /// Source notation of the input file.
        #[arg(long, value_enum)]
        format: ImportFormatArg, // bnf|ebnf|abnf|peg|antlr|lark|gbnf|tree-sitter
        /// Grammar file to import.
        #[arg(required = true)]
        input: std::path::PathBuf,
        /// Output notation for the imported grammar (default LiNo).
        #[arg(long, value_enum, default_value_t = GrammarFormatArg::Lino)]
        to: GrammarFormatArg,
        #[arg(long)]
        out: Option<std::path::PathBuf>,
    },

    /// Emit an IR/LiNo grammar to another notation (C1–C3, C7).
    EmitGrammar {
        /// Target notation.
        #[arg(long, value_enum)]
        format: EmitFormatArg, // bnf|ebnf|abnf|peg|gbnf|tree-sitter
        /// Grammar file (IR/LiNo) to emit from.
        #[arg(required = true)]
        input: std::path::PathBuf,
        #[arg(long)]
        out: Option<std::path::PathBuf>,
    },

    /// Translate a grammar's human-facing surface across languages (C6).
    TranslateGrammar {
        /// Grammar file (IR/LiNo) to translate.
        #[arg(required = true)]
        input: std::path::PathBuf,
        /// Source natural language (e.g. en).
        #[arg(long)]
        from_language: String,
        /// Target natural language (e.g. ru).
        #[arg(long)]
        to_language: String,
        #[arg(long)]
        out: Option<std::path::PathBuf>,
    },
}
```

`GrammarFormatArg` / `ImportFormatArg` / `EmitFormatArg` / `StrategyArg` are
`#[derive(ValueEnum)]` enums that map 1-to-1 onto A1's `GrammarFormat`
(`MetaLanguage, Bnf, Ebnf, Abnf, Peg, Antlr, Lark, Gbnf, TreeSitter, Inferred` —
A1 §Design) plus a `Lino` surface option. Keep the import/emit variants scoped to
formats whose importer/emitter exists at M5 (`bnf` via [`B1`](./B1-bnf-importer.md)
and `emit_bnf`/`emit_ebnf`/`emit_abnf` via [`C1`](./C1-bnf-ebnf-abnf-emitters.md)
are the hard dependencies; gate not-yet-implemented formats behind a clear
"unsupported format" error rather than a panic, and widen as B2–B7 / C2–C3 / C7
land).

Dispatch mirrors `src/main.rs:33-37`:

```rust
match cli.command {
    Command::Describe => describe(),
    Command::Verify { language, text } => verify(&language, &text),
    Command::Infer { examples, format, strategy, metrics, out } =>
        infer(&examples, format, strategy, metrics, out.as_deref()),
    Command::ImportGrammar { format, input, to, out } =>
        import_grammar(format, &input, to, out.as_deref()),
    Command::EmitGrammar { format, input, out } =>
        emit_grammar(format, &input, out.as_deref()),
    Command::TranslateGrammar { input, from_language, to_language, out } =>
        translate_grammar(&input, &from_language, &to_language, out.as_deref()),
}
```

Each handler is a free function (like `describe`/`verify`, `src/main.rs:39-56`):
read input, call the library, write the rendered grammar to `--out` or stdout,
and `eprintln!` + `std::process::exit(1)` on any `Err` (same pattern as
`verify`, `src/main.rs:47-54`).

### Usage examples (exact)

```bash
# Infer a grammar from a directory of correct example programs, emit LiNo to stdout.
cargo run -- infer examples/json-samples/

# Infer from explicit files, emit GBNF, print D1 metrics to stderr, write to a file.
cargo run -- infer a.txt b.txt c.txt --format gbnf --metrics --out json.gbnf

# Import a BNF grammar into the IR, rendered as LiNo (default).
cargo run -- import-grammar --format bnf grammars/postal.bnf

# Import an EBNF grammar and re-render it as BNF (cross-format round-trip).
cargo run -- import-grammar --format ebnf grammars/expr.ebnf --to bnf

# Emit an IR/LiNo grammar to GBNF for llama.cpp constraint use.
cargo run -- emit-grammar --format gbnf grammars/expr.lino --out expr.gbnf

# Translate a grammar's rule names/comments English -> Russian (structure preserved).
cargo run -- translate-grammar grammars/expr.lino --from-language en --to-language ru
```

### Output formats

- `infer` / `import-grammar` / `emit-grammar` / `translate-grammar` print the
  rendered grammar in the requested notation to stdout (default), or write it to
  `--out <path>`. The default surface is **LiNo** (the project's textual storage
  surface, P-8 — `src/lino_serialization.rs`), matching how `describe` emits
  LiNo-style lines that round-trip through `parse()`/`reconstruct_text()`
  (`README.md` §CLI).
- `infer --metrics` additionally writes a one-line precision/recall/F1/wall-clock
  summary to **stderr** (the D1 metric definitions, [`D1`](./D1-inference-evaluation-harness.md)),
  so stdout stays a clean grammar suitable for piping.
- Errors (parse failure, unsupported format, unreadable file) go to stderr; the
  process exits non-zero (mirrors `verify`, `src/main.rs:47-54`).

## File-level plan

| File | Change |
|---|---|
| `src/main.rs` | Add four `Command` variants + their `match` arms (`src/main.rs:16-37`), the `ValueEnum` arg enums, and the four handler functions modelled on `describe`/`verify` (`src/main.rs:39-56`). |
| `src/lib.rs` | No new public API required if importer/emitter/infer entry points are already re-exported (A1/B1/C1/D5); add a thin `cli` helper module only if shared rendering (IR↔LiNo↔format) warrants it — otherwise keep logic in `main.rs`. |
| `README.md` | Extend §CLI with the four new subcommands + the usage examples above. |
| `tests/integration/mod.rs` | Register CLI integration tests (invoke the built binary via `assert_cmd` or `std::process::Command`). |
| `tests/fixtures/grammar/` | Reuse B1/C1 fixtures; add a tiny example-text corpus for `infer`. |
| `changelog.d/` | Add a fragment (see CONTRIBUTING / `README.md` changelog section). |

## Reuse

- `clap::{Parser, Subcommand}` scaffold + the `describe`/`verify` handler pattern — `src/main.rs:1-56` (copy the enum-variant + free-function + `process::exit(1)` shape). `clap` is already a dependency with the `derive` feature (`Cargo.toml:36`).
- A1 `Grammar` + `GrammarFormat` + LiNo serialisation (`src/lino_serialization.rs`) — render/parse the IR — [`A1`](./A1-grammar-ir.md).
- `import_bnf` (and B2–B7 siblings) — [`B1`](./B1-bnf-importer.md).
- `emit_bnf`/`emit_ebnf`/`emit_abnf` (and C2/C3/C7 siblings) — [`C1`](./C1-bnf-ebnf-abnf-emitters.md).
- `infer_grammar(...)` + D1 metrics — [`D5`](./D5-blackbox-cfg-inference.md), [`D1`](./D1-inference-evaluation-harness.md).
- Cross-language translation entry point — [`C6`](./C6-concept-aligned-translation.md) (per [`solution-plans.md`](../solution-plans.md) §Epic C, C6 reuses `reconstruct_text_as`).

## Acceptance criteria

- [ ] `cargo run -- infer <dir>` reads example texts, runs D5, and prints a
      grammar in the default (LiNo) notation to stdout; `--format gbnf|bnf|…`
      switches the output notation; `--out <path>` writes to a file instead.
- [ ] `cargo run -- infer <files> --metrics` prints precision/recall/F1/wall-clock
      to **stderr** while stdout still carries only the grammar.
- [ ] `cargo run -- import-grammar --format bnf <file>` imports BNF (B1) and prints
      the IR rendered as LiNo; `--to bnf` re-renders via C1 (cross-format path).
- [ ] `cargo run -- emit-grammar --format gbnf <file>` emits GBNF (C3 once landed;
      `bnf`/`ebnf`/`abnf` via C1 are the M5-guaranteed targets).
- [ ] `cargo run -- translate-grammar <file> --from-language en --to-language ru`
      translates the grammar's human-facing surface (C6), preserving structure.
- [ ] Unsupported/not-yet-implemented `--format` values and unreadable inputs
      produce a clear stderr message and a non-zero exit (no panic), mirroring
      `verify` (`src/main.rs:47-54`).
- [ ] `--help` for each subcommand documents every flag and its default.
- [ ] README §CLI lists the four subcommands with the usage examples above.
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features` (clippy
      `pedantic`/`nursery` are `warn` per `Cargo.toml:103-106`), and
      `cargo test --all-features` all pass; `rust-script scripts/check-no-src-tests.rs`
      passes (tests live under `tests/`, not `src/`).

## Tests

- `tests/integration/` (invoke the built binary, e.g. via `std::process::Command`
  or `assert_cmd`):
  - `infer` over a fixture example-corpus → stdout is a non-empty grammar in the
    requested notation; exit code 0.
  - `infer --metrics` → stderr contains a precision/recall/F1 line; stdout is
    still a valid grammar.
  - `import-grammar --format bnf <fixture.bnf>` → stdout renders the imported IR;
    importing then `--to bnf` yields BNF equivalent to the input (cross-format).
  - `emit-grammar --format bnf <fixture.lino>` → stdout is valid BNF.
  - `translate-grammar … --from-language en --to-language ru` → rule structure
    unchanged, human-facing tokens translated.
  - bad `--format`, missing file, and empty examples each exit non-zero with a
    stderr message and no panic.
- Keep example corpora small and under `tests/fixtures/grammar/`.

## References

- CLI scaffold + handler style: `src/main.rs:1-56`; README §CLI.
- `clap` derive: <https://docs.rs/clap> (already a dependency, `Cargo.toml:36`).
- Dependencies: [`A1`](./A1-grammar-ir.md), [`B1`](./B1-bnf-importer.md),
  [`C1`](./C1-bnf-ebnf-abnf-emitters.md), [`D5`](./D5-blackbox-cfg-inference.md),
  [`D1`](./D1-inference-evaluation-harness.md),
  [`C6`](./C6-concept-aligned-translation.md), [`E2`](./E2-inferred-grammar-runtime-parser.md).
- [`existing-capabilities.md`](../existing-capabilities.md) §1, §3 (E1 row),
  [`solution-plans.md`](../solution-plans.md) §Epic E (E1).
