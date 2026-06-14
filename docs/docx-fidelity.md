# DOCX (OOXML) round-trip fidelity matrix

DOCX is an OPC (ZIP) package of OOXML (WordprocessingML) parts, not a text
markup language. A faithful, general DOCX reader — DEFLATE-compressed parts,
themes, styles inheritance, sections, tables, drawings, footnotes, tracked
changes — is out of scope for this crate. Instead `meta-language` defines a
**constrained, self-describing OOXML profile** that maps document structure onto
the shared, language-free formatting concept ontology (issue #83) and
round-trips losslessly through the links network.

This document records exactly what the profile preserves, what it normalizes,
and what a general (out-of-profile) DOCX loses, so the fidelity contract is
explicit.

## Two layers of round-trip

The DOCX support has two distinct, independently testable layers:

1. **OOXML content layer (text).** `render_docx_document` / `parse_docx_document`
   render and parse the primary `word/document.xml` part as text. Two sub-layers
   here:
   - **Byte-exact network round-trip** — `parse(document_xml, "docx", …)` builds
     a lossless network from per-character `Token` leaves, so
     `reconstruct_text()` returns the *exact* input bytes for **any** input
     (in-profile or not). The concept-tagged structure links are additive and
     never alter reconstruction.
   - **Concept-tree round-trip** — for documents in the profile,
     `parse_docx_document` recovers the same `FormattingDocument` that
     `render_docx_document` was given, so `parse(render(doc)) == doc` and
     `render(parse(xml)) == xml`. This is the layer that crosses formats
     (Markdown/HTML/PDF ⇄ DOCX).
2. **OPC package layer (binary).** `render_docx_package` / `parse_docx_package`
   wrap that `word/document.xml` together with the fixed packaging parts into a
   real `.docx` ZIP and read it back. Packages are written as **stored**
   (uncompressed) ZIP entries with a self-implemented CRC-32, so they open in
   conformant word processors with no third-party dependency.

The matrix below describes the **concept-tree** layer. The byte-exact layer is
always lossless.

## The OOXML profile

A profile document is a `word/document.xml` whose body encodes structure with
standard WordprocessingML elements:

| Concept            | Encoding in `word/document.xml`                                            |
| ------------------ | ------------------------------------------------------------------------- |
| Heading (level n)  | `<w:p><w:pPr><w:pStyle w:val="Heading{n}"/></w:pPr>…</w:p>` (n = 1..6)     |
| Paragraph          | `<w:p>…</w:p>` (no `pStyle`, no `numPr`)                                   |
| Bullet list        | consecutive `<w:p>` with `<w:numPr>…<w:numId w:val="1"/></w:numPr>`        |
| Ordered list       | consecutive `<w:p>` with `<w:numPr>…<w:numId w:val="2"/></w:numPr>`        |
| List item          | one such `<w:p>` paragraph (no marker text in the runs)                    |
| Regular run        | `<w:r><w:t xml:space="preserve">{text}</w:t></w:r>`                        |
| Strong (bold) run  | `<w:r><w:rPr><w:b/></w:rPr><w:t…>{text}</w:t></w:r>` → `strong` concept    |
| Emphasis (italic)  | `<w:r><w:rPr><w:i/></w:rPr><w:t…>{text}</w:t></w:r>` → `emphasis` concept  |

The `.docx` package adds the parts a conformant reader requires:
`[Content_Types].xml`, `_rels/.rels`, `word/_rels/document.xml.rels`,
`word/styles.xml` (defining `Heading1`…`Heading6`), and `word/numbering.xml`
(defining the bullet `numId` 1 and decimal `numId` 2).

## Supported — full concept-tree fidelity

| Feature                         | Round-trips | Notes                                            |
| ------------------------------- | :---------: | ------------------------------------------------ |
| Headings, levels 1–6            |     ✅      | Level preserved via `w:pStyle` `Heading{n}`.     |
| Paragraphs                      |     ✅      |                                                  |
| Bullet lists + items            |     ✅      | Grouped from consecutive `numId="1"` paragraphs. |
| Ordered lists + items           |     ✅      | Grouped from consecutive `numId="2"` paragraphs. |
| Bold (strong) inline runs       |     ✅      | `<w:b/>` ⇄ `strong` concept.                     |
| Italic (emphasis) inline runs   |     ✅      | `<w:i/>` ⇄ `emphasis` concept.                   |
| Mixed runs within a block       |     ✅      | Adjacent same-style runs merge on parse.         |
| `&`, `<`, `>` in run text       |     ✅      | Escaped as `&amp;`, `&lt;`, `&gt;`.              |
| `.docx` package round-trip      |     ✅      | Valid stored-entry ZIP with CRC-32 per part.     |
| Byte-exact reconstruction       |     ✅      | Always, for any text input (see layer 1 above).  |

## Normalized — preserved as concepts, not as authored markup

These carry no semantic loss at the concept layer, but the serialized OOXML is
regenerated rather than reproduced node-for-node when re-rendering from the
concept tree:

| Aspect                       | Behavior                                                            |
| ---------------------------- | ------------------------------------------------------------------ |
| Combined bold **and** italic | Collapses to a single run style (child wins), matching the PDF profile. |
| Run boundaries               | Re-segmented to one run per contiguous style; original splits not kept. |
| List marker glyphs           | Re-derived from `numId`; not stored as literal text in runs.       |
| `numId` values               | Normalized to 1 (bullet) / 2 (ordered) regardless of source ids.   |
| Packaging parts              | `styles.xml` / `numbering.xml` / rels are fixed boilerplate, not modeled per-document. |
| ZIP compression              | Always stored (uncompressed); original entry compression not preserved. |

## Unsupported — lossy at the concept layer (out of profile)

OOXML that is **not** in the profile still round-trips **byte-exactly** as a
network (content layer), but `parse_docx_document` returns an **empty**
`FormattingDocument` (no false structure) for anything with no recognizable
`<w:p>` paragraphs, and these features are not surfaced as concepts:

| Feature                                       | Status | Rationale                                       |
| --------------------------------------------- | :----: | ----------------------------------------------- |
| DEFLATE-compressed package parts              |   ❌   | Package profile is stored (uncompressed) only.  |
| Tables (`<w:tbl>`)                            |   ❌   | Not in the founding concept set for DOCX.       |
| Images / drawings (`<w:drawing>`)            |   ❌   | `image` concept not emitted to OOXML.           |
| Hyperlinks (`<w:hyperlink>`)                 |   ❌   | `hyperlink` text is kept but unstyled/unlinked. |
| Strikethrough, inline code, blockquote        |   ❌   | Not mapped to run/paragraph properties yet.     |
| Nested / multi-level lists (`w:ilvl` > 0)    |   ❌   | Single level (`ilvl="0"`) modeled.              |
| Custom styles, themes, numbering definitions  |   ❌   | Only `Heading{n}` and `numId` 1/2 are modeled.  |
| Sections, headers/footers, footnotes, fields  |   ❌   | Out of scope.                                   |
| Tracked changes, comments, bookmarks          |   ❌   | Out of scope.                                   |

Unsupported inline concepts (for example `hyperlink` or `image`) keep their text
content in the rendered OOXML — it is shown unstyled rather than dropped — but
the concept tag itself does not survive a DOCX round-trip.

## Why this design

Mapping to the shared concept ontology means a Markdown `**bold**`, an HTML
`<strong>bold</strong>`, a profile PDF `/F2 … (bold) Tj`, and a DOCX run with
`<w:b/>` all denote the one language-free `strong` concept. That is what lets
`translate_markup_document` and `reconstruct_text_as("DOCX", …)` move a document
across formats while preserving its heading/paragraph/list and bold/italic
structure, and what keeps the binary container honest about exactly which
features survive the trip.
