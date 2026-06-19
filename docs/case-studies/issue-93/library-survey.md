<!-- Source of truth: this file is the issue #93 library/ecosystem survey.
     Compiled by automated research agents and verified on 2026-06-19. -->

# Survey: Existing Libraries, Tools & Ecosystem Repos for Grammar Extensibility & Grammar Inference

> Compiled 2026-06-19 for the `link-foundation/meta-language` project ([issue #93](https://github.com/link-foundation/meta-language/issues/93): "Easy grammar extensibility and grammar inference"). All licenses, versions, and star/activity figures were verified against the crates.io JSON API, docs.rs, lib.rs, GitHub/GitHub-API, RFC-Editor, W3C, and ISO catalog pages on this date. GitHub star counts are GitHub's own rounded figures and are approximate. **Permissive = MIT / Apache-2.0 / BSD / Unlicense (reusable); copyleft = GPL / AGPL / LGPL (flagged); CC-NC and "no license" = not reusable as code.**

**Reading guide.** The project's stated goal (issue #93) is a Rust implementation that (i) infers a grammar from examples, (ii) emits that grammar **in the meta-language itself**, and (iii) translates it to Rust / JavaScript / PEG / BNF / etc., over a **shared-concept layer inherited from `meta-notation`**. Each component below is rated for three reuse roles: **(a)** representing/parsing grammar definition languages, **(b)** generating parsers, **(c)** inferring grammars from examples.

---

## A. Rust parsing & parser-generator crates

These are candidate **codegen targets** (translate an inferred meta-grammar into their grammar format or code) and/or **runtime engines**.

### A.0 At-a-glance

| Crate | License | Latest | Stars (approx) | Model | Grammar input | Codegen phase |
|---|---|---|---|---|---|---|
| **pest** | MIT OR Apache-2.0 | 2.8.6 (2026-02-05) | ~5.4k (monorepo) | PEG | external `.pest` (or inline) | build-time proc-macro (`pest_derive`) |
| **pest_meta** | MIT OR Apache-2.0 | 2.8.6 | (shared monorepo) | PEG meta-parser/validator/optimizer | `.pest` `&str` **or** hand-built `ast::Rule` | runtime IR library (+ `pest_generator` for codegen) |
| **nom** | MIT | 8.0.0 | ~10.4k | parser-combinator (zero-copy, streaming) | hand-written Rust | runtime library |
| **winnow** | MIT | 1.0.3 | ~915 | parser-combinator (fork of nom) | hand-written Rust | runtime library |
| **combine** | MIT | 4.6.7 | ~1.4k | parser-combinator (LL(1)+opt-in lookahead) | hand-written Rust | runtime library |
| **lalrpop** | Apache-2.0 OR MIT | 0.23.1 (2026-03-11) | ~3.5k | LR(1) (opt. LALR(1)) | external `.lalrpop` DSL | build-time via `build.rs` |
| **chumsky** | MIT | 0.13.0 (1.0 still alpha) | ~4.5k | parser-combinator (error recovery, Pratt) | hand-written Rust | runtime library |
| **peg** (rust-peg) | MIT | 0.8.6 (2026-05-04) | ~1.6k | PEG (recursive descent) | inline `peg::parser!{}` in Rust | compile-time proc-macro |
| **lelwel** | MIT OR Apache-2.0 | 0.10.4 | ~197 | LL(1) recursive descent (resilient, CST) | external `.llw` file | build-time (build.rs/CLI), plain Rust |
| **tree-sitter** (Rust crate) | MIT | 0.26.9 (2026-05-19) | ~25.9k (core) | GLR, incremental | JS `grammar.js` → C | runtime FFI binding to compiled C |
| **earlgrey** | MIT | 0.4.1 (2024-10-11) | ~68 (tox monorepo) | Earley | EBNF string **or** `GrammarBuilder` | runtime library |
| **santiago** | **GPL-3.0-only** ⚠️ | 1.3.1 (2022) | ~110 | Earley (+lexer) | Rust code/macros | runtime library |
| **gearley** | MIT OR Apache-2.0 | 0.0.4 (2019) | ~45 | Earley (Marpa-style) | Rust via `cfg` crate | runtime library (experimental) |

### A.1 pest (PEG) — strong, low-effort codegen target
- `pest` — "The Elegant Parser" — <https://crates.io/crates/pest>. **MIT OR Apache-2.0** ([crates.io](https://crates.io/api/v1/crates/pest); [github.com/pest-parser/pest](https://github.com/pest-parser/pest)). Latest **2.8.6** (2026-02-05); ~5.4k stars on the monorepo; ~263.7M downloads; `no_std`, Rust 1.83+.
- Grammar = **PEG syntax in external `.pest` files** (or `#[grammar_inline="…"]`). Runtime is **PEG** (ordered choice, unambiguous). Codegen is **build-time via the `pest_derive` proc-macro** (wraps `pest_generator`). **Reuse (b):** a translator only needs to **emit valid `.pest` text** (a string); `pest_derive` compiles it to a Rust parser at build time. ([pest.rs](https://pest.rs); [docs.rs/pest_derive](https://docs.rs/pest_derive/))

### A.2 pest_meta — the introspectable PEG grammar AST (most important target for inference)
- `pest_meta` — "pest meta language parser and validator" — <https://crates.io/crates/pest_meta>. **MIT OR Apache-2.0** ([meta/Cargo.toml](https://github.com/pest-parser/pest/blob/master/meta/Cargo.toml)). **2.8.6**; ~227.9M downloads; depends only on `pest`.
- Input is **`.pest` grammar source as `&str`**; it is a runtime parse→validate→optimize pipeline. **Exposes a fully public, targetable AST** ([docs.rs/pest_meta/ast](https://docs.rs/pest_meta/latest/pest_meta/ast/index.html)): `Rule { name: String, ty: RuleType, expr: Expr }`, `RuleType` (Normal/Silent/Atomic/CompoundAtomic/NonAtomic), and an **18-variant `Expr`** PEG algebra (`Str, Insens, Range, Ident, PeekSlice, PosPred, NegPred, Seq, Choice, Opt, Rep, RepOnce, RepExact, RepMin, RepMax, RepMinMax, Skip, Push`) ([enum.Expr](https://docs.rs/pest_meta/latest/pest_meta/ast/enum.Expr.html)). Top-level `parse_and_optimize(grammar: &str)` and `optimizer::optimize(Vec<Rule>)`.
- **Reuse (a)+(b)+(c):** the standout. Two strategies — **(a)** build `ast::Rule`/`ast::Expr` directly in memory from an inference result, validate/optimize via `pest_meta`, then codegen via `pest_generator`; or **(b)** emit `.pest` text and `parse_and_optimize`. This is the strongest *introspectable, programmatically-targetable* grammar IR of all surveyed crates, under a permissive license. (Signatures read from docs.rs, not compiled here.)

### A.3 nom (parser combinators)
- `nom` — <https://crates.io/crates/nom>. **MIT** ([api.github.com/repos/rust-bakery/nom](https://api.github.com/repos/rust-bakery/nom)). **8.0.0**; ~10.4k stars; ~567M downloads; last push 2025-08-26. **No external grammar file — hand-written Rust**, zero-copy, streaming, byte/bit/string oriented. **Reuse (b):** emit Rust calling combinators (`tag`/`alt`/`many0`/tuples). Mechanical mapping is clean; frictions are `IResult`/error-type annotations and API churn (v8 GATs redesign — pin a version). ([github.com/rust-bakery/nom](https://github.com/rust-bakery/nom))

### A.4 winnow (nom fork) — arguably the best combinator target
- `winnow` — <https://crates.io/crates/winnow>. **MIT** (`LICENSE-MIT`; GitHub's `NOASSERTION` is a filename-classifier artifact, [Cargo.toml](https://raw.githubusercontent.com/winnow-rs/winnow/main/Cargo.toml)). **1.0.3**; ~915 stars; last push 2026-06-16 — very active; high download volume (used by `toml_edit`). Explicitly **"a fork of the venerable nom"** ([docs.rs `_topic::why`](https://docs.rs/winnow/latest/winnow/_topic/why/index.html)); avoids GATs; signature `Fn(&mut I) -> O`; "batteries-included" single crate. **Reuse (b):** emit Rust calling winnow combinators — simplest generated types of the three combinator libs; pin a version (willing to break APIs).

### A.5 combine (parser combinators)
- `combine` — <https://crates.io/crates/combine>. **MIT**. **4.6.7**; ~1.4k stars; **last release 2024-04-10 (less active)**. Parsec-style, **LL(1) by default with opt-in lookahead**; most flexible input (`&[u8]`, `&str`, iterators, `Read`). **Reuse (b):** least convenient of the three — deeply nested generics, and a generator must explicitly insert `attempt`/lookahead for backtracking grammars. ([github.com/Marwes/combine](https://github.com/Marwes/combine))

### A.6 lalrpop (LR(1)) — clean declarative target if the grammar is LR-acceptable
- `lalrpop` — <https://crates.io/crates/lalrpop>. **Apache-2.0 OR MIT**. **0.23.1** (2026-03-11); ~3.5k stars; ~58.1M downloads. External **`.lalrpop` DSL** (yacc-like + embedded Rust); **LR(1)** default, optional LALR(1). Codegen is **build-time via `build.rs`** (`lalrpop::process_root()` → `.rs` in `OUT_DIR`, pulled in with `lalrpop_mod!`). **Reuse (b):** emit a `.lalrpop` file; **caveat:** LR(1)/LALR(1) conflicts in an *arbitrary inferred* grammar must be resolved before acceptance. ([lalrpop.github.io tutorial](https://lalrpop.github.io/lalrpop/tutorial/001_adding_lalrpop.html))

### A.7 chumsky (parser combinators) — error recovery + Pratt
- `chumsky` — <https://crates.io/crates/chumsky>. **MIT**. **Stable 0.13.0; 1.0 still alpha** ([issue #543](https://github.com/zesterer/chumsky/issues/543)). **Repo migrated: GitHub archived ~2026-04-02 → [Codeberg](https://codeberg.org/zesterer/chumsky)** (latest commit 2026-06-05). ~4.5k stars. Rust-code combinators, **built-in error recovery + Pratt parsing**, left recursion with memoization, `no_std`. **Reuse (b):** emit Rust building combinators; attractive for real languages, but generate type-checking Rust and accept pre-1.0 churn.

### A.8 peg / rust-peg (PEG proc-macro) — ergonomic PEG target
- `peg` — <https://crates.io/crates/peg>. **MIT**. **0.8.6** (2026-05-04); ~1.6k stars; ~27.4M downloads. Grammar written **inline in Rust** via `peg::parser!{ grammar … }`; generated parser is **recursive descent**; **compile-time proc-macro** (no `build.rs`). **Reuse (b):** because PEG is ordered-choice/unambiguous, an inferred grammar often maps **more directly than onto LALRPOP's LR(1)**; you emit Rust containing a `peg::parser!{}` block. ([docs.rs/peg](https://docs.rs/peg/latest/peg/))

### A.9 lelwel (LL(1), resilient, lossless CST) — strong Rust-only fit
- `lelwel` — <https://crates.io/crates/lelwel>. **MIT OR Apache-2.0**. crates.io **0.10.4**; ~197 stars; ~47k downloads. External **`.llw` file**; **LL(1) recursive descent** with direct left recursion, operator precedence, semantic predicates, producing a **homogeneous lossless CST** with hand-written-grade error resilience. Emits **plain, debuggable Rust at build time, explicitly not via a proc-macro**; usable from `build.rs`, the `llw` CLI, or `lelwel-ls`. **Reuse (b):** emit an `.llw` file → Rust recursive-descent parser. The **lossless-CST output aligns with this project's lossless-links design**. Caveat: LL(1) restrictions, niche maturity. ([README](https://raw.githubusercontent.com/0x2a-42/lelwel/main/README.md))

### A.10 tree-sitter (Rust bindings) — runtime FFI, JS→C codegen
- `tree-sitter` — <https://crates.io/crates/tree-sitter>. **MIT**. **0.26.9** (2026-05-19); ~25.9k stars (core); adopted by Neovim/Zed/Helix/Emacs/GitHub. Grammars are **JavaScript `grammar.js`** → `tree-sitter generate` → **C** `parser.c` (needs C toolchain); the Rust crate is a **runtime binding**, not a Rust generator. Runtime is **GLR, incremental, error-tolerant**. **Reuse:** the codegen target is **JS, not Rust** (JS→C→FFI hops); already a central front-end in this repo (see README's tree-sitter adapter). Best as a **runtime CST source**, not a Rust build-time target. ([CLI generate](https://tree-sitter.github.io/tree-sitter/cli/generate.html); [Wikipedia](https://en.wikipedia.org/wiki/Tree-sitter_(parser_generator)))

### A.11 Earley crates
- **earlgrey** (best Earley target) — <https://crates.io/crates/earlgrey>; repo `rodolf0/tox`. **MIT**. **0.4.1** (2024-10-11, most recent of the family); ~68 stars. **Dual input:** `EbnfGrammarParser` (parse an **EBNF string at runtime**) *and* `GrammarBuilder` (programmatic). **Earley**, supports ambiguity + all parse trees. **Reuse (b)+(c):** emit an EBNF string or build via `GrammarBuilder`; runtime, embeddable, MIT. ([docs.rs/earlgrey](https://docs.rs/earlgrey/latest/earlgrey/))
- **santiago** — <https://crates.io/crates/santiago>. ⚠️ **GPL-3.0-only** — copyleft red flag; **avoid as a backend** for a permissive project. **1.3.1 (2022, no release since)**; ~110 stars. Earley + bundled lexer, BNF-like Rust macros. ([crates.io](https://crates.io/api/v1/crates/santiago))
- **gearley** — <https://crates.io/crates/gearley>. **MIT OR Apache-2.0**. **0.0.4 (2019), experimental/unmaintained**; ~45 stars; Marpa-style (Aycock–Horspool); grammars via the separate `cfg` crate. Interesting as the canonical Rust Marpa-style engine; **not a production backend**. ([github.com/pczarn/gearley](https://github.com/pczarn/gearley); [lib.rs](https://lib.rs/crates/gearley))
- *Others:* `earley` (v0.1.0, 2016, **no license — unusable**); `gramatica` (notable as it **compiles grammars to Rust at build time** via a binary — license/maturity unverified, worth a follow-up if build-time Earley codegen is desired).

### A.12 Cross-cutting answers
- **External grammar file:** pest (`.pest`), lalrpop (`.lalrpop`), lelwel (`.llw`), tree-sitter (JS→C), earlgrey (EBNF string, optional). **Inline-in-Rust macro:** peg. **Hand-written/generated Rust (no grammar file):** nom, winnow, combine, chumsky, santiago, gearley, earlgrey-`GrammarBuilder`.
- **Build-time codegen:** pest (`pest_derive`), peg (proc-macro), lalrpop (`build.rs`), lelwel (`build.rs`/CLI), tree-sitter (CLI→C). **Pure runtime:** nom, winnow, combine, chumsky, pest_meta, all Earley crates.
- **Introspectable grammar AST to target programmatically (key for inference):** **pest_meta** (best), earlgrey (`GrammarBuilder`/`Grammar`), gearley (via `cfg`), santiago (GPL). Combinator libs + peg are targeted by *generating Rust source*, not by populating a grammar IR.
- **License flags:** only **santiago is GPL-3.0-only**; `earley` has **no license**. All others permissive.

---

## B. Grammar definition / interchange formats & their parsers

### PART 1 — The formats

- **PEG (Parsing Expression Grammars)** — Bryan Ford, *POPL '04*, pp. 111–122, **DOI 10.1145/964001.964011** ([author page](https://bford.info/pub/lang/peg/); [PDF](https://bford.info/pub/lang/peg.pdf); [ACM](https://dl.acm.org/doi/10.1145/964001.964011)). Recognition-based: **prioritized/ordered choice `/`** (first match wins → unambiguous by construction); greedy repetition `* + ?`; **syntactic predicates** `&e` (and) / `!e` (not) for unlimited lookahead; scannerless; **linear time via packrat memoization** ([packrat page](https://bford.info/packrat/)). *(PDF body not text-extractable; specifics from the abstract + corroborating records.)*
- **BNF (Backus–Naur Form)** — origin John Backus 1959 (ALGOL 58) / Peter Naur, ALGOL 60 Report (1960–63) ([Wikipedia: ALGOL 60](https://en.wikipedia.org/wiki/ALGOL_60)). Knuth, "backus normal form vs. Backus Naur form," *CACM* 7(12):735–736, 1964, **DOI 10.1145/355588.365140** ([ACM](https://dl.acm.org/doi/10.1145/355588.365140)). `<nt> ::= … | …`. **No single ISO/RFC standard.**
- **EBNF** — **ISO/IEC 14977:1996**, "Information technology — Syntactic metalanguage — Extended BNF" ([iso.org/standard/26153.html](https://www.iso.org/standard/26153.html)). Adds `{ }` (repeat), `[ ]` (optional), `( )`, `,` concat, `=` def, `|`, `;` terminator. *(ISO page authoritative; free full-text not confirmed from iso.org.)*
- **ABNF** — **RFC 5234** (STD 68, Jan 2008) ([rfc-editor.org/rfc/rfc5234](https://www.rfc-editor.org/rfc/rfc5234)) + **RFC 7405** (case-sensitive `%s`/`%i`, Dec 2014) ([rfc7405](https://www.rfc-editor.org/rfc/rfc7405)). `name = elements` (no angle brackets), `/` alternatives, `=/` incremental, `*` repetition, `%x`/`%d`/`%b` value/ranges, core rules (Appendix B). Case-insensitive strings by default.
- **W3C EBNF (XML 1.0)** — Section 6 "Notation" of XML 1.0 5th ed. ([w3.org/TR/xml/#sec-notation](https://www.w3.org/TR/xml/#sec-notation)). **Distinct from ISO 14977:** `symbol ::= expression`; `[a-z]`, `#xN` code points, `?*+|()`, `-` exception. Reused by XPath/XQuery specs.
- **ANTLR4 `.g4`** — Terence Parr; generates lexers/parsers in many targets; uses **ALL(\*) ("adaptive LL(\*)")** ([antlr.org](https://www.antlr.org/); [ALL(\*) report](https://www.antlr.org/papers/allstar-techreport.pdf)). Reference grammar corpus: [antlr/grammars-v4](https://github.com/antlr/grammars-v4) — **per-grammar licensing in file headers** (e.g. Java grammar is BSD-3-Clause; others MIT/Apache — check each `.g4`).
- **tree-sitter `grammar.js`** — JavaScript DSL; `grammar({ name, rules })`, rules are JS functions of `$`, composed with `seq/choice/repeat/optional/prec/token/field/alias`; compiled to a C parser ([grammar DSL docs](https://tree-sitter.github.io/tree-sitter/creating-parsers/2-the-grammar-dsl.html)).
- **Bison/Yacc** — GNU Bison: annotated CFG → deterministic LR/GLR (LALR(1)/IELR(1)/canonical LR(1)), Yacc-compatible; four-section `.y` file ([gnu.org/software/bison/manual](https://www.gnu.org/software/bison/manual/)). Yacc: Stephen C. Johnson, Bell Labs ~1975 ([Wikipedia](https://en.wikipedia.org/wiki/Yacc)). *(gnu.org returned 429; structure verified from manual excerpts + mirrors.)*
- **Lark `.lark`** — EBNF-based; **lowercase rules, UPPERCASE terminals**; `| ? * + ~n`; `%ignore`, `%import`; Earley + LALR(1) backends ([lark grammar ref](https://lark-parser.readthedocs.io/en/latest/grammar.html)).
- **GBNF (GGML BNF, llama.cpp)** — "an extension of BNF that primarily adds a few modern regex-like features"; `nonterminal ::= sequence...`; **`root` rule = start**; `| () * + ? {m,n} [^...]` ([grammars/README.md](https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md)).
- **Guidance / Outlines** — Guidance: a Python DSL (`gen()`/`select()`) that can enforce any CFG; no standalone file format ([guidance-ai/guidance](https://github.com/guidance-ai/guidance)). Outlines: consumes **Lark-format EBNF** for CFG generation ([CFG docs](https://dottxt-ai.github.io/outlines/reference/generation/cfg/)).

### PART 2 — Rust crates that PARSE these formats into an AST (the key deliverable)

| Crate | Version | License | Permissive? | Parses / output |
|---|---|---|---|---|
| `bnf` | 0.6.0 | **MIT** | Yes | `Grammar`→`Production`→`Expression`→`Term{Terminal,Nonterminal}`; bundles an Earley matcher; normalizes groups/optionals to `__anon*` NTs ([docs.rs/bnf](https://docs.rs/bnf/latest/bnf/)) |
| `ebnf` | 0.1.4 | **MIT** | Yes | `Grammar`/`Expression{lhs,rhs}` + rich `Node` enum (String/Regex/Terminal/Multiple/Group/Optional/Repeat/…); Wikipedia/EBNF-Evaluator dialect; nom+serde ([docs.rs/ebnf](https://docs.rs/ebnf/latest/ebnf/)) |
| `ebnf-parser` | 0.1.0 | ⚠️ **GPL-3.0-only** | **No** | ISO-14977 LL(1) parser; **abandoned 2022** ([lib.rs](https://lib.rs/crates/ebnf-parser)) |
| `kbnf-syntax` | 0.5.3 | **MIT** | Yes | fork of `ebnf` + embeddable regex (internal to `kbnf`) |
| `abnf` | 0.13.0 | **MIT OR Apache-2.0** | Yes | `Vec<Rule>`; `Rule{name(),node(),kind()}`; `Node` enum (Alternatives/Concatenation/Repetition/Rulename/Group/Optional/String/TerminalValues/Prose); de-facto ABNF parser ([docs.rs/abnf](https://docs.rs/abnf/latest/abnf/)) |
| `abnf-core` | 0.6.0 | **MIT OR Apache-2.0** | Yes | nom parsers for ABNF *core rules* only (helper) |
| `abnf_to_pest` | 0.5.1 | **MIT OR Apache-2.0** | Yes | **transpiler:** ABNF → `.pest` text (part of dhall-rust) |
| `pest_meta` | 2.8.6 | **MIT OR Apache-2.0** | Yes | the `.pest` PEG AST (`ast::{Rule,Expr,RuleType}`) — see §A.2 |
| `antlr-rust` | 0.3.0-beta (0.2.2 stable) | **BSD-3-Clause** | Yes | ANTLR4 **runtime only**; the Java ANTLR tool generates Rust from `.g4` ahead of time — **no runtime `.g4`→AST** ([antlr4rust](https://github.com/rrevenantt/antlr4rust)) |
| `peg` (rust-peg) | 0.8.6 | **MIT** | Yes | compile-time PEG generator (grammar fixed at compile time) |
| `peginator` | 0.7.0 | **MIT** | Yes | compile-time PEG → typed AST generator ([peginator](https://github.com/badicsalex/peginator)) |
| `dynparser` | 0.4.2 (2018) | ⚠️ **GPL-3.0** | **No** | **the genuine runtime PEG-text→AST** (`rules_from_peg()`); abandoned ([jleahred/dynparser](https://github.com/jleahred/dynparser)) |
| `ungrammar` | 1.16.1 | **MIT OR Apache-2.0** | Yes | **CST-shape DSL** (not a parser): `Grammar`/`Node`/`Token`/`Rule(seq/alt/opt/rep/labeled/node/token)`; canonical copy now in [rust-analyzer/lib/ungrammar](https://github.com/rust-lang/rust-analyzer/tree/master/lib/ungrammar) (standalone repo archived 2025-11-09) |
| `grammartec` | 0.3.1 | ⚠️ **AGPL-3.0** (published crate) | **No (network copyleft)** | Nautilus grammar-tree fuzzing engine; Python/JSON grammars (see §C) |
| `tree-sitter` | 0.26.9 | **MIT** | Yes | runtime CST (consumes *compiled* grammars) |
| `tree-sitter-cli` | 0.26.9 | **MIT** | Yes | generator; **`grammar.js` must be JS-evaluated → `grammar.json`**, which Rust then consumes (serde) — no pure-Rust `grammar.js` parser exists |
| `railroad` | 0.3.7 | **MIT** | Yes | SVG syntax diagrams from a **Rust API**, **not** a grammar-text parser |

**Non-existent (verified 404 on crates.io):** `bnf_parser`, `peg-parser`, `w3c-ebnf`.

**Bottom line for the project.** For *parsing grammar definition languages today* (role a), the permissive, mature options are **`bnf`** (BNF), **`ebnf`**/`kbnf-syntax` (EBNF dialect + regex), **`abnf`** (ABNF), and **`pest_meta`** (the `.pest` PEG dialect). **Gaps:** no permissive **runtime generic-PEG-text→AST** crate (only GPL `dynparser`); no pure-Rust **`grammar.js`** parser (target `grammar.json` instead); no runtime **`.g4`** parser (ANTLR's Java tool generates Rust); no **W3C-EBNF** crate. `ungrammar` is the closest in *spirit* to this project's lossless-CST + concept model (a CST-shape DSL paired with a hand-written parser), and is permissive. **Flag/avoid:** `ebnf-parser` (GPL), `dynparser` (GPL), `grammartec` (AGPL).

---

## C. Grammar inference / learning tools

Take examples (± an oracle/parser) and emit a grammar/automaton. Most directly serves the issue's core requirement ("infer programming-language grammar from examples of correct texts").

### C.1 Black-box CFG inference from examples (the core target)

- **GLADE** — code [github.com/obastani/glade](https://github.com/obastani/glade); paper "Synthesizing Program Input Grammars," **PLDI 2017** (Bastani, Sharma, Aiken, Liang) ([MSR](https://www.microsoft.com/en-us/research/publication/synthesizing-program-input-grammars-2/)). **Java; Apache-2.0** ([license API](https://api.github.com/repos/obastani/glade/license)); **unmaintained** (last push 2020). Algorithm: from a membership oracle + valid seeds, (1) learn a **regex generalizing each seed** via oracle-guided substring generalization, (2) **merge subexpressions into a recursive CFG**. **Reuse (c): study/port** (it is the baseline everyone cites; read the PLDI 2022 critical replication first ([pldi22](https://pldi22.sigplan.org/details/pldi-2022-pldi/49/-Synthesizing-Input-Grammars-A-Critical-Evaluation))).
- **Arvada** — [github.com/neil-kulkarni/arvada](https://github.com/neil-kulkarni/arvada); **ASE 2021** "Learning Highly Recursive Input Grammars" ([arXiv 2108.13340](https://arxiv.org/abs/2108.13340)). **Python; MIT** ([license API](https://api.github.com/repos/neil-kulkarni/arvada/license)). Algorithm: **tree-based "bubble-and-merge"** — start from flat parse trees, iteratively bubble sibling sequences into new nonterminals (enabling recursion) and merge, each validated against a boolean oracle. Reports ~4.98× recall over GLADE on recursive grammars. **Reuse (c): strong port candidate** (but TreeVada supersedes it).
- **TreeVada** — [github.com/rifatarefin/treevada](https://github.com/rifatarefin/treevada); **ICSE 2024** "Fast Deterministic Black-box Context-free Grammar Inference" ([arXiv 2308.06163](https://arxiv.org/abs/2308.06163)). **Python; MIT** ([license API](https://api.github.com/repos/rifatarefin/treevada/license)). **Deterministic** redesign of Arvada (uses bracket/paren priors to pre-structure trees, removes nondeterministic search); higher recall/precision/F1 and ~2.4× faster. **Reuse (c): recommended primary port-to-Rust target** for example-driven CFG inference — MIT, deterministic, fastest, recent.
- **Mimid** — [github.com/vrthra/mimid](https://github.com/vrthra/mimid); **FSE 2020** "Mining Input Grammars from Dynamic Control Flow" ([ACM](https://dl.acm.org/doi/abs/10.1145/3368089.3409679)). Jupyter/C/Python. ⚠️ **License: Fuzzing-Book License = code MIT but content CC BY-NC-SA 4.0 (non-commercial copyleft)**; repo is mostly notebooks → **treat as study-only** ([LICENSE.md](https://github.com/vrthra/mimid/blob/master/LICENSE.md)). **White-box** (instruments the parser, traces dynamic control flow) — a different problem from inference-from-examples. **Reuse (c): study the control-flow-tracing idea only.**
- **REINAM** — paper-only, **FSE 2019** ([ACM](https://dl.acm.org/doi/10.1145/3338906.3338958)). ⚠️ **No public repo / no license — legally unusable.** Idea: bootstrap a CFG (GLADE-style, no seeds, via Pex symbolic execution) then **RL-guided generalization** rewarded by precision/recall. **Reuse (c): study (clean-room from paper) only.**

### C.2 Constraint / invariant learning over grammars

- **ISLa** — [github.com/rindPHI/isla](https://github.com/rindPHI/isla); "Input Invariants," **ESEC/FSE 2022** ([arXiv 2208.12049](https://arxiv.org/abs/2208.12049)). Python. ⚠️ **GPL-3.0** ([COPYING](https://raw.githubusercontent.com/rindPHI/isla/main/COPYING)). A grammar-aware string-constraint solver: FOL with quantifiers over **derivation trees** + **SMT** for atomic string/numeric predicates (expresses def-before-use, length, equal-counts — things a CFG can't). **Reuse: study** the FOL-over-trees + SMT model (a clean template for layering semantics over syntax in the meta-language); **do not vendor** — reimplement from the permissive spec.
- **ISLearn** — [github.com/rindPHI/islearn](https://github.com/rindPHI/islearn). Python. ⚠️ **GPL-3.0 + GPL dep**. **Most on-target "learn from examples":** mines ISLa invariants by (1) grammar+k-path mutation augmentation, (2) instantiate from a **pattern catalog**, (3) filter patterns not holding across samples, (4) combine into **DNF**, (5) rank by **specificity & recall**. **Reuse (c): clean-room port** of the instantiate→filter→DNF→rank pipeline + a Rust constraint/SMT backend.

### C.3 Active & passive automata learning libraries

- **LearnLib** — [github.com/LearnLib/learnlib](https://github.com/LearnLib/learnlib). **Java; Apache-2.0** ([LICENSE.txt](https://github.com/LearnLib/learnlib/blob/develop/LICENSE.txt)); **mature/active** (0.18.0, 2025). The de-facto active-automata-learning reference. **Algorithms:** all active learners use the **MAT (Minimally Adequate Teacher)** framework — **membership** + **equivalence** queries. **L\*** (Angluin) maintains an **observation table** (prefixes × distinguishing suffixes), enforces *closedness/consistency*, builds a DFA from distinct row signatures, and refines on counterexamples. **Kearns–Vazirani / Observation Pack** replace the table with a **discrimination tree**. **TTT** (Isberner) uses three coupled tree structures with a **redundancy-free discrimination tree**, keeping discriminators short and decomposing counterexamples incrementally → space-optimal in counterexample length (best for long counterexamples). Also ships **NL\*** (NFAs), **ADT/DHC/L#**, and passive **RPNI**/**OSTIA**. **Reuse (c): study/port** — for learning from labeled examples, the relevant pieces are passive **RPNI** (state-merging from samples) and **L\*/TTT** with a sample-backed MAT teacher; a clean-room Rust port is most idiomatic (Java otherwise needs a JVM bridge).
- **libalf** — [libalf.informatik.rwth-aachen.de](https://libalf.informatik.rwth-aachen.de/) / [github.com/libalf/libalf](https://github.com/libalf/libalf). **C++; ⚠️ LGPL-3.0** (+ bundled **MiniSat** under a separate license) ([LICENSE](https://github.com/libalf/libalf/blob/master/libalf/LICENSE)); **legacy/inactive**. Implements **Angluin L\*, RPNI, NL\*, Biermann (MiniSat-backed), Kearns–Vazirani, DeLeTe2, Rivest–Schapire**. Bundles the major **passive** learners (RPNI/Biermann/DeLeTe2). **Reuse: algorithm reference for a clean-room Rust port** (LGPL + C++ build + dormancy argue against FFI).
- **flexfringe** — [github.com/tudelft-cda-lab/FlexFringe](https://github.com/tudelft-cda-lab/FlexFringe) (successor to **DFASAT**). **C++; ⚠️ GPL-3.0** ([LICENSE](https://github.com/tudelft-cda-lab/FlexFringe/blob/main/LICENSE)); **actively maintained**. **Algorithms (passive state-merging = grammar induction from sample strings):** build an **(Augmented) Prefix Tree Acceptor (APTA)**, then greedily merge states in the **red-blue / blue-fringe framework** (red = confirmed, blue = candidates; perform best consistent merge, recursively fold successors to stay deterministic, promote blue→red when no consistent merge). Pluggable scoring: **EDSM (Evidence-Driven State Merging)** (scores by overlapping accept/reject evidence), **ALERGIA** (merges probabilistic states when symbol/final frequencies are statistically compatible under a Hoeffding bound), plus AIC/MDI/k-tails; **DFASAT** encodes the residual merge problem as **SAT** and solves with an external solver. Also **RTI+** (real-time automata); reads Abbadingo; emits DOT/JSON. **Reuse (c):** run as an **external CLI** on example traces and consume JSON/DOT (GPL covers the tool, not your data), **or** reimplement APTA+red-blue+EDSM(+ALERGIA) in Rust from the papers.

### C.4 Grammar-based fuzzers / generators (consume grammars; design references)

- **Grammarinator** — [github.com/renatahodovan/grammarinator](https://github.com/renatahodovan/grammarinator). **Python (+C++ backend); BSD-3-Clause** ([LICENSE.rst](https://github.com/renatahodovan/grammarinator/blob/master/LICENSE.rst)); **mature/active**. Generates tests from **ANTLR v4 grammars** with grammar-aware mutation/recombination. **Reuse (b): study** for grammar-driven generation; strong fit if the project emits/consumes ANTLR-style grammars (large public corpus).
- **nautilus / grammartec** — [github.com/nautilus-fuzz/nautilus](https://github.com/nautilus-fuzz/nautilus); grammar engine is the **Rust `grammartec`** workspace crate. ⚠️ **CRITICAL license trap:** default branch **`mit-main` is MIT**, but the original **`master` is AGPL-3.0** (relicensed 2024-05-31); `grammartec/Cargo.toml` has **no `license` field** and is **not on crates.io**. **Hard rule: pin `mit-main` (or a commit ≥ 2024-05-31); never `master`.** Not actively developed (last push 2024-08). Coverage-guided grammar fuzzer with **tree-level structural mutations**. **Reuse (b)+(c): vendor/fork `grammartec` (mit-main)** or study its tree/mutation design — it is already Rust and the ancestor of LibAFL's grammar tooling. *(This is the same crate flagged in §B as AGPL on crates.io history — the MIT path exists only on `mit-main`.)*
- **autarkie** — [github.com/R9295/autarkie](https://github.com/R9295/autarkie). **Rust; Apache-2.0** ([LICENSE](https://github.com/R9295/autarkie/blob/master/LICENSE)); new but active (0.1.0 Apr 2025; `autarkie_libfuzzer` 0.9.4). **LibAFL-based grammar fuzzer that uses proc-macros to auto-generate a grammar fuzzer from Rust type definitions** (the grammar *is* your Rust types). **Reuse (b)+(c): depend or study the derive-macro pattern** — the most natural Rust-native fit, especially relevant since this project's AST/types are Rust (you could derive generation/inference scaffolding from the link types).

### C.5 Sequitur & toolboxes

- **Sequitur** — Nevill-Manning & Witten, *JAIR* 7:67–82, 1997 ([arXiv cs/9709102](https://arxiv.org/abs/cs/9709102)); reference site [sequitur.info](http://www.sequitur.info/) (⚠️ **TLS error during research — reference-code license unverified**). **Online, linear-time CFG inference from a single sequence**, maintaining **digram uniqueness** (no repeated adjacent pair on RHSs) + **rule utility** (every rule used >once). **Reuse (c): port from the paper** (unencumbered algorithm); infers a CFG from one sequence (no negative examples) — complements RPNI/Arvada.
- **Rust `sequitur` crates: none implement the algorithm.** The crates.io [`sequitur`](https://crates.io/crates/sequitur) is an unrelated **file-sequence** library (MIT). Closest existing Rust GI crate is **`rust-lstar`** (Angluin L\* for DFAs — *automaton*, not CFG; license/maturity unverified). → **You would port Sequitur yourself.**
- **GIToolbox** — [code.google.com/archive/p/gitoolbox](https://code.google.com/archive/p/gitoolbox/); ICGI 2010 "Grammatical Inference Algorithms in MATLAB" ([Springer](https://link.springer.com/chapter/10.1007/978-3-642-15488-1_22)). **MATLAB; MIT** ([project page](https://huang3.github.io/2016/11/21/gitoolbox/)); unmaintained. Bundles **RPNI, EDSM, K-Testable, ALERGIA, MDI, OSTIA**. **Reuse (c): study/port source** for these classic GI algorithms (cannot depend from Rust).

### C.6 Bottom line (inference)
- **Safe to study/port/depend (permissive):** GLADE (Apache, Java), Arvada (MIT, Py), **TreeVada (MIT, Py — top pick for CFG-from-examples)**, LearnLib (Apache, Java — RPNI/L\*/TTT), Grammarinator (BSD, Py), **autarkie (Apache, Rust)**, **grammartec (MIT on `mit-main`, Rust)**, GIToolbox (MIT, MATLAB), Sequitur (port from paper).
- **Copyleft → external tool or clean-room:** ISLa, ISLearn, flexfringe (GPL-3.0); libalf (LGPL-3.0).
- **Study-only (no usable code):** Mimid (CC-NC), REINAM (no repo/license).
- **Mapping to roles:** CFG-from-example-strings → **TreeVada** (Rust port); regular/automaton-from-labeled-examples → **RPNI** (port from LearnLib/libalf/GIToolbox) or run flexfringe as a GPL CLI; CFG-from-one-sequence → **Sequitur**; semantic invariants over a grammar → **ISLearn** pipeline (reimplement); Rust-native today → **autarkie** + **grammartec**.

---

## D. The link-foundation ecosystem

> Verified live on 2026-06-19 via `gh repo view` / `gh api` / `gh search`. **Whole-ecosystem license = The Unlicense (public domain) — maximally permissive for reuse.**

### D.1 meta-notation — CENTRAL (the issue's mandated basis)
- **Repo:** [github.com/link-foundation/meta-notation](https://github.com/link-foundation/meta-notation). **Unlicense; Rust (52.8%) + TypeScript (45.8%); 0 stars; last push 2026-03-21.** **A functional implementation, not a spec** (published on npm and crates.io; 170+ shared tests across both implementations).
- **Purpose & syntax:** "A notation for the largest possible set of languages," focused on parsing **common delimiters**: brackets `()` `{}` `[]` (nested), quotes `''` `""` `` ` `` (content kept as opaque strings, no nested parsing), and unquoted **text blocks**. The parser transforms raw text into a sequence of typed blocks (`paren`/`curly`/`square`/`singleQuote`/`doubleQuote`/`backtick`/`text`) and supports **lossless round-trip** (serialize back to original text). JS/TS implementation uses a **PEG.js grammar**; Rust adds serde. Tested across 25+ programming languages and several natural languages.
- **Relation to LiNo / meta-language:** explicitly a **simplification of links-notation that removes the `:` self-reference syntax** to maximize cross-language compatibility — a deliberate trade of expressiveness for universality. README: *"The implementation is similar to the concepts in metalanguage, but leverages all the tools from links-notation to do it right and efficiently."*
- **Reuse for #93:** this is the **inheritance root** the issue requires — the meta-language's surface notation should be a superset/derivative of meta-notation's delimiter model. Its **lossless, delimiter-typed, language-agnostic block model maps directly onto this repo's existing lossless-links + projected-CST design**, and its PEG.js/serde-Rust split is a working reference for the grammar surface. (The local `meta-language` README already references the meta-notation lineage and a `LiNo` import path.)

### D.2 links-notation (LiNo)
- **Repo:** [github.com/link-foundation/links-notation](https://github.com/link-foundation/links-notation). **NB:** `linksplatform/Protocols.Lino` now **redirects to this repo**. **Unlicense; Rust; 5 stars; last push 2026-06-15.** Published to crates.io, npm, PyPI, NuGet, Maven, Go — **six language bindings** (`links-notation` / `Link.Foundation.Links.Notation`).
- **Purpose & syntax:** converts any string containing links notation into a list of links and back. Built on two concepts — **references** and **links** (a link references other links; any arity). Surface examples: `papa (lovesMama: loves mama)` (named doublet), triplets, N-tuples, indented multi-line blocks, self-referential points. Rust entry point `parse_lino(...)`.
- **Reuse for #93:** the **textual storage/interchange surface** for grammars expressed in the meta-language, and the parent of meta-notation. This repo already does **structural LiNo parsing** (per its README); LiNo is the natural serialization for an inferred meta-grammar before translation. (Local survey: **~138 tests per language binding**, per `docs/case-studies/issue-3/ecosystem-foundations.md`.)

### D.3 meta-expression (semantic lexicon) — ⚠️ NOT FOUND on 2026-06-19
- **Status:** `link-foundation/meta-expression` returns **HTTP 404** (both `gh repo view` and direct `gh api repos/...`), and a global `gh search repos meta-expression` returns only unrelated bioinformatics/FEM projects. **The repo is not publicly accessible at this date.**
- **Discrepancy flag:** this repo's **own** earlier survey `docs/case-studies/issue-3/ecosystem-foundations.md` (dated 2026-06-05) cites it as a real `PARITY_TARGET` with specifics — `semantic-lexicon.json` holding **351 concepts**, languages `en/hi/ru/zh`, and the worked example *"Hawaii is a state." → "Гавайи это штат."* with Wikidata IDs. So meta-expression existed publicly ~2 weeks ago and appears to have since been **renamed, made private, or deleted**. The **351-concept semantic lexicon it described is already absorbed into this repo** (the `meta-language` README mentions `seed_common_concept_ontology()` seeding a "default 351-concept semantic lexicon"). **Reuse for #93:** the shared-concept layer the issue relies on for 1-to-1 cross-language translation is **already internalized**; treat the external repo as unavailable and depend on the in-repo concept ontology + LiNo concept-set import.

### D.4 Other named repos (one line each)
- **relative-meta-logic** — [repo](https://github.com/link-foundation/relative-meta-logic). **Unlicense; Rust + JS; last push 2026-06-18 (very active).** A probabilistic/relative logic framework (formerly "Associative-Dependent Logic"): dependent-type kernel, many-valued + probabilistic truth values, paradox handling (liar paradox), runtime-redefinable operators, `.lino` corpus, and **Lean 4 / Rocq export** + an LSP. (Local survey ties it to this repo's `TruthValue`/`ProbabilisticTruthValue` semantics.)
- **lino-objects-codec** — [repo](https://github.com/link-foundation/lino-objects-codec). **Unlicense; JS (+ Py/Rust/C#); last push 2026-05-10.** Universal object ⇆ Links-Notation codec with **`decode(encode(x)) == x`**, preserving **shared and circular references**.
- **link-cli (`clink`)** — [repo](https://github.com/link-foundation/link-cli). **Unlicense; Rust (+ C# + WASM); 8 stars; last push 2026-05-20.** Manipulates links via a **single match→substitute operation** (create/update/delete/swap with `$`-variables); ships C#/NuGet, Rust/crates.io, and a WASM browser workbench. (This repo's `SubstitutionRule`/`apply_substitution()` mirrors it.)
- **formal-ai** — ⚠️ **NOT FOUND on 2026-06-19** (`gh api repos/link-foundation/formal-ai` → HTTP 404; global search returns only unrelated projects). Same discrepancy as meta-expression: cited as a real `PARITY_TARGET` in this repo's 2026-06-05 `ecosystem-foundations.md` (formalization corpus under `data/seed/*.lino` + `data/benchmarks/*.lino`; the issue-#1 "706-case corpus" figure was already flagged there as unverifiable). Now publicly inaccessible.

### D.5 BONUS — directly-relevant ecosystem repos the issue did NOT name (high value for #93)
Discovered via `gh repo list link-foundation`; all **Unlicense**:
- **grammar-inference** — [repo](https://github.com/link-foundation/grammar-inference). *"A tool to inference grammar from multiple examples of text in the same domain."* **Rust; 0 stars; last push 2026-05-01.** ⚠️ **Currently a STUB:** its README is the **unmodified `rust-ai-driven-development-pipeline-template`** (no inference code yet). **This is the most on-point repo for issue #93 — it is the intended home/sibling of the feature and is awaiting implementation.**
- **grammar-expressions** — [repo](https://github.com/link-foundation/grammar-expressions). *"A hybrid of Regular Expressions and PEGs based on Links Notation tokenizer."* **Rust + JS; Unlicense; last push 2026-05-09.** A **PEG/regex hybrid engine** (ordered choice `a|ab` → `a` wins; `* + ?`; char classes; **named captures `{name:expr}`**; rewrite rules), with **no third-party runtime deps**; roadmap targets links-notation compatibility + packrat. **Reuse (a)+(b): a ready, permissive, links-native grammar-expression engine to build on or target.**
- **rules-inference** — [repo](https://github.com/link-foundation/rules-inference). *"A tool to inference rules from events/changes and requirements."* **Rust; Unlicense.** ⚠️ Also currently the **unmodified Rust template (stub).** Sibling inference effort.
- **lino-tokenizer** — [repo](https://github.com/link-foundation/lino-tokenizer). *"…tokenize Unicode String as sequence of references in Links Notation."* Unlicense. The tokenizer layer beneath grammar-expressions/meta-notation.
- **meta-theory** — [repo](https://github.com/link-foundation/meta-theory). *"The links meta-theory."* **Unlicense; JS.** Archive of the foundational Links Theory articles (v0.0.0–0.0.2); **source of truth for the "link / network / point" definitions** this repo's terminology guardrail depends on.
- **meta-ontology** — [repo](https://github.com/link-foundation/meta-ontology). *"An ontology that can describe itself."* Unlicense; Rust.
- Also present and topically adjacent: **lino-schema** (LiNo dialect for LiNo schemas), **lino-document-markup-language**, **javascript-to-rust-translator** & **python-to-javascript-translator** (rule-based translators — directly relevant to the issue's "translate the inferred grammar to Rust/JS" requirement), **transformer** (LiNo-based Turing-complete transformer), **linksql** (query language for links).

---

## E. LLM-assisted grammar / structured generation

Crux question per tool: does it consume a **portable grammar format** we could **emit/target**? All tools below are **permissively licensed (MIT/Apache-2.0)** — no GPL/AGPL blockers.

- **GBNF (llama.cpp)** — [github.com/ggml-org/llama.cpp](https://github.com/ggml-org/llama.cpp), [grammars/README.md](https://github.com/ggml-org/llama.cpp/blob/master/grammars/README.md). **C++; MIT; ~117k stars; pushed daily.** Consumes **GBNF** (BNF dialect, `::=`, `root` start, regex-like `| * + ? {}`), and bundles **`json_schema_to_grammar.py`/.js (JSON Schema → GBNF)**. **Target by emitting? Yes — strongly** (emit GBNF text, or emit JSON Schema + convert).
- **XGrammar (MLC)** — [github.com/mlc-ai/xgrammar](https://github.com/mlc-ai/xgrammar). **C++/Python; Apache-2.0; ~1.75k stars; active.** *"We currently use the GBNF format (GGML BNF)"* ([EBNF tutorial](https://xgrammar.mlc.ai/docs/tutorials/ebnf_guided_generation.html)); also JSON Schema. **Target by emitting? Yes — the same GBNF text as llama.cpp**, and XGrammar is the **default backend of vLLM and SGLang**, so one GBNF emitter reaches all of them.
- **Outlines (dottxt-ai)** — [github.com/dottxt-ai/outlines](https://github.com/dottxt-ai/outlines). **Python; Apache-2.0; ~14k stars; active.** Consumes **regex, JSON Schema, Pydantic, and CFGs via Lark (EBNF)** — `generate.cfg(model, grammar)` with `.lark` files ([CFG docs](https://dottxt-ai.github.io/outlines/reference/generation/cfg/)). **Target? Yes**, via JSON Schema or a **Lark-EBNF** grammar — note **Lark-EBNF ≠ GBNF** (a second dialect).
- **Guidance (guidance-ai)** — [guidance](https://github.com/guidance-ai/guidance) + Rust engine [llguidance](https://github.com/guidance-ai/llguidance). **MIT; ~21.5k stars; active.** Idiomatically a Python API; underneath consumes **a Lark-variant CFG with embedded JSON Schema/regex**; since [PR #1150](https://github.com/guidance-ai/guidance/pull/1150) exposes **`lark()` and `gbnf()` constructors** (+ `gbnf_to_lark`). **Target? Yes** (emit GBNF via `gbnf()`), but the primary interface is hand-written Python.
- **lm-format-enforcer** — [github.com/noamgat/lm-format-enforcer](https://github.com/noamgat/lm-format-enforcer). **Python; MIT; ~2k stars; active.** Consumes **JSON Schema** + **regex** only; **no CFG/BNF/EBNF support**. **Target? Partially** (emit JSON Schema or a regex subset; a CFG must be down-converted).
- **json-schema-to-grammar converters** — llama.cpp's built-in (Py/JS) and standalone **`adrienbrault/json-schema-to-gbnf`** (TS, MIT) ([repo](https://github.com/adrienbrault/json-schema-to-gbnf)); plus a Rust `gbnf` crate. Confirm a well-trodden **JSON Schema → GBNF** bridge.
- **Host engines:** **SGLang** (Apache-2.0) — backends **XGrammar (default)/Outlines/llguidance**, accepts JSON Schema, regex, EBNF ([docs](https://docs.sglang.io/advanced_features/structured_outputs.html)). **vLLM** (Apache-2.0) — backends **XGrammar/guidance** (default `auto`), `guided_json`/`guided_regex`/`guided_grammar` (EBNF/GBNF `::=`)/`guided_choice` ([docs](https://docs.vllm.ai/en/latest/features/structured_outputs.html)).

**Lingua franca / recommendation.** **JSON Schema** is the broadest-reach *constraint* format (consumed by every tool) but cannot express arbitrary CFGs. **GBNF (GGML BNF)** is the de-facto portable **grammar** target: native to llama.cpp, adopted by XGrammar (the **default backend of vLLM and SGLang**), and ingestible by Guidance via `gbnf()`. **Emit GBNF as the primary CFG target** (one text format reaches the most engines, all permissive), **JSON Schema as a secondary** target (maximizes reach for data-shaped output, with battle-tested JSON-Schema→GBNF converters), and add a **Lark-EBNF** emitter only if Outlines/Guidance-CFG interop becomes a requirement. GBNF being plain `::=` BNF makes it a natural fit for this Rust codebase and a clean translation target from the `pest_meta`/`bnf`/`ebnf` ASTs in §A–B.

---

## Final table — Component → reuse recommendation

Legend: **DEPEND** = add as a dependency / emit toward; **PORT** = reimplement in Rust (permissive source); **STUDY** = clean-room from paper/spec (no usable/permissive code); **AVOID-LICENSE** = GPL/AGPL/CC-NC/none → do not link/vendor.

| Component | Category | License | Recommendation |
|---|---|---|---|
| **pest** | Rust PEG generator | MIT/Apache-2.0 | **DEPEND** (emit `.pest`; build-time parser) |
| **pest_meta** | PEG grammar AST/IR | MIT/Apache-2.0 | **DEPEND** (target `ast::Rule`/`Expr`; best introspectable IR) |
| **nom / winnow** | combinators | MIT | **DEPEND** (emit Rust; prefer **winnow**) |
| **combine** | combinators | MIT | DEPEND (lower priority; less active, verbose types) |
| **lalrpop** | LR(1) generator | Apache-2.0/MIT | DEPEND if grammar is LR-acceptable (emit `.lalrpop`) |
| **chumsky** | combinators (recovery/Pratt) | MIT | DEPEND (pre-1.0; emit Rust) |
| **peg** (rust-peg) | PEG proc-macro | MIT | DEPEND (ergonomic PEG codegen) |
| **lelwel** | LL(1) resilient, lossless CST | MIT/Apache-2.0 | **DEPEND/STUDY** (lossless-CST aligns with this repo) |
| **tree-sitter** | GLR runtime (FFI) | MIT | DEPEND as runtime front-end (JS→C codegen, not Rust target) |
| **earlgrey** | Earley | MIT | DEPEND (ambiguous grammars; EBNF-string or builder) |
| **santiago** | Earley | **GPL-3.0** | **AVOID-LICENSE** |
| **gearley / earley** | Earley | MIT-Apache / none | STUDY (gearley); AVOID (`earley`, no license) |
| **bnf** | BNF parser → AST | MIT | **DEPEND** (parse BNF input) |
| **ebnf / kbnf-syntax** | EBNF parser → AST | MIT | **DEPEND** (parse EBNF input; Wikipedia dialect) |
| **abnf / abnf-core / abnf_to_pest** | ABNF parser/transpiler | MIT/Apache-2.0 | **DEPEND** (parse ABNF; ABNF→pest bridge) |
| **ungrammar** | CST-shape DSL | MIT/Apache-2.0 | **DEPEND/STUDY** (closest to lossless-CST + concept model) |
| **ebnf-parser / dynparser** | EBNF/PEG-text parser | **GPL-3.0** | **AVOID-LICENSE** (reimplement runtime-PEG-text→AST) |
| **antlr-rust** | ANTLR4 runtime | BSD-3-Clause | DEPEND only if adopting ANTLR (Java tool does codegen) |
| **railroad** | syntax diagrams | MIT | DEPEND (docs/diagrams; Rust API, not a parser) |
| **GLADE** | CFG inference (oracle) | Apache-2.0 (Java) | PORT/STUDY (baseline algorithm) |
| **Arvada** | recursive CFG inference | MIT (Python) | PORT (superseded by TreeVada) |
| **TreeVada** | deterministic CFG inference | MIT (Python) | **PORT** (top pick for CFG-from-examples) |
| **Mimid** | white-box grammar mining | **CC BY-NC-SA** | **AVOID-LICENSE** / STUDY idea only |
| **REINAM** | RL grammar inference | **none/no repo** | STUDY (from paper) |
| **ISLa / ISLearn** | invariant learning/solving | **GPL-3.0** | **AVOID-LICENSE** → STUDY/clean-room ISLearn pipeline |
| **LearnLib** | active/passive automata (L\*/TTT/RPNI) | Apache-2.0 (Java) | **PORT/STUDY** (RPNI + MAT L\*/TTT) |
| **libalf** | automata learning (RPNI/Biermann) | **LGPL-3.0** | AVOID-LICENSE → STUDY/port algorithms |
| **flexfringe / DFASAT** | state-merging (EDSM/ALERGIA/SAT) | **GPL-3.0** | **AVOID-LICENSE** → external CLI or reimplement |
| **Grammarinator** | ANTLR-grammar fuzzer | BSD-3-Clause (Python) | STUDY (grammar-driven generation) |
| **nautilus / grammartec** | Rust grammar fuzzer | **MIT on `mit-main`** / AGPL on `master` | **PORT/DEPEND — pin `mit-main` only** |
| **autarkie** | Rust LibAFL grammar fuzzer | Apache-2.0 (Rust) | **DEPEND/STUDY** (derive-from-Rust-types pattern) |
| **GIToolbox** | GI algorithms (RPNI/EDSM/…) | MIT (MATLAB) | PORT (algorithm source) |
| **Sequitur** | online CFG from one sequence | reference-code **unverified** | **PORT** (from the paper) |
| **GBNF (llama.cpp)** | structured-gen grammar | MIT | **DEPEND** (primary CFG emit target) |
| **XGrammar** | structured-gen engine | Apache-2.0 | DEPEND (emit GBNF; default vLLM/SGLang backend) |
| **Outlines** | structured-gen | Apache-2.0 | DEPEND (emit JSON Schema / Lark-EBNF) |
| **Guidance / llguidance** | structured-gen | MIT | DEPEND (emit GBNF via `gbnf()`) |
| **lm-format-enforcer** | structured-gen | MIT | DEPEND (JSON Schema / regex only) |
| **json-schema-to-gbnf** | converter | MIT | DEPEND (JSON Schema → GBNF bridge) |
| **link-foundation/meta-notation** | inheritance root | Unlicense | **DEPEND** (mandated basis; delimiter/lossless model) |
| **links-notation (LiNo)** | textual surface | Unlicense | **DEPEND** (grammar serialization; 6 bindings) |
| **link-foundation/grammar-inference** | sibling tool (STUB) | Unlicense | **DEPEND/EXTEND** (intended home — currently template stub) |
| **link-foundation/grammar-expressions** | PEG/regex hybrid (links-native) | Unlicense | **DEPEND/PORT** (ready links-native engine to build on) |
| **rules-inference / lino-tokenizer / meta-ontology / meta-theory** | ecosystem | Unlicense | DEPEND/STUDY (siblings; rules-inference is a stub) |
| **js→rust / py→js translators** | rule-based translators | Unlicense | STUDY (relevant to "translate grammar to Rust/JS") |
| **meta-expression / formal-ai** | semantic lexicon / corpus | (was Unlicense) | **UNAVAILABLE 2026-06-19 (HTTP 404)** — 351-concept lexicon already internalized in this repo |

---

### Key takeaways for issue #93
1. **Inheritance root confirmed and reusable.** `meta-notation` (Unlicense, Rust+TS, working, 170+ tests) is a real, lossless, delimiter-typed, language-agnostic parser — the mandated basis maps cleanly onto this repo's existing lossless-links/projected-CST design.
2. **A sibling implementation slot already exists but is empty.** `link-foundation/grammar-inference` is literally "infer grammar from multiple examples in the same domain" — but it is **currently an unmodified Rust template stub**. The feature has a home awaiting code, and `grammar-expressions` (a links-native PEG/regex hybrid, Unlicense) is a ready building block.
3. **Best inference algorithm to port:** **TreeVada** (MIT, deterministic, fastest CFG-from-examples), with Arvada/GLADE as references and **RPNI** (from Apache LearnLib) for regular sublanguages; **Sequitur** for single-sequence CFGs.
4. **Best codegen IR + targets:** **`pest_meta`'s public AST** is the strongest introspectable grammar IR to emit into; **winnow/pest/peg/lelwel** are the cleanest Rust codegen targets; **`bnf`/`ebnf`/`abnf`** parse the input formats the issue lists (BNF/EBNF/ABNF); **GBNF** is the portable LLM-interop target.
5. **License hygiene:** avoid/clean-room **flexfringe, ISLa/ISLearn, santiago, dynparser, ebnf-parser** (GPL), **libalf** (LGPL), **Mimid** (CC-NC); and for **grammartec/nautilus** pin the **`mit-main`** branch — never `master` (AGPL).
6. **Honesty flag:** **`meta-expression` and `formal-ai` are not publicly accessible (HTTP 404) as of 2026-06-19**, though this repo's own 2026-06-05 survey cites them; their key asset — the **351-concept semantic lexicon** — already appears internalized here via `seed_common_concept_ontology()`.
agentId: a5ac27cc943d50abe (use SendMessage with to: 'a5ac27cc943d50abe' to continue this agent)
<usage>subagent_tokens: 129262
tool_uses: 23
duration_ms: 1278061</usage>