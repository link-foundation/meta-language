//! Visible document text and source coordinates.

use std::{collections::BTreeMap, sync::LazyLock};

use super::{LinkId, LinkNetwork, LinkType};
use crate::{language_catalog::canonical_language_name, source::ByteRange};

mod character_references;
mod document_text_policy;
use character_references::{ReferenceSyntax, character_reference};
use document_text_policy::document_scalar_is_whitespace;

/// Controls how a document projects into visible text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisibleDocumentTextOptions<'options> {
    /// Text between visible blocks and at explicit line breaks.
    pub block_separator: &'options str,
    /// Decode named and numeric character references.
    pub decode_entities: bool,
    /// Collapse ordinary whitespace outside literal code and Word text runs.
    pub collapse_whitespace: bool,
    /// Keep U+00A0 rather than converting it to ASCII space.
    pub keep_nonbreaking_space: bool,
    /// String placed before each visible list item. Empty omits bullets.
    pub list_bullet: &'options str,
}

impl Default for VisibleDocumentTextOptions<'_> {
    fn default() -> Self {
        Self {
            block_separator: "\n",
            decode_entities: true,
            collapse_whitespace: true,
            keep_nonbreaking_space: false,
            list_bullet: "",
        }
    }
}

/// How a source range contributes to the visible output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VisibleDocumentTextSpanKind {
    /// Bytes of source text copied directly.
    Source,
    /// One decoded character reference, possibly with several scalars.
    Reference,
    /// A whitespace sequence collapsed into one separator.
    Whitespace,
    /// A block boundary or explicit line break.
    Separator,
    /// A generated list item marker.
    Bullet,
    /// A source character or token with a different visible value.
    Replacement,
}

/// One monotonic range mapping, with UTF-8 byte coordinates on both sides.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VisibleDocumentTextSpan {
    /// Range in the projected text.
    pub text: ByteRange,
    /// Covering source byte range. Some generated separators use an empty range.
    pub source: ByteRange,
    /// How the source produced the visible text.
    pub kind: VisibleDocumentTextSpanKind,
}

/// A visible projection and its original source spans.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct VisibleDocumentText {
    /// Projected readable text.
    pub text: String,
    /// Ordered, contiguous output ranges and monotonic source ranges.
    pub map: Vec<VisibleDocumentTextSpan>,
}

impl VisibleDocumentText {
    /// Finds the original UTF-8 span covering a nonempty visible byte range.
    #[must_use]
    pub fn source_range(&self, start: usize, end: usize) -> Option<ByteRange> {
        if start >= end || !self.text.is_char_boundary(start) || !self.text.is_char_boundary(end) {
            return None;
        }
        let first = self
            .map
            .get(self.map.partition_point(|span| span.text.end() <= start))?;
        let last = self
            .map
            .get(self.map.partition_point(|span| span.text.end() < end))?;
        Some(ByteRange::new(
            source_offset(first, start, false),
            source_offset(last, end, true),
        ))
    }
}

fn source_offset(span: &VisibleDocumentTextSpan, offset: usize, end: bool) -> usize {
    if span.kind == VisibleDocumentTextSpanKind::Source {
        return span.source.start() + offset - span.text.start();
    }
    if end {
        span.source.end()
    } else {
        span.source.start()
    }
}

impl LinkNetwork {
    /// Projects a document network without modifying its source links.
    #[must_use]
    pub fn visible_text(&self, options: VisibleDocumentTextOptions<'_>) -> VisibleDocumentText {
        let language = self
            .links()
            .find(|link| link.metadata().link_type() == Some(LinkType::Language))
            .and_then(|link| link.metadata().term())
            .and_then(canonical_language_name);
        let source = self.reconstruct_text();
        let whole = ByteRange::new(0, source.len());
        let mut chunks = Vec::new();
        match language {
            Some("HTML" | "Markdown" | "DOCX") => {
                let syntax = DocumentSyntax::new(self, &source);
                let records = syntax.in_language(language);
                match language {
                    Some("HTML") => {
                        let mut edits = syntax.html_edits(&records);
                        order_chunks(&mut edits);
                        project_edits(whole, &edits, &mut chunks, ReferenceSyntax::Html);
                    }
                    Some("Markdown") => chunks = syntax.markdown_selections(&records),
                    _ => chunks = syntax.docx_selections(&records),
                }
            }
            _ => chunks.push(Chunk::new(whole, ChunkKind::Text(None, true))),
        }
        Projection::new(options).project(chunks, &source)
    }
}

