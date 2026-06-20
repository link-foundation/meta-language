use std::fmt;

use crate::link_network::{Link, LinkId, LinkNetwork, LinkType};

/// Structural query over links.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LinkQuery {
    link_type: Option<LinkType>,
    term: Option<String>,
    language: Option<String>,
    named: Option<bool>,
    pattern: Option<QueryPattern>,
    pattern_source: Option<String>,
    predicates: Vec<QueryPredicate>,
}

impl LinkQuery {
    /// Creates an empty query that matches every link.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a query restricted to a link type.
    #[must_use]
    pub fn by_type(link_type: LinkType) -> Self {
        Self::new().with_link_type(link_type)
    }

    /// Parses a tree-sitter-query-like S-expression query.
    ///
    /// The structural engine binds captures and leaves predicate evaluation to
    /// a caller-provided [`QueryPredicateHost`].
    pub fn from_sexpression(source: &str) -> Result<Self, QueryParseError> {
        let tokens = tokenize(source)?;
        let mut parser = QueryParser::new(tokens);
        let (pattern, predicates) = parser.parse()?;
        Ok(Self {
            pattern: Some(pattern),
            pattern_source: Some(source.to_string()),
            predicates,
            ..Self::default()
        })
    }

    /// Restricts matches to a link type.
    #[must_use]
    pub const fn with_link_type(mut self, link_type: LinkType) -> Self {
        self.link_type = Some(link_type);
        self
    }

    /// Restricts matches to a term.
    #[must_use]
    pub fn with_term(mut self, term: impl Into<String>) -> Self {
        self.term = Some(term.into());
        self
    }

    /// Restricts matches to a language label.
    #[must_use]
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Restricts matches by the named flag.
    #[must_use]
    pub const fn with_named(mut self, named: bool) -> Self {
        self.named = Some(named);
        self
    }

    pub(crate) fn matches_in_network(
        &self,
        network: &LinkNetwork,
        link: &Link,
        predicate_host: &impl QueryPredicateHost,
    ) -> Vec<QueryMatch> {
        if !self.matches_metadata(link) {
            return Vec::new();
        }

        let captures = self.pattern.as_ref().map_or_else(
            || vec![QueryCaptures::default()],
            |pattern| pattern.match_root(network, link.id()),
        );

        captures
            .into_iter()
            .filter(|captures| {
                self.predicates
                    .iter()
                    .all(|predicate| predicate_host.evaluate(predicate, captures, network))
            })
            .map(|captures| QueryMatch::new(link.id(), captures))
            .collect()
    }

    fn matches_metadata(&self, link: &Link) -> bool {
        let metadata = link.metadata();
        self.link_type
            .map_or(true, |link_type| metadata.link_type() == Some(link_type))
            && self
                .term
                .as_deref()
                .map_or(true, |term| metadata.term() == Some(term))
            && self
                .language
                .as_deref()
                .map_or(true, |language| metadata.language() == Some(language))
            && self
                .named
                .map_or(true, |named| metadata.is_named() == named)
    }

    pub(crate) const fn link_type_filter(&self) -> Option<LinkType> {
        self.link_type
    }

    pub(crate) fn term_filter(&self) -> Option<&str> {
        self.term.as_deref()
    }

    pub(crate) fn language_filter(&self) -> Option<&str> {
        self.language.as_deref()
    }

    pub(crate) const fn named_filter(&self) -> Option<bool> {
        self.named
    }

    pub(crate) fn pattern_source(&self) -> Option<&str> {
        self.pattern_source.as_deref()
    }
}

/// A structural query match and its capture bindings.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryMatch {
    link_id: LinkId,
    captures: QueryCaptures,
}

impl QueryMatch {
    const fn new(link_id: LinkId, captures: QueryCaptures) -> Self {
        Self { link_id, captures }
    }

    /// Link selected by the query root pattern.
    #[must_use]
    pub const fn link_id(&self) -> LinkId {
        self.link_id
    }

    /// Captures bound while matching this result.
    #[must_use]
    pub const fn captures(&self) -> &QueryCaptures {
        &self.captures
    }
}

/// Ordered capture bindings for a query match.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct QueryCaptures {
    values: Vec<QueryCapture>,
}

