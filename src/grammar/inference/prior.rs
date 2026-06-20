//! Delimiter-skeleton structural prior for positive grammar inference.

use std::collections::BTreeSet;

use super::lexical::{categorise, CharCategory};
use crate::{LinkNetwork, ParseConfiguration};

const PRIOR_LANGUAGE: &str = "grammar-prior";

/// Half-open byte span `[start, end)` into the owning example string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ByteSpan {
    /// First byte in the span.
    pub start: usize,
    /// Byte immediately after the span.
    pub end: usize,
}

impl ByteSpan {
    /// Creates a byte span.
    ///
    /// # Panics
    ///
    /// Panics when `start` is greater than `end`.
    #[must_use]
    pub const fn new(start: usize, end: usize) -> Self {
        assert!(start <= end, "byte span start must not exceed end");
        Self { start, end }
    }

    /// Returns `true` when the span contains no bytes.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

/// The kind of an opaque terminal leaf.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LeafKind {
    /// Unquoted text.
    Text,
    /// Single-quoted text kept opaque.
    SingleQuote,
    /// Double-quoted text kept opaque.
    DoubleQuote,
    /// Backtick-quoted text kept opaque.
    Backtick,
}

/// Delimiter family for a seed-tree group.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Delimiter {
    /// Parenthesized `(...)` group.
    Paren,
    /// Curly-braced `{...}` group.
    Curly,
    /// Square-bracketed `[...]` group.
    Square,
    /// Synthetic root wrapping one whole example.
    Root,
}

/// One node of a seed parse tree over a single example string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SeedNode {
    /// An opaque terminal slice.
    Leaf {
        /// Span of the leaf in the owning example.
        span: ByteSpan,
        /// Leaf classification.
        kind: LeafKind,
    },
    /// A bracketed or synthetic grouped constituent.
    Group {
        /// Delimiter family that produced this group.
        delimiter: Delimiter,
        /// Ordered child nodes.
        children: Vec<Self>,
        /// Span of the group in the owning example.
        span: ByteSpan,
    },
}

/// One positive example paired with its seed tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SeedTree {
    /// Original example string.
    pub example: String,
    /// Synthetic root group for the example.
    pub root: SeedNode,
}

/// A batch of seed trees plus the shared terminal alphabet.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StructuralPrior {
    /// One seed tree per input example, preserving input order.
    pub trees: Vec<SeedTree>,
    /// Distinct terminal slices observed across all leaves, sorted lexicographically.
    pub alphabet: Vec<String>,
}

/// Whitespace policy for text leaves.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WhitespacePolicy {
    /// Strip ASCII whitespace around text runs and split internal runs into fine leaves.
    #[default]
    Trim,
    /// Keep each text run as one verbatim leaf.
    Keep,
}

/// Configuration for structural-prior construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PriorOptions {
    /// Merge each text run into a single leaf after whitespace handling.
    pub coalesce_runs: bool,
    /// Whitespace handling for text runs.
    pub whitespace: WhitespacePolicy,
}

impl Default for PriorOptions {
    fn default() -> Self {
        Self {
            coalesce_runs: false,
            whitespace: WhitespacePolicy::Trim,
        }
    }
}

/// Builds the delimiter-skeleton structural prior for positive examples.
#[must_use]
pub fn build_structural_prior(examples: &[String], opts: PriorOptions) -> StructuralPrior {
    let mut alphabet = BTreeSet::new();
    let trees = examples
        .iter()
        .map(|example| {
            let root = build_seed_root(example, opts);
            collect_alphabet(&root, example, &mut alphabet);
            SeedTree {
                example: example.clone(),
                root,
            }
        })
        .collect();

    StructuralPrior {
        trees,
        alphabet: alphabet.into_iter().collect(),
    }
}

fn build_seed_root(example: &str, opts: PriorOptions) -> SeedNode {
    let _skeleton = skeletonise(example);
    let children = DelimiterParser::new(example, opts)
        .parse()
        .unwrap_or_else(|()| flat_leaves(example, opts));

    SeedNode::Group {
        delimiter: Delimiter::Root,
        children,
        span: ByteSpan::new(0, example.len()),
    }
}

fn skeletonise(text: &str) -> LinkNetwork {
    LinkNetwork::parse(text, PRIOR_LANGUAGE, ParseConfiguration::default())
}

