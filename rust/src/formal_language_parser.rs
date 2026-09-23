//! Lossless portable frontends for Lean 4 and Rocq/Coq source.
//!
//! Both languages permit syntax extensions that cannot be resolved by a static
//! parser without the project environment. This layer therefore exposes a
//! concrete lexical hierarchy, retains every source byte, and reports malformed
//! strings, comments, and delimiter structure. It deliberately does not label
//! unresolved notation as elaborated or kernel-checked syntax.

use crate::line_index::LineIndex;
use crate::{
    ByteRange, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType, ParseConfiguration,
    SourceSpan,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FormalLanguage {
    Lean,
    Rocq,
}

impl FormalLanguage {
    fn from_label(label: &str) -> Option<Self> {
        if label.eq_ignore_ascii_case("lean") || label.eq_ignore_ascii_case("lean4") {
            Some(Self::Lean)
        } else if label.eq_ignore_ascii_case("rocq") || label.eq_ignore_ascii_case("coq") {
            Some(Self::Rocq)
        } else {
            None
        }
    }

    const fn root_term(self) -> &'static str {
        match self {
            Self::Lean => "file",
            Self::Rocq => "source_file",
        }
    }

    fn is_keyword(self, value: &str) -> bool {
        match self {
            Self::Lean => LEAN_KEYWORDS.contains(&value),
            Self::Rocq => ROCQ_KEYWORDS.contains(&value),
        }
    }

    fn is_builtin_type(self, value: &str) -> bool {
        match self {
            Self::Lean => LEAN_BUILTIN_TYPES.contains(&value),
            Self::Rocq => ROCQ_BUILTIN_TYPES.contains(&value),
        }
    }
}

const LEAN_BUILTIN_TYPES: &[&str] = &[
    "Bool", "Char", "Float", "Int", "Nat", "Prop", "Sort", "String", "Type", "UInt64",
];
const ROCQ_BUILTIN_TYPES: &[&str] = &["bool", "nat", "Prop", "Set", "SProp", "Type", "Z"];

const LEAN_KEYWORDS: &[&str] = &[
    "abbrev",
    "axiom",
    "class",
    "def",
    "deriving",
    "do",
    "else",
    "end",
    "example",
    "export",
    "if",
    "import",
    "in",
    "inductive",
    "instance",
    "let",
    "macro",
    "match",
    "namespace",
    "notation",
    "opaque",
    "open",
    "partial",
    "private",
    "protected",
    "structure",
    "syntax",
    "theorem",
    "universe",
    "variable",
    "where",
];

const ROCQ_KEYWORDS: &[&str] = &[
    "Axiom",
    "Check",
    "Class",
    "CoFixpoint",
    "CoInductive",
    "Compute",
    "Definition",
    "End",
    "Eval",
    "Export",
    "Fixpoint",
    "From",
    "Goal",
    "Hint",
    "Import",
    "Include",
    "Inductive",
    "Instance",
    "Lemma",
    "Ltac",
    "Module",
    "Notation",
    "Parameter",
    "Print",
    "Proof",
    "Qed",
    "Record",
    "Require",
    "Section",
    "Theorem",
    "Universe",
    "Variable",
    "Variables",
];

#[derive(Clone, Debug, PartialEq, Eq)]
struct Token<'a> {
    text: &'a str,
    term: &'static str,
    span: SourceSpan,
    flags: LinkFlags,
    named: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OpenGroup {
    delimiter: char,
    group: LinkId,
}

