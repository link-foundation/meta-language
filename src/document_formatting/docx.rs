//! OOXML (`WordprocessingML`) `word/document.xml` content layer for the shared
//! document-formatting concept tree.
//!
//! DOCX is an OPC (ZIP) package of OOXML parts; this module models its primary
//! `word/document.xml` part — the text content layer that carries the document
//! structure. It renders a language-free [`FormattingDocument`] into the OOXML
//! body markup and parses that markup back into the same concept tree, so the
//! tree round-trips DOCX ⇄ Markdown/HTML/PDF through the shared ontology
//! (issue #83). The binary OPC packaging is assembled in [`super::opc`].
//!
//! # Representation
//!
//! Block role is carried by paragraph properties (`<w:pPr>`):
//!
//! - Heading level `n`: `<w:pStyle w:val="Heading{n}"/>` (`n` = 1..6).
//! - Paragraph: neither `pStyle` nor `numPr`.
//! - List item: `<w:numPr><w:ilvl w:val="0"/><w:numId w:val="{id}"/></w:numPr>`
//!   with `id` = 1 for a bullet list and 2 for an ordered list. OOXML has no
//!   list container element, so consecutive items sharing a `numId` group into
//!   one [`BlockNode::List`].
//!
//! Inline style is carried by run properties (`<w:rPr>`):
//!
//! - Regular run: bare `<w:r><w:t>…</w:t></w:r>`.
//! - Strong (bold): `<w:rPr><w:b/></w:rPr>` → the `strong` concept.
//! - Emphasis (italic): `<w:rPr><w:i/></w:rPr>` → the `emphasis` concept.
//!
//! See `docs/docx-fidelity.md` for the full round-trip fidelity matrix.

use super::document::{BlockNode, FormattingDocument, InlineNode};

/// Inline style carried by a single run.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RunStyle {
    Regular,
    Strong,
    Emphasis,
}

impl RunStyle {
    /// Wraps content text in the inline concept node for this style.
    fn wrap(self, text: String) -> InlineNode {
        match self {
            Self::Regular => InlineNode::Text(text),
            Self::Strong => wrapped("strong", text),
            Self::Emphasis => wrapped("emphasis", text),
        }
    }
}

fn wrapped(concept: &str, text: String) -> InlineNode {
    InlineNode::Wrapped {
        concept: concept.to_string(),
        attributes: std::collections::BTreeMap::new(),
        children: vec![InlineNode::Text(text)],
    }
}

const HEADER: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"yes\"?>\n";
const BODY_OPEN: &str =
    "<w:document xmlns:w=\"http://schemas.openxmlformats.org/wordprocessingml/2006/main\"><w:body>";
const BODY_CLOSE: &str = "<w:sectPr/></w:body></w:document>\n";

const BULLET_NUM_ID: &str = "1";
const ORDERED_NUM_ID: &str = "2";

// --- rendering -------------------------------------------------------------

/// Renders a language-free [`FormattingDocument`] into OOXML
/// `word/document.xml` markup in the documented profile.
#[must_use]
pub fn render_docx_document(document: &FormattingDocument) -> String {
    let mut output = String::from(HEADER);
    output.push_str(BODY_OPEN);
    for block in &document.blocks {
        render_block(&mut output, block);
    }
    output.push_str(BODY_CLOSE);
    output
}

fn render_block(output: &mut String, block: &BlockNode) {
    match block {
        BlockNode::Heading { level, children } => {
            let level = (*level).clamp(1, 6);
            output.push_str("<w:p><w:pPr><w:pStyle w:val=\"Heading");
            output.push_str(&level.to_string());
            output.push_str("\"/></w:pPr>");
            render_runs(output, children);
            output.push_str("</w:p>");
        }
        BlockNode::Paragraph { children } => {
            output.push_str("<w:p>");
            render_runs(output, children);
            output.push_str("</w:p>");
        }
        BlockNode::List { concept, items } => {
            let num_id = if concept == "ordered-list" {
                ORDERED_NUM_ID
            } else {
                BULLET_NUM_ID
            };
            for item in items {
                output.push_str("<w:p><w:pPr><w:numPr><w:ilvl w:val=\"0\"/><w:numId w:val=\"");
                output.push_str(num_id);
                output.push_str("\"/></w:numPr></w:pPr>");
                render_runs(output, item);
                output.push_str("</w:p>");
            }
        }
    }
}

