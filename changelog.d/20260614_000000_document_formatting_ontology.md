---
bump: minor
---

### Added
- Shared, language-free document-formatting concept ontology (`document_formatting` module): inline concepts (emphasis, strong, strikethrough, inline-code, hyperlink, image, line-break) and block concepts (heading with level, paragraph, blockquote, bullet/ordered lists, list-item, code-block with language, thematic-break, table/row/cell), each seeded with Markdown and HTML syntax mappings.
- `seed_common_concept_ontology()` now also seeds the formatting concepts, and `ConceptOntologySeedReport::formatting_concepts()` reports how many were added.
- `LinkNetwork::resolve_document_format` / `render_document_format` / `translate_document_format` so the same concept link reconstructs as `**…**` in Markdown and `<strong>…</strong>` in HTML; Markdown `**bold**` and HTML `<strong>bold</strong>` reach the one shared `strong` concept under semantic projection.
- `FormattingDocument` concept layer with `parse_markup_document` and `translate_markup_document` for full Markdown ⇄ HTML document round-trips through one concept ontology (heading/paragraph/list/bold/italic/link).
