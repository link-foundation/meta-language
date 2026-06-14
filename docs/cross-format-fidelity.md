# Cross-format reconstruction & round-trip fidelity matrix

`meta-language` reconstructs a parsed document into any of its supported document
formats — `txt`, `Markdown`, `HTML`, `PDF`, `DOCX` — over one shared,
language-free formatting concept ontology (issue #83). A document parsed from any
of these formats reconstructs into any other; when the source uses only concepts
both formats support, the document survives the trip without loss, and when the
target cannot represent a concept it degrades through a **documented** lossy
fallback rather than silent data loss (issue #86).

## Entry point

`LinkNetwork::reconstruct_text_as(target_format, configuration)` is the single
entry point. It records the source format on the document root at parse time and
routes any document-format target through the shared concept layer:

```rust
use meta_language::{LinkNetwork, ParseConfiguration};

let markdown = "# Status Report\n\nThe system is **ready** for *launch*.\n\n- First item\n- Second item";
let network = LinkNetwork::parse(markdown, "Markdown", ParseConfiguration::default());

// Same-format reconstruction is byte-exact.
assert_eq!(
    network.reconstruct_text_as("Markdown", ParseConfiguration::default()),
    markdown,
);

// Cross-format reconstruction translates through the concept layer.
let html = network.reconstruct_text_as("HTML", ParseConfiguration::default());
assert!(html.contains("<h1>Status Report</h1>"));
assert!(html.contains("<strong>ready</strong>"));

// txt is the lossy fallback target: prose survives, markup is dropped.
let txt = network.reconstruct_text_as("txt", ParseConfiguration::default());
assert!(txt.contains("Status Report"));
assert!(!txt.contains('#'));
```

Behavior rules:

- **No document source language** (for example a plain natural-language network)
  → `reconstruct_text_as` returns the byte-exact reconstruction unchanged.
- **Same source and target format** → byte-exact source is returned (the
  founding-subset parsers are not guaranteed to re-serialize arbitrary input, so
  same-format reconstruction never risks normalizing the original bytes).
- **Different formats** → the source is parsed into a `FormattingDocument` (the
  concept layer) and re-rendered in the target format. Concepts the target
  cannot represent natively degrade through the fallbacks below.

The lower-level `translate_markup_document(source, target, text)` performs the
same parse-into-concepts then render-as-target without needing a parsed network.

## Capability profiles

Each format exposes a `LanguageProfile` over the formatting concept ontology via
`document_format_profile(format)`. The profile lists the concepts the format
represents natively (`supports_concept`) and, for every concept it cannot, the
documented fallback (`concept_fallback` / `fallbacks`). The invariant the test
suite enforces: for every format and every entry in `CROSS_FORMAT_CONCEPTS`, the
concept is **either** natively supported **or** has exactly one documented
fallback.

## Per-format concept support

| Concept       |  txt  | Markdown | HTML  |  PDF  | DOCX  |
| ------------- | :---: | :------: | :---: | :---: | :---: |
| paragraph     |  ✅   |    ✅    |  ✅   |  ✅   |  ✅   |
| heading       |  ⚠️   |    ✅    |  ✅   |  ✅   |  ✅   |
| bullet-list   |  ⚠️   |    ✅    |  ✅   |  ✅   |  ✅   |
| ordered-list  |  ⚠️   |    ⚠️    |  ✅   |  ✅   |  ✅   |
| list-item     |  ⚠️   |    ✅    |  ✅   |  ✅   |  ✅   |
| strong (bold) |  ⚠️   |    ✅    |  ✅   |  ✅   |  ✅   |
| emphasis      |  ⚠️   |    ✅    |  ✅   |  ✅   |  ✅   |
| hyperlink     |  ⚠️   |    ✅    |  ✅   |  ⚠️   |  ⚠️   |

✅ native, round-trips losslessly · ⚠️ lossy fallback (see below)

## Documented lossy fallbacks

| Format   | Concept      | Fallback                                                         |
| -------- | ------------ | --------------------------------------------------------------- |
| txt      | heading      | flattened to a plain paragraph (heading level dropped)          |
| txt      | bullet-list  | flattened to plain lines with a `- ` marker per item            |
| txt      | ordered-list | flattened to plain lines with a `N. ` marker per item           |
| txt      | list-item    | rendered as a single plain line                                 |
| txt      | strong       | rendered as unstyled plain text                                 |
| txt      | emphasis     | rendered as unstyled plain text                                 |
| txt      | hyperlink    | rendered as its visible text (URL dropped)                      |
| Markdown | ordered-list | rendered with bullet `- ` markers (ordering not preserved)      |
| PDF      | hyperlink    | rendered as its visible text, unstyled (URL dropped)            |
| DOCX     | hyperlink    | rendered as its visible text, unstyled (URL dropped)            |

HTML represents every cross-format concept natively, so it is the lossless hub:
any document can be rendered to HTML without a fallback. `txt` is the universal
lossy floor: it keeps prose but discards all structural and inline formatting.

## Round-trip guarantee

For every ordered pair `(A, B)` of `{txt, Markdown, HTML, PDF, DOCX}`, a sample
built from the concepts **both** `A` and `B` support survives
`A → concepts → B → concepts → A` with an identical concept tree at each hop.
This is verified by `every_ordered_format_pair_round_trips_through_the_concept_layer`
in `tests/unit/cross_format_reconstruction.rs`, which compares the recovered
`FormattingDocument` after each translation across all 25 pairs.

When a pair does not share a concept (for example a heading targeted at `txt`),
the sample for that pair omits it, because by construction the concept would be
subject to the documented fallback rather than round-trip — which is exactly the
fidelity contract this matrix records. See `docs/pdf-fidelity.md` and
`docs/docx-fidelity.md` for the per-format internals of the binary targets.
