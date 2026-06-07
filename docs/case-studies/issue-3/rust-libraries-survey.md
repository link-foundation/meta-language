# Existing Rust Libraries Survey (Lossless Parsing & Language Coverage)

> Compiled 2026-06-05 from crates.io, docs.rs, and each crate's repository.
> Supports the requirement in
> [issue #3](https://github.com/link-foundation/meta-language/issues/3) to make
> the Rust libraries "include everything required to process (in and out) for all
> the top 10 programming and 10 natural languages," and the
> [issue #1](https://github.com/link-foundation/meta-language/issues/1) directive
> to reuse existing components rather than reinvent them.

The crate's design (one lossless links network, projected into CST / AST /
semantic views) means most third-party parsers are useful only as **front-end
adapters**: they tokenize/parse a language, and an adapter lowers their output
into links while preserving byte spans. The decisive question for each crate is
therefore *"does it preserve every byte (trivia + spans) so we can reconstruct
the original exactly?"* — most AST libraries do **not**.

## A. Universal parser front-end: tree-sitter

- **`tree-sitter` 0.26.9** (Rust binding, MIT) — incremental GLR parser exposing a
  lossless CST with byte ranges + row/column for every node, error recovery
  (`ERROR`/`MISSING` nodes), and an S-expression query engine. `Node::utf8_text`,
  `Node::byte_range`, `TreeCursor` walk. <https://crates.io/crates/tree-sitter>
- **Official grammar crates on crates.io** (each MIT, published by the
  tree-sitter org): `tree-sitter-python`, `tree-sitter-c`, `tree-sitter-java`,
  `tree-sitter-cpp`, `tree-sitter-c-sharp`, `tree-sitter-javascript`,
  `tree-sitter-r`. These cover **7 of the 10** TIOBE targets out of the box.

**This is the single highest-leverage dependency.** A `tree-sitter` → links adapter
(walk the tree, emit one link per node with its byte span, attach trivia from gaps
between child spans) yields `LosslessParsing` + `TriviaPreservation` +
`ErrorRecovery` + `QueryMatching` + `MixedLanguageRegions` (via tree-sitter's own
injection mechanism) for every grammar that exists — across both programming and
markup languages — with one adapter.

### Programming-language grammar gaps (the hard 3 of 10)

| TIOBE target | Best available Rust/tree-sitter option | Status |
|---|---|---|
| Python | `tree-sitter-python` (official, MIT) | ✅ solid |
| C | `tree-sitter-c` (official, MIT) | ✅ solid |
| Java | `tree-sitter-java` (official, MIT) | ✅ solid |
| C++ | `tree-sitter-cpp` (official, MIT) | ✅ solid |
| C# | `tree-sitter-c-sharp` (official, MIT) | ✅ solid |
| JavaScript | `tree-sitter-javascript` (official, MIT) | ✅ solid |
| R | `tree-sitter-r` (official, MIT) | ✅ solid |
| **Visual Basic** | **No official crate.** Third-party only: `arborium-vb`, CodeAnt-AI's `tree-sitter-vb` fork. | ⚠️ **biggest gap** |
| **SQL** | Fragmented: `tree-sitter-sequel` (most maintained), `tree-sitter-sql-bigquery`, DerekStride's `tree-sitter-sql`. Dialect-specific. | ⚠️ fragmented |
| **Delphi/Object Pascal** | Only generic `tree-sitter-pascal` (Isopod/maxxnino); not Delphi-specific. | ⚠️ partial |

The three gaps (Visual Basic, SQL, Delphi) are where original grammar work or
vendoring a third-party grammar will be required. Visual Basic is the most
exposed: there is no authoritative grammar in any ecosystem.

## B. Native Rust lossless CST libraries (the architectural references)

- **`rowan` 0.16.x** (dual Apache-2.0/MIT) — the green/red immutable-tree library
  from rust-analyzer. Green nodes are deduplicated and position-independent; red
  nodes add parent pointers + absolute offsets. This is the closest external model
  to the crate's `NetworkSnapshot`/immutable-links design and is the reference for
  `SnapshotVersioning`. <https://crates.io/crates/rowan>
- **`cstree` 0.12.x** (dual Apache-2.0/MIT) — rowan fork adding `Send + Sync`
  trees, a cached/persistent red layer, and **string interning** (identical token
  text shares storage). The interning model maps onto the crate's "identical
  terms are the same link" dedup. <https://crates.io/crates/cstree>
- **`ra_ap_syntax`** — rust-analyzer's own syntax crate (rowan-based), the only
  *native* full-fidelity lossless CST for **Rust** source. Useful if Rust ever
  joins the language set; also a reference implementation of "lossless parser on
  top of rowan."
- **`biome_js_parser`** (+ `biome_rowan`) — Biome's lossless JS/TS parser, a
  native-Rust alternative to tree-sitter-javascript that produces a full-fidelity
  CST and is built for codemods/formatting. A strong second source for the
  JavaScript target (and TS).

**Takeaway:** rowan/cstree are *architecture references and a candidate storage
substrate*, not parsers. They show how to do immutable, interned, byte-exact trees
in Rust — exactly the properties the links network needs.

## C. Markup languages (Markdown / HTML / CSS — the "mixed mode" core)

| Target | Crate | Lossless? | Notes |
|---|---|---|---|
| Markdown | `pulldown-cmark` | ❌ not byte-exact | CommonMark pull-parser; emits events with **source byte offsets** (`OffsetIter`), which an adapter can use to reconstruct. Fast, MSRV-friendly. |
| Markdown | `comrak` | ❌ not byte-exact | GFM superset (tables, strikethrough, task lists, footnotes); AST with `sourcepos` (line/col spans). Best for GFM feature coverage. |
| Markdown | `markdown-rs` (`markdown` 1.x) | ⚠️ closest | CommonMark + GFM/MDX with a position-carrying AST and a token concept; the most amenable to lossless adaptation. |
| HTML | `lol_html` | ✅ **byte-preserving** | Cloudflare's streaming rewriter; preserves bytes it doesn't rewrite — the natural fit for lossless HTML. |
| HTML | `html5ever` (+ `markup5ever`) | ❌ normalizes | Spec-compliant WHATWG parser (used in Servo) but it normalizes/auto-corrects, so not byte-exact. Good for *correctness*, bad for *fidelity*. |
| HTML | `tree-sitter-html` | ⚠️ via adapter | tree-sitter grammar gives spans like any other language; consistent with the universal adapter path. |
| CSS | `cssparser` (Servo) | ✅ build-your-own | Low-level tokenizer with spans; lossless if you keep tokens. The Servo-grade foundation. |
| CSS | `lightningcss` | ❌ strips comments | Fast parser/transformer but discards comments — not lossless. Good for semantics, not fidelity. |
| CSS | `tree-sitter-css` | ⚠️ via adapter | grammar + spans, consistent with the universal path. |

**Mixed-mode embedding:** tree-sitter's **injection** mechanism already models
"Markdown fenced code is language X," "HTML `<script>` is JS," "HTML `<style>` is
CSS" — the exact four `GRAMMAR_EMBEDDING_TARGETS`. Using tree-sitter end-to-end
(html + css + javascript + the fenced-code injection queries) gives mixed-mode
parsing into a single tree (→ single network) without bespoke glue. The fidelity
question (Markdown/CSS not byte-exact in non-tree-sitter libs) disappears when the
fidelity comes from tree-sitter spans rather than the library's own serializer.

## D. Natural-language processing (segmentation, scripts, identification)

The crate's natural-language fixtures need: (1) tokenization/segmentation that
respects each script, (2) language identification, (3) byte-exact reconstruction
of non-ASCII text. No single crate does all 10 languages' linguistics; the
realistic plan is segmentation + identification, not full morphology.

| Need | Crate | Coverage |
|---|---|---|
| Grapheme/word/sentence segmentation | `unicode-segmentation` | UAX #29 — works for space-delimited scripts (English, Spanish, French, Russian, Hindi, Bengali, Arabic, Urdu, Portuguese). **Does NOT segment Chinese** (no spaces). |
| CJK + dictionary tokenization | `lindera` | Morphological analyzer with CJK dictionaries (incl. Chinese via supported dictionaries) — fills the Mandarin gap. |
| Language identification | `lingua` | High-accuracy detector covering **75 languages including all 10 targets**; good on short text. |
| Language identification (fast) | `whatlang` | 69 languages incl. all 10; faster, lighter, slightly less accurate. |
| Unicode normalization | `unicode-normalization` | NFC/NFD/NFKC/NFKD — needed so Arabic/Urdu/Hindi compare and reconstruct correctly. |
| Bidi (Arabic/Urdu) | `unicode-bidi` | UAX #9 bidirectional algorithm for RTL scripts. |

**Reconstruction caveat:** byte-exact reconstruction of natural language is a
*storage* property (keep the original UTF-8 bytes + spans), not an NLP property.
The NLP crates add *segmentation and identification* links over the text; they must
never mutate the stored bytes. The existing `LANGUAGE_FIXTURES` (e.g. `你好。`,
`नमस्ते।`, `سلام۔`) already exercise non-ASCII byte ranges through
`reconstruct_text()`; the segmentation/ID layer is additive.

## E. Storage substrate (links network persistence)

- **`doublets` 0.4.0** (Unlicense — license-compatible with this repo) —
  linksplatform's associative-storage engine: links as doublets (`source`,
  `target`) over united/split memory or file-backed stores, via the `Doublets`
  trait (`create`, `update`, `delete`, `each`/query, `count`). This is the
  ecosystem-native persistence layer if the network outgrows in-memory `Vec`
  storage. (Note: the bare name `doublets-rs` is **not** a crate; the crate is
  `doublets`, repo `linksplatform/doublets-rs`.)

## Recommended dependency posture

1. **Adopt `tree-sitter` + the 7 official grammar crates** as the universal
   front-end → one adapter yields the bulk of programming/markup parsing with
   lossless spans, error recovery, queries, and injection-based mixed mode.
2. **Commission/vendor grammars for the 3 gaps** (Visual Basic, SQL dialect(s),
   Delphi) — Visual Basic needs the most original work.
3. **Treat `rowan`/`cstree` as architecture references** for the immutable,
   interned, byte-exact internal representation (and a candidate substrate).
4. **Use `tree-sitter-html`/`-css`/`-javascript` + injection** for mixed mode
   rather than stitching `comrak` + `html5ever` + `lightningcss`, to keep one
   fidelity model. Keep `pulldown-cmark`/`comrak` offset data as a cross-check.
5. **Layer `lingua`/`whatlang` + `unicode-segmentation` + `lindera`** for the
   natural-language segmentation/identification links; never let them touch stored
   bytes.
6. **Keep `doublets` in view** as the ecosystem-native persistence option.

Every one of these is permissively licensed (MIT / Apache-2.0 / Unlicense),
compatible with this repository's `Unlicense`.