static RULES: LazyLock<BTreeMap<&str, &str>> = LazyLock::new(|| {
    include_str!("../../data/document-projection-rules.lino")
        .lines()
        .map(|line| {
            line[1..line.len() - 1]
                .split_once(' ')
                .expect("document projection rule")
        })
        .collect()
});

fn document_projection_rule(rule: &str) -> std::str::SplitAsciiWhitespace<'static> {
    RULES
        .get(rule)
        .expect("known projection rule")
        .split_ascii_whitespace()
}

fn rule_includes(rule: &str, term: &str) -> bool {
    document_projection_rule(rule).any(|member| member == term)
}

struct SyntaxRecord<'source> {
    term: &'source str,
    language: Option<&'static str>,
    range: ByteRange,
    children: &'source [LinkId],
}

struct DocumentSyntax<'source> {
    source: &'source str,
    records: BTreeMap<LinkId, SyntaxRecord<'source>>,
}

impl<'source> DocumentSyntax<'source> {
    fn new(network: &'source LinkNetwork, source: &'source str) -> Self {
        let records = network
            .links()
            .filter_map(|link| {
                let metadata = link.metadata();
                if metadata.link_type() != Some(LinkType::Syntax) {
                    return None;
                }
                let range = metadata.span()?.byte_range();
                if range.start() == range.end() || source.get(range.start()..range.end()).is_none()
                {
                    return None;
                }
                Some((
                    link.id(),
                    SyntaxRecord {
                        term: metadata.term()?,
                        language: metadata.language().and_then(canonical_language_name),
                        range,
                        children: link.references(),
                    },
                ))
            })
            .collect();
        Self { source, records }
    }

