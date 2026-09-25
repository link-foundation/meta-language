use std::sync::OnceLock;

use regex::Regex;

use crate::line_index::LineIndex;
use crate::lino_grammar::{parse_lino_cst, LinoNode};
use crate::natural_language::{annotate_natural_language, canonical_natural_language};
use crate::tree_sitter_adapter::SpanOffset;
use crate::{
    ByteRange, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration,
    SourceSpan,
};

/// Built-in structured-text grammar selected for a language label.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TextGrammar {
    Plain,
    Lino,
    Pdf,
    Natural,
}

impl TextGrammar {
    fn for_language(language: &str) -> Option<Self> {
        match language.to_ascii_lowercase().as_str() {
            "txt" | "text" | "plain text" => Some(Self::Plain),
            "lino" => Some(Self::Lino),
            "pdf" => Some(Self::Pdf),
            _ if canonical_natural_language(language).is_some() => Some(Self::Natural),
            _ => None,
        }
    }
}

pub fn parse_plain(text: &str, language: &str, configuration: ParseConfiguration) -> LinkNetwork {
    parse_document(text, language, configuration, TextGrammar::Plain)
}

pub fn parse_lino(text: &str, language: &str, configuration: ParseConfiguration) -> LinkNetwork {
    parse_document(text, language, configuration, TextGrammar::Lino)
}

pub fn parse_pdf(text: &str, language: &str, configuration: ParseConfiguration) -> LinkNetwork {
    parse_document(text, language, configuration, TextGrammar::Pdf)
}

pub fn parse_natural(text: &str, language: &str, configuration: ParseConfiguration) -> LinkNetwork {
    let (mut network, document) = LinkNetwork::new_parse_document(text, language);
    insert_grammar(
        &mut network,
        document,
        text,
        language,
        configuration,
        TextGrammar::Natural,
        SpanOffset::zero(),
    );
    network.attach_embedded_regions(document, text, language, configuration);
    annotate_natural_language(&mut network, document, text, language, configuration);
    network
}

/// Parses an embedded region written in a built-in structured-text language
/// into `network` below `region`, returning the grammar root, or `None` when
/// `language` has no built-in structured-text grammar.
pub fn parse_embedded_region_into(
    network: &mut LinkNetwork,
    region: LinkId,
    text: &str,
    language: &str,
    span: SourceSpan,
    configuration: ParseConfiguration,
) -> Option<LinkId> {
    let grammar = TextGrammar::for_language(language)?;
    let offset = SpanOffset::new(span.byte_range().start(), span.start_point());
    Some(insert_grammar(
        network,
        region,
        text,
        language,
        configuration,
        grammar,
        offset,
    ))
}

fn parse_document(
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
    grammar: TextGrammar,
) -> LinkNetwork {
    let (mut network, document) = LinkNetwork::new_parse_document(text, language);
    insert_grammar(
        &mut network,
        document,
        text,
        language,
        configuration,
        grammar,
        SpanOffset::zero(),
    );
    network.attach_embedded_regions(document, text, language, configuration);
    network
}

fn insert_grammar(
    network: &mut LinkNetwork,
    parent: LinkId,
    text: &str,
    language: &str,
    configuration: ParseConfiguration,
    grammar: TextGrammar,
    offset: SpanOffset,
) -> LinkId {
    let source = Source {
        text,
        language,
        lines: LineIndex::new(text),
        offset,
        configuration,
    };
    match grammar {
        TextGrammar::Plain => insert_lines(
            network,
            parent,
            &source,
            "text_document",
            |_| ("line", false),
            balanced_parentheses(text),
        ),
        TextGrammar::Lino => insert_grammar_node(network, parent, &source, &parse_lino_cst(text)),
        TextGrammar::Pdf => insert_lines(
            network,
            parent,
            &source,
            "pdf_file",
            pdf_line,
            valid_pdf_container(text),
        ),
        TextGrammar::Natural => insert_sentences(network, parent, &source),
    }
}

/// Text being parsed together with its position in the host document.
struct Source<'a> {
    text: &'a str,
    language: &'a str,
    lines: LineIndex,
    offset: SpanOffset,
    configuration: ParseConfiguration,
}

