---
bump: minor
---

### Added
- PDF document-format support (issue #84): a documented, uncompressed text PDF profile (`document_formatting::render_pdf_document` / `parse_pdf_document`) that renders a language-free `FormattingDocument` to a valid single-page PDF (correct `xref` offsets, object table, and stream `Length`) and parses it back into the same concept tree. Block role is carried by marked content (`/H1`…`/H6`, `/P`, `/UL`/`/OL`, `/LI`) and inline bold/italic by the selected font resource (`/F1` regular, `/F2` strong, `/F3` emphasis).
- `parse("…", "pdf", …)` dispatches to a new `pdf_parser` that builds a byte-exact lossless network (`reconstruct_text()` returns the input verbatim) and adds additive `Concept`/`Object` structure links recovering heading/paragraph/list/list-item and bold/italic.
- `reconstruct_text_as("PDF", …)` renders a structurally equivalent PDF through the shared formatting concept layer: a PDF source re-renders byte-for-byte, while a Markdown/HTML source is translated into an equivalent PDF, and `translate_markup_document` now bridges Markdown/HTML ⇄ PDF.
- `PDF` markup target in `MARKUP_LANGUAGE_TARGETS` with a bold + heading + paragraph round-trip `LANGUAGE_FIXTURES` entry, plus `docs/pdf-fidelity.md` documenting the round-trip fidelity matrix for supported and lossy/unsupported PDF features.