pub fn parse(text: &str, language: &str, configuration: ParseConfiguration) -> Option<LinkNetwork> {
    let formal_language = FormalLanguage::from_label(language)?;
    let lines = LineIndex::new(text);
    let tokens = scan(text, formal_language, &lines);
    let (mut network, document) = LinkNetwork::new_parse_document(text, language);
    let document_span = span(&lines, 0, text.len());
    let root = network.insert_link(
        [document],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(true)
            .with_term(formal_language.root_term())
            .with_language(language)
            .with_span(document_span),
    );
    let mut groups = Vec::<OpenGroup>::new();

    for token in tokens {
        let parent = groups.last().map_or(root, |group| group.group);
        if let Some(delimiter) = opening_delimiter(token.text) {
            let group = network.insert_link(
                [parent],
                LinkMetadata::new()
                    .with_link_type(LinkType::Syntax)
                    .with_named(true)
                    .with_term(group_term(delimiter))
                    .with_language(language)
                    .with_span(token.span),
            );
            insert_token(
                &mut network,
                document,
                group,
                &token,
                language,
                configuration,
            );
            groups.push(OpenGroup { delimiter, group });
            continue;
        }

        if let Some(delimiter) = closing_delimiter(token.text) {
            let matching = groups
                .last()
                .is_some_and(|group| closes(group.delimiter, delimiter));
            let mut token = token;
            if !matching {
                token.flags = token.flags.with_error();
            }
            let parent = groups.last().map_or(root, |group| group.group);
            insert_token(
                &mut network,
                document,
                parent,
                &token,
                language,
                configuration,
            );
            if matching {
                let group = *groups.last().expect("matching delimiter has an open group");
                let start = network
                    .link(group.group)
                    .and_then(|link| link.metadata().span())
                    .expect("group carries its opening span");
                network.set_span(
                    group.group,
                    SourceSpan::new(
                        ByteRange::new(start.byte_range().start(), token.span.byte_range().end()),
                        start.start_point(),
                        token.span.end_point(),
                    ),
                );
                groups.pop();
            }
            continue;
        }

        insert_token(
            &mut network,
            document,
            parent,
            &token,
            language,
            configuration,
        );
    }

    for group in groups {
        let start = network
            .link(group.group)
            .and_then(|link| link.metadata().span())
            .expect("group carries its opening span");
        network.set_span(
            group.group,
            SourceSpan::new(
                ByteRange::new(start.byte_range().start(), document_span.byte_range().end()),
                start.start_point(),
                document_span.end_point(),
            ),
        );
        network.set_flags(group.group, LinkFlags::containing_error());
    }
    Some(network)
}

fn insert_token(
    network: &mut LinkNetwork,
    document: LinkId,
    parent: LinkId,
    token: &Token<'_>,
    language: &str,
    configuration: ParseConfiguration,
) {
    let syntax = network.insert_link(
        [parent],
        LinkMetadata::new()
            .with_link_type(LinkType::Syntax)
            .with_named(token.named)
            .with_term(token.term)
            .with_language(language)
            .with_span(token.span)
            .with_flags(token.flags),
    );
    let source = network.insert_link(
        [syntax],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_named(token.named)
            .with_term(token.text)
            .with_language(language)
            .with_span(token.span)
            .with_flags(token.flags),
    );
    if token.flags.is_extra() {
        network.attach_trivia(
            document,
            source,
            token.span,
            configuration.trivia_attachment_policy(),
        );
    }
}

fn scan<'a>(text: &'a str, language: FormalLanguage, lines: &LineIndex) -> Vec<Token<'a>> {
    let mut result = Vec::new();
    let mut offset = 0;
    while offset < text.len() {
        let start = offset;
        let character = character_at(text, offset);
        let mut flags = LinkFlags::clean();
        let (end, term, named) = if character.is_whitespace() {
            flags = flags.with_extra();
            (
                consume_while(text, offset, char::is_whitespace),
                "whitespace",
                false,
            )
        } else if language == FormalLanguage::Lean && text[offset..].starts_with("--") {
            flags = flags.with_extra();
            (consume_line_comment(text, offset), "comment", true)
        } else if let Some((open, close)) = block_comment(text, offset, language) {
            let (end, closed) = consume_nested_comment(text, offset, open, close);
            flags = flags.with_extra();
            if !closed {
                flags = flags.with_error();
            }
            (end, "comment", true)
        } else if character == '"' || (language == FormalLanguage::Lean && character == '\'') {
            let (end, closed) = consume_quoted(text, offset, character);
            if !closed {
                flags = flags.with_error();
            }
            (
                end,
                if character == '"' {
                    "string"
                } else {
                    "char_literal"
                },
                true,
            )
        } else if is_identifier_start(character) {
            let end = consume_while(text, offset, is_identifier_continue);
            let value = &text[offset..end];
            (
                end,
                if language.is_keyword(value) {
                    "keyword"
                } else if language.is_builtin_type(value) {
                    "primitive_type"
                } else {
                    "identifier"
                },
                true,
            )
        } else if character.is_ascii_digit() {
            (
                consume_while(text, offset, |value| {
                    value.is_ascii_alphanumeric() || matches!(value, '_' | '.')
                }),
                "number",
                true,
            )
        } else {
            let end = offset + character.len_utf8();
            let term = if is_open(character) {
                "open_delimiter"
            } else if is_close(character) {
                "close_delimiter"
            } else if character.is_control() {
                flags = flags.with_error();
                "unknown"
            } else {
                "operator"
            };
            (
                end,
                term,
                !matches!(term, "open_delimiter" | "close_delimiter" | "operator"),
            )
        };

        result.push(Token {
            text: &text[start..end],
            term,
            span: span(lines, start, end),
            flags,
            named,
        });
        offset = end;
    }
    result
}