impl QueryCaptures {
    fn with_capture(mut self, name: &str, link_id: LinkId) -> Self {
        self.values.push(QueryCapture {
            name: name.to_string(),
            link_id,
        });
        self
    }

    /// Returns the first link bound to a capture name.
    #[must_use]
    pub fn first(&self, name: &str) -> Option<LinkId> {
        self.values
            .iter()
            .find(|capture| capture.name == name)
            .map(|capture| capture.link_id)
    }

    /// Iterates capture bindings in match order.
    pub fn iter(&self) -> impl Iterator<Item = &QueryCapture> {
        self.values.iter()
    }
}

/// One capture binding.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryCapture {
    name: String,
    link_id: LinkId,
}

impl QueryCapture {
    /// Capture name without the leading `@`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Link bound to this capture.
    #[must_use]
    pub const fn link_id(&self) -> LinkId {
        self.link_id
    }
}

/// Host hook for evaluating text, regex, semantic, or other predicates.
pub trait QueryPredicateHost {
    /// Returns whether a predicate accepts the current capture set.
    fn evaluate(
        &self,
        predicate: &QueryPredicate,
        captures: &QueryCaptures,
        network: &LinkNetwork,
    ) -> bool;
}

pub(crate) struct RejectPredicateHost;

impl QueryPredicateHost for RejectPredicateHost {
    fn evaluate(
        &self,
        _predicate: &QueryPredicate,
        _captures: &QueryCaptures,
        _network: &LinkNetwork,
    ) -> bool {
        false
    }
}

/// Predicate expression parsed from an S-expression query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryPredicate {
    name: String,
    arguments: Vec<QueryPredicateArgument>,
}

impl QueryPredicate {
    /// Predicate name without the leading `#`.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Predicate arguments in source order.
    #[must_use]
    pub fn arguments(&self) -> &[QueryPredicateArgument] {
        &self.arguments
    }
}

/// Predicate argument parsed from an S-expression query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QueryPredicateArgument {
    /// Capture reference such as `@name`.
    Capture(String),
    /// Host literal such as `"main"` or an unquoted atom.
    Literal(String),
}

impl QueryPredicateArgument {
    /// Returns the referenced capture name, if this argument is a capture.
    #[must_use]
    pub fn capture_name(&self) -> Option<&str> {
        match self {
            Self::Capture(name) => Some(name),
            Self::Literal(_) => None,
        }
    }

    /// Returns the literal value, if this argument is a literal.
    #[must_use]
    pub fn literal(&self) -> Option<&str> {
        match self {
            Self::Capture(_) => None,
            Self::Literal(value) => Some(value),
        }
    }
}

/// Error returned while parsing an S-expression query.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueryParseError {
    message: String,
}

impl QueryParseError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for QueryParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for QueryParseError {}

#[derive(Clone, Debug, PartialEq, Eq)]
struct QueryPattern {
    root: QueryNodePattern,
    capture: Option<String>,
}

