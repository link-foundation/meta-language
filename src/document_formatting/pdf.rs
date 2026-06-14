//! A documented, text-only PDF profile for the document-formatting concept layer.
//!
//! PDF is a binary container. A faithful, general PDF reader (with stream
//! compression, embedded fonts, content reflow, and scanned-image recovery) is
//! out of scope for this crate. Instead this module defines a **constrained,
//! self-describing PDF profile**: an uncompressed, single-page, ASCII PDF whose
//! content stream carries the document structure with standard PDF operators
//! plus a small, documented marked-content convention. Documents produced by
//! [`render_pdf_document`] are valid PDFs (correct `xref`, object offsets, and
//! stream `Length`) that open in conformant viewers, and [`parse_pdf_document`]
//! recovers the same [`FormattingDocument`] concept tree they were built from.
//!
//! # Representation
//!
//! Block role is encoded with marked content:
//!
//! - `/H1 BDC … EMC` … `/H6 BDC … EMC` — headings carrying their level.
//! - `/P BDC … EMC` — paragraphs.
//! - `/UL BDC … EMC` / `/OL BDC … EMC` — bullet / ordered lists, each holding
//!   `/LI BDC … EMC` items.
//!
//! Inline style is encoded with the font resource selected before each shown
//! string:
//!
//! - `/F1` — regular text (`emphasis`/`strong` absent).
//! - `/F2` — strong (bold), mapped to the `strong` concept.
//! - `/F3` — emphasis (italic), mapped to the `emphasis` concept.
//!
//! Each run is one `/Fn size Tf` selector followed by one `(text) Tj` show, so
//! parsing and rendering are exact inverses for the supported feature set.
//!
//! See `docs/pdf-fidelity.md` for the full round-trip fidelity matrix.

use std::fmt::Write as _;

use super::document::{BlockNode, FormattingDocument, InlineNode};

/// Inline style carried by a single shown text run.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RunStyle {
    Regular,
    Strong,
    Emphasis,
}

impl RunStyle {
    /// The font resource name used for this style in the rendered content stream.
    const fn font(self) -> &'static str {
        match self {
            Self::Regular => "F1",
            Self::Strong => "F2",
            Self::Emphasis => "F3",
        }
    }

    /// Resolves a font resource name back to the inline style it encodes.
    fn from_font(font: &str) -> Option<Self> {
        match font {
            "F1" => Some(Self::Regular),
            "F2" => Some(Self::Strong),
            "F3" => Some(Self::Emphasis),
            _ => None,
        }
    }

    /// Wraps content text in the inline concept node for this style.
    fn wrap(self, text: String) -> InlineNode {
        match self {
            Self::Regular => InlineNode::Text(text),
            Self::Strong => InlineNode::Wrapped {
                concept: "strong".to_string(),
                attributes: std::collections::BTreeMap::new(),
                children: vec![InlineNode::Text(text)],
            },
            Self::Emphasis => InlineNode::Wrapped {
                concept: "emphasis".to_string(),
                attributes: std::collections::BTreeMap::new(),
                children: vec![InlineNode::Text(text)],
            },
        }
    }
}

/// Renders a language-free [`FormattingDocument`] into a valid, uncompressed
/// PDF in the documented text profile.
#[must_use]
pub fn render_pdf_document(document: &FormattingDocument) -> String {
    let content = render_content_stream(document);
    assemble_pdf(&content)
}

/// Parses a PDF written in the documented text profile back into the
/// language-free concept layer.
///
/// Returns an empty document when no recognizable profile content stream is
/// present, so general (out-of-profile) PDFs degrade gracefully rather than
/// producing a corrupt tree.
#[must_use]
pub fn parse_pdf_document(text: &str) -> FormattingDocument {
    let Some(content) = content_stream(text) else {
        return FormattingDocument::default();
    };
    FormattingDocument {
        blocks: parse_content_stream(content),
    }
}

/// Whether `text` is a PDF in this profile carrying at least one recognized block.
#[must_use]
pub fn pdf_profile_is_recognized(text: &str) -> bool {
    !parse_pdf_document(text).blocks.is_empty()
}

// --- rendering -------------------------------------------------------------

/// First baseline (in PDF points from the page bottom) and the vertical step
/// between lines. Coordinates are visual only; parsing ignores them.
const TOP_BASELINE: i32 = 720;
const LINE_STEP: i32 = 22;

