# F1 — Grammar-subsystem user & architecture docs

> **Epic:** F — Documentation & fidelity · **Blocked by:** [`A1`](./A1-grammar-ir.md) · **Blocks:** —
> **Requirements:** P-1 · **Milestone:** M1
> Part of the issue #93 grammar-extensibility & inference initiative. Background:
> [`solution-plans.md`](../solution-plans.md) §Epic F (F1), §5 milestone M1,
> [`requirements.md`](../requirements.md) P-1.

## Context

Requirement **P-1** is "documentation describing what the meta language is and
what it is for" ([`requirements.md`](../requirements.md) P-1). The repo already
documents its substrate well — [`README.md`](../../../../README.md) has
`## What Is Implemented`, `## Usage`, `## CLI`, and `## Development` sections, and
[`docs/`](../../../../docs) holds focused pages (`cross-format-fidelity.md`,
`pdf-fidelity.md`, `docx-fidelity.md`, `parity-roadmap.md`). The grammar subsystem
introduced across Epics A–E adds a large new surface — an IR, importers, emitters,
codegen, inference, translation, a CLI, and a runtime parser — and **none of it is
documented yet**.

F1 delivers the user-facing and architecture documentation for that subsystem. It
is blocked only by [`A1`](./A1-grammar-ir.md) (the IR is the keystone every other
page links to) and lands in **Milestone M1** alongside A1/A2/A3 so the documentation
grows with the code rather than trailing it. As later issues land, each extends the
relevant page (each Epic-B/C/D/E issue's plan already lists "extend the grammar
docs"); F1 establishes the structure, the architecture overview, and the
author-facing how-to, and seeds the per-stage pages so those extensions have a home.

## Goal

Create a coherent grammar-subsystem documentation set under
[`docs/grammar/`](../../../../docs) plus README integration, explaining **what** the
grammar layer is, **why** it exists, and **how** to author ([`A2`](./A2-grammar-surface-syntax.md)),
import ([`B1`](./B1-bnf-importer.md)–B7), emit ([`C1`](./C1-bnf-ebnf-abnf-emitters.md)–[`C3`](./C3-gbnf-emitter.md)),
generate parsers ([`C4`](./C4-rust-parser-codegen.md)/[`C5`](./C5-javascript-parser-codegen.md)),
infer ([`D5`](./D5-blackbox-cfg-inference.md)), translate ([`C6`](./C6-concept-aligned-translation.md)),
and run ([`E2`](./E2-inferred-grammar-runtime-parser.md)) grammars — with an
architecture overview of the grammar IR and its links encoding, and runnable code
examples kept honest by `cargo test --doc`.

## Scope

**In scope**
- A documentation home `docs/grammar/` with an index plus pages (see Design).
- An **architecture overview**: the [`A1`](./A1-grammar-ir.md) IR (`Grammar`,
  `GrammarRule`, `GrammarExpr`, `RuleKind`, `GrammarFormat`), how a grammar is
  *stored as links* (the [`A1`](./A1-grammar-ir.md) `ToLinks`/`FromLinks` encoding,
  `LinkType::Grammar` at `src/link_network.rs:51`, the `grammar` self-description
  root at `src/self_description.rs:28-32`), and the end-to-end pipeline diagram
  (author/import → IR → infer → emit/translate/codegen → run).
- A **user how-to** for each stage, cross-linking the owning issue and the public
  API, with runnable, doctested snippets where the API exists.
- **README integration**: a new `## Grammar subsystem` section (after `## CLI`)
  summarising the layer and linking into `docs/grammar/`.

**Out of scope** (owned elsewhere)
- The APIs being documented — F1 documents what A1–E ship; it adds no library code.
- The grammar-format **fidelity matrix** page → [`F2`](./F2-grammar-format-fidelity-matrix.md)
  (F1 links to it; F2 owns it).
- The concept-ontology reference for grammar constructs → [`A3`](./A3-grammar-concept-ontology.md)
  (F1's architecture page links to A3's documentation).
- Website assembly / `rustdoc` hosting — already handled by
  [`scripts/build-site.rs`](../../../../scripts/build-site.rs) and the `Deploy
  Website` job; F1 only adds Markdown + rustdoc comments that flow into the existing
  pipeline.

## Design / specification

### Documentation layout (`docs/grammar/`)

| Page | Contents |
|---|---|
| `docs/grammar/README.md` | Index + one-paragraph "what & why" (P-1), the pipeline diagram, and links to every page below and to the owning issues. |
| `docs/grammar/architecture.md` | The IR ([`A1`](./A1-grammar-ir.md)): `Grammar`/`GrammarRule`/`GrammarExpr`/`RuleKind`/`GrammarFormat`; the links encoding (`ToLinks`/`FromLinks`, the round-trip invariant), `LinkType::Grammar` (`src/link_network.rs:51`), the `grammar` self-description root (`src/self_description.rs:28-32`); projections (`src/link_network.rs:64-103`); how it relates to the concept ontology ([`A3`](./A3-grammar-concept-ontology.md)). |
| `docs/grammar/authoring.md` | Writing grammars in the surface syntax ([`A2`](./A2-grammar-surface-syntax.md)) — `parse_grammar_surface` / `write_grammar_surface`; validation & friendly errors ([`E4`](./E4-grammar-authoring-ergonomics.md) `validate`). |
| `docs/grammar/import-export.md` | Importing other notations ([`B1`](./B1-bnf-importer.md)–B7) and emitting them back ([`C1`](./C1-bnf-ebnf-abnf-emitters.md)–[`C3`](./C3-gbnf-emitter.md)); the format table; links to [`F2`](./F2-grammar-format-fidelity-matrix.md) for fidelity. |
| `docs/grammar/codegen.md` | Generating parsers from a grammar: Rust ([`C4`](./C4-rust-parser-codegen.md)) and JavaScript ([`C5`](./C5-javascript-parser-codegen.md)); the runnable examples from [`E5`](./E5-end-to-end-integration-examples.md). |
| `docs/grammar/inference.md` | Inferring a grammar from examples ([`D5`](./D5-blackbox-cfg-inference.md)) and the delimiter-skeleton prior ([`D6`](./D6-delimiter-structural-prior.md)); evaluation ([`D1`](./D1-inference-evaluation-harness.md)). |
| `docs/grammar/translation.md` | Translating a grammar between notations through the concept layer ([`C6`](./C6-concept-aligned-translation.md)), mirroring the document-format translation story in [`docs/cross-format-fidelity.md`](../../../../docs/cross-format-fidelity.md). |
| `docs/grammar/cli-and-runtime.md` | The [`E1`](./E1-cli-grammar-subcommands.md) subcommands (`infer`/`import-grammar`/`emit-grammar`/`translate-grammar`) and registering an inferred grammar as a runtime parser ([`E2`](./E2-inferred-grammar-runtime-parser.md), `ParserRegistry` at `src/parser_registry.rs:50-159`). |

Each page opens with a one-line "owned by issue X" pointer, a minimal example, and
"see also" links — matching the tone of the existing `docs/*.md` pages.

### Architecture overview content

The `architecture.md` page is the anchor (it is what P-1 most directly satisfies for
the grammar layer). It must explain, in prose + one diagram:

1. **The grammar is data in the network.** A `Grammar` is not a side object; it
   lowers to links tagged `LinkType::Grammar` (`src/link_network.rs:51`), rooted at
   the existing `grammar` self-description node (`src/self_description.rs:28-32`,
   which already declares `references: ["grammar","concept"]`). State the
   [`A1`](./A1-grammar-ir.md) round-trip invariant (`from_links(to_links(g)) == g`).
2. **One IR, many fronts.** A diagram showing author/import → **IR** →
   infer/emit/translate/codegen/run, with each arrow labelled by its owning issue.
3. **Concept alignment.** How grammar-construct concepts ([`A3`](./A3-grammar-concept-ontology.md))
   let translation ([`C6`](./C6-concept-aligned-translation.md)) reuse the same concept-layer
   reconstruction proven for documents ([`docs/cross-format-fidelity.md`](../../../../docs/cross-format-fidelity.md)).

### Doctested examples

Where a public API already exists at F1's writing time (the [`A1`](./A1-grammar-ir.md)
builder is the guaranteed one, since A1 is the only hard dependency), the snippet is
a real rustdoc doctest on the relevant item (so `cargo test --doc` compiles and runs
it). For APIs from not-yet-merged issues, the Markdown snippet is fenced as
```text``` (not `rust`) so it is never compiled prematurely, and is upgraded to a
doctest by the owning issue when that API lands. This keeps F1 mergeable at M1 while
guaranteeing the *committed* examples are executed.

