//! Language-free concept-layer document tree and Markdown/HTML converters.
//!
//! A [`FormattingDocument`] is the language-free concept layer: a tree of
//! blocks and inline spans tagged with their shared concept ids. Markdown and
//! HTML each parse *into* this layer and render *out of* it through the seeded
//! per-format templates, so a Markdown document using bold/italic/heading/
//! list/link round-trips to HTML and back through one concept ontology.

use std::collections::BTreeMap;

use super::DocumentFormatInstance;
use crate::link_network::LinkNetwork;

/// An inline span in the language-free concept layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InlineNode {
    /// Literal text run.
    Text(String),
    /// Content wrapped by an inline formatting concept (`strong`, `emphasis`,
    /// `hyperlink`, …) with optional named attributes.
    Wrapped {
        /// Exact concept id of the inline formatting.
        concept: String,
        /// Named attributes (for example `href` for a hyperlink).
        attributes: BTreeMap<String, String>,
        /// Nested inline content.
        children: Vec<Self>,
    },
}

/// A block in the language-free concept layer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BlockNode {
    /// Section heading carrying a level.
    Heading {
        /// Heading level from 1 (most significant) to 6.
        level: u8,
        /// Inline heading content.
        children: Vec<InlineNode>,
    },
    /// Paragraph of running text.
    Paragraph {
        /// Inline paragraph content.
        children: Vec<InlineNode>,
    },
    /// Bullet or ordered list.
    List {
        /// `bullet-list` or `ordered-list`.
        concept: String,
        /// Inline content of each list item.
        items: Vec<Vec<InlineNode>>,
    },
}

impl BlockNode {
    /// The concept id this block maps onto in the shared ontology.
    #[must_use]
    pub fn concept_id(&self) -> &str {
        match self {
            Self::Heading { .. } => "heading",
            Self::Paragraph { .. } => "paragraph",
            Self::List { concept, .. } => concept,
        }
    }
}

/// A language-free document in the concept layer.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FormattingDocument {
    /// Ordered blocks making up the document.
    pub blocks: Vec<BlockNode>,
}

impl LinkNetwork {
    /// Renders a language-free [`FormattingDocument`] (the concept layer) into
    /// `language` surface syntax using the seeded per-format templates.
    #[must_use]
    pub fn render_markup_document(&self, language: &str, document: &FormattingDocument) -> String {
        if language == "PDF" {
            return super::render_pdf_document(document);
        }
        let block_separator = if language == "Markdown" { "\n\n" } else { "\n" };
        document
            .blocks
            .iter()
            .map(|block| self.render_block(language, block))
            .collect::<Vec<_>>()
            .join(block_separator)
    }

    fn render_block(&self, language: &str, block: &BlockNode) -> String {
        match block {
            BlockNode::Heading { level, children } => {
                let mut instance =
                    DocumentFormatInstance::from_content(self.render_inline(language, children));
                instance.level = Some(*level);
                self.render_document_format("heading", language, &instance)
                    .unwrap_or_default()
            }
            BlockNode::Paragraph { children } => {
                let instance =
                    DocumentFormatInstance::from_content(self.render_inline(language, children));
                self.render_document_format("paragraph", language, &instance)
                    .unwrap_or_default()
            }
            BlockNode::List { concept, items } => {
                let item_separator = if language == "Markdown" { "\n" } else { "" };
                let rendered_items = items
                    .iter()
                    .map(|item| {
                        let instance = DocumentFormatInstance::from_content(
                            self.render_inline(language, item),
                        );
                        self.render_document_format("list-item", language, &instance)
                            .unwrap_or_default()
                    })
                    .collect::<Vec<_>>()
                    .join(item_separator);
                let instance = DocumentFormatInstance::from_content(rendered_items);
                self.render_document_format(concept, language, &instance)
                    .unwrap_or_default()
            }
        }
    }

    fn render_inline(&self, language: &str, nodes: &[InlineNode]) -> String {
        let mut output = String::new();
        for node in nodes {
            match node {
                InlineNode::Text(text) => output.push_str(&escape_text(language, text)),
                InlineNode::Wrapped {
                    concept,
                    attributes,
                    children,
                } => {
                    let instance = DocumentFormatInstance {
                        content: self.render_inline(language, children),
                        level: None,
                        attributes: attributes.clone(),
                    };
                    if let Some(rendered) =
                        self.render_document_format(concept, language, &instance)
                    {
                        output.push_str(&rendered);
                    }
                }
            }
        }
        output
    }

    /// Parses `text` written in `source_language` into the language-free concept
    /// layer, then renders it as `target_language` surface syntax.
    ///
    /// This is the cross-format reconstruction substrate: a Markdown document
    /// using bold/italic/heading/list/link round-trips to HTML and back through
    /// the shared concept ontology.
    #[must_use]
    pub fn translate_markup_document(
        &self,
        source_language: &str,
        target_language: &str,
        text: &str,
    ) -> Option<String> {
        let document = parse_markup_document(source_language, text)?;
        Some(self.render_markup_document(target_language, &document))
    }
}

