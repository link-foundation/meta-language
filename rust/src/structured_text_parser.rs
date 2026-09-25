use std::sync::OnceLock;

use regex::Regex;

use crate::builtin_grammar::{propagate_errors, GrammarNode};
use crate::line_index::LineIndex;
use crate::lino_grammar::parse_lino_cst;
use crate::natural_language::{annotate_natural_language, canonical_natural_language};
use crate::pdf_grammar::parse_pdf_cst;
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
    let tree = match grammar {
        TextGrammar::Plain => parse_plain_text_cst(text),
        TextGrammar::Lino => parse_lino_cst(text),
        TextGrammar::Pdf => parse_pdf_cst(text),
        TextGrammar::Natural => parse_natural_language_cst(text),
    };
    insert_grammar_node(network, parent, &source, &tree)
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
/// its fields and the trivia of its extras; a missing node is a zero-width
/// Syntax link without a token.
fn insert_grammar_node(
    network: &mut LinkNetwork,
    parent: LinkId,
    source: &Source<'_>,
    node: &GrammarNode,
) -> LinkId {
    let flags = if node.is_error {
        LinkFlags::error()
    } else if node.is_missing {
        LinkFlags::missing()
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
    if node.is_token() {
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

/// Parses plain text with the built-in line grammar: a `text_document` of
/// lines holding the shared lexical nodes. Unbalanced parentheses make the
/// document an `ERROR`-flagged root.
pub fn parse_plain_text_cst(text: &str) -> GrammarNode {
    let mut lines = Vec::new();
    let mut start = 0;
    for line in text.split_inclusive('\n') {
        let end = start + line.len();
        lines.push(GrammarNode::node(
            "line",
            start,
            end,
            lexical_nodes(text, start, end),
        ));
        start = end;
    }
    if start < text.len() || text.is_empty() {
        lines.push(GrammarNode::node(
            "line",
            start,
            text.len(),
            lexical_nodes(text, start, text.len()),
        ));
    }
    let mut root = GrammarNode::node("text_document", 0, text.len(), lines);
    root.is_error = !balanced_parentheses(text);
    propagate_errors(&mut root);
    root
}

/// Parses natural-language text with the built-in sentence grammar: a
/// `natural_language_document` of sentences holding the shared lexical nodes.
pub fn parse_natural_language_cst(text: &str) -> GrammarNode {
    let sentences = sentence_ranges(text)
        .into_iter()
        .map(|(start, end)| {
            GrammarNode::node("sentence", start, end, lexical_nodes(text, start, end))
        })
        .collect();
    let mut root = GrammarNode::node("natural_language_document", 0, text.len(), sentences);
    propagate_errors(&mut root);
    root
}

/// The shared lexical nodes of `text[start..end]`: whitespace extras, words,
/// punctuation and `ERROR` nodes for runs of control characters.
fn lexical_nodes(text: &str, start: usize, end: usize) -> Vec<GrammarNode> {
    lexical_pattern()
        .find_iter(&text[start..end])
        .map(|segment_match| {
            let segment = segment_match.as_str();
            let segment_start = start + segment_match.start();
            let segment_end = start + segment_match.end();
            if segment.chars().all(char::is_whitespace) {
                GrammarNode::extra("whitespace", false, segment_start, segment_end)
            } else if segment.chars().all(is_control_character) {
                GrammarNode::error(segment_start, segment_end, Vec::new())
            } else if word_pattern().is_match(segment) {
                GrammarNode::node("word", segment_start, segment_end, Vec::new())
            } else {
                GrammarNode::node("punctuation", segment_start, segment_end, Vec::new())
            }
        })
        .collect()
}

// The built-in lexical grammar shared with the JavaScript runtime: maximal runs
// of Unicode whitespace, of word characters, of control characters that are
// not whitespace (which the grammar rejects), and of any other characters.
const WORD_CHARACTERS: &str = r"\p{L}\p{N}\p{M}_'\-";
const CONTROL_CHARACTERS: &str = r"\x00-\x08\x0E-\x1F\x7F-\x{84}\x{86}-\x{9F}";

const fn is_control_character(character: char) -> bool {
    matches!(
        character,
        '\u{0}'..='\u{8}' | '\u{E}'..='\u{1F}' | '\u{7F}'..='\u{84}' | '\u{86}'..='\u{9F}'
    )
}

fn lexical_pattern() -> &'static Regex {
    static PATTERN: OnceLock<Regex> = OnceLock::new();
    PATTERN.get_or_init(|| {
        Regex::new(&format!(
            r"\p{{White_Space}}+|[{WORD_CHARACTERS}]+|[{CONTROL_CHARACTERS}]+|[^\p{{White_Space}}{WORD_CHARACTERS}{CONTROL_CHARACTERS}]+"
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