impl Source<'_> {
    fn span(&self, start: usize, end: usize) -> SourceSpan {
        SourceSpan::new(
            ByteRange::new(self.offset.byte(start), self.offset.byte(end)),
            self.offset.point(self.lines.byte_point(start)),
            self.offset.point(self.lines.byte_point(end)),
        )
    }
}

/// Inserts a built-in grammar tree of byte offsets below `parent`, recording
/// its fields and the trivia of its whitespace extras.
fn insert_grammar_node(
    network: &mut LinkNetwork,
    parent: LinkId,
    source: &Source<'_>,
    node: &LinoNode,
) -> LinkId {
    let flags = if node.is_error {
        LinkFlags::error()
    } else if node.has_error {
        LinkFlags::containing_error()
    } else if node.extra {
        LinkFlags::extra()
    } else {
        LinkFlags::clean()
    };
    let span = source.span(node.start, node.end);
    let syntax = network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(node.named)
            .with_term(node.term)
            .with_language(source.language)
            .with_span(span)
            .with_flags(flags),
    );
    if node.is_leaf() {
        let token = network.insert_link(
            [syntax],
            LinkMetadata::new()
                .with_link_type(LinkType::Token)
                .with_named(node.named)
                .with_term(&source.text[node.start..node.end])
                .with_language(source.language)
                .with_span(span)
                .with_flags(flags),
        );
        if node.extra {
            network.attach_trivia(
                syntax,
                token,
                span,
                source.configuration.trivia_attachment_policy(),
            );
        }
    }
    for child in &node.children {
        let child_id = insert_grammar_node(network, syntax, source, child);
        if let Some(field) = child.field {
            network.insert_field(syntax, field, child_id);
        }
    }
    syntax
}

fn insert_sentences(network: &mut LinkNetwork, parent: LinkId, source: &Source<'_>) -> LinkId {
    let root = insert_syntax(
        network,
        parent,
        "natural_language_document",
        source.language,
        source.span(0, source.text.len()),
        LinkFlags::clean(),
    );
    for (start, end) in sentence_ranges(source.text) {
        let sentence = insert_syntax(
            network,
            root,
            "sentence",
            source.language,
            source.span(start, end),
            LinkFlags::clean(),
        );
        insert_lexical_nodes(network, sentence, source, start, end);
    }
    root
}

fn insert_lines(
    network: &mut LinkNetwork,
    parent: LinkId,
    source: &Source<'_>,
    root_term: &str,
    classify_line: fn(&str) -> (&'static str, bool),
    document_is_valid: bool,
) -> LinkId {
    let text = source.text;
    let mut classified = Vec::new();
    let mut start = 0;
    for line in text.split_inclusive('\n') {
        let end = start + line.len();
        classified.push((start, end, classify_line(line)));
        start = end;
    }
    if start < text.len() || text.is_empty() {
        classified.push((start, text.len(), classify_line(&text[start..])));
    }
    let has_line_error = classified.iter().any(|(_, _, (_, error))| *error);
    // A line error is located below the root; a document-level validation
    // failure without a located line error makes the root itself the error.
    let root_flags = if has_line_error {
        LinkFlags::containing_error()
    } else if document_is_valid {
        LinkFlags::clean()
    } else {
        LinkFlags::error()
    };
    let root = insert_syntax(
        network,
        parent,
        root_term,
        source.language,
        source.span(0, text.len()),
        root_flags,
    );

    for (start, end, (term, error)) in classified {
        let flags = if error {
            LinkFlags::error()
        } else {
            LinkFlags::clean()
        };
        let line = insert_syntax(
            network,
            root,
            term,
            source.language,
            source.span(start, end),
            flags,
        );
        insert_lexical_nodes(network, line, source, start, end);
    }
    root
}

fn insert_lexical_nodes(
    network: &mut LinkNetwork,
    parent: LinkId,
    source: &Source<'_>,
    start: usize,
    end: usize,
) {
    for segment_match in lexical_pattern().find_iter(&source.text[start..end]) {
        let segment = segment_match.as_str();
        let segment_start = start + segment_match.start();
        let segment_end = start + segment_match.end();
        let whitespace = segment.chars().all(char::is_whitespace);
        let term = if whitespace {
            "whitespace"
        } else if word_pattern().is_match(segment) {
            "word"
        } else {
            "punctuation"
        };
        let span = source.span(segment_start, segment_end);
        let flags = if whitespace {
            LinkFlags::extra()
        } else {
            LinkFlags::clean()
        };
        // Whitespace is an anonymous extra, as tree-sitter reports extras.
        let syntax = network.insert_link(
            [parent],
            LinkMetadata::new()
                .with_link_type(LinkType::Syntax)
                .with_named(!whitespace)
                .with_term(term)
                .with_language(source.language)
                .with_span(span)
                .with_flags(flags),
        );
        let token = network.insert_link(
            [syntax],
            LinkMetadata::new()
                .with_link_type(LinkType::Token)
                .with_named(!whitespace)
                .with_term(segment)
                .with_language(source.language)
                .with_span(span)
                .with_flags(flags),
        );
        if whitespace {
            network.attach_trivia(
                syntax,
                token,
                span,
                source.configuration.trivia_attachment_policy(),
            );
        }
    }
}

// The built-in lexical grammar shared with the JavaScript runtime: maximal runs
// of Unicode whitespace, of word characters, and of any other characters.
const WORD_CHARACTERS: &str = r"\p{L}\p{N}\p{M}_'\-";

fn lexical_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(&format!(
            r"\p{{White_Space}}+|[{WORD_CHARACTERS}]+|[^\p{{White_Space}}{WORD_CHARACTERS}]+"
        ))
        .expect("the built-in lexical grammar is a valid pattern")
    })
}

