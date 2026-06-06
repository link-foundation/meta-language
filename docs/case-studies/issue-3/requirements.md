# Requirements Register — Issue #3

> Every requirement extracted from
> [issue #3](https://github.com/link-foundation/meta-language/issues/3) and its
> referenced founding vision
> [issue #1](https://github.com/link-foundation/meta-language/issues/1), each given
> a stable ID, traced to the **current implementation state** in this repository,
> and marked with a gap classification. Solution approaches for each ID are in
> [`solution-plans.md`](./solution-plans.md); filed implementation issues and
> their source specs are in [`proposed-issues/`](./proposed-issues/).

## How to read the "Current state" column

The repository today is a **self-contained Rust scaffold** (`src/`, 2,613 lines,
sole dependency `clap`). It ships:

- `LinkNetwork::parse` — a generic, byte-lossless LiNo-style parser/tokenizer
  (`src/link_network.rs`), **not** a grammar for any specific programming or
  natural language.
- The full public API surface (`src/lib.rs`): `LinkNetwork`, `NetworkProjection`,
  `verify_full_match` → `VerificationReport`, `reconstruct_text`, `LinkQuery`,
  `SubstitutionRule`/`apply_substitution`, `NetworkSnapshot`/
  `MutableNetworkSnapshot`, `TruthValue`, `EmbeddedRegion`, `ParseConfiguration`
  (with `TriviaAttachmentPolicy` + `RegionDetectionPolicy`).
- Registries in `src/parity.rs` (`PARITY_TARGETS`, `MARKUP_LANGUAGE_TARGETS`,
  `PROGRAMMING_LANGUAGE_TARGETS`, `NATURAL_LANGUAGE_TARGETS`,
  `GRAMMAR_EMBEDDING_TARGETS`, `PARITY_FIXTURES`, `LANGUAGE_FIXTURES`) plus
  `tests/unit/link_network.rs` gates asserting they stay present and that every
  fixture round-trips through `reconstruct_text`.

So the status vocabulary is:

| Status | Meaning |
|---|---|
| **Tracked** | A registry entry + executable fixture exists and is gated by tests, but it is backed by the generic scaffold parser — no real grammar yet. |
| **API-scaffolded** | The public type/method exists with generic behavior over the scaffold network. |
| **Partial** | Some real, language-agnostic behavior is implemented. |
| **Not started** | No code or registry entry yet. |

The honest summary: **the contract surface and the tracking harness exist; the
real per-language grammar integration and the competitor test corpora do not.**
Issue #3 is the request to plan exactly that work and break it into issues.

---

## Part A — Issue #3's own (process) requirements

These are the requirements the issue places on *this* deliverable (the case study).

| ID | Requirement (verbatim intent) | Current state | Status |
|---|---|---|---|
| **I3-1** | "create issues in this repository, to make sure our rust libraries include everything required to process for in and out for all top 10 programming and top 10 natural languages, as well as full support for txt, markdown and html in mixed mode" | 16 implementation issues filed with `gh` as [#5](https://github.com/link-foundation/meta-language/issues/5) through [#20](https://github.com/link-foundation/meta-language/issues/20); source specs and the idempotent creation script remain in [`proposed-issues/`](./proposed-issues/) | **Done** |
| **I3-2** | "copy all the tests from competitors and make sure we support all the features similar projects already support, so we beat them all, and their users can smoothly transition to our library" | Test-suite locations, formats, licenses, and per-project adaptation plans documented in [`competitor-test-suites.md`](./competitor-test-suites.md); tracked by implementation issues [#19](https://github.com/link-foundation/meta-language/issues/19) and [#20](https://github.com/link-foundation/meta-language/issues/20) | **Planned** |
| **I3-3** | "See issue #1 for vision and initial implementation" | Vision requirements extracted verbatim into Part C below; issue #1 raw JSON in [`raw-data/issue-1.json`](./raw-data/issue-1.json) | **Done** |
| **I3-4** | "collect data … compile that data to `./docs/case-studies/issue-{id}` folder" | `docs/case-studies/issue-3/` created: `raw-data/` (issues #1/#3, PR #2/#4 JSON) + research docs | **Done** |
| **I3-5** | "do deep case study analysis (also make sure to search online for additional facts and data)" | Three live-sourced research docs + [`online-research.md`](./online-research.md); rankings re-verified; founding-issue figures corrected | **Done** |
| **I3-6** | "list of each and all requirements from the issue" | This document (Parts A–E) | **Done** |
| **I3-7** | "propose possible solutions and solution plans for each requirement (check known existing components/libraries)" | [`solution-plans.md`](./solution-plans.md) + [`rust-libraries-survey.md`](./rust-libraries-survey.md) | **Done** |

**Note on I3-1 (creating issues):** the first draft left the issues as specs
because creating public GitHub issues is an outward-facing action. After maintainer
feedback on PR #4 explicitly requested creation, `create-issues.sh --create` was
run on 2026-06-06 and filed issues [#5](https://github.com/link-foundation/meta-language/issues/5)
through [#20](https://github.com/link-foundation/meta-language/issues/20). The
script is idempotent and now skips those issues by exact title.

---

## Part B — Language & format coverage requirements (the core "what")

From the issue #3 title and body: "in and out for all top 10 programming and top
10 natural languages, as well as full support for txt, markdown and html in mixed
mode." "In and out" = parse (in) **and** reconstruct (out), losslessly.

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **LANG-PL** | Parse **and** reconstruct each of the 10 TIOBE-May-2026 programming languages (Python, C, Java, C++, C#, JavaScript, Visual Basic, R, SQL, Delphi/Object Pascal) | All 10 in `PROGRAMMING_LANGUAGE_TARGETS` + one `LANGUAGE_FIXTURES` round-trip each, via scaffold parser | **Tracked** (no real grammars) |
| **LANG-NL** | Parse **and** reconstruct each of the 10 total-speaker natural languages (English, Mandarin, Hindi, Spanish, French, MSA, Bengali, Russian, Portuguese, Urdu) | All 10 in `NATURAL_LANGUAGE_TARGETS` + one non-ASCII `LANGUAGE_FIXTURES` round-trip each | **Tracked** (no segmentation/ID) |
| **LANG-TXT** | "full support for **txt**" — plain text as a first-class container | **Not in `MARKUP_LANGUAGE_TARGETS`** (only Markdown + HTML) | **Not started** ⚠️ gap |
| **LANG-MD** | "full support for **markdown**" | In `MARKUP_LANGUAGE_TARGETS` + fixture | **Tracked** (no CommonMark/GFM grammar) |
| **LANG-HTML** | "full support for **html**" | In `MARKUP_LANGUAGE_TARGETS` + fixture | **Tracked** (no HTML grammar) |
| **LANG-MIX** | "in **mixed mode**" — txt/markdown/html with embedded code/HTML/CSS/JS parsed into **one** links network with cross-language links on shared byte ranges | `GRAMMAR_EMBEDDING_TARGETS` (MD+code, MD+HTML, HTML+JS, HTML+CSS) + `mixed_regions.rs` (`detect_embedded_regions`, name- and content-driven `RegionDetectionPolicy`) | **Partial** (scaffold region detection; no real sub-grammars) |

**Gap callouts in Part B:**
- **LANG-TXT is the one explicitly-named target with no registry entry.** `txt`
  appears in the issue title and body but `MARKUP_LANGUAGE_TARGETS` has only
  Markdown and HTML. Plain text is the degenerate container (entire content is one
  prose/trivia region), and is also the fallback when content-sniffing fails — so
  it is both a real target and an architectural default. Implementation issue
  [#5](https://github.com/link-foundation/meta-language/issues/5) adds it.
- **Visual Basic, SQL, Delphi** have no off-the-shelf Rust/tree-sitter grammar
  (see [`rust-libraries-survey.md`](./rust-libraries-survey.md) §A). These three
  are the long-pole programming targets.
- **Mandarin** cannot use UAX#29 word segmentation (no spaces); it needs
  dictionary segmentation (`lindera`). **Arabic/Urdu** need bidi + normalization.

---

## Part C — Vision capability requirements (from issue #1 §3, verbatim)

Issue #1 §3 enumerates the substantive capabilities with checkboxes. Each is
reproduced here with its ID and current state. These are the "features similar
projects already support" that I3-2 says we must match and exceed.

### C.1 Universal structural representation (issue #1 §3.1)

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-1** | One mutable links network representing **CST + AST + semantic** relations **simultaneously over the same links** (different link-sets, not different trees) | `NetworkProjection::{Lossless,ConcreteSyntax,AbstractSyntax,Semantic}` projects one network into these views (`src/link_network.rs`) | **API-scaffolded** |
| **CORE-2** | **Lossless by construction** — reprint exact original bytes; trivia attached via explicit links; support **both** containment-attached (Rowan-style) **and** token-attached (Roslyn-style) trivia as configurable policy | `reconstruct_text` round-trips; `TriviaAttachmentPolicy` enum exists in `src/configuration.rs` | **Partial** (policy enum present; byte-lossless on scaffold) |
| **CORE-3** | Rich per-link metadata: link type, **named vs anonymous**, **fields as labeled links**, byte ranges, row/col points, flags `isError`/`hasError`/`isMissing`/`isExtra` | `LinkMetadata`, `LinkType` (15 variants), `LinkFlags`, `ByteRange`/`Point`/`SourceSpan` all present | **API-scaffolded** |
| **CORE-4** | **Error recovery / partial parsing** — always produce a network; ERROR/MISSING as links; "fully matches language X?" ⇔ no error/missing links in region | `verify_full_match` → `VerificationReport` with `VerificationIssueKind`; `LinkFlags` carry error/missing | **Partial** (works on scaffold; not grammar-driven) |

### C.2 Parse / verify / mixed languages (issue #1 §3.2)

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-5** | `parse(text, language) -> network` for a selected programming or natural language | `LinkNetwork::parse` exists but is language-generic (no per-language dispatch) | **Partial** |
| **CORE-6** | **Verify** a text fully matches a language, returning failing regions/links | `verify_full_match()` returns issues with spans | **API-scaffolded** |
| **CORE-7** | **Mixed-language parsing with auto-detected regions** → one unified network with cross-language links on shared byte ranges; **both** name-driven (fenced ` ```rust `) **and** content-driven (sniffing) detection | `mixed_regions.rs` (`detect_embedded_regions`, `EmbeddedRegion`); `RegionDetectionPolicy` covers name + content | **Partial** (detection scaffold; sub-regions not really parsed) |

### C.3 Self-description (issue #1 §3.3)

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-8** | The meta language **describes itself in its own terms**: `link`, `reference`, `relation link`, `language`, `grammar`, `type`, `concept` (and "node" = self-referential link, "edge" = connecting link) are links with definitions written in those terms | README documents self-description roots; `LinkType` enum + `definition` field on `LinkMetadata`; `describe` CLI subcommand | **Partial** |
| **CORE-9** | Include the **common roots of the common ontologies** — one shared base set from which language vocabularies derive (ties to RML `(Type: Type Type)` self-referential root) | Self-description roots listed; no derivation mechanism yet | **Partial** |

### C.4 Shared concepts mapped to per-language syntax (issue #1 §3.4)

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-10** | A **common concept layer** (function, binding, application, sequence, branch, loop, …) each mapped to different concrete syntax per language; languages mixing features compose concept sets; two languages sharing a concept reference the **same** concept link (no duplication) | `LinkType::Concept` + `Semantic` projection exist; no populated concept ontology or per-language syntax mapping | **Not started** (type exists, content does not) |

### C.5 Snapshots & mutation (issue #1 §3.5)

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-11** | **Immutable snapshots** (persistent, structurally shared) **and** **mutable snapshots** for in-place editing | `NetworkSnapshot` + `MutableNetworkSnapshot` in `src/snapshots.rs` | **API-scaffolded** |
| **CORE-12** | **Versioning** of the network over time (named references, provenance, forward version commits) | `snapshots.rs` provides provenance + forward version commits | **Partial** |

### C.6 Transformation by substitution (issue #1 §3.6)

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-13** | Transform via **substitution rules** on link-cli's single match→substitute operation | `SubstitutionRule` + `apply_substitution()` → `SubstitutionReport` (`src/substitution.rs`); create/update/delete/swap fixtures | **API-scaffolded** |
| **CORE-14** | **Advanced matching** by **syntax** (tree-sitter-query-like S-expressions over links: quantifiers, alternation, anchors, captures, fields, negated fields), by **meaning** (concept-level + RML evaluation), and by **type** | `LinkQuery` matches by link type / term / language / named flag (`src/query.rs`) — structural only, no S-expr/meaning/type matching | **Partial** (basic query only) |
| **CORE-15** | Predicates/text conditions as a **pluggable host-evaluated layer** (engine does structural match + capture binding; host does regex/eq/semantic) | Not implemented | **Not started** |

### C.7 Reconstruction & cross-language target (issue #1 §3.7)

| ID | Requirement | Current state | Status |
|---|---|---|---|
| **CORE-16** | **Reconstruct** to text: same language (lossless for unchanged regions, clean pretty-print for changed) **and** to a **different target language** via the common concept layer | `reconstruct_text()` (same-language); cross-language path depends on CORE-10 | **Partial** (same-lang only) |
| **CORE-17** | **Configurable formalization / deformalization (naturalization)** — text→network and network→text configurable (the knob formal-ai / meta-expression need; formalization levels 1–4) | `ParseConfiguration` exists; no formalization-level / naturalization config | **Partial** |

---

## Part D — Competitor & ecosystem parity requirements (issue #1 §6, I3-2)

Each project is a "test suite to satisfy." Detail + adaptation plans:
[`competitor-test-suites.md`](./competitor-test-suites.md) (external) and
[`ecosystem-foundations.md`](./ecosystem-foundations.md) (internal).

| ID | Source project | Required parity | Current state | Status |
|---|---|---|---|---|
| **PAR-1** | tree-sitter | Lossless CST, error recovery, mixed-language injection, S-expr query | Target + 1 fixture; capabilities tracked | **Tracked** |
| **PAR-2** | LibCST | Python lossless parse, trivia, same-language reconstruction | Target + 1 fixture | **Tracked** |
| **PAR-3** | Recast | JS/TS parse-print preservation (only modified subtrees reprint) | Target + 1 fixture | **Tracked** |
| **PAR-4** | jscodeshift | Transform workflows over JS/TS (find→replace→reserialize) | Target + 1 fixture + substitution tests | **Tracked** |
| **PAR-5** | Rowan + cstree | Persistent CST, immutable snapshots, checkpoint, interning | Targets + fixtures + snapshot tests | **Tracked** |
| **PAR-6** | Roslyn | C# syntax, trivia, diagnostics, formatting | Target + 1 fixture + recovery test | **Tracked** |
| **PAR-7** | links-notation | LiNo doublets/triplets/N-tuples/indentation/self-reference | Target + fixture + self-ref tests | **Tracked** |
| **PAR-8** | link-cli | Single match→substitute (create/update/delete/swap, variables) | Target + create/update/delete/swap tests | **Tracked** |
| **PAR-9** | lino-objects-codec | `decode(encode(x))==x`, shared + circular refs | Target + object fixture + identity tests | **Tracked** |
| **PAR-10** | relative-meta-logic | Dependent types, many-valued, paradox, `TruthValue` | Target + dependent-type fixture + `TruthValue` tests | **Tracked** |
| **PAR-11** | formal-ai | Formalization corpus replay as no-regression gate | Target + 1 fixture; corpus **not** replayed | **Tracked** (corpus not imported) |
| **PAR-12** | meta-expression | formalize→semantic→naturalize round-trip, cross-language, spans | Target + fixture + en/es reconstruction | **Tracked** |

For each `PAR-x`, "Tracked" means one illustrative fixture exists; the **bulk
adoption** of each upstream's real corpus (I3-2's "copy **all** the tests") is the
outstanding work, tracked by implementation issues
[#19](https://github.com/link-foundation/meta-language/issues/19) and
[#20](https://github.com/link-foundation/meta-language/issues/20).

---

## Part E — Cross-cutting / non-functional requirements

| ID | Requirement | Source | Current state |
|---|---|---|---|
| **NFR-1** | Terminology: **never use "graph"**; everything is links / links network; translate external node/edge/tree vocabulary at the adapter boundary | issue #1 terminology note | Honored in code + docs |
| **NFR-2** | Tests live under `tests/` only — CI rejects `#[test]`/`#[cfg(test)]`/`mod tests` under `src/` | `CONTRIBUTING.md` | Honored |
| **NFR-3** | Changelog fragment in `changelog.d/` per user-facing runtime change (bump: major/minor/patch) | `CONTRIBUTING.md` + CI change classifier | This PR is docs-only and does not trigger the changelog gate; each filed implementation issue includes a changelog-fragment acceptance criterion |
| **NFR-4** | Crate-size / file-size limits enforced by `scripts/check-*.rs` | `CONTRIBUTING.md` | Must keep docs out of crate package |
| **NFR-5** | "Try both if no conflicts; both as configurable alternatives if they conflict" — coexisting solutions should both be supported, conflicting ones offered as configuration | konard on issue #1 (`raw-data/issue-1-comments.json`) | Drives the "configurable policy" stance (e.g. trivia policy, region detection) |
| **NFR-6** | First implementation language is **Rust**; other languages may follow (parity tradition) | issue #1 §5 | Rust crate in place |
| **NFR-7** | Reuse existing ecosystem components (`links-notation`, `link-cli`, `doublets`, etc.) rather than reinvent | issue #1 §2, I3-7 | Survey done; not yet wired as deps |

---

## Traceability summary

- **Process requirements (I3-1…I3-7):** all addressed by this case study; I3-1
  delivers filed GitHub implementation issues [#5](https://github.com/link-foundation/meta-language/issues/5)
  through [#20](https://github.com/link-foundation/meta-language/issues/20).
- **Coverage requirements (Part B):** 20 languages + Markdown + HTML are **Tracked**
  (registry + fixture, scaffold-backed); **txt is the one missing registry entry**;
  mixed mode is **Partial**.
- **Capability requirements (CORE-1…CORE-17):** contract surface exists for all;
  ~6 are **API-scaffolded**, ~8 **Partial**, **CORE-10** and **CORE-15** are
  effectively **Not started** (type exists, behavior does not).
- **Parity requirements (PAR-1…PAR-12):** all **Tracked** with one fixture each;
  full corpus adoption outstanding.

The single most important honest finding: **this repository has an excellent
tracking and contract scaffold, but real grammar integration (tree-sitter +
gap-grammars), the common concept ontology, and the imported competitor corpora
are the substantive work issue #3 asks us to plan.** That plan is
[`solution-plans.md`](./solution-plans.md).
