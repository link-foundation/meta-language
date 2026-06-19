# Existing Capabilities & Gap Analysis (issue #93)

What the `meta-language` crate already provides that issue #93 can build on, and
what is genuinely missing. Grounded in the source tree at the issue-93 branch
point (crate version `0.45.0`). File references are `path:line` against that
tree.

## 1. Reusable substrate that already exists

| Capability | Where | Relevance to #93 |
|---|---|---|
| Mutable links network, lossless parse, byte-exact `reconstruct_text()` | `src/link_network.rs`, `README.md:14-27` | The grammar IR (P-8) is *stored as links* in this network; inference results and imported grammars become first-class links. |
| `LinkType` enum **already has a `Grammar` variant** | `src/link_network.rs:51`, `:112` | The role tag for grammar links exists — but **nothing constructs grammar links yet** (see gaps). |
| Self-description root for `grammar` | `src/self_description.rs:28-32` (`term: "grammar"`, `references: ["grammar","concept"]`, `link_type: LinkType::Grammar`) | A grammar is already declared to *reference concepts* — the seed of the concept-aligned model (P-11). |
| Network projections (Lossless / ConcreteSyntax / AbstractSyntax / Semantic) | `src/link_network.rs:64-103` | Lets an inferred/imported grammar be viewed at CST, AST, or concept level for free. |
| Concept ontology: exact-match interning, language-bound expressions, external-id aliases, LiNo concept-set import, `seed_common_concept_ontology()` (351-concept lexicon + structural PL concepts) | `README.md:66-69`, `src/concept_ontology.rs`, `src/semantics.rs` | The "shared concepts" P-11 relies on. Grammar-construct concepts (rule, sequence, choice, repetition…) extend this same table (A3). |
| Cross-language reconstruction `reconstruct_text_as(...)` + `FormalizationLevel` | `README.md:235-258`, `src/configuration.rs` | The proven 1-to-1 translation path (English↔Russian↔concept). The grammar translator (C6) reuses the same concept-layer mechanism. |
| `TranslationRule` / `TranslationTemplate` / `TranslationRuleRegistry` / `TranslationRuleSet` with `to_lino`/`from_lino` | `src/translation_rules.rs:23-405` | A declarative, LiNo-serializable rewrite engine — the natural substrate for grammar→{Rust,JS,…} codegen (C4, C5) instead of hand-rolled string templating. |
| `rust_codec`: `ToLinks`/`FromLinks`, `RustTypeShape`/`RustFieldShape`/`RustTypeKind` | `src/rust_codec.rs`, `lib.rs:83-86` | Existing Rust-shape modelling to emit Rust types/parsers from a grammar (C4). |
| `LinkQuery` (S-expression, tree-sitter-query-like), `find()`/`replace()`, `query_algebra::LinkRule` | `README.md:54-57`, `src/query.rs`, `src/query_algebra.rs` | Pattern matching over the network — used by inference (pattern discovery) and by the translator (rule LHS matching). |
| Pluggable **`ParserRegistry` + `LanguageParser` trait** (case-insensitive dispatch, user registrations shadow built-ins) | `src/parser_registry.rs:50-159`, `src/language_parser.rs:7-47` | **The extension point.** A format importer (B*) and an inferred runtime parser (E2) register here without forking the pipeline. |
| tree-sitter adapter, ~30 wired grammars (Python, Java, C/C++, C#, JS/TS, Rust, Go, Ruby, SQL, HTML, CSS, JSON, YAML, TOML, XML, INI, protobuf, GraphQL, PHP, Swift, Kotlin, Scala, Lua, Perl, Pascal, VB) | `src/tree_sitter_adapter.rs:136-207`, `Cargo.toml:57-85` | Each is a *golden CST oracle* — a free source of correct parse trees to bootstrap and evaluate inference (D5, D1), and a `grammar.json` import source (B5). |
| LiNo parsing/serialization (`links-notation` 0.13) | `Cargo.toml:53`, `src/lino_parser.rs`, `src/lino_serialization.rs` | The textual surface a meta-grammar serialises to (P-8) before translation. |
| Parity registry + executable `PARITY_FIXTURES`, `LANGUAGE_FIXTURES`, `GRAMMAR_EMBEDDING_TARGETS`, `PROGRAMMING_LANGUAGE_TARGETS` | `src/parity.rs:340-811`, `lib.rs:66-72` | Existing executable-fixture discipline + ready example corpora for inference inputs and competitor gates (D1, E3). |
| Mixed-region grammar embedding (Markdown code fences, HTML script/style) | `README.md:50-53`, `src/mixed_regions.rs` | Precedent for "one unified network from multiple grammars" — relevant to multi-grammar inference. |
| Document-format round-trip + per-format fidelity matrices (PDF/DOCX/MD/HTML), `docs/cross-format-fidelity.md` | `README.md:84-110` | The **template** for the grammar-format fidelity matrix (F2) and the cross-format reconstruction pattern (C6). |
| `LinkStore`/`EngineLinkStore` (+ optional `doublets`), snapshots, incremental `apply_edit()` | `README.md:36-62` | Persistence + incremental reparse for large grammar corpora. |
| CLI scaffold (`describe`, `verify`) via `clap` | `README.md:285-295`, `src/main.rs` | The place new `infer` / `translate-grammar` subcommands attach (E1). |

## 2. meta-notation lineage (P-4) — already mirrored, not yet explicit

`meta-notation` (`link-foundation/meta-notation`, Unlicense, Rust+TS, 170+
shared tests) parses the universal delimiter skeleton — brackets `() {} []`
(nested), quotes `'' "" \`\`` (opaque), and text blocks — with lossless
round-trip. This repo already embodies the same model: lossless tokenisation,
delimiter-aware structure, and LiNo (the parent notation of meta-notation). What
is **missing** is using that delimiter skeleton as an *explicit structural prior*
for inference (D6) and declaring the meta-language grammar surface as a formal
*derivative* of meta-notation (A2). See
[`library-survey.md`](./library-survey.md) §D.1.

## 3. Gaps — what #93 must add (nothing below exists today)

| Gap | Evidence it is absent | Proposed issue(s) |
|---|---|---|
| **No grammar IR / algebra.** `LinkType::Grammar` is only a label + self-description node; there is no type/struct that models rules and PEG/CFG expressions (sequence, ordered choice, repetition, optional, terminal, non-terminal, char-class, predicates, captures), and nothing constructs `Grammar` links. | No `grammar`/`grammar_ir` module in `src/lib.rs:1-36`; `Grammar` appears only at `link_network.rs:51,112` and `self_description.rs`. | **A1** |
| **No grammar surface syntax** for authoring grammars in the meta-language / meta-notation. | No grammar parser module; `lino_parser` handles links, not rules. | **A2** |
| **No grammar-construct concepts** in the ontology. | `seed_common_concept_ontology()` seeds linguistic + structural PL concepts, not grammar-algebra concepts. | **A3** |
| **No grammar-format importers** (BNF, EBNF, ABNF, PEG, ANTLR `.g4`, Lark, GBNF, tree-sitter `grammar.json`). | No `bnf`/`ebnf`/`abnf`/`pest_meta` deps in `Cargo.toml:35-89`; no importer modules. | **B1–B7** |
| **No grammar-format emitters** (IR → BNF/EBNF/ABNF/PEG/GBNF text). | No emitter modules; `reconstruct_text_as` targets document/natural formats, not grammar formats. | **C1–C3** |
| **No parser codegen** (IR → Rust / JavaScript parser source). | `rust_codec` emits *data* types, not *parsers*; no JS codegen anywhere. | **C4, C5** |
| **No grammar inference of any kind** (no state-merging/RPNI, no Sequitur, no black-box CFG induction, no constraint learning). | No inference module; nothing depends on or ports TreeVada/Arvada/GLADE/LearnLib. | **D2–D9** |
| **No inference evaluation harness** (precision/recall/F1 by sampling, MDL/Occam scoring, golden corpus). | No metrics module; parity fixtures test parse parity, not inference quality. | **D1** |
| **No competitor benchmark suite** for P-12. | No benchmark harness or vendored corpora. | **E3** |
| **No CLI for inference / grammar translation.** | `src/main.rs` exposes only `describe`/`verify`. | **E1** |
| **No runtime use of an inferred grammar** as a registered parser. | `ParserRegistry` exists but no inferred-grammar `LanguageParser` impl. | **E2** |

## 4. Architectural conclusion

Issue #93 is a **large greenfield feature with unusually good seams**. The
grammar IR (A1) is the keystone: it is *stored in the existing links network*,
*tagged with the existing `LinkType::Grammar`*, *aligned to the existing concept
ontology* (A3 → P-11), *imported into* via new `LanguageParser`-style adapters
(B*), *emitted/translated* through the existing `TranslationRule`/`rust_codec`
machinery (C*), *produced* by the new inference engine (D*), and *consumed* at
runtime through the existing `ParserRegistry` (E2). Almost every new component
attaches to an existing, tested boundary rather than replacing one — which is
why the work decomposes cleanly into the dependency DAG in
[`solution-plans.md`](./solution-plans.md).
</content>