const fn heading_size(level: u8) -> u8 {
    match level {
        1 => 24,
        2 => 20,
        3 => 18,
        4 => 16,
        5 => 14,
        _ => 13,
    }
}

const BODY_SIZE: u8 = 12;

fn render_content_stream(document: &FormattingDocument) -> String {
    let mut output = String::new();
    let mut baseline = TOP_BASELINE;
    for block in &document.blocks {
        render_block(&mut output, block, &mut baseline);
    }
    output
}

fn render_block(output: &mut String, block: &BlockNode, baseline: &mut i32) {
    match block {
        BlockNode::Heading { level, children } => {
            let level = (*level).clamp(1, 6);
            let _ = writeln!(output, "/H{level} BDC");
            render_text_object(output, children, heading_size(level), None, baseline);
            output.push_str("EMC\n");
        }
        BlockNode::Paragraph { children } => {
            output.push_str("/P BDC\n");
            render_text_object(output, children, BODY_SIZE, None, baseline);
            output.push_str("EMC\n");
        }
        BlockNode::List { concept, items } => {
            let ordered = concept == "ordered-list";
            output.push_str(if ordered { "/OL BDC\n" } else { "/UL BDC\n" });
            for (index, item) in items.iter().enumerate() {
                let marker = if ordered {
                    format!("{}. ", index + 1)
                } else {
                    "- ".to_string()
                };
                output.push_str("/LI BDC\n");
                render_text_object(output, item, BODY_SIZE, Some(&marker), baseline);
                output.push_str("EMC\n");
            }
            output.push_str("EMC\n");
        }
    }
}