## File-level plan

| File | Change |
|---|---|
| `docs/grammar/README.md` | New. Index, "what & why" (P-1), pipeline diagram, links. |
| `docs/grammar/architecture.md` | New. IR + links-encoding + pipeline architecture overview. |
| `docs/grammar/authoring.md` | New. Surface syntax + validation how-to. |
| `docs/grammar/import-export.md` | New. Importers + emitters + format table. |
| `docs/grammar/codegen.md` | New. Rust/JS parser codegen how-to. |
| `docs/grammar/inference.md` | New. Inference-from-examples how-to. |
| `docs/grammar/translation.md` | New. Cross-notation translation how-to. |
| `docs/grammar/cli-and-runtime.md` | New. CLI subcommands + runtime registry. |
| `README.md` | Add `## Grammar subsystem` after `## CLI` (line 285) linking into `docs/grammar/`. |
| `src/grammar/mod.rs` (or `src/lib.rs`) | Add/expand a module-level `//!` doctest example for the A1 builder so `cargo test --doc` covers the headline snippet. |
| `changelog.d/` | Add a docs fragment (CI treats docs-only PRs specially per `.github/workflows/release.yml:76`). |

## Reuse

- Existing docs structure & tone — `docs/cross-format-fidelity.md`,
  `docs/pdf-fidelity.md`, `docs/parity-roadmap.md`; the README section pattern
  (`## What Is Implemented`, `## Usage`, `## CLI`, `## Development`).