    fn in_language(&self, language: Option<&str>) -> Vec<&SyntaxRecord<'source>> {
        let mut records: Vec<_> = self
            .records
            .values()
            .filter(|record| record.language == language)
            .collect();
        records.sort_by_key(|record| (record.range.start(), std::cmp::Reverse(record.range.end())));
        records
    }

    fn child(
        &self,
        record: &SyntaxRecord<'source>,
        matches: impl Fn(&str) -> bool,
    ) -> Option<&SyntaxRecord<'source>> {
        record
            .children
            .iter()
            .filter_map(|id| self.records.get(id))
            .find(|child| matches(child.term))
    }

    fn read(&self, range: ByteRange) -> &'source str {
        self.source
            .get(range.start()..range.end())
            .unwrap_or_default()
    }

    fn html_name(&self, tag: &SyntaxRecord<'source>) -> String {
        self.child(tag, |kind| {
            matches!(kind, "tag_name" | "erroneous_end_tag_name")
        })
        .map_or_else(String::new, |child| {
            self.read(child.range).to_ascii_lowercase()
        })
    }

    fn html_edits(&self, records: &[&SyntaxRecord<'source>]) -> Vec<Chunk<'source>> {
        let mut edits = Vec::new();
        for record in records {
            let kind = if rule_includes("hiddenHtmlSyntax", record.term) {
                ChunkKind::Skip
            } else if matches!(record.term, "element" | "markup_element") {
                let Some(tag) = self.child(record, |term| rule_includes("htmlTag", term)) else {
                    continue;
                };
                if !rule_includes("hiddenHtml", &self.html_name(tag)) {
                    continue;
                }
                ChunkKind::Skip
            } else if rule_includes("htmlTag", record.term) {
                let name = self.html_name(record);
                match name.as_str() {
                    "br" => ChunkKind::Separator(true),
                    "li" if record.term == "start_tag" => ChunkKind::Bullet,
                    "td" | "th" if record.term == "end_tag" => ChunkKind::Space,
                    name if rule_includes("htmlBlock", name) => ChunkKind::Separator(false),
                    _ => ChunkKind::Skip,
                }
            } else {
                continue;
            };
            edits.push(Chunk::new(record.range, kind));
        }
        edits
    }

    fn markdown_edits(&self, records: &[&SyntaxRecord<'source>]) -> Vec<Chunk<'source>> {
        let mut edits = Vec::new();
        for record in records {
            let range = record.range;
            let start = range.start();
            let end = range.end();
            let kind = if rule_includes("markdownLink", record.term) {
                if let Some(label) = self.child(record, |term| {
                    matches!(term, "link_text" | "image_description")
                }) {
                    edits.push(Chunk::new(
                        ByteRange::new(start, label.range.start()),
                        ChunkKind::Skip,
                    ));
                    edits.push(Chunk::new(
                        ByteRange::new(label.range.end(), end),
                        ChunkKind::Skip,
                    ));
                    continue;
                }
                ChunkKind::Skip
            } else if rule_includes("markdownHiddenSyntax", record.term) {
                ChunkKind::Skip
            } else {
                match record.term {
                    "code_span" => {
                        let delimiters: Vec<_> = record
                            .children
                            .iter()
                            .filter_map(|id| self.records.get(id))
                            .filter(|child| child.term == "code_span_delimiter")
                            .collect();
                        let [first, last] = delimiters.as_slice() else {
                            continue;
                        };
                        let mut code_start = first.range.end();
                        let mut code_end = last.range.start();
                        let code = self.read(ByteRange::new(code_start, code_end));
                        if code.starts_with([' ', '\r', '\n'])
                            && code.ends_with([' ', '\r', '\n'])
                            && code.contains(|character| !matches!(character, ' ' | '\r' | '\n'))
                        {
                            code_start += if code.starts_with("\r\n") { 2 } else { 1 };
                            code_end -= if code.ends_with("\r\n") { 2 } else { 1 };
                        }
                        ChunkKind::Code(ByteRange::new(code_start, code_end))
                    }
                    "backslash_escape" => {
                        ChunkKind::Replace(self.read(ByteRange::new(start + 1, end)))
                    }
                    "hard_line_break" => ChunkKind::Separator(true),
                    "uri_autolink" | "email_autolink" => {
                        edits.push(Chunk::new(
                            ByteRange::new(start, start + 1),
                            ChunkKind::Skip,
                        ));
                        edits.push(Chunk::new(ByteRange::new(end - 1, end), ChunkKind::Skip));
                        continue;
                    }
                    _ => continue,
                }
            };
            edits.push(Chunk::new(range, kind));
        }
        order_chunks(&mut edits);
        edits
    }

    fn markdown_selections(&self, records: &[&SyntaxRecord<'source>]) -> Vec<Chunk<'source>> {
        let edits = self.markdown_edits(records);
        let mut tags = self.html_edits(&self.in_language(Some("HTML")));
        order_chunks(&mut tags);
        let mut markers = records
            .iter()
            .copied()
            .filter(|record| record.term.starts_with("list_marker_"))
            .peekable();
        let mut last_end = 0;
        let mut chunks = Vec::new();
        for record in records
            .iter()
            .copied()
            .filter(|record| rule_includes("markdownTextContainer", record.term))
        {
            let range = record.range;
            let start = range.start();
            if start < last_end {
                continue;
            }
            let mut marker = None;
            while markers.peek().is_some_and(|item| item.range.end() <= start) {
                let candidate = markers.next().expect("visible list marker");
                if candidate.range.start() >= last_end {
                    marker = Some(candidate.range);
                }
            }
            let gap = ByteRange::new(last_end, marker.map_or(start, |range| range.start()));
            let separator = if record.term == "pipe_table_cell"
                && !chunks.is_empty()
                && !self.read(gap).contains(['\r', '\n'])
            {
                ChunkKind::Space
            } else {
                ChunkKind::Separator(false)
            };
            chunks.push(Chunk::new(gap, separator));
            if let Some(marker) = marker {
                chunks.push(Chunk::new(marker, ChunkKind::Bullet));
            }
            match record.term {
                "html_block" => project_edits(range, &tags, &mut chunks, ReferenceSyntax::Html),
                "code_fence_content" | "indented_code_block" => {
                    chunks.push(Chunk::new(range, ChunkKind::Text(None, true)))
                }
                _ => project_edits(range, &edits, &mut chunks, ReferenceSyntax::Markdown),
            }
            last_end = range.end();
        }
        chunks
    }

    fn docx_selections(&self, records: &[&SyntaxRecord<'source>]) -> Vec<Chunk<'source>> {
        let mut chunks = Vec::new();
        for record in records.iter().filter(|record| record.term == "element") {
            let Some(open) = self.child(record, |term| matches!(term, "STag" | "EmptyElemTag"))
            else {
                continue;
            };
            let Some(name) = self.child(open, |term| term == "Name") else {
                continue;
            };
            let kind = match self.read(name.range) {
                "w:p" => ChunkKind::Separator(false),
                "w:tab" => ChunkKind::Replace("\t"),
                "w:br" | "w:cr" => ChunkKind::Separator(true),
                "w:t" => {
                    if let Some(close) = self.child(record, |term| term == "ETag")
                        && open.range.end() < close.range.start()
                    {
                        let range = ByteRange::new(open.range.end(), close.range.start());
                        chunks.push(Chunk::new(
                            range,
                            ChunkKind::Text(Some(ReferenceSyntax::Xml), true),
                        ));
                    }
                    continue;
                }
                _ => continue,
            };
            chunks.push(Chunk::new(open.range, kind));
        }
        order_chunks(&mut chunks);
        chunks
    }
}

#[derive(Clone, Copy)]
enum ChunkKind<'source> {
    Skip,
    Separator(bool),
    Bullet,
    Space,
    Replace(&'source str),
    Code(ByteRange),
    Text(Option<ReferenceSyntax>, bool),
}

#[derive(Clone, Copy)]
struct Chunk<'source> {
    range: ByteRange,
    kind: ChunkKind<'source>,
}

impl<'source> Chunk<'source> {
    const fn new(range: ByteRange, kind: ChunkKind<'source>) -> Self {
        Self { range, kind }
    }
}

fn order_chunks(chunks: &mut [Chunk<'_>]) {
    chunks.sort_by_key(|chunk| (chunk.range.start(), std::cmp::Reverse(chunk.range.end())));
}

fn project_edits<'source>(
    range: ByteRange,
    edits: &[Chunk<'source>],
    chunks: &mut Vec<Chunk<'source>>,
    syntax: ReferenceSyntax,
) {
    let mut cursor = range.start();
    let first = edits.partition_point(|edit| edit.range.start() < cursor);
    let text = ChunkKind::Text(Some(syntax), false);
    for edit in edits[first..]
        .iter()
        .take_while(|edit| edit.range.start() < range.end())
    {
        let start = edit.range.start();
        if start < cursor || edit.range.end() > range.end() {
            continue;
        }
        if start > cursor {
            chunks.push(Chunk::new(ByteRange::new(cursor, start), text));
        }
        chunks.push(*edit);
        cursor = edit.range.end();
    }
    if cursor < range.end() {
        chunks.push(Chunk::new(ByteRange::new(cursor, range.end()), text));
    }
}

struct Projection<'options> {
    output: VisibleDocumentText,
    options: VisibleDocumentTextOptions<'options>,
    space: Option<ByteRange>,
    separators: Vec<(ByteRange, bool)>,
}

impl<'options> Projection<'options> {
    const fn new(options: VisibleDocumentTextOptions<'options>) -> Self {
        Self {
            output: VisibleDocumentText {
                text: String::new(),
                map: Vec::new(),
            },
            options,
            space: None,
            separators: Vec::new(),
        }
    }

    fn project(mut self, chunks: Vec<Chunk<'_>>, source: &str) -> VisibleDocumentText {
        use VisibleDocumentTextSpanKind::{Bullet, Replacement, Whitespace};
        for chunk in chunks {
            let range = chunk.range;
            match chunk.kind {
                ChunkKind::Skip => {}
                ChunkKind::Separator(explicit) => self.boundary(range, explicit),
                ChunkKind::Bullet => {
                    if self.options.list_bullet.is_empty() {
                        self.boundary(range, false);
                    } else {
                        self.boundary(ByteRange::new(range.start(), range.start()), false);
                        self.emit(self.options.list_bullet, range, Bullet, true);
                    }
                }
                ChunkKind::Space => self.emit(" ", range, Whitespace, false),
                ChunkKind::Replace(value) => self.emit(value, range, Replacement, true),
                ChunkKind::Code(code) => self.read_text(source, code, None, true, true),
                ChunkKind::Text(syntax, preserve) => {
                    self.read_text(source, range, syntax, preserve, false)
                }
            }
        }
        self.output
    }

    fn boundary(&mut self, range: ByteRange, explicit: bool) {
        self.space = None;
        if self.output.text.is_empty() || (!explicit && !self.separators.is_empty()) {
            return;
        }
        if explicit && matches!(self.separators.as_slice(), [(_, false)]) {
            self.separators.clear();
        }
        self.separators.push((range, explicit));
    }

    fn read_text(
        &mut self,
        source: &str,
        range: ByteRange,
        syntax: Option<ReferenceSyntax>,
        preserve: bool,
        code: bool,
    ) {
        use VisibleDocumentTextSpanKind::{Reference, Replacement, Source};
        let mut start = range.start();
        while start < range.end() {
            let rest = &source[start..range.end()];
            if self.options.decode_entities
                && rest.starts_with('&')
                && let Some((width, decoded)) =
                    syntax.and_then(|syntax| character_reference(rest, syntax))
            {
                self.emit(
                    &decoded,
                    ByteRange::new(start, start + width),
                    Reference,
                    preserve,
                );
                start += width;
                continue;
            }
            let character = rest.chars().next().expect("nonempty source text");
            let width = character.len_utf8();
            if code && matches!(character, '\r' | '\n') {
                let consumed = if rest.starts_with("\r\n") { 2 } else { width };
                self.emit(
                    " ",
                    ByteRange::new(start, start + consumed),
                    Replacement,
                    true,
                );
                start += consumed;
                continue;
            }
            self.emit(
                &rest[..width],
                ByteRange::new(start, start + width),
                Source,
                preserve,
            );
            start += width;
        }
    }

    fn emit(
        &mut self,
        text: &str,
        range: ByteRange,
        mut kind: VisibleDocumentTextSpanKind,
        preserve: bool,
    ) {
        use VisibleDocumentTextSpanKind::{Replacement, Separator, Source, Whitespace};
        let replacement;
        let text = if !self.options.keep_nonbreaking_space && text.contains('\u{a0}') {
            replacement = text.replace('\u{a0}', " ");
            if kind == Source {
                kind = Replacement;
            }
            replacement.as_str()
        } else {
            text
        };
        if self.options.collapse_whitespace
            && !preserve
            && text
                .chars()
                .all(|character| document_scalar_is_whitespace(f64::from(u32::from(character))))
        {
            if !self.output.text.is_empty() && self.separators.is_empty() {
                self.space = Some(ByteRange::new(
                    self.space.map_or(range.start(), |space| space.start()),
                    range.end(),
                ));
            }
            return;
        }
        let separators = std::mem::take(&mut self.separators);
        for (range, _) in separators {
            self.append(self.options.block_separator, range, Separator);
        }
        if let Some(space) = self.space.take() {
            self.append(" ", space, Whitespace);
        }
        self.append(text, range, kind);
    }

    fn append(&mut self, text: &str, source: ByteRange, kind: VisibleDocumentTextSpanKind) {
        if text.is_empty() {
            return;
        }
        let start = self.output.text.len();
        let end = start + text.len();
        if kind == VisibleDocumentTextSpanKind::Source
            && let Some(previous) = self.output.map.last_mut()
            && previous.kind == kind
            && previous.source.end() == source.start()
        {
            previous.text = ByteRange::new(previous.text.start(), end);
            previous.source = ByteRange::new(previous.source.start(), source.end());
        } else {
            self.output.map.push(VisibleDocumentTextSpan {
                text: ByteRange::new(start, end),
                source,
                kind,
            });
        }
        self.output.text.push_str(text);
    }
}