struct DelimiterParser<'input> {
    text: &'input str,
    opts: PriorOptions,
    cursor: usize,
}

impl<'input> DelimiterParser<'input> {
    const fn new(text: &'input str, opts: PriorOptions) -> Self {
        Self {
            text,
            opts,
            cursor: 0,
        }
    }

    fn parse(mut self) -> Result<Vec<SeedNode>, ()> {
        self.parse_children_until(None)
    }

    fn parse_children_until(&mut self, closing: Option<char>) -> Result<Vec<SeedNode>, ()> {
        let mut children = Vec::new();
        while self.cursor < self.text.len() {
            let character = self.current_char().expect("cursor is inside text");
            if Some(character) == closing {
                self.advance_char();
                return Ok(children);
            }

            match character {
                '(' => children.push(self.parse_group(Delimiter::Paren, ')')?),
                '{' => children.push(self.parse_group(Delimiter::Curly, '}')?),
                '[' => children.push(self.parse_group(Delimiter::Square, ']')?),
                ')' | '}' | ']' => return Err(()),
                '\'' | '"' | '`' => children.push(self.parse_quoted(character)?),
                _ => children.extend(self.parse_text_run()),
            }
        }

        if closing.is_some() {
            Err(())
        } else {
            Ok(children)
        }
    }

    fn parse_group(&mut self, delimiter: Delimiter, closing: char) -> Result<SeedNode, ()> {
        let start = self.cursor;
        self.advance_char();
        let children = self.parse_children_until(Some(closing))?;

        Ok(SeedNode::Group {
            delimiter,
            children,
            span: ByteSpan::new(start, self.cursor),
        })
    }

    fn parse_quoted(&mut self, quote: char) -> Result<SeedNode, ()> {
        let start = self.cursor;
        let end = quoted_end(self.text, start, quote).ok_or(())?;
        self.cursor = end;

        Ok(SeedNode::Leaf {
            span: ByteSpan::new(start, end),
            kind: quote_kind(quote),
        })
    }

    fn parse_text_run(&mut self) -> Vec<SeedNode> {
        let start = self.cursor;
        while self.cursor < self.text.len() {
            let character = self.current_char().expect("cursor is inside text");
            if is_structural_delimiter(character) || is_quote(character) {
                break;
            }
            self.advance_char();
        }
        text_leaves(self.text, start, self.cursor, self.opts)
    }

    fn current_char(&self) -> Option<char> {
        self.text[self.cursor..].chars().next()
    }

    fn advance_char(&mut self) {
        self.cursor += self
            .current_char()
            .expect("cursor is inside text")
            .len_utf8();
    }
}

fn flat_leaves(text: &str, opts: PriorOptions) -> Vec<SeedNode> {
    let mut leaves = Vec::new();
    let mut cursor = 0;
    let mut text_start = 0;

    while cursor < text.len() {
        let character = text[cursor..]
            .chars()
            .next()
            .expect("cursor is inside text");
        if is_quote(character) {
            if let Some(end) = quoted_end(text, cursor, character) {
                leaves.extend(text_leaves(text, text_start, cursor, opts));
                leaves.push(SeedNode::Leaf {
                    span: ByteSpan::new(cursor, end),
                    kind: quote_kind(character),
                });
                cursor = end;
                text_start = cursor;
                continue;
            }
        }

        cursor += character.len_utf8();
    }

    leaves.extend(text_leaves(text, text_start, text.len(), opts));
    leaves
}

fn text_leaves(text: &str, start: usize, end: usize, opts: PriorOptions) -> Vec<SeedNode> {
    if start == end {
        return Vec::new();
    }

    match opts.whitespace {
        WhitespacePolicy::Keep => text_leaf(start, end).into_iter().collect(),
        WhitespacePolicy::Trim if opts.coalesce_runs => trim_ascii_span(text, start, end)
            .and_then(|(trimmed_start, trimmed_end)| text_leaf(trimmed_start, trimmed_end))
            .into_iter()
            .collect(),
        WhitespacePolicy::Trim => split_trimmed_text(text, start, end),
    }
}