fn render_text_object(
    output: &mut String,
    nodes: &[InlineNode],
    size: u8,
    marker: Option<&str>,
    baseline: &mut i32,
) {
    let mut runs = Vec::new();
    if let Some(marker) = marker {
        runs.push((RunStyle::Regular, marker.to_string()));
    }
    flatten_runs(nodes, RunStyle::Regular, &mut runs);
    merge_adjacent_runs(&mut runs);

    output.push_str("BT\n");
    let _ = writeln!(output, "72 {baseline} Td");
    for (style, text) in runs {
        let _ = writeln!(output, "/{} {size} Tf", style.font());
        let _ = writeln!(output, "({}) Tj", escape_pdf_string(&text));
    }
    output.push_str("ET\n");
    *baseline -= LINE_STEP;
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

fn escape_pdf_string(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for character in text.chars() {
        match character {
            '\\' => escaped.push_str("\\\\"),
            '(' => escaped.push_str("\\("),
            ')' => escaped.push_str("\\)"),
            other => escaped.push(other),
        }
    }
    escaped
}

/// Assembles the cross-reference table, trailer, and object bodies around an
/// already-rendered content stream.
fn assemble_pdf(content: &str) -> String {
    let objects = [
        "<< /Type /Catalog /Pages 2 0 R >>".to_string(),
        "<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_string(),
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] \
            /Resources << /Font << /F1 4 0 R /F2 5 0 R /F3 6 0 R >> >> /Contents 7 0 R >>"
            .to_string(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_string(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Bold >>".to_string(),
        "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica-Oblique >>".to_string(),
        format!(
            "<< /Length {} >>\nstream\n{content}endstream",
            content.len()
        ),
    ];

    let mut body = String::from("%PDF-1.7\n");
    let mut offsets = Vec::with_capacity(objects.len());
    for (index, object) in objects.iter().enumerate() {
        offsets.push(body.len());
        let _ = writeln!(body, "{} 0 obj\n{object}\nendobj", index + 1);
    }

    let xref_offset = body.len();
    let count = objects.len() + 1;
    let _ = writeln!(body, "xref\n0 {count}");
    body.push_str("0000000000 65535 f \n");
    for offset in offsets {
        let _ = writeln!(body, "{offset:010} 00000 n ");
    }
    let _ = writeln!(
        body,
        "trailer\n<< /Size {count} /Root 1 0 R >>\nstartxref\n{xref_offset}\n%%EOF"
    );
    body
}

// --- parsing ---------------------------------------------------------------

/// Extracts the bytes between the first `stream`/`endstream` pair.
fn content_stream(text: &str) -> Option<&str> {
    let start = text.find("stream\n")? + "stream\n".len();
    let end = text[start..].find("endstream")? + start;
    Some(&text[start..end])
}

/// A block context opened by a marked-content `BDC` operator.
enum Context {
    Heading(u8),
    Paragraph,
    List {
        ordered: bool,
        items: Vec<Vec<InlineNode>>,
    },
    ListItem,
}

fn parse_content_stream(content: &str) -> Vec<BlockNode> {
    let mut blocks = Vec::new();
    let mut stack: Vec<Context> = Vec::new();
    let mut runs: Vec<(RunStyle, String)> = Vec::new();
    let mut font = RunStyle::Regular;

    for line in content.lines() {
        let line = line.trim();
        if let Some(level) = heading_marker(line) {
            stack.push(Context::Heading(level));
            runs.clear();
        } else if line == "/P BDC" {
            stack.push(Context::Paragraph);
            runs.clear();
        } else if line == "/UL BDC" || line == "/OL BDC" {
            stack.push(Context::List {
                ordered: line == "/OL BDC",
                items: Vec::new(),
            });
        } else if line == "/LI BDC" {
            stack.push(Context::ListItem);
            runs.clear();
        } else if let Some(style) = font_selector(line) {
            font = style;
        } else if let Some(text) = show_string(line) {
            runs.push((font, text));
        } else if line == "EMC" {
            close_context(&mut stack, &mut blocks, &mut runs);
        }
        // `BT`, `ET`, `Td`, and any other operators carry no structure.
    }

    blocks
}

fn close_context(
    stack: &mut Vec<Context>,
    blocks: &mut Vec<BlockNode>,
    runs: &mut Vec<(RunStyle, String)>,
) {
    let Some(context) = stack.pop() else {
        return;
    };
    match context {
        Context::Heading(level) => {
            blocks.push(BlockNode::Heading {
                level,
                children: runs_to_inline(std::mem::take(runs), false),
            });
        }
        Context::Paragraph => {
            blocks.push(BlockNode::Paragraph {
                children: runs_to_inline(std::mem::take(runs), false),
            });
        }
        Context::ListItem => {
            let children = runs_to_inline(std::mem::take(runs), true);
            if let Some(Context::List { items, .. }) = stack.last_mut() {
                items.push(children);
            }
        }
        Context::List { ordered, items } => {
            blocks.push(BlockNode::List {
                concept: if ordered {
                    "ordered-list"
                } else {
                    "bullet-list"
                }
                .to_string(),
                items,
            });
        }
    }
}

fn heading_marker(line: &str) -> Option<u8> {
    let rest = line.strip_prefix("/H")?.strip_suffix(" BDC")?;
    let level: u8 = rest.parse().ok()?;
    (1..=6).contains(&level).then_some(level)
}

fn font_selector(line: &str) -> Option<RunStyle> {
    let rest = line.strip_prefix('/')?.strip_suffix(" Tf")?;
    let font = rest.split_whitespace().next()?;
    RunStyle::from_font(font)
}

fn show_string(line: &str) -> Option<String> {
    let body = line.strip_prefix('(')?;
    let inner = body.strip_suffix(") Tj")?;
    Some(unescape_pdf_string(inner))
}

fn unescape_pdf_string(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut chars = text.chars();
    while let Some(character) = chars.next() {
        if character == '\\' {
            match chars.next() {
                Some(escaped) => output.push(escaped),
                None => output.push('\\'),
            }
        } else {
            output.push(character);
        }
    }
    output
}

fn runs_to_inline(runs: Vec<(RunStyle, String)>, strip_list_marker: bool) -> Vec<InlineNode> {
    let mut runs = runs;
    if strip_list_marker {
        strip_marker(&mut runs);
    }
    merge_adjacent_runs(&mut runs);
    runs.into_iter()
        .filter(|(_, text)| !text.is_empty())
        .map(|(style, text)| style.wrap(text))
        .collect()
}

/// Removes a leading `- ` or `N. ` list marker from the first text run.
fn strip_marker(runs: &mut [(RunStyle, String)]) {
    let Some((style, text)) = runs.first_mut() else {
        return;
    };
    if *style != RunStyle::Regular {
        return;
    }
    if let Some(rest) = text.strip_prefix("- ") {
        *text = rest.to_string();
        return;
    }
    if let Some(dot) = text.find(". ") {
        if text[..dot]
            .chars()
            .all(|character| character.is_ascii_digit())
            && dot > 0
        {
            *text = text[dot + 2..].to_string();
        }
    }
}