fn word_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(&format!(r"^[{WORD_CHARACTERS}]+$")).expect("the built-in word pattern is valid")
    })
}

// The built-in sentence grammar shared with the JavaScript runtime: a sentence
// ends after a run of terminal punctuation, any closing punctuation or quotes,
// and the whitespace that follows them.
fn sentence_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(
            r"(?s).*?[.!?\x{0964}\x{3002}\x{061F}\x{06D4}\x{FF01}\x{FF1F}]+[\p{Pe}\p{Pf}\x{0022}\x{0027}]*\p{White_Space}*",
        )
        .expect("the built-in sentence grammar is a valid pattern")
    })
}

fn sentence_ranges(text: &str) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut start = 0;
    for sentence in sentence_pattern().find_iter(text) {
        ranges.push((sentence.start(), sentence.end()));
        start = sentence.end();
    }
    if start < text.len() {
        ranges.push((start, text.len()));
    }
    ranges
}

fn insert_syntax(
    network: &mut LinkNetwork,
    parent: LinkId,
    term: &str,
    language: &str,
    span: SourceSpan,
    flags: LinkFlags,
) -> LinkId {
    network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term(term)
            .with_language(language)
            .with_span(span)
            .with_flags(flags),
    )
}

fn pdf_line(line: &str) -> (&'static str, bool) {
    let trimmed = line.trim();
    if trimmed.starts_with("%PDF-") {
        ("header", false)
    } else if trimmed == "%%EOF" {
        ("end_of_file", false)
    } else if is_pdf_object_header(trimmed) {
        ("object_header", false)
    } else if trimmed == "endobj" {
        ("object_end", false)
    } else if trimmed == "xref" {
        ("cross_reference_table", false)
    } else if trimmed == "trailer" {
        ("trailer", false)
    } else if matches!(trimmed, "stream" | "endstream") {
        ("stream_boundary", false)
    } else {
        ("pdf_line", false)
    }
}

fn is_pdf_object_header(text: &str) -> bool {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN
        .get_or_init(|| {
            Regex::new(r"^[0-9]+\p{White_Space}+[0-9]+\p{White_Space}+obj$")
                .expect("the PDF object header pattern is valid")
        })
        .is_match(text)
}

fn valid_pdf_container(text: &str) -> bool {
    static HEADER: OnceLock<Regex> = OnceLock::new();
    static TRAILER: OnceLock<Regex> = OnceLock::new();
    HEADER
        .get_or_init(|| Regex::new(r"^%PDF-[0-9]+\.[0-9]+").expect("valid PDF header pattern"))
        .is_match(text)
        && TRAILER
            .get_or_init(|| {
                Regex::new(r"%%EOF\p{White_Space}*$").expect("valid PDF trailer pattern")
            })
            .is_match(text)
}

fn balanced_parentheses(text: &str) -> bool {
    let mut depth = 0_usize;
    for character in text.chars() {
        match character {
            '(' => depth += 1,
            ')' if depth == 0 => return false,
            ')' => depth -= 1,
            _ => {}
        }
    }
    depth == 0
}