impl QueryPattern {
    fn match_root(&self, network: &LinkNetwork, link_id: LinkId) -> Vec<QueryCaptures> {
        let captures = self
            .root
            .matches(network, link_id, QueryCaptures::default());
        captures
            .into_iter()
            .map(|captures| {
                if let Some(capture) = &self.capture {
                    captures.with_capture(capture, link_id)
                } else {
                    captures
                }
            })
            .collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct QueryNodePattern {
    kind: QueryNodeKind,
    children: Vec<QueryChildPattern>,
}

impl QueryNodePattern {
    fn matches(
        &self,
        network: &LinkNetwork,
        link_id: LinkId,
        captures: QueryCaptures,
    ) -> Vec<QueryCaptures> {
        let Some(link) = network.link(link_id) else {
            return Vec::new();
        };
        if !self.kind.matches(link) || self.has_negated_field(network, link_id) {
            return Vec::new();
        }

        let structural_children = structural_children(network, link_id);
        let context = MatchContext {
            network,
            parent: link_id,
            structural_children: &structural_children,
        };
        match_child_patterns(&context, &self.children, 0, false, captures)
            .into_iter()
            .map(|(_child_index, captures)| captures)
            .collect()
    }

    fn has_negated_field(&self, network: &LinkNetwork, link_id: LinkId) -> bool {
        self.children.iter().any(|child| {
            if let QueryChildPattern::NegatedField(label) = child {
                has_field(network, link_id, label)
            } else {
                false
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum QueryNodeKind {
    Exact(String),
    Wildcard,
}

impl QueryNodeKind {
    fn matches(&self, link: &Link) -> bool {
        match self {
            Self::Exact(kind) => link.metadata().term() == Some(kind.as_str()),
            Self::Wildcard => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum QueryChildPattern {
    Anchor,
    NegatedField(String),
    Pattern(QueryChildExpression),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct QueryChildExpression {
    field: Option<String>,
    expression: QueryExpression,
    capture: Option<String>,
    quantifier: QueryQuantifier,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum QueryExpression {
    Node(QueryNodePattern),
    Alternation(Vec<QueryNodePattern>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum QueryQuantifier {
    One,
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

struct MatchContext<'a> {
    network: &'a LinkNetwork,
    parent: LinkId,
    structural_children: &'a [LinkId],
}

fn match_child_patterns(
    context: &MatchContext<'_>,
    patterns: &[QueryChildPattern],
    child_index: usize,
    anchored: bool,
    captures: QueryCaptures,
) -> Vec<(usize, QueryCaptures)> {
    let Some((pattern, remaining)) = patterns.split_first() else {
        return vec![(child_index, captures)];
    };

    match pattern {
        QueryChildPattern::Anchor => {
            if remaining.is_empty() || remaining.iter().all(is_negated_field_pattern) {
                if child_index == context.structural_children.len() {
                    match_child_patterns(context, remaining, child_index, true, captures)
                } else {
                    Vec::new()
                }
            } else {
                match_child_patterns(context, remaining, child_index, true, captures)
            }
        }
        QueryChildPattern::NegatedField(_) => {
            match_child_patterns(context, remaining, child_index, anchored, captures)
        }
        QueryChildPattern::Pattern(expression) => match_quantified_expression(
            context,
            expression,
            remaining,
            child_index,
            anchored,
            &captures,
        ),
    }
}

const fn is_negated_field_pattern(pattern: &QueryChildPattern) -> bool {
    matches!(pattern, QueryChildPattern::NegatedField(_))
}

fn match_quantified_expression(
    context: &MatchContext<'_>,
    expression: &QueryChildExpression,
    remaining: &[QueryChildPattern],
    child_index: usize,
    anchored: bool,
    captures: &QueryCaptures,
) -> Vec<(usize, QueryCaptures)> {
    match expression.quantifier {
        QueryQuantifier::One => match_one_then_continue(
            context,
            expression,
            remaining,
            child_index,
            anchored,
            captures,
        ),
        QueryQuantifier::ZeroOrOne => {
            let mut results =
                match_child_patterns(context, remaining, child_index, anchored, captures.clone());
            results.extend(match_one_then_continue(
                context,
                expression,
                remaining,
                child_index,
                anchored,
                captures,
            ));
            results
        }
        QueryQuantifier::ZeroOrMore => {
            let mut results =
                match_child_patterns(context, remaining, child_index, anchored, captures.clone());
            results.extend(match_repeated_expression(
                context,
                expression,
                remaining,
                child_index,
                anchored,
                captures,
            ));
            results
        }
        QueryQuantifier::OneOrMore => match_repeated_expression(
            context,
            expression,
            remaining,
            child_index,
            anchored,
            captures,
        ),
    }
}

fn match_repeated_expression(
    context: &MatchContext<'_>,
    expression: &QueryChildExpression,
    remaining: &[QueryChildPattern],
    child_index: usize,
    anchored: bool,
    captures: &QueryCaptures,
) -> Vec<(usize, QueryCaptures)> {
    let mut results = Vec::new();
    for (next_index, next_captures) in
        match_expression_at_positions(context, expression, child_index, anchored, captures)
    {
        results.extend(match_child_patterns(
            context,
            remaining,
            next_index,
            false,
            next_captures.clone(),
        ));
        results.extend(match_repeated_expression(
            context,
            expression,
            remaining,
            next_index,
            true,
            &next_captures,
        ));
    }
    results
}

fn match_one_then_continue(
    context: &MatchContext<'_>,
    expression: &QueryChildExpression,
    remaining: &[QueryChildPattern],
    child_index: usize,
    anchored: bool,
    captures: &QueryCaptures,
) -> Vec<(usize, QueryCaptures)> {
    match_expression_at_positions(context, expression, child_index, anchored, captures)
        .into_iter()
        .flat_map(|(next_index, captures)| {
            match_child_patterns(context, remaining, next_index, false, captures)
        })
        .collect()
}

fn match_expression_at_positions(
    context: &MatchContext<'_>,
    expression: &QueryChildExpression,
    child_index: usize,
    anchored: bool,
    captures: &QueryCaptures,
) -> Vec<(usize, QueryCaptures)> {
    let positions: Box<dyn Iterator<Item = usize>> = if anchored {
        Box::new(std::iter::once(child_index))
    } else {
        Box::new(child_index..context.structural_children.len())
    };

    positions
        .filter(|position| *position < context.structural_children.len())
        .flat_map(|position| {
            let child = context.structural_children[position];
            match_expression(
                context.network,
                context.parent,
                child,
                expression,
                captures.clone(),
            )
            .into_iter()
            .map(move |captures| (position + 1, captures))
        })
        .collect()
}

fn match_expression(
    network: &LinkNetwork,
    parent: LinkId,
    child: LinkId,
    expression: &QueryChildExpression,
    captures: QueryCaptures,
) -> Vec<QueryCaptures> {
    if let Some(field) = &expression.field {
        let field_targets = field_targets(network, parent, field);
        if !field_targets.contains(&child) {
            return Vec::new();
        }
    }

    let matches = match &expression.expression {
        QueryExpression::Node(node) => node.matches(network, child, captures),
        QueryExpression::Alternation(alternatives) => alternatives
            .iter()
            .flat_map(|alternative| alternative.matches(network, child, captures.clone()))
            .collect(),
    };

    matches
        .into_iter()
        .map(|captures| {
            if let Some(capture) = &expression.capture {
                captures.with_capture(capture, child)
            } else {
                captures
            }
        })
        .collect()
}

fn structural_children(network: &LinkNetwork, parent: LinkId) -> Vec<LinkId> {
    network
        .links()
        .filter(|link| link.references().first().copied() == Some(parent))
        .filter(|link| {
            !matches!(
                link.metadata().link_type(),
                Some(LinkType::Field | LinkType::Trivia)
            )
        })
        .map(Link::id)
        .collect()
}

fn has_field(network: &LinkNetwork, parent: LinkId, label: &str) -> bool {
    !field_targets(network, parent, label).is_empty()
}

fn field_targets(network: &LinkNetwork, parent: LinkId, label: &str) -> Vec<LinkId> {
    network
        .links()
        .filter(|link| link.metadata().link_type() == Some(LinkType::Field))
        .filter_map(|link| {
            let references = link.references();
            let [field_parent, field_label, field_child] = references else {
                return None;
            };
            if *field_parent != parent {
                return None;
            }
            let label_link = network.link(*field_label)?;
            (label_link.metadata().term() == Some(label)).then_some(*field_child)
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum QueryToken {
    LParen,
    RParen,
    LBracket,
    RBracket,
    Colon,
    Dot,
    Bang,
    Question,
    Star,
    Plus,
    Ident(String),
    Capture(String),
    Literal(String),
}

struct QueryParser {
    tokens: Vec<QueryToken>,
    position: usize,
}

impl QueryParser {
    const fn new(tokens: Vec<QueryToken>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn parse(&mut self) -> Result<(QueryPattern, Vec<QueryPredicate>), QueryParseError> {
        let mut pattern = None;
        let mut predicates = Vec::new();

        while !self.is_at_end() {
            if self.next_is_predicate() {
                predicates.push(self.parse_predicate()?);
            } else if pattern.is_none() {
                let root = self.parse_node_pattern()?;
                let capture = self.parse_optional_capture();
                pattern = Some(QueryPattern { root, capture });
            } else {
                return Err(QueryParseError::new(
                    "query may contain one root pattern followed by predicates",
                ));
            }
        }

        Ok((
            pattern.ok_or_else(|| QueryParseError::new("query is missing a root pattern"))?,
            predicates,
        ))
    }

    fn next_is_predicate(&self) -> bool {
        matches!(
            (self.peek(), self.peek_next()),
            (
                Some(QueryToken::LParen),
                Some(QueryToken::Ident(identifier))
            ) if identifier.starts_with('#')
        )
    }

    fn parse_predicate(&mut self) -> Result<QueryPredicate, QueryParseError> {
        self.expect(&QueryToken::LParen)?;
        let name = match self.advance() {
            Some(QueryToken::Ident(name)) if name.starts_with('#') => {
                name.trim_start_matches('#').to_string()
            }
            _ => return Err(QueryParseError::new("predicate must start with #name")),
        };

        let mut arguments = Vec::new();
        while !matches!(self.peek(), Some(QueryToken::RParen)) {
            match self.advance() {
                Some(QueryToken::Capture(name)) => {
                    arguments.push(QueryPredicateArgument::Capture(name));
                }
                Some(QueryToken::Literal(value) | QueryToken::Ident(value)) => {
                    arguments.push(QueryPredicateArgument::Literal(value));
                }
                Some(_) => return Err(QueryParseError::new("invalid predicate argument")),
                None => return Err(QueryParseError::new("unterminated predicate")),
            }
        }
        self.expect(&QueryToken::RParen)?;

        Ok(QueryPredicate { name, arguments })
    }

    fn parse_node_pattern(&mut self) -> Result<QueryNodePattern, QueryParseError> {
        self.expect(&QueryToken::LParen)?;
        let kind = match self.advance() {
            Some(QueryToken::Ident(identifier)) if identifier == "_" => QueryNodeKind::Wildcard,
            Some(QueryToken::Ident(identifier)) => QueryNodeKind::Exact(identifier),
            _ => return Err(QueryParseError::new("node pattern is missing a kind")),
        };

        let mut children = Vec::new();
        while !matches!(self.peek(), Some(QueryToken::RParen)) {
            if self.is_at_end() {
                return Err(QueryParseError::new("unterminated node pattern"));
            }
            children.push(self.parse_child_pattern()?);
        }
        self.expect(&QueryToken::RParen)?;

        Ok(QueryNodePattern { kind, children })
    }

    fn parse_child_pattern(&mut self) -> Result<QueryChildPattern, QueryParseError> {
        match self.peek() {
            Some(QueryToken::Dot) => {
                self.advance();
                Ok(QueryChildPattern::Anchor)
            }
            Some(QueryToken::Bang) => {
                self.advance();
                let Some(QueryToken::Ident(label)) = self.advance() else {
                    return Err(QueryParseError::new("negated field is missing a label"));
                };
                Ok(QueryChildPattern::NegatedField(label))
            }
            _ => {
                let field = self.parse_optional_field()?;
                let expression = if matches!(self.peek(), Some(QueryToken::LBracket)) {
                    self.parse_alternation()?
                } else {
                    QueryExpression::Node(self.parse_node_pattern()?)
                };
                let (capture, quantifier) = self.parse_capture_and_quantifier();
                Ok(QueryChildPattern::Pattern(QueryChildExpression {
                    field,
                    expression,
                    capture,
                    quantifier,
                }))
            }
        }
    }

    fn parse_alternation(&mut self) -> Result<QueryExpression, QueryParseError> {
        self.expect(&QueryToken::LBracket)?;
        let mut alternatives = Vec::new();
        while !matches!(self.peek(), Some(QueryToken::RBracket)) {
            if self.is_at_end() {
                return Err(QueryParseError::new("unterminated alternation"));
            }
            alternatives.push(self.parse_node_pattern()?);
        }
        self.expect(&QueryToken::RBracket)?;
        if alternatives.is_empty() {
            return Err(QueryParseError::new("alternation must contain patterns"));
        }
        Ok(QueryExpression::Alternation(alternatives))
    }

    fn parse_optional_field(&mut self) -> Result<Option<String>, QueryParseError> {
        if !matches!(
            (self.peek(), self.peek_next()),
            (Some(QueryToken::Ident(_)), Some(QueryToken::Colon))
        ) {
            return Ok(None);
        }

        let Some(QueryToken::Ident(label)) = self.advance() else {
            return Err(QueryParseError::new("field is missing a label"));
        };
        self.expect(&QueryToken::Colon)?;
        Ok(Some(label))
    }

    fn parse_capture_and_quantifier(&mut self) -> (Option<String>, QueryQuantifier) {
        let mut capture = self.parse_optional_capture();
        let mut quantifier = self.parse_optional_quantifier();
        if capture.is_none() {
            capture = self.parse_optional_capture();
        }
        if quantifier == QueryQuantifier::One {
            quantifier = self.parse_optional_quantifier();
        }
        (capture, quantifier)
    }

    fn parse_optional_capture(&mut self) -> Option<String> {
        if let Some(QueryToken::Capture(name)) = self.peek().cloned() {
            self.advance();
            Some(name)
        } else {
            None
        }
    }

    fn parse_optional_quantifier(&mut self) -> QueryQuantifier {
        match self.peek() {
            Some(QueryToken::Question) => {
                self.advance();
                QueryQuantifier::ZeroOrOne
            }
            Some(QueryToken::Star) => {
                self.advance();
                QueryQuantifier::ZeroOrMore
            }
            Some(QueryToken::Plus) => {
                self.advance();
                QueryQuantifier::OneOrMore
            }
            _ => QueryQuantifier::One,
        }
    }

    fn expect(&mut self, expected: &QueryToken) -> Result<(), QueryParseError> {
        let Some(actual) = self.advance() else {
            return Err(QueryParseError::new("unexpected end of query"));
        };
        if std::mem::discriminant(&actual) == std::mem::discriminant(expected) {
            Ok(())
        } else {
            Err(QueryParseError::new("unexpected token in query"))
        }
    }

    fn advance(&mut self) -> Option<QueryToken> {
        let token = self.tokens.get(self.position).cloned()?;
        self.position += 1;
        Some(token)
    }

    fn peek(&self) -> Option<&QueryToken> {
        self.tokens.get(self.position)
    }

    fn peek_next(&self) -> Option<&QueryToken> {
        self.tokens.get(self.position + 1)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
}

fn tokenize(source: &str) -> Result<Vec<QueryToken>, QueryParseError> {
    let mut tokens = Vec::new();
    let mut characters = source.chars().peekable();

    while let Some(character) = characters.peek().copied() {
        match character {
            whitespace if whitespace.is_whitespace() => {
                characters.next();
            }
            '(' => push_single(&mut tokens, &mut characters, QueryToken::LParen),
            ')' => push_single(&mut tokens, &mut characters, QueryToken::RParen),
            '[' => push_single(&mut tokens, &mut characters, QueryToken::LBracket),
            ']' => push_single(&mut tokens, &mut characters, QueryToken::RBracket),
            ':' => push_single(&mut tokens, &mut characters, QueryToken::Colon),
            '.' => push_single(&mut tokens, &mut characters, QueryToken::Dot),
            '!' => push_single(&mut tokens, &mut characters, QueryToken::Bang),
            '?' => push_single(&mut tokens, &mut characters, QueryToken::Question),
            '*' => push_single(&mut tokens, &mut characters, QueryToken::Star),
            '+' => push_single(&mut tokens, &mut characters, QueryToken::Plus),
            '@' => {
                characters.next();
                tokens.push(QueryToken::Capture(read_atom(&mut characters)));
            }
            '"' => tokens.push(QueryToken::Literal(read_string(&mut characters)?)),
            _ => tokens.push(QueryToken::Ident(read_atom(&mut characters))),
        }
    }

    Ok(tokens)
}

fn push_single(
    tokens: &mut Vec<QueryToken>,
    characters: &mut std::iter::Peekable<std::str::Chars<'_>>,
    token: QueryToken,
) {
    characters.next();
    tokens.push(token);
}

fn read_atom(characters: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
    let mut atom = String::new();
    while let Some(character) = characters.peek().copied() {
        if character.is_whitespace()
            || matches!(character, '(' | ')' | '[' | ']' | ':' | '!' | '@' | '"')
        {
            break;
        }
        atom.push(character);
        characters.next();
    }
    atom
}

fn read_string(
    characters: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<String, QueryParseError> {
    let mut literal = String::new();
    characters.next();

    while let Some(character) = characters.next() {
        match character {
            '"' => return Ok(literal),
            '\\' => {
                let Some(escaped) = characters.next() else {
                    return Err(QueryParseError::new("unterminated string escape"));
                };
                literal.push(match escaped {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    other => other,
                });
            }
            other => literal.push(other),
        }
    }

    Err(QueryParseError::new("unterminated string literal"))
}