fn block_comment(
    text: &str,
    offset: usize,
    language: FormalLanguage,
) -> Option<(&'static str, &'static str)> {
    match language {
        FormalLanguage::Lean if text[offset..].starts_with("/-") => Some(("/-", "-/")),
        FormalLanguage::Rocq if text[offset..].starts_with("(*") => Some(("(*", "*)")),
        _ => None,
    }
}

fn consume_nested_comment(text: &str, offset: usize, open: &str, close: &str) -> (usize, bool) {
    let mut cursor = offset + open.len();
    let mut depth = 1;
    while cursor < text.len() {
        if text[cursor..].starts_with(open) {
            depth += 1;
            cursor += open.len();
        } else if text[cursor..].starts_with(close) {
            depth -= 1;
            cursor += close.len();
            if depth == 0 {
                return (cursor, true);
            }
        } else {
            cursor += character_at(text, cursor).len_utf8();
        }
    }
    (text.len(), false)
}

fn consume_line_comment(text: &str, offset: usize) -> usize {
    text[offset..]
        .find('\n')
        .map_or(text.len(), |relative| offset + relative)
}

fn consume_quoted(text: &str, offset: usize, quote: char) -> (usize, bool) {
    let mut cursor = offset + quote.len_utf8();
    let mut escaped = false;
    while cursor < text.len() {
        let character = character_at(text, cursor);
        cursor += character.len_utf8();
        if escaped {
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else if character == quote {
            return (cursor, true);
        } else if character == '\n' {
            return (cursor, false);
        }
    }
    (text.len(), false)
}

fn consume_while(text: &str, offset: usize, predicate: impl Fn(char) -> bool) -> usize {
    let mut cursor = offset;
    while cursor < text.len() {
        let character = character_at(text, cursor);
        if !predicate(character) {
            break;
        }
        cursor += character.len_utf8();
    }
    cursor
}

fn span(lines: &LineIndex, start: usize, end: usize) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(start, end),
        lines.byte_point(start),
        lines.byte_point(end),
    )
}

fn opening_delimiter(text: &str) -> Option<char> {
    let character = text.chars().next()?;
    (text.len() == character.len_utf8() && is_open(character)).then_some(character)
}

fn closing_delimiter(text: &str) -> Option<char> {
    let character = text.chars().next()?;
    (text.len() == character.len_utf8() && is_close(character)).then_some(character)
}

const fn is_open(character: char) -> bool {
    matches!(character, '(' | '[' | '{')
}

const fn is_close(character: char) -> bool {
    matches!(character, ')' | ']' | '}')
}

const fn closes(open: char, close: char) -> bool {
    matches!((open, close), ('(', ')') | ('[', ']') | ('{', '}'))
}

const fn group_term(delimiter: char) -> &'static str {
    match delimiter {
        '(' => "parenthesized_expression",
        '[' => "bracketed_expression",
        '{' => "block",
        _ => "group",
    }
}

fn is_identifier_start(character: char) -> bool {
    character == '_' || character.is_alphabetic()
}

fn is_identifier_continue(character: char) -> bool {
    character == '_' || character == '\'' || character.is_alphanumeric()
}

fn character_at(text: &str, offset: usize) -> char {
    text[offset..]
        .chars()
        .next()
        .expect("offset inside source is a character boundary")
}
