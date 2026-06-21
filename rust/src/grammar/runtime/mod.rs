//! Runtime parser for first-class grammar values.
//!
//! [`GrammarParser`] interprets the grammar IR directly. It uses PEG-style
//! ordered choice, longest-local-match determinisation for unordered choice,
//! greedy repetition, and a same-rule/same-position guard so left-recursive
//! grammars fail closed instead of looping.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::grammar::inference::eval::MembershipOracle;
use crate::grammar::{CharClassItem, Grammar, GrammarExpr};
use crate::{
    ByteRange, LanguageParser, LinkFlags, LinkId, LinkMetadata, LinkNetwork, LinkType,
    ParseConfiguration, ParserRegistry, Point, SourceSpan,
};

const EXPR_EMPTY: &str = "grammar::runtime::expr::empty";
const EXPR_TERMINAL: &str = "grammar::runtime::expr::terminal";
const EXPR_TERMINAL_INSENSITIVE: &str = "grammar::runtime::expr::terminal-insensitive";
const EXPR_CHAR_RANGE: &str = "grammar::runtime::expr::char-range";
const EXPR_CHAR_CLASS: &str = "grammar::runtime::expr::char-class";
const EXPR_ANY_CHAR: &str = "grammar::runtime::expr::any-char";
const EXPR_CHOICE: &str = "grammar::runtime::expr::choice";
const EXPR_SEQUENCE: &str = "grammar::runtime::expr::sequence";
const EXPR_OPTIONAL: &str = "grammar::runtime::expr::optional";
const EXPR_ZERO_OR_MORE: &str = "grammar::runtime::expr::zero-or-more";
const EXPR_ONE_OR_MORE: &str = "grammar::runtime::expr::one-or-more";
const EXPR_REPEAT: &str = "grammar::runtime::expr::repeat";
const EXPR_AND: &str = "grammar::runtime::expr::and";
const EXPR_NOT: &str = "grammar::runtime::expr::not";

/// A [`LanguageParser`] that interprets a [`Grammar`] at runtime.
///
/// The parser is intentionally in-process: it does not generate source code.
/// It accepts input only when the start rule consumes the complete string.
/// Failed or partial parses return `false` from [`accepts`](Self::accepts) and
/// fall back to [`LinkNetwork::parse_lossless_text`] through the
/// [`LanguageParser`] implementation.
#[derive(Clone, Debug)]
pub struct GrammarParser {
    grammar: Grammar,
    diagnostics: Vec<String>,
}

impl GrammarParser {
    /// Builds a runtime parser for `grammar`.
    ///
    /// Construction records basic grammar diagnostics, including missing start
    /// rules and undefined non-terminal references. Diagnostic grammars fail
    /// closed at parse time instead of panicking.
    #[must_use]
    pub fn new(grammar: Grammar) -> Self {
        let diagnostics = validate_grammar(&grammar);
        Self {
            grammar,
            diagnostics,
        }
    }

    /// Wrapped grammar.
    #[must_use]
    pub const fn grammar(&self) -> &Grammar {
        &self.grammar
    }

    /// Construction-time diagnostics for malformed grammar references.
    #[must_use]
    pub fn diagnostics(&self) -> &[String] {
        &self.diagnostics
    }

    /// Membership query: does the grammar accept all of `text`?
    ///
    /// This is the oracle surface consumed by grammar-inference evaluation and
    /// active-learning callers.
    #[must_use]
    pub fn accepts(&self, text: &str) -> bool {
        self.parse_full(text).is_some()
    }

    fn parse_full(&self, text: &str) -> Option<ParseNode> {
        if !self.diagnostics.is_empty() {
            return None;
        }

        RuntimeMatcher::new(&self.grammar, text)
            .parse_full()
            .ok()
            .flatten()
    }

