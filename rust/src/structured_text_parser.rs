use std::sync::OnceLock;

use serde_json::Value;

use crate::builtin_grammar::{GrammarNode, propagate_errors};
use crate::line_index::LineIndex;
use crate::lino_grammar::parse_lino_cst;
use crate::natural_language::{annotate_natural_language, canonical_natural_language};
use crate::pdf_grammar::parse_pdf_cst;
use crate::tree_sitter_adapter::SpanOffset;
use crate::{
    ByteRange, FeatureGrammarParser, FeatureParseOptions, LinkFlags, LinkId, LinkMetadata,
    LinkNetwork, LinkType, ParseConfiguration, SourceSpan, SyntaxTree, compile_feature_grammar,
    parse_grammar_links,
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

/// Runs the generated native line grammar with shared lexical rules.
pub fn parse_plain_text_cst(text: &str) -> GrammarNode {
    parse_native_text_cst(text, "plain")
}

/// Runs the generated native sentence grammar with shared lexical rules.
pub fn parse_natural_language_cst(text: &str) -> GrammarNode {
    parse_native_text_cst(text, "natural")
}

fn text_grammar_metadata() -> &'static Value {
    static METADATA: OnceLock<Value> = OnceLock::new();
    METADATA.get_or_init(|| {
        serde_json::from_str(include_str!("data/native-text-grammar.json"))
            .expect("generated native text metadata is valid JSON")
    })
}

fn text_grammar_parser() -> &'static FeatureGrammarParser {
    static PARSER: OnceLock<FeatureGrammarParser> = OnceLock::new();
    PARSER.get_or_init(|| {
        let grammar = parse_grammar_links(include_str!("data/native-text-grammar.lino"))
            .expect("generated native text grammar is valid Links Notation");
        compile_feature_grammar(&grammar, None, FeatureParseOptions::default())
            .expect("generated native text grammar compiles")
    })
}

fn parse_native_text(text: &str, start_rule: &str) -> Option<SyntaxTree> {
    let options = FeatureParseOptions {
        start_rule: Some(start_rule.to_owned()),
        ..FeatureParseOptions::default()
    };
    text_grammar_parser()
        .parse_tree(text.as_bytes(), &options)
        .ok()
        .filter(|outcome| outcome.ok)
        .and_then(|outcome| outcome.tree)
}

fn projected_term(kind: &str) -> &'static str {
    text_grammar_metadata()["projection"][kind]["term"]
        .as_str()
        .expect("generated text kind has a projection")
}

fn parse_native_text_cst(text: &str, mode: &str) -> GrammarNode {
    let metadata = text_grammar_metadata();
    let start_rule = metadata["starts"][mode]
        .as_str()
        .expect("generated text grammar has this mode");
    let term = projected_term(start_rule);
    let Some(SyntaxTree::Node { children, .. }) = parse_native_text(text, start_rule) else {
        let mut root = GrammarNode::node(
            term,
            0,
            text.len(),
            vec![GrammarNode::error(0, text.len(), Vec::new())],
        );
        propagate_errors(&mut root);
        return root;
    };
    let regions = children
        .into_iter()
        .filter_map(|region| {
            let SyntaxTree::Node {
                kind, start, end, ..
            } = region
            else {
                return None;
            };
            Some(GrammarNode::node(
                projected_term(&kind),
                start,
                end,
                lexical_nodes(&text[start..end], start),
            ))
        })
        .collect();
    let mut root = GrammarNode::node(term, 0, text.len(), regions);
    root.is_error = metadata["validation"][mode]
        .as_array()
        .expect("generated text validation rules are an array")
        .iter()
        .any(|rule| {
            parse_native_text(text, rule.as_str().expect("validation rule is text")).is_none()
        });
    propagate_errors(&mut root);
    root
}

fn lexical_nodes(text: &str, offset: usize) -> Vec<GrammarNode> {
    let start_rule = text_grammar_metadata()["starts"]["lexical"]
        .as_str()
        .expect("generated text grammar has a lexical start");
    let Some(SyntaxTree::Node { children, .. }) = parse_native_text(text, start_rule) else {
        return vec![GrammarNode::error(offset, offset + text.len(), Vec::new())];
    };
    children
        .into_iter()
        .filter_map(|token| {
            let SyntaxTree::Token {
                kind: Some(kind),
                start,
                end,
                ..
            } = token
            else {
                return None;
            };
            let projection = &text_grammar_metadata()["projection"][&kind];
            let start = offset + start;
            let end = offset + end;
            let term = projected_term(&kind);
            if projection["error"].as_bool().unwrap_or(false) {
                Some(GrammarNode::error(start, end, Vec::new()))
            } else if projection["extra"].as_bool().unwrap_or(false) {
                Some(GrammarNode::extra(
                    term,
                    projection["named"].as_bool().unwrap_or(true),
                    start,
                    end,
                ))
            } else {
                Some(GrammarNode::node(term, start, end, Vec::new()))
            }
        })
        .collect()
}
