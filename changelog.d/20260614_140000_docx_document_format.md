---
bump: minor
---

### Added
- DOCX (OOXML) document-format support (issue #85): a documented OOXML text profile (`document_formatting::render_docx_document` / `parse_docx_document`) that renders a language-free `FormattingDocument` to `word/document.xml` WordprocessingML and parses it back into the same concept tree. Block role is carried by paragraph properties (`<w:pStyle w:val="HeadingN"/>` headings, bare `<w:p>` paragraphs, `<w:numPr>` `numId` 1/2 bullet/ordered list items) and inline bold/italic by run properties (`<w:b/>` → `strong`, `<w:i/>` → `emphasis`).
- A binary OPC packaging layer (`document_formatting::render_docx_package` / `parse_docx_package`) that assembles a valid `.docx` ZIP (stored entries with a self-implemented CRC-32, no new dependencies) containing `[Content_Types].xml`, the relationship parts, `word/document.xml`, `word/styles.xml`, and `word/numbering.xml`, and reads `word/document.xml` back out.
- `parse("…", "docx", …)` dispatches to a new `docx_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("DOCX", …)` renders structurally equivalent OOXML through the shared formatting concept layer: a DOCX source re-renders byte-for-byte, while a Markdown/HTML/PDF source is translated into equivalent OOXML, and `translate_markup_document` now bridges Markdown/HTML/PDF ⇄ DOCX.
- `DOCX` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + bullet-list round-trip `LANGUAGE_FIXTURES` entry, plus `docs/docx-fidelity.md` documenting the two-layer round-trip fidelity matrix for supported and lossy/unsupported OOXML features.