/// Parses `text` written in `language` into the language-free concept layer.
///
/// Supports the `Markdown` and `HTML` markup targets for the founding
/// bold/italic/heading/list/link feature set.
#[must_use]
pub fn parse_markup_document(language: &str, text: &str) -> Option<FormattingDocument> {
    match language {
        "Markdown" => Some(parse_markdown_document(text)),
        "HTML" => Some(parse_html_document(text)),
        "PDF" => Some(super::parse_pdf_document(text)),
        _ => None,
    }
}

fn parse_markdown_document(text: &str) -> FormattingDocument {
    let mut blocks = Vec::new();
    let mut group: Vec<&str> = Vec::new();

    for line in text.lines() {
        if line.trim().is_empty() {
            flush_markdown_block(&mut blocks, &group);
            group.clear();
        } else {
            group.push(line);
        }
    }
    flush_markdown_block(&mut blocks, &group);

    FormattingDocument { blocks }
}

fn flush_markdown_block(blocks: &mut Vec<BlockNode>, lines: &[&str]) {
    if lines.is_empty() {
        return;
    }

    if lines.iter().all(|line| line.starts_with("- ")) {
        let items = lines
            .iter()
            .map(|line| parse_inline_markdown(&line[2..]))
            .collect();
        blocks.push(BlockNode::List {
            concept: "bullet-list".to_string(),
            items,
        });
        return;
    }

    if let [line] = lines {
        let hashes = line
            .chars()
            .take_while(|character| *character == '#')
            .count();
        if (1..=6).contains(&hashes) && line[hashes..].starts_with(' ') {
            let level = u8::try_from(hashes).expect("heading level within 1..=6");
            blocks.push(BlockNode::Heading {
                level,
                children: parse_inline_markdown(&line[hashes + 1..]),
            });
            return;
        }
    }

    blocks.push(BlockNode::Paragraph {
        children: parse_inline_markdown(&lines.join(" ")),
    });
}

fn parse_inline_markdown(input: &str) -> Vec<InlineNode> {
    let mut nodes = Vec::new();
    let mut text = String::new();
    let mut cursor = 0usize;

    while cursor < input.len() {
        let rest = &input[cursor..];
        if let Some(inner_len) = wrapped_span(rest, "**", "**") {
            flush_text(&mut nodes, &mut text);
            let inner = &rest[2..2 + inner_len];
            nodes.push(wrapped("strong", parse_inline_markdown(inner)));
            cursor += 4 + inner_len;
        } else if let Some(inner_len) = wrapped_span(rest, "*", "*") {
            flush_text(&mut nodes, &mut text);
            let inner = &rest[1..=inner_len];
            nodes.push(wrapped("emphasis", parse_inline_markdown(inner)));
            cursor += 2 + inner_len;
        } else if let Some((text_inner, href, consumed)) = markdown_link(rest) {
            flush_text(&mut nodes, &mut text);
            nodes.push(hyperlink(href, parse_inline_markdown(text_inner)));
            cursor += consumed;
        } else {
            let character = rest.chars().next().expect("non-empty remainder");
            text.push(character);
            cursor += character.len_utf8();
        }
    }

    flush_text(&mut nodes, &mut text);
    nodes
}

/// Returns the byte length of the content wrapped between `open` and `close`.
fn wrapped_span(rest: &str, open: &str, close: &str) -> Option<usize> {
    let body = rest.strip_prefix(open)?;
    // For `*`, reject `**` so strong is preferred over emphasis.
    if open == "*" && body.starts_with('*') {
        return None;
    }
    body.find(close)
}

fn markdown_link(rest: &str) -> Option<(&str, &str, usize)> {
    let body = rest.strip_prefix('[')?;
    let text_end = body.find("](")?;
    let text_inner = &body[..text_end];
    let after = &body[text_end + 2..];
    let href_end = after.find(')')?;
    let href = &after[..href_end];
    let consumed = 1 + text_end + 2 + href_end + 1;
    Some((text_inner, href, consumed))
}

fn parse_html_document(text: &str) -> FormattingDocument {
    let mut blocks = Vec::new();

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(block) = parse_html_heading(line)
            .or_else(|| parse_html_list(line))
            .or_else(|| parse_html_paragraph(line))
        {
            blocks.push(block);
        }
    }

    FormattingDocument { blocks }
}