- Existing doctest gate — CI runs `cargo test --doc --verbose`
  (`.github/workflows/release.yml:262-263`); rustdoc fences already appear in
  `src/main.rs`, `src/lino_serialization.rs`, etc.
- Existing website pipeline — [`scripts/build-site.rs`](../../../../scripts/build-site.rs)
  and the `Deploy Website` job assemble `rustdoc` under `/api/`; F1's rustdoc
  comments flow through unchanged.
- The grounding analyses — [`existing-capabilities.md`](../existing-capabilities.md)
  (IR-as-links, `ParserRegistry`, concept ontology) is the source for the
  architecture page's claims.

## Acceptance criteria

- [ ] `docs/grammar/` exists with the eight pages above; `docs/grammar/README.md`
      answers "what is the grammar layer and what is it for" (P-1) and links every
      page.
- [ ] `architecture.md` documents the [`A1`](./A1-grammar-ir.md) IR, the
      links encoding (`LinkType::Grammar`, the `grammar` self-description root), the
      round-trip invariant, and the pipeline diagram with per-issue arrow labels.
- [ ] Every how-to page names its owning issue and the public API it covers, and
      links to the next stage; `import-export.md` links to
      [`F2`](./F2-grammar-format-fidelity-matrix.md) for fidelity.
- [ ] README gains a `## Grammar subsystem` section after `## CLI` that links into
      `docs/grammar/`.
- [ ] Every fenced ```rust``` snippet in the committed docs/rustdoc is a real
      doctest and passes `cargo test --doc --verbose`; snippets for not-yet-merged
      APIs are fenced ```text``` so nothing is compiled prematurely.
- [ ] All internal Markdown links resolve to existing files (verified by the
      link-resolution test below — the repo has no external link-checker, so this is
      enforced by a committed test, not assumed CI).
- [ ] `cargo fmt --check`, `cargo clippy --all-targets --all-features`, and
      `cargo test --all-features` pass (clippy lints unchanged; the only code change
      is rustdoc comments).

## Tests

- **Doctests:** the headline A1-builder example is a rustdoc doctest, run by
  `cargo test --doc` (already in CI at `release.yml:262`).
- **Link resolution:** a unit test under `tests/unit/` (e.g.
  `grammar_docs_links.rs`) walks every `docs/grammar/*.md`, extracts relative
  Markdown links, and asserts each target path exists — giving F1 a reproducible
  "links valid" gate without a new dependency (the same discipline E5 uses for
  fixtures). Registered in `tests/unit/mod.rs`.
- **Presence check:** the same test (or a sibling) asserts each expected
  `docs/grammar/*.md` page exists, so a renamed/removed page fails CI rather than
  silently breaking the index.

## References

- [`requirements.md`](../requirements.md) P-1; [`solution-plans.md`](../solution-plans.md)
  §Epic F (F1), §5 milestone M1.
- Subsystem issues linked by the docs: [`A1`](./A1-grammar-ir.md),
  [`A2`](./A2-grammar-surface-syntax.md), [`A3`](./A3-grammar-concept-ontology.md),
  [`B1`](./B1-bnf-importer.md), [`C1`](./C1-bnf-ebnf-abnf-emitters.md),
  [`C3`](./C3-gbnf-emitter.md), [`C4`](./C4-rust-parser-codegen.md),
  [`C5`](./C5-javascript-parser-codegen.md), [`C6`](./C6-concept-aligned-translation.md),
  [`D5`](./D5-blackbox-cfg-inference.md), [`E1`](./E1-cli-grammar-subcommands.md),
  [`E2`](./E2-inferred-grammar-runtime-parser.md), [`E4`](./E4-grammar-authoring-ergonomics.md),
  [`F2`](./F2-grammar-format-fidelity-matrix.md).
- Models & grounding: [`README.md`](../../../../README.md) (§What Is Implemented:14-27,
  §CLI:285-295, §Development:353-360), `docs/cross-format-fidelity.md`,
  `docs/pdf-fidelity.md`, [`existing-capabilities.md`](../existing-capabilities.md),
  CI doc gate `.github/workflows/release.yml:262-263`.