fn render_runs(output: &mut String, nodes: &[InlineNode]) {
    let mut runs = Vec::new();
    flatten_runs(nodes, RunStyle::Regular, &mut runs);
    merge_adjacent_runs(&mut runs);
    for (style, text) in runs {
        if text.is_empty() {
            continue;
        }
        output.push_str("<w:r>");
        match style {
            RunStyle::Strong => output.push_str("<w:rPr><w:b/></w:rPr>"),
            RunStyle::Emphasis => output.push_str("<w:rPr><w:i/></w:rPr>"),
            RunStyle::Regular => {}
        }
        output.push_str("<w:t xml:space=\"preserve\">");
        output.push_str(&escape_xml(&text));
        output.push_str("</w:t></w:r>");
    }
}

fn flatten_runs(nodes: &[InlineNode], style: RunStyle, runs: &mut Vec<(RunStyle, String)>) {
    for node in nodes {
        match node {
            InlineNode::Text(text) => runs.push((style, text.clone())),
            InlineNode::Wrapped {
                concept, children, ..
            } => {
                let child_style = match concept.as_str() {
                    "strong" => RunStyle::Strong,
                    "emphasis" => RunStyle::Emphasis,
                    // Unsupported inline concepts (hyperlink, image, …) keep the
                    // surrounding style; their text is preserved but unstyled.
                    _ => style,
                };
                flatten_runs(children, child_style, runs);
            }
        }
    }
}

fn merge_adjacent_runs(runs: &mut Vec<(RunStyle, String)>) {
    let mut merged: Vec<(RunStyle, String)> = Vec::with_capacity(runs.len());
    for (style, text) in runs.drain(..) {
        if let Some(last) = merged.last_mut() {
            if last.0 == style {
                last.1.push_str(&text);
                continue;
            }
        }
        merged.push((style, text));
    }
    *runs = merged;
}

fn escape_xml(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            other => escaped.push(other),
        }
    }
    escaped
}

// --- parsing ---------------------------------------------------------------

/// Parses OOXML `word/document.xml` markup in the documented profile back into
/// the language-free concept layer.
///
/// Returns an empty document when no recognizable `<w:p>` paragraphs are
/// present, so out-of-profile XML degrades gracefully rather than producing a
/// corrupt tree.
#[must_use]
pub fn parse_docx_document(text: &str) -> FormattingDocument {
    FormattingDocument {
        blocks: parse_blocks(text),
    }
}

/// Whether `text` is OOXML `document.xml` carrying at least one recognized block.
#[must_use]
pub fn docx_profile_is_recognized(text: &str) -> bool {
    !parse_docx_document(text).blocks.is_empty()
}

/// A pending run of consecutive list-item paragraphs sharing one `numId`.
struct PendingList {
    concept: String,
    items: Vec<Vec<InlineNode>>,
}

fn parse_blocks(text: &str) -> Vec<BlockNode> {
    let mut blocks = Vec::new();
    let mut pending: Option<PendingList> = None;

    for paragraph in paragraphs(text) {
        if let Some(level) = heading_level(paragraph) {
            flush_pending(&mut blocks, &mut pending);
            blocks.push(BlockNode::Heading {
                level,
                children: parse_runs(paragraph),
            });
        } else if let Some(num_id) = list_num_id(paragraph) {
            let concept = if num_id == ORDERED_NUM_ID {
                "ordered-list"
            } else {
                "bullet-list"
            };
            let item = parse_runs(paragraph);
            match pending.as_mut() {
                Some(list) if list.concept == concept => list.items.push(item),
                _ => {
                    flush_pending(&mut blocks, &mut pending);
                    pending = Some(PendingList {
                        concept: concept.to_string(),
                        items: vec![item],
                    });
                }
            }
        } else {
            flush_pending(&mut blocks, &mut pending);
            blocks.push(BlockNode::Paragraph {
                children: parse_runs(paragraph),
            });
        }
    }

    flush_pending(&mut blocks, &mut pending);
    blocks
}

fn flush_pending(blocks: &mut Vec<BlockNode>, pending: &mut Option<PendingList>) {
    if let Some(list) = pending.take() {
        blocks.push(BlockNode::List {
            concept: list.concept,
            items: list.items,
        });
    }
}

/// Yields the inner markup of each `<w:p>…</w:p>` paragraph in document order.
fn paragraphs(text: &str) -> Vec<&str> {
    let mut found = Vec::new();
    let mut rest = text;
    while let Some(inner) = next_element(&mut rest, "w:p") {
        found.push(inner);
    }
    found
}