    fn try_parse_network(
        &self,
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> Option<LinkNetwork> {
        let tree = self.parse_full(text)?;
        let (mut network, document) = LinkNetwork::new_parse_document(text, language);
        let context = EmitContext {
            text,
            language,
            configuration,
        };
        emit_parse_node(&mut network, document, &tree, context);
        network.attach_embedded_regions(document, text, language, configuration);
        Some(network)
    }
}

impl LanguageParser for GrammarParser {
    fn parse_source(
        &self,
        text: &str,
        language: &str,
        configuration: ParseConfiguration,
    ) -> LinkNetwork {
        self.try_parse_network(text, language, configuration)
            .unwrap_or_else(|| LinkNetwork::parse_lossless_text(text, language, configuration))
    }
}

impl MembershipOracle for GrammarParser {
    fn accepts(&self, text: &str) -> bool {
        Self::accepts(self, text)
    }
}

/// Register `grammar` under `key`, shadowing the built-in dispatch for that key.
pub fn register_grammar(
    registry: &mut ParserRegistry,
    key: impl Into<String>,
    grammar: Grammar,
) -> &mut ParserRegistry {
    registry.register(key, Arc::new(GrammarParser::new(grammar)))
}

/// Builder-style variant mirroring [`ParserRegistry::with_parser`].
#[must_use]
pub fn with_grammar(
    registry: ParserRegistry,
    key: impl Into<String>,
    grammar: Grammar,
) -> ParserRegistry {
    registry.with_parser(key, Arc::new(GrammarParser::new(grammar)))
}

fn validate_grammar(grammar: &Grammar) -> Vec<String> {
    let mut diagnostics = Vec::new();
    if grammar.rules().is_empty() {
        diagnostics.push("grammar has no start rule".to_string());
    }
    if let Some(start) = grammar.start() {
        if grammar.rule(start).is_none() {
            diagnostics.push(format!("start rule `{start}` is not defined"));
        }
    }
    diagnostics.extend(
        grammar
            .undefined_nonterminals()
            .into_iter()
            .map(|rule| format!("undefined non-terminal `{rule}`")),
    );
    diagnostics
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct RuntimeMatcher<'grammar, 'text> {
    grammar: &'grammar Grammar,
    text: &'text str,
    memo: HashMap<(String, usize), Option<ParseNode>>,
    active: HashSet<(String, usize)>,
}

impl<'grammar, 'text> RuntimeMatcher<'grammar, 'text> {
    fn new(grammar: &'grammar Grammar, text: &'text str) -> Self {
        Self {
            grammar,
            text,
            memo: HashMap::new(),
            active: HashSet::new(),
        }
    }

    fn parse_full(&mut self) -> Result<Option<ParseNode>, MatchError> {
        let Some(start) = self.grammar.start_rule() else {
            return Ok(None);
        };
        Ok(self
            .match_rule(start.name(), 0)?
            .filter(|node| node.end == self.text.len()))
    }

    fn match_rule(&mut self, name: &str, position: usize) -> Result<Option<ParseNode>, MatchError> {
        if !self.valid_position(position) {
            return Ok(None);
        }

        let key = (name.to_string(), position);
        if let Some(cached) = self.memo.get(&key) {
            return Ok(cached.clone());
        }
        if self.active.contains(&key) {
            return Err(MatchError::LeftRecursive);
        }

        let Some(rule) = self.grammar.rule(name) else {
            return Ok(None);
        };

        self.active.insert(key.clone());
        let result = self.match_expr(rule.expr(), position);
        self.active.remove(&key);

        let child = result?;
        let node = child
            .map(|child| ParseNode::structural(rule_term(name), position, child.end, vec![child]));
        self.memo.insert(key, node.clone());
        Ok(node)
    }

    fn match_expr(
        &mut self,
        expr: &GrammarExpr,
        position: usize,
    ) -> Result<Option<ParseNode>, MatchError> {
        if !self.valid_position(position) {
            return Ok(None);
        }

        match expr {
            GrammarExpr::Empty => Ok(Some(ParseNode::structural(
                EXPR_EMPTY,
                position,
                position,
                Vec::new(),
            ))),
            GrammarExpr::Terminal(value) => {
                Ok(self.match_terminal(EXPR_TERMINAL, value, position, false))
            }
            GrammarExpr::TerminalInsensitive(value) => {
                Ok(self.match_terminal(EXPR_TERMINAL_INSENSITIVE, value, position, true))
            }
            GrammarExpr::CharRange(start, end) => Ok(self
                .char_at(position)
                .filter(|(value, _next)| start <= value && value <= end)
                .map(|(_value, next)| ParseNode::token(EXPR_CHAR_RANGE, position, next))),
            GrammarExpr::CharClass { negated, items } => Ok(self
                .char_at(position)
                .filter(|(value, _next)| class_accepts(*value, *negated, items))
                .map(|(_value, next)| ParseNode::token(EXPR_CHAR_CLASS, position, next))),
            GrammarExpr::AnyChar => Ok(self
                .char_at(position)
                .map(|(_value, next)| ParseNode::token(EXPR_ANY_CHAR, position, next))),
            GrammarExpr::NonTerminal(name) => {
                let child = self.match_rule(name, position)?;
                Ok(child.map(|child| {
                    ParseNode::structural(non_terminal_term(name), position, child.end, vec![child])
                }))
            }
            GrammarExpr::Choice {
                ordered,
                alternatives,
            } => self.match_choice(*ordered, alternatives, position),
            GrammarExpr::Sequence(items) => self.match_sequence(items, position),
            GrammarExpr::Optional(inner) => {
                if let Some(child) = self.match_expr(inner, position)? {
                    Ok(Some(ParseNode::structural(
                        EXPR_OPTIONAL,
                        position,
                        child.end,
                        vec![child],
                    )))
                } else {
                    Ok(Some(ParseNode::structural(
                        EXPR_OPTIONAL,
                        position,
                        position,
                        Vec::new(),
                    )))
                }
            }
            GrammarExpr::ZeroOrMore(inner) => {
                self.match_repetition(EXPR_ZERO_OR_MORE, inner, position, 0, None)
            }
            GrammarExpr::OneOrMore(inner) => {
                self.match_repetition(EXPR_ONE_OR_MORE, inner, position, 1, None)
            }
            GrammarExpr::Repeat { expr, min, max } => {
                self.match_repetition(EXPR_REPEAT, expr, position, *min, *max)
            }
            GrammarExpr::And(inner) => {
                if self.match_expr(inner, position)?.is_some() {
                    Ok(Some(ParseNode::structural(
                        EXPR_AND,
                        position,
                        position,
                        Vec::new(),
                    )))
                } else {
                    Ok(None)
                }
            }
            GrammarExpr::Not(inner) => {
                if self.match_expr(inner, position)?.is_none() {
                    Ok(Some(ParseNode::structural(
                        EXPR_NOT,
                        position,
                        position,
                        Vec::new(),
                    )))
                } else {
                    Ok(None)
                }
            }
            GrammarExpr::Capture { label, expr } => {
                let child = self.match_expr(expr, position)?;
                Ok(child.map(|child| {
                    ParseNode::structural(
                        capture_term(label.as_deref()),
                        position,
                        child.end,
                        vec![child],
                    )
                }))
            }
        }
    }

    fn match_choice(
        &mut self,
        ordered: bool,
        alternatives: &[GrammarExpr],
        position: usize,
    ) -> Result<Option<ParseNode>, MatchError> {
        if ordered {
            for alternative in alternatives {
                if let Some(child) = self.match_expr(alternative, position)? {
                    return Ok(Some(ParseNode::structural(
                        EXPR_CHOICE,
                        position,
                        child.end,
                        vec![child],
                    )));
                }
            }
            return Ok(None);
        }

        let mut best: Option<ParseNode> = None;
        for alternative in alternatives {
            let candidate = self.match_expr(alternative, position)?;
            let replace_best = candidate
                .as_ref()
                .is_some_and(|node| best.as_ref().map_or(true, |best| node.end > best.end));
            if replace_best {
                best = candidate;
            }
        }

        Ok(best.map(|child| ParseNode::structural(EXPR_CHOICE, position, child.end, vec![child])))
    }

    fn match_sequence(
        &mut self,
        items: &[GrammarExpr],
        position: usize,
    ) -> Result<Option<ParseNode>, MatchError> {
        let mut current = position;
        let mut children = Vec::with_capacity(items.len());
        for item in items {
            let Some(child) = self.match_expr(item, current)? else {
                return Ok(None);
            };
            current = child.end;
            children.push(child);
        }

        Ok(Some(ParseNode::structural(
            EXPR_SEQUENCE,
            position,
            current,
            children,
        )))
    }

    fn match_repetition(
        &mut self,
        term: &'static str,
        inner: &GrammarExpr,
        position: usize,
        min: usize,
        max: Option<usize>,
    ) -> Result<Option<ParseNode>, MatchError> {
        if max.is_some_and(|max| max < min) {
            return Ok(None);
        }

        let mut current = position;
        let mut count = 0;
        let mut children = Vec::new();
        loop {
            if max.is_some_and(|max| count >= max) {
                break;
            }

            let Some(child) = self.match_expr(inner, current)? else {
                break;
            };

            if child.end == current {
                if count < min {
                    count = min;
                    children.push(child);
                }
                break;
            }

            current = child.end;
            count += 1;
            children.push(child);
        }

        if count >= min {
            Ok(Some(ParseNode::structural(
                term, position, current, children,
            )))
        } else {
            Ok(None)
        }
    }

    fn match_terminal(
        &self,
        term: &'static str,
        value: &str,
        position: usize,
        insensitive: bool,
    ) -> Option<ParseNode> {
        let matches = if insensitive {
            starts_with_ascii_insensitive(&self.text[position..], value)
        } else {
            self.text[position..].starts_with(value)
        };
        matches.then(|| ParseNode::token(term, position, position + value.len()))
    }

    fn char_at(&self, position: usize) -> Option<(char, usize)> {
        self.text[position..]
            .chars()
            .next()
            .map(|value| (value, position + value.len_utf8()))
    }

    fn valid_position(&self, position: usize) -> bool {
        position <= self.text.len() && self.text.is_char_boundary(position)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum MatchError {
    LeftRecursive,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ParseNode {
    term: String,
    start: usize,
    end: usize,
    children: Vec<Self>,
    token: bool,
}

impl ParseNode {
    fn structural(term: impl Into<String>, start: usize, end: usize, children: Vec<Self>) -> Self {
        Self {
            term: term.into(),
            start,
            end,
            children,
            token: false,
        }
    }

    fn token(term: impl Into<String>, start: usize, end: usize) -> Self {
        Self {
            term: term.into(),
            start,
            end,
            children: Vec::new(),
            token: true,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct EmitContext<'source> {
    text: &'source str,
    language: &'source str,
    configuration: ParseConfiguration,
}

fn emit_parse_node(
    network: &mut LinkNetwork,
    owner: LinkId,
    node: &ParseNode,
    context: EmitContext<'_>,
) -> LinkId {
    let span = span_for_range(context.text, node.start, node.end);
    let node_id = network.insert_link(
        [owner],
        LinkMetadata::new()
            .with_link_type(LinkType::Grammar)
            .with_named(true)
            .with_term(&node.term)
            .with_language(context.language)
            .with_span(span)
            .with_flags(LinkFlags::clean()),
    );

    for child in &node.children {
        emit_parse_node(network, node_id, child, context);
    }

    if node.token && node.start < node.end {
        emit_token(network, node_id, node.start, node.end, context);
    }

    node_id
}

fn emit_token(
    network: &mut LinkNetwork,
    owner: LinkId,
    start: usize,
    end: usize,
    context: EmitContext<'_>,
) -> LinkId {
    let text = &context.text[start..end];
    let span = span_for_range(context.text, start, end);
    let flags = if text.chars().all(char::is_whitespace) {
        LinkFlags::extra()
    } else {
        LinkFlags::clean()
    };
    let token = network.insert_link(
        [owner],
        LinkMetadata::new()
            .with_link_type(LinkType::Token)
            .with_named(!text.chars().all(char::is_whitespace))
            .with_term(text)
            .with_language(context.language)
            .with_span(span)
            .with_flags(flags),
    );

    if flags.is_extra() {
        network.attach_trivia(
            owner,
            token,
            span,
            context.configuration.trivia_attachment_policy(),
        );
    }

    token
}

fn span_for_range(text: &str, start: usize, end: usize) -> SourceSpan {
    SourceSpan::new(
        ByteRange::new(start, end),
        point_at_byte(text, start),
        point_at_byte(text, end),
    )
}

fn point_at_byte(text: &str, byte: usize) -> Point {
    let mut row = 0;
    let mut line_start = 0;
    for (index, value) in text.bytes().enumerate().take(byte) {
        if value == b'\n' {
            row += 1;
            line_start = index + 1;
        }
    }
    Point::new(row, byte - line_start)
}

fn starts_with_ascii_insensitive(text: &str, value: &str) -> bool {
    text.get(..value.len())
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case(value))
}

fn class_accepts(value: char, negated: bool, items: &[CharClassItem]) -> bool {
    let contains = items.iter().any(|item| match item {
        CharClassItem::Char(item) => *item == value,
        CharClassItem::Range(start, end) => *start <= value && value <= *end,
    });
    contains != negated
}

fn rule_term(name: &str) -> String {
    format!("grammar::runtime::rule::{name}")
}

fn non_terminal_term(name: &str) -> String {
    format!("grammar::runtime::expr::non-terminal::{name}")
}

fn capture_term(label: Option<&str>) -> String {
    label.map_or_else(
        || "grammar::runtime::capture".to_string(),
        |label| format!("grammar::runtime::capture::{label}"),
    )
}
