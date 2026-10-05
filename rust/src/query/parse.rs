//! The parser of the tree-sitter query language the [`LinkQuery`] patterns are
//! written in.

use super::{
    QueryChildExpression, QueryChildPattern, QueryExpression, QueryNodeKind, QueryNodePattern,
    QueryParseError, QueryPattern, QueryPredicate, QueryPredicateArgument, QueryQuantifier,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum QueryToken {
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

pub(super) struct QueryParser {
    tokens: Vec<QueryToken>,
    position: usize,
}

impl QueryParser {
    pub(super) const fn new(tokens: Vec<QueryToken>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub(super) fn parse(&mut self) -> Result<(QueryPattern, Vec<QueryPredicate>), QueryParseError> {
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

    const fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
}

pub(super) fn tokenize(source: &str) -> Result<Vec<QueryToken>, QueryParseError> {
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

pub(super) fn push_single(
    tokens: &mut Vec<QueryToken>,
    characters: &mut std::iter::Peekable<std::str::Chars<'_>>,
    token: QueryToken,
) {
    characters.next();
    tokens.push(token);
}

pub(super) fn read_atom(characters: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
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

pub(super) fn read_string(
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