/// Heading level from a `<w:pStyle w:val="Heading{n}"/>` paragraph property.
fn heading_level(paragraph: &str) -> Option<u8> {
    let value = attribute_value(paragraph, "<w:pStyle", "w:val")?;
    let digits = value
        .strip_prefix("Heading")
        .or_else(|| value.strip_prefix("heading "))?;
    let level: u8 = digits.trim().parse().ok()?;
    (1..=6).contains(&level).then_some(level)
}

/// The `numId` of a list-item paragraph, when present.
fn list_num_id(paragraph: &str) -> Option<String> {
    attribute_value(paragraph, "<w:numId", "w:val").map(str::to_string)
}

fn parse_runs(paragraph: &str) -> Vec<InlineNode> {
    let mut runs: Vec<(RunStyle, String)> = Vec::new();
    let mut rest = paragraph;
    while let Some(run) = next_element(&mut rest, "w:r") {
        let style = if has_toggle(run, "b") {
            RunStyle::Strong
        } else if has_toggle(run, "i") {
            RunStyle::Emphasis
        } else {
            RunStyle::Regular
        };
        let text = run_text(run);
        if !text.is_empty() {
            runs.push((style, text));
        }
    }
    merge_adjacent_runs(&mut runs);
    runs.into_iter()
        .map(|(style, text)| style.wrap(text))
        .collect()
}

/// Concatenated text of every `<w:t>…</w:t>` element inside a run.
fn run_text(run: &str) -> String {
    let mut text = String::new();
    let mut rest = run;
    while let Some(inner) = next_element(&mut rest, "w:t") {
        text.push_str(&unescape_xml(inner));
    }
    text
}

/// Whether a run carries an enabled `<w:{tag}>` toggle property (for example
/// `<w:b/>`), honoring an explicit `w:val="false"`/`"0"`/`"none"` disable.
fn has_toggle(run: &str, tag: &str) -> bool {
    let needle = format!("<w:{tag}");
    let mut rest = run;
    while let Some(index) = rest.find(&needle) {
        let after = &rest[index + needle.len()..];
        // Reject longer element names such as `<w:bCs` when matching `<w:b`.
        match after.chars().next() {
            Some('>' | '/' | ' ') => {
                let tag_end = after.find('>').unwrap_or(after.len());
                let attributes = &after[..tag_end];
                if !toggle_disabled(attributes) {
                    return true;
                }
                rest = &after[tag_end..];
            }
            _ => rest = after,
        }
    }
    false
}

fn toggle_disabled(attributes: &str) -> bool {
    attribute_value(attributes, "", "w:val")
        .is_some_and(|value| matches!(value, "false" | "0" | "off" | "none"))
}

/// Reads the `attribute` value from the first `tag` element in `text`. When
/// `tag` is empty the lookup is performed against `text` directly.
fn attribute_value<'a>(text: &'a str, tag: &str, attribute: &str) -> Option<&'a str> {
    let scope = if tag.is_empty() {
        text
    } else {
        let start = text.find(tag)?;
        let after = &text[start..];
        let end = after.find('>').map_or(after.len(), |index| index + 1);
        &after[..end]
    };
    let needle = format!("{attribute}=\"");
    let start = scope.find(&needle)? + needle.len();
    let end = scope[start..].find('"')? + start;
    Some(&scope[start..end])
}

/// Consumes the next `<{tag}>…</{tag}>` element from `rest`, advancing `rest`
/// past it and returning the inner markup. Self-closing `<{tag}/>` elements are
/// skipped (their inner content is empty) and reported as an empty string.
fn next_element<'a>(rest: &mut &'a str, tag: &str) -> Option<&'a str> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    loop {
        let index = rest.find(&open)?;
        let after = &rest[index + open.len()..];
        // Reject longer element names (for example `<w:pPr` when seeking `<w:p`).
        let boundary = after.chars().next();
        if !matches!(boundary, Some('>' | '/' | ' ')) {
            *rest = after;
            continue;
        }
        let tag_end = after.find('>')?;
        if after[..tag_end].ends_with('/') {
            // Self-closing element: no inner content.
            *rest = &after[tag_end + 1..];
            return Some("");
        }
        let body = &after[tag_end + 1..];
        let close_index = body.find(&close)?;
        let inner = &body[..close_index];
        *rest = &body[close_index + close.len()..];
        return Some(inner);
    }
}

fn unescape_xml(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}