fn split_trimmed_text(text: &str, start: usize, end: usize) -> Vec<SeedNode> {
    let mut leaves = Vec::new();
    let mut cursor = start;

    while cursor < end {
        cursor = skip_ascii_whitespace(text, cursor, end);
        if cursor >= end {
            break;
        }

        let token_start = cursor;
        let token_category = current_category(text, cursor);
        cursor += current_char(text, cursor).len_utf8();

        if !is_atomic(token_category) {
            while cursor < end {
                let next = current_char(text, cursor);
                if next.is_ascii_whitespace() {
                    break;
                }

                let next_category = categorise(next);
                if continues_text_token(token_category, next_category) {
                    cursor += next.len_utf8();
                } else {
                    break;
                }
            }
        }

        leaves.push(SeedNode::Leaf {
            span: ByteSpan::new(token_start, cursor),
            kind: LeafKind::Text,
        });
    }

    leaves
}

fn trim_ascii_span(text: &str, start: usize, end: usize) -> Option<(usize, usize)> {
    let trimmed_start = skip_ascii_whitespace(text, start, end);
    let mut trimmed_end = end;

    while trimmed_start < trimmed_end {
        let character_start = previous_char_start(text, trimmed_start, trimmed_end);
        let character = current_char(text, character_start);
        if !character.is_ascii_whitespace() {
            break;
        }
        trimmed_end = character_start;
    }

    (trimmed_start < trimmed_end).then_some((trimmed_start, trimmed_end))
}

fn skip_ascii_whitespace(text: &str, mut cursor: usize, end: usize) -> usize {
    while cursor < end {
        let character = current_char(text, cursor);
        if !character.is_ascii_whitespace() {
            break;
        }
        cursor += character.len_utf8();
    }
    cursor
}

fn text_leaf(start: usize, end: usize) -> Option<SeedNode> {
    (start < end).then_some(SeedNode::Leaf {
        span: ByteSpan::new(start, end),
        kind: LeafKind::Text,
    })
}

fn quoted_end(text: &str, start: usize, quote: char) -> Option<usize> {
    let mut cursor = start + quote.len_utf8();
    while cursor < text.len() {
        let character = current_char(text, cursor);
        if character == '\\' {
            cursor += character.len_utf8();
            if cursor < text.len() {
                cursor += current_char(text, cursor).len_utf8();
            }
            continue;
        }

        if character == quote {
            let close_end = cursor + quote.len_utf8();
            if text[close_end..].starts_with(quote) {
                cursor = close_end + quote.len_utf8();
                continue;
            }
            return Some(close_end);
        }

        cursor += character.len_utf8();
    }

    None
}

fn collect_alphabet(node: &SeedNode, example: &str, alphabet: &mut BTreeSet<String>) {
    match node {
        SeedNode::Leaf { span, .. } => {
            alphabet.insert(example[span.start..span.end].to_string());
        }
        SeedNode::Group { children, .. } => {
            for child in children {
                collect_alphabet(child, example, alphabet);
            }
        }
    }
}

fn current_char(text: &str, cursor: usize) -> char {
    text[cursor..]
        .chars()
        .next()
        .expect("cursor is inside text")
}

fn current_category(text: &str, cursor: usize) -> CharCategory {
    categorise(current_char(text, cursor))
}

fn previous_char_start(text: &str, start: usize, end: usize) -> usize {
    text[start..end]
        .char_indices()
        .last()
        .map_or(start, |(offset, _)| start + offset)
}

fn continues_text_token(current: CharCategory, next: CharCategory) -> bool {
    if is_atomic(current) || is_atomic(next) {
        return false;
    }

    current == next || (current == CharCategory::Letter && next == CharCategory::Digit)
}

const fn is_atomic(category: CharCategory) -> bool {
    matches!(
        category,
        CharCategory::Delimiter | CharCategory::Punctuation
    )
}

const fn is_structural_delimiter(value: char) -> bool {
    matches!(value, '(' | ')' | '[' | ']' | '{' | '}')
}

const fn is_quote(value: char) -> bool {
    matches!(value, '\'' | '"' | '`')
}

const fn quote_kind(quote: char) -> LeafKind {
    match quote {
        '\'' => LeafKind::SingleQuote,
        '"' => LeafKind::DoubleQuote,
        '`' => LeafKind::Backtick,
        _ => unreachable!(),
    }
}