fn parse_html_heading(line: &str) -> Option<BlockNode> {
    let after_marker = line.strip_prefix("<h")?;
    let digit = after_marker.chars().next()?;
    let level = u8::try_from(digit.to_digit(10)?)
        .ok()
        .filter(|value| (1..=6).contains(value))?;
    let open = format!("<h{level}>");
    let close = format!("</h{level}>");
    let inner = line.strip_prefix(&open)?.strip_suffix(&close)?;
    Some(BlockNode::Heading {
        level,
        children: parse_inline_html(inner),
    })
}

fn parse_html_list(line: &str) -> Option<BlockNode> {
    let (concept, inner) = if let Some(inner) = wrapped_inner(line, "<ul>", "</ul>") {
        ("bullet-list", inner)
    } else if let Some(inner) = wrapped_inner(line, "<ol>", "</ol>") {
        ("ordered-list", inner)
    } else {
        return None;
    };

    let mut items = Vec::new();
    let mut rest = inner;
    while let Some(start) = rest.find("<li>") {
        let after = &rest[start + 4..];
        let end = after.find("</li>")?;
        items.push(parse_inline_html(&after[..end]));
        rest = &after[end + 5..];
    }

    Some(BlockNode::List {
        concept: concept.to_string(),
        items,
    })
}

fn parse_html_paragraph(line: &str) -> Option<BlockNode> {
    let inner = wrapped_inner(line, "<p>", "</p>")?;
    Some(BlockNode::Paragraph {
        children: parse_inline_html(inner),
    })
}

fn parse_inline_html(input: &str) -> Vec<InlineNode> {
    let mut nodes = Vec::new();
    let mut text = String::new();
    let mut cursor = 0usize;

    while cursor < input.len() {
        let rest = &input[cursor..];
        if let Some((inner, consumed)) = html_tag_span(rest, "strong") {
            flush_html_text(&mut nodes, &mut text);
            nodes.push(wrapped("strong", parse_inline_html(inner)));
            cursor += consumed;
        } else if let Some((inner, consumed)) = html_tag_span(rest, "em") {
            flush_html_text(&mut nodes, &mut text);
            nodes.push(wrapped("emphasis", parse_inline_html(inner)));
            cursor += consumed;
        } else if let Some((href, inner, consumed)) = html_anchor(rest) {
            flush_html_text(&mut nodes, &mut text);
            nodes.push(hyperlink(href, parse_inline_html(inner)));
            cursor += consumed;
        } else {
            let character = rest.chars().next().expect("non-empty remainder");
            text.push(character);
            cursor += character.len_utf8();
        }
    }

    flush_html_text(&mut nodes, &mut text);
    nodes
}

/// Matches an inline `<tag>…</tag>` span at the start of `rest`, returning the
/// inner content and the number of bytes consumed.
fn html_tag_span<'a>(rest: &'a str, tag: &str) -> Option<(&'a str, usize)> {
    let open = format!("<{tag}>");
    let close = format!("</{tag}>");
    let body = rest.strip_prefix(&open)?;
    let inner_end = body.find(&close)?;
    let inner = &body[..inner_end];
    let consumed = open.len() + inner_end + close.len();
    Some((inner, consumed))
}

fn html_anchor(rest: &str) -> Option<(&str, &str, usize)> {
    let body = rest.strip_prefix("<a href=\"")?;
    let href_end = body.find('"')?;
    let href = &body[..href_end];
    let after_attr = &body[href_end..];
    let inner_start = after_attr.strip_prefix("\">")?;
    let inner_end = inner_start.find("</a>")?;
    let inner = &inner_start[..inner_end];
    let consumed = rest.len() - (inner_start.len() - inner_end - "</a>".len());
    Some((href, inner, consumed))
}

fn wrapped_inner<'a>(input: &'a str, open: &str, close: &str) -> Option<&'a str> {
    input.strip_prefix(open)?.strip_suffix(close)
}

fn wrapped(concept: &str, children: Vec<InlineNode>) -> InlineNode {
    InlineNode::Wrapped {
        concept: concept.to_string(),
        attributes: BTreeMap::new(),
        children,
    }
}

fn hyperlink(href: &str, children: Vec<InlineNode>) -> InlineNode {
    let mut attributes = BTreeMap::new();
    attributes.insert("href".to_string(), href.to_string());
    InlineNode::Wrapped {
        concept: "hyperlink".to_string(),
        attributes,
        children,
    }
}

fn flush_text(nodes: &mut Vec<InlineNode>, text: &mut String) {
    if !text.is_empty() {
        nodes.push(InlineNode::Text(std::mem::take(text)));
    }
}

fn flush_html_text(nodes: &mut Vec<InlineNode>, text: &mut String) {
    if !text.is_empty() {
        nodes.push(InlineNode::Text(unescape_text(&std::mem::take(text))));
    }
}

fn escape_text(language: &str, text: &str) -> String {
    if language == "HTML" {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
    } else {
        text.to_string()
    }
}

fn unescape_text(text: &str) -> String {
    text.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}
