# Solution Plans — Issue #3

> For every requirement ID in [`requirements.md`](./requirements.md), this document
> proposes a concrete solution, names the existing component/library that solves
> (or accelerates) it, records alternatives, and lays out a phased plan. Library
> facts come from [`rust-libraries-survey.md`](./rust-libraries-survey.md),
> [`competitor-test-suites.md`](./competitor-test-suites.md), and
> [`ecosystem-foundations.md`](./ecosystem-foundations.md).

## Guiding principles (from the issue thread)

1. **Reuse over reinvent** (issue #1 §2, I3-7): adopt existing permissively-licensed
   crates as front-end adapters; the crate's novelty is the *unified links network*,
   not a new parser per language.
2. **Try both when compatible, configure when conflicting** (konard,
   `raw-data/issue-1-comments.json`, NFR-5): where two approaches coexist (e.g.
   name-driven *and* content-driven region detection), ship both; where they
   conflict (e.g. Rowan-style vs Roslyn-style trivia attachment), expose a
   configuration knob with a documented default.
3. **Fidelity comes from spans, not the sub-library's serializer** (CORE-2): we
   keep original UTF-8 bytes + byte ranges and reconstruct from those, so a
   front-end that is *not* itself byte-exact (pulldown-cmark, html5ever) is still
   usable — we never round-trip through its printer.
4. **Never leak external vocabulary** (NFR-1): every adapter translates
   nodes/edges/trees/AST into links at the boundary.

---

## Solution 1 — Universal parser front-end via tree-sitter

**Covers:** LANG-PL (7 of 10), LANG-MD, LANG-HTML, LANG-MIX, CORE-1, CORE-3,
CORE-4, CORE-5, CORE-7, PAR-1, and underpins CORE-2/CORE-16.

**Approach.** Add one `tree-sitter` → links adapter: walk the parse tree depth-first,
emit one link per node carrying its `LinkType`, named/anonymous flag, field label
(tree-sitter fields → `LinkType::Field` labeled links), `ByteRange`, and row/col
`Point`s; synthesize `Trivia`/`Token` links from the byte gaps between child spans;
map tree-sitter `ERROR`/`MISSING` nodes to `LinkFlags` (`is_error`/`is_missing`/
`has_error`). Because every link carries a byte range and we retain the source
buffer, `reconstruct_text()` is byte-exact regardless of the grammar.

**Existing components (all MIT, on crates.io):**
- `tree-sitter` 0.26.9 (engine: lossless spans, error recovery, S-expr queries,
  injection).
- Official grammar crates: `tree-sitter-python`, `-c`, `-java`, `-cpp`,
  `-c-sharp`, `-javascript`, `-r` → **7 of 10** programming targets immediately.
- `tree-sitter-html`, `tree-sitter-css` → markup + mixed-mode sub-grammars.

**Why this is the keystone.** A single adapter delivers `LosslessParsing`,
`TriviaPreservation`, `ErrorRecovery`, `QueryMatching`, and `MixedLanguageRegions`
for every grammar that exists, across programming *and* markup languages. It also
gives PAR-1 (tree-sitter parity) almost for free.

**Alternatives (NFR-5 "try both"):**
- For **JavaScript**, additionally wire `biome_js_parser` (native-Rust lossless
  CST) as a second source — useful to cross-check the tree-sitter adapter and to
  serve codemod-grade JS/TS. Non-conflicting → ship both behind a parser-selection
  config.
- For **Rust** (if ever added), `ra_ap_syntax` (rowan-based) is the native lossless
  option.

**Plan.** (1) Define the `LanguageParser` trait + tree-sitter adapter. (2) Wire the
7 official grammars, replacing each scaffold `LANGUAGE_FIXTURES` entry's backing
with a real parse while keeping the byte-exact round-trip assertion. (3) Add query
lowering (tree-sitter S-expr → `LinkQuery`). → proposed issues **`#03`, `#11`**.

---

## Solution 2 — Grammars for the three gap languages

**Covers:** LANG-PL (remaining 3 of 10): Visual Basic, SQL, Delphi/Object Pascal.

**Approach & components (from survey §A):**
- **Visual Basic** — *the biggest gap; no official grammar in any ecosystem.*
  Options, in order: (a) vendor/evaluate a third-party tree-sitter grammar
  (`arborium-vb`, CodeAnt-AI's `tree-sitter-vb` fork) and harden it; (b) if none is
  adequate, author a tree-sitter grammar for the VB.NET subset we need. Highest
  effort — call it out explicitly as a long pole.
- **SQL** — fragmented; adopt the most-maintained `tree-sitter-sequel`, accept it is
  dialect-leaning, and treat dialects (ANSI / PostgreSQL / SQLite / BigQuery) as
  separate language keys under an `sql` family rather than pretending one grammar
  covers all. (NFR-5: multiple dialect grammars coexist → register each.)
- **Delphi/Object Pascal** — only generic `tree-sitter-pascal` exists; adopt it and
  document the Delphi-specific constructs it misses; extend or fork as needed.

**Plan.** One proposed issue per gap language (**`#06`, `#07`, `#08`**) since each is an
independent, sizeable grammar-acquisition effort with its own risk profile.

---

## Solution 3 — Plain-text (`txt`) container

**Covers:** LANG-TXT (the missing registry entry), and the sniffing fallback for
LANG-MIX.

**Approach.** `txt` is the degenerate container: the whole buffer is one
prose/region link (all trivia/text, no embedded grammar) and is also the
**default fallback** when content-sniffing cannot identify a language. Implementation
is small: add `Txt` to `MARKUP_LANGUAGE_TARGETS`, add a `LANGUAGE_FIXTURES` entry
(a multi-line UTF-8 text sample that round-trips), and make the region detector
fall back to a single `txt` region. No external library needed.

**Why it matters.** It is explicitly named in the issue title/body, it closes the
one un-tracked target, and it makes the mixed-mode contract total (every byte
belongs to *some* region, even if that region is "plain text"). → proposed issue
**`#01`** (small, do first).

---

## Solution 4 — Natural-language segmentation & identification

**Covers:** LANG-NL (all 10), supports CORE-5/CORE-6 for natural languages.

**Approach.** Byte-exact reconstruction of natural language is a *storage* property
(keep the UTF-8 bytes + spans — already true on the scaffold). The added value is a
**segmentation + identification** link layer that never mutates stored bytes:
- **Segmentation:** `unicode-segmentation` (UAX#29 graphemes/words/sentences) for
  the 9 space-or-delimiter scripts; `lindera` (CJK dictionary segmentation) for
  **Mandarin**, which UAX#29 cannot word-segment.
- **Identification:** `lingua` (75 languages incl. all 10, accurate on short text)
  as primary; `whatlang` (faster, lighter) as an alternative. NFR-5: non-conflicting
  → expose detector choice as config, default `lingua`.
- **Normalization & bidi:** `unicode-normalization` (NFC/NFD) so Hindi/Bengali/
  Arabic/Urdu compare and reconstruct correctly; `unicode-bidi` (UAX#9) for
  Arabic/Urdu RTL handling. These annotate; they do not rewrite the stored bytes.

**Plan.** A `NaturalLanguageSegmenter` layer emitting `Token`/`Semantic` links over
the existing lossless text, with a script-aware dispatch (CJK→lindera, else→
unicode-segmentation). Identification populates a `Language` link per region. →
proposed issue **`#05`**.

---

## Solution 5 — Mixed-mode embedding (one network, cross-language links)

**Covers:** LANG-MIX, CORE-7, PAR-1 (injection).

**Approach.** Reuse **tree-sitter injection**: it already models "Markdown fenced
code is language X", "HTML `<script>` is JS", "HTML `<style>` is CSS" — exactly the
four `GRAMMAR_EMBEDDING_TARGETS`. The adapter parses the host (Markdown/HTML/txt),
detects embedded regions **both** ways (NFR-5):
- **name-driven** — fenced ` ```rust `, `<script type=...>`, file extension;
- **content-driven** — sniff the region body (e.g. `lingua` for prose, lightweight
  signature checks for code) when no name is present.
Each embedded region is parsed by its own grammar adapter, and the resulting links
are attached to the host network with cross-language links keyed on the **shared
byte range** — yielding *one* network, not N disjoint trees (the explicit
divergence from tree-sitter named in issue #1 §1).

**Existing components.** tree-sitter injection queries + the grammar crates from
Solution 1; `lingua`/`whatlang` for content sniffing; `pulldown-cmark`/`comrak`
offset data as a cross-check for Markdown fence spans.

**Plan.** Extend `mixed_regions.rs` from detection-only to detect-then-parse-then-
link; add fixtures for all four embedding targets asserting a single connected
network + byte-exact whole-document reconstruction. → proposed issue **`#04`**.

---

## Solution 6 — Internal representation & snapshots

**Covers:** CORE-1, CORE-2, CORE-11, CORE-12, PAR-5; NFR-5 trivia policy.

**Approach.** Keep the links network as the single source of truth; adopt the
**green/red immutable-tree** model as the architectural reference for snapshots:
- **Immutable + structural sharing:** model `NetworkSnapshot` on rowan/cstree green
  nodes (deduplicated, position-independent) so snapshots share structure cheaply
  (CORE-11). `cstree`'s **string interning** maps onto "identical terms are the same
  link."
- **Trivia policy (conflicting → configurable, NFR-5):** support **both**
  containment-attached (Rowan-style) and token-attached (Roslyn-style) trivia via
  `TriviaAttachmentPolicy` (enum already present); document a default.
- **Storage substrate (optional):** if the in-memory `Vec` store outgrows memory,
  back it with the ecosystem-native **`doublets`** crate (Unlicense) via its
  `Doublets` trait.

**Existing components.** `rowan`, `cstree` (architecture references / candidate
substrate), `doublets` (persistence). These are *references and options*, not
mandatory dependencies — the current scaffold already satisfies the API shape.

**Plan.** Decide green/red vs current representation; implement persistent sharing
for `NetworkSnapshot`; benchmark interning. → proposed issue **`#10`**.

---

## Solution 7 — Substitution / transform engine (link-cli model)

**Covers:** CORE-13, CORE-14, CORE-15, PAR-8, and the transform half of I3-2.

**Approach.** Adopt link-cli's **single match→substitute operation** exactly
(verified forms in `ecosystem-foundations.md`): create `() ((1 1))`, update
`((1: 1 1)) ((1: 1 2))`, delete `((1 1)) ()`, swap
`((($index: $source $target)) (($index: $target $source)))` with variable binding.
Then enrich the *matcher* (not the operation) along three axes (CORE-14):
- **by syntax** — tree-sitter-query-like S-expressions over links (quantifiers,
  alternation, anchors, captures, fields, negated fields); lower to `LinkQuery`.
- **by meaning** — concept-level match via the common concept layer (Solution 8) +
  relative-meta-logic evaluation.
- **by type** — match on `LinkType` / type links.
Keep **predicates pluggable and host-evaluated** (CORE-15), mirroring tree-sitter's
engine/host split (engine binds captures; host runs regex/eq/semantic predicates).

**Existing components.** link-cli (operation model, ecosystem); tree-sitter query
semantics (the matcher grammar to emulate); jscodeshift/Recast (the
transform-then-reserialize-preserving-unchanged-bytes guarantee to match).

**Plan.** (1) Confirm create/update/delete/swap parity with link-cli's own tests.
(2) Add S-expression query surface lowering to `LinkQuery`. (3) Add the host
predicate hook. → proposed issues **`#09`, `#11`**.

---

## Solution 8 — Concept ontology, self-description & cross-language reconstruction

**Covers:** CORE-8, CORE-9, CORE-10, CORE-16, CORE-17, PAR-11, PAR-12.

**Approach.** This is the crate's deepest novelty and the largest remaining work.
- **Self-description (CORE-8/9):** materialize the README's self-description roots
  (`link`, `reference`, `relation link`, `language`, `grammar`, `type`, `concept`,
  `point`, `field`, `trivia`, `region`, `object`) as actual links whose
  `definition` is written in those same terms; seed the "common roots of common
  ontologies" from relative-meta-logic's `(Type: Type Type)` self-referential root.
- **Common concept layer (CORE-10):** seed a shared concept set (function, binding,
  application, sequence, branch, loop, …) from **meta-expression's
  `semantic-lexicon.json` (351 concepts, en/hi/ru/zh)**; map each concept to
  per-language concrete syntax; enforce that two languages sharing a concept
  reference the **same** concept link (dedup), which is what makes cross-language
  transform meaningful.
- **Cross-language reconstruction (CORE-16):** reconstruct a network into a
  *different* target language by walking concept links → target syntax. Validate
  with meta-expression's worked example: "Hawaii is a state." (Q782 / Q35657) →
  "Гавайи это штат."
- **Configurable (de)formalization (CORE-17):** expose formalization levels 1–4 /
  naturalization as `ParseConfiguration` knobs so formal-ai and meta-expression can
  drive text→network and network→text without leaking their internals.

**Existing components.** meta-expression (`semantic-lexicon.json` interlingua,
formalize/naturalize contract, Wikidata QID anchoring); relative-meta-logic
(dependent types, many-valued/probabilistic `TruthValue`, paradox handling, Lean/
Coq/Isabelle export); formal-ai (formalization corpus). All in the ecosystem, all
reusable.

**Plan.** (1) Seed self-description roots as links with in-language definitions.
(2) Import the semantic lexicon as concept links. (3) Per-language concept→syntax
maps for the initial language set. (4) Cross-language reconstruction + the Hawaii
fixture. (5) Formalization-level config. → proposed issues **`#12`, `#13`, `#14`**.

---

## Solution 9 — Adopt competitor & ecosystem test corpora ("copy all the tests")

**Covers:** I3-2 in full; deepens PAR-1…PAR-12 from one fixture each to real
coverage.

**Approach.** For each upstream, port its canonical assertion shape (documented in
`competitor-test-suites.md`) into `PARITY_FIXTURES`/`LANGUAGE_FIXTURES`, retaining a
provenance comment (upstream path + license). The four universal pillars become the
structural template for every fixture:
1. **Lossless round-trip** — `reconstruct_text() == input` + `verify_full_match().
   is_clean()` (every upstream has this: tree-sitter `:cst`, LibCST
   `assertEqual(mod.code, src)`, Recast `strictEqual(source, code)`, Roslyn
   `ToFullString()`).
2. **Explicit trivia** — a comment + blank-line case under each
   `TriviaAttachmentPolicy`.
3. **Error recovery** — negative fixtures asserting `is_error`/`is_missing` links
   *and* round-trip of the broken source (tree-sitter `error_corpus`, Roslyn
   `_MissingIdentifiers`, LibCST `test_parse_errors`).
4. **Query + transform** — a `LinkQuery` selecting a target + a `SubstitutionRule`
   rewriting only that target with all other bytes preserved (tree-sitter queries +
   jscodeshift `__testfixtures__` input/output pairs).

**Licensing.** All six external upstreams are MIT or Apache-2.0/MIT; all ecosystem
projects are Unlicense/permissive → compatible with this repo's Unlicense (caveat:
prefer LibCST `_nodes/tests/` over `_parser/parso/`). Safe to adapt test data.

**Specific corpus imports.** formal-ai `data/seed/*.lino` + `data/benchmarks/*.lino`
as a no-regression gate (PAR-11, replacing issue #1's unverified "706" with the
actual files); links-notation cross-language identity suite (~138 tests/lang);
link-cli create/update/delete/swap; lino-objects-codec shared+circular round-trip;
meta-expression Hawaii / "1+1=2" / "this statement is false" cases.

**Plan.** One proposed issue per upstream cluster so the porting is reviewable in
slices (**`#15`** external suites, **`#16`** ecosystem corpora).

---

## Build-vs-reuse decision summary

| Requirement cluster | Decision | Primary component | Effort |
|---|---|---|---|
| 7 mainstream PL grammars | **Reuse** | official tree-sitter grammar crates | Low |
| VB / SQL / Delphi grammars | **Reuse-then-extend / build** | third-party + original | High |
| txt container | **Build (trivial)** | none | Very low |
| Markdown / HTML / CSS | **Reuse** | tree-sitter-{html,css} + pulldown-cmark/comrak offsets | Low–Med |
| Mixed mode | **Reuse** | tree-sitter injection | Med |
| NL segmentation/ID | **Reuse** | unicode-segmentation + lindera + lingua/whatlang + unicode-bidi/normalization | Med |
| Immutable snapshots | **Reuse pattern / optional dep** | rowan/cstree model; `doublets` substrate | Med |
| Substitution engine | **Reuse model** | link-cli single operation | Med |
| Concept ontology / self-description | **Reuse data + build mapping** | meta-expression lexicon, RML | High |
| Cross-language reconstruction | **Build on concept layer** | meta-expression contract | High |
| Competitor corpora | **Reuse (adapt tests)** | all upstreams (permissive) | Med (broad) |

**Net:** the dependency posture is "tree-sitter + official grammars as the
keystone, ecosystem crates for storage/transform/semantics, original work only for
the 3 gap grammars, the concept ontology, and cross-language reconstruction." Every
reused component is permissively licensed and compatible with this repo's Unlicense.

---

## Suggested phasing (maps to `proposed-issues/`)

1. **Phase 0 — close tracking gaps (small, immediate):** txt container (`#01`),
   natural-language ordering note (`#02`).
2. **Phase 1 — the keystone:** tree-sitter adapter + 7 grammars (`#03`), mixed-mode
   embedding (`#04`), NL segmentation/ID (`#05`).
3. **Phase 2 — gap grammars (parallelizable, independent):** Visual Basic (`#06`),
   SQL dialects (`#07`), Delphi (`#08`).
4. **Phase 3 — transform & representation:** substitution/query enrichment (`#09`),
   persistent snapshots (`#10`), unified query+transform surface (`#11`).
5. **Phase 4 — semantics (deepest):** self-description roots (`#12`), common concept
   ontology (`#13`), cross-language reconstruction + formalization config (`#14`).
6. **Phase 5 — corpus adoption (broad, ongoing):** external competitor suites (`#15`),
   ecosystem corpora (`#16`).

This phasing front-loads cheap wins and the keystone, parallelizes the independent
gap grammars, and defers the deepest semantic work — while every phase keeps the
byte-exact round-trip gate green.
