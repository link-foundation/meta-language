# PDF round-trip fidelity matrix

PDF is a binary container, not a text markup language. A faithful, general PDF
reader — stream compression, embedded/subset fonts, content reflow, arbitrary
positioning, scanned-image OCR — is out of scope for this crate. Instead
`meta-language` defines a **constrained, self-describing text PDF profile** that
maps document structure onto the shared, language-free formatting concept
ontology (issue #83) and round-trips losslessly through the links network.

This document records exactly what the profile preserves, what it normalizes,
and what a general (out-of-profile) PDF loses, so the fidelity contract is
explicit.

## Two layers of round-trip

The PDF support has two distinct, independently testable round-trips:

1. **Byte-exact network round-trip** — `parse(pdf, "pdf", …)` builds a lossless
   network from per-character `Token` leaves, so `reconstruct_text()` returns the
   *exact* input bytes for **any** input (in-profile or not). The
   concept-tagged structure links are additive and never alter reconstruction.
2. **Concept-tree round-trip** — for documents in the text profile,
   `parse_pdf_document` recovers the same `FormattingDocument` that
   `render_pdf_document` was given, so `parse(render(doc)) == doc` and
   `render(parse(pdf)) == pdf`. This is the layer that crosses formats
   (Markdown/HTML ⇄ PDF).

The matrix below describes the **concept-tree** layer. The byte-exact layer is
always lossless.

## The text PDF profile

A profile document is an uncompressed, single-page, ASCII PDF whose content
stream encodes structure with standard operators plus a small marked-content
convention:

| Concept            | Encoding in the content stream                         |
| ------------------ | ------------------------------------------------------ |
| Heading (level n)  | `/H{n} BDC … EMC` (n = 1..6)                            |
| Paragraph          | `/P BDC … EMC`                                          |
| Bullet list        | `/UL BDC … EMC` wrapping `/LI BDC … EMC` items          |
| Ordered list       | `/OL BDC … EMC` wrapping `/LI BDC … EMC` items          |
| List item          | `/LI BDC … EMC` (marker text `- ` / `N. ` prefix)       |
| Regular run        | `/F1 {size} Tf` then `({text}) Tj`                      |
| Strong (bold) run  | `/F2 {size} Tf` then `({text}) Tj` → `strong` concept   |
| Emphasis (italic)  | `/F3 {size} Tf` then `({text}) Tj` → `emphasis` concept |

Fonts are the three standard Type1 faces (`Helvetica`, `Helvetica-Bold`,
`Helvetica-Oblique`), so the documents open and extract text in conformant
viewers without embedded font programs.

## Supported — full concept-tree fidelity

| Feature                         | Round-trips | Notes                                            |
| ------------------------------- | :---------: | ------------------------------------------------ |
| Headings, levels 1–6            |     ✅      | Level preserved via `/H{n}`.                     |
| Paragraphs                      |     ✅      |                                                  |
| Bullet lists + items            |     ✅      | `- ` marker re-derived on parse.                 |
| Ordered lists + items           |     ✅      | `N. ` marker re-derived from item position.      |
| Bold (strong) inline runs       |     ✅      | `/F2` font ⇄ `strong` concept.                   |
| Italic (emphasis) inline runs   |     ✅      | `/F3` font ⇄ `emphasis` concept.                 |
| Mixed runs within a block       |     ✅      | Adjacent same-style runs merge on parse.         |
| Literal `(`, `)`, `\` in text   |     ✅      | Escaped as `\(`, `\)`, `\\` in the show string.  |
| Byte-exact reconstruction       |     ✅      | Always, for any input (see layer 1 above).       |

## Normalized — preserved as concepts, not as visual layout

These carry no semantic loss at the concept layer, but the *visual* PDF is
regenerated rather than reproduced pixel-for-pixel when re-rendering from the
concept tree:

| Aspect                  | Behavior                                                        |
| ----------------------- | -------------------------------------------------------------- |
| Glyph positions (`Td`)  | Re-laid out top-down at fixed baselines; coordinates ignored on parse. |
| Font sizes              | Derived from block role (heading level / body), not preserved per-run. |
| Page geometry           | Fixed US-Letter `MediaBox`; original media box not modeled.    |
| Whitespace/line breaks  | Normalized to one show string per run; soft line breaks not modeled. |

## Unsupported — lossy at the concept layer (out of profile)

A PDF that is **not** in the text profile (or uses features the concept layer
does not model) still round-trips **byte-exactly** as a network, but
`parse_pdf_document` returns an **empty** `FormattingDocument` (no false
structure), and these features are not surfaced as concepts:

| Feature                                   | Status | Rationale                                  |
| ----------------------------------------- | :----: | ------------------------------------------ |
| Compressed / `FlateDecode` content streams |   ❌   | Profile is uncompressed ASCII.             |
| Embedded / subset font programs           |   ❌   | Standard Type1 faces only.                 |
| Tables, columns, absolute layout          |   ❌   | Not in the founding concept set for PDF.   |
| Images / `XObject` graphics               |   ❌   | `image` concept not emitted to PDF.        |
| Hyperlink annotations                     |   ❌   | `hyperlink` text is kept but unstyled/unlinked. |
| Strikethrough, inline code, blockquote    |   ❌   | Not mapped to PDF marked content yet.      |
| Form fields, JavaScript, encryption       |   ❌   | Out of scope.                              |
| Multi-page documents                      |   ❌   | Single page per document.                  |

Unsupported inline concepts (for example `hyperlink` or `image`) keep their text
content in the rendered PDF — it is shown unstyled rather than dropped — but the
concept tag itself does not survive a PDF round-trip.

## Why this design

Mapping to the shared concept ontology means a Markdown `**bold**`, an HTML
`<strong>bold</strong>`, and a profile PDF `/F2 … (bold) Tj` all denote the one
language-free `strong` concept. That is what lets `translate_markup_document`
and `reconstruct_text_as("PDF", …)` move a document across formats while
preserving its heading/paragraph/list and bold/italic structure, and what keeps
the binary container honest about exactly which features survive the trip.
