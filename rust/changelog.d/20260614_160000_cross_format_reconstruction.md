---
bump: minor
---

### Added
- Cross-format document reconstruction and round-trip translation (issue #86): `reconstruct_text_as("txt" | "Markdown" | "HTML" | "PDF" | "DOCX", …)` now works over the shared, language-free formatting concept layer (issue #83), so a document parsed from any supported format reconstructs into any other when the source uses only concepts both formats support. A same-format target re-renders byte-for-byte; a cross-format target is translated through the concept tree, preserving heading/paragraph/list and bold/italic/link structure.
- `txt` joins Markdown, HTML, PDF, and DOCX as a first-class document format in `parse_markup_document` / `render_markup_document`: blank-line-separated paragraphs parse into the concept layer, and the concept layer flattens to plain text (headings to plain lines, lists to `- `/`N. ` markers, inline styling dropped) as the documented lossy fallback target.
- Per-format capability profiles (`document_format_profile`, `DOCUMENT_FORMATS`, `CROSS_FORMAT_CONCEPTS`, `canonical_document_format`) expose each format's `LanguageProfile` over the formatting concept ontology, reporting for every cross-format concept either native support or a documented lossy fallback rather than silent data loss.
- `LanguageProfile` gained `with_concept_fallback` / `concept_fallback` / `fallbacks` to declare and query the lossy fallback for concepts a target cannot represent natively.
- A round-trip matrix test covering every ordered pair of `{txt, Markdown, HTML, PDF, DOCX}` (a sample built from the concepts both formats share survives `A → concepts → B → concepts → A`), plus `docs/cross-format-fidelity.md` documenting the cross-format translation entry point and the per-format fidelity matrix.
