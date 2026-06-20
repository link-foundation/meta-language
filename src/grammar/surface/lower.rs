use std::collections::BTreeSet;

use super::token::{Delimiter, QuoteKind, Token};
use super::{lowering_error, GrammarSurfaceError};
use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule};

pub(super) fn lower_document(tokens: &[Token]) -> Result<Grammar, GrammarSurfaceError> {
    let mut grammar = Grammar::new().with_source_format(GrammarFormat::MetaLanguage);
    let mut explicit_start = None;

    for token in tokens {
        let Token::Group {
            delimiter: Delimiter::Round,
            tokens,
        } = token
        else {
            return Err(lowering_error(
                None,
                "top-level grammar item must be a parenthesized rule definition",
            ));
        };

        let (name, expr_tokens) = split_named_group(tokens, None)?;
        if name == "start" {
            explicit_start = Some(lower_start_directive(expr_tokens)?);
        } else {
            let expr = lower_expr(expr_tokens, Some(name))?;
            grammar.add_rule(GrammarRule::new(name, expr));
        }
    }

    if let Some(start) = explicit_start {
        grammar.set_start(start);
    }

    validate_references(&grammar)?;
    Ok(grammar)
}

fn split_named_group<'tokens>(
    tokens: &'tokens [Token],
    rule: Option<&str>,
) -> Result<(&'tokens str, &'tokens [Token]), GrammarSurfaceError> {
    let [Token::Atom(name), colon, rest @ ..] = tokens else {
        return Err(lowering_error(
            rule,
            "expected a named relation of the form (name: expression)",
        ));
    };
    if !colon.is_atom(":") {
        return Err(lowering_error(rule, "expected ':' after relation name"));
    }
    if !valid_name(name) {
        return Err(lowering_error(
            rule,
            format!("invalid relation name {name:?}"),
        ));
    }
    Ok((name, rest))
}

fn lower_start_directive(tokens: &[Token]) -> Result<String, GrammarSurfaceError> {
    let [Token::Atom(name)] = tokens else {
        return Err(lowering_error(
            Some("start"),
            "start directive must contain one rule name",
        ));
    };
    if !valid_name(name) {
        return Err(lowering_error(
            Some("start"),
            format!("invalid start rule name {name:?}"),
        ));
    }
    Ok(name.clone())
}

fn validate_references(grammar: &Grammar) -> Result<(), GrammarSurfaceError> {
    let defined = grammar
        .rules()
        .iter()
        .map(|rule| rule.name().to_string())
        .collect::<BTreeSet<_>>();

    if let Some(start) = grammar.start() {
        if !defined.contains(start) {
            return Err(GrammarSurfaceError::UndefinedReference {
                rule: "start".to_string(),
                name: start.to_string(),
            });
        }
    }

    for rule in grammar.rules() {
        let mut references = BTreeSet::new();
        collect_nonterminals(rule.expr(), &mut references);
        if let Some(name) = references.into_iter().find(|name| !defined.contains(name)) {
            return Err(GrammarSurfaceError::UndefinedReference {
                rule: rule.name().to_string(),
                name,
            });
        }
    }
    Ok(())
}

fn collect_nonterminals(expr: &GrammarExpr, references: &mut BTreeSet<String>) {
    match expr {
        GrammarExpr::NonTerminal(name) => {
            references.insert(name.clone());
        }
        GrammarExpr::Choice { alternatives, .. } | GrammarExpr::Sequence(alternatives) => {
            for item in alternatives {
                collect_nonterminals(item, references);
            }
        }
        GrammarExpr::Optional(expr)
        | GrammarExpr::ZeroOrMore(expr)
        | GrammarExpr::OneOrMore(expr)
        | GrammarExpr::And(expr)
        | GrammarExpr::Not(expr)
        | GrammarExpr::Capture { expr, .. }
        | GrammarExpr::Repeat { expr, .. } => collect_nonterminals(expr, references),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => {}
    }
}

fn lower_expr(tokens: &[Token], rule: Option<&str>) -> Result<GrammarExpr, GrammarSurfaceError> {
    lower_choice(tokens, rule)
}

fn lower_choice(tokens: &[Token], rule: Option<&str>) -> Result<GrammarExpr, GrammarSurfaceError> {
    let mut separator = None;
    let mut start = 0;
    let mut parts = Vec::new();

    for (index, token) in tokens.iter().enumerate() {
        let Some(atom) = token.atom() else {
            continue;
        };
        if atom != "/" && atom != "|" {
            continue;
        }
        match separator {
            Some(existing) if existing != atom => {
                return Err(lowering_error(
                    rule,
                    "ordered '/' and unordered '|' choices cannot be mixed at one level",
                ));
            }
            None => separator = Some(atom),
            Some(_) => {}
        }
        if start == index {
            return Err(lowering_error(
                rule,
                "choice separator is missing a left operand",
            ));
        }
        parts.push(&tokens[start..index]);
        start = index + 1;
    }

    let Some(separator) = separator else {
        return lower_sequence(tokens, rule);
    };
    if start == tokens.len() {
        return Err(lowering_error(
            rule,
            "choice separator is missing a right operand",
        ));
    }
    parts.push(&tokens[start..]);
    let alternatives = parts
        .into_iter()
        .map(|part| lower_sequence(part, rule))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(GrammarExpr::Choice {
        ordered: separator == "/",
        alternatives,
    })
}

fn lower_sequence(
    tokens: &[Token],
    rule: Option<&str>,
) -> Result<GrammarExpr, GrammarSurfaceError> {
    let mut parser = SequenceParser::new(tokens, rule);
    let mut items = Vec::new();
    while !parser.is_done() {
        items.push(parser.parse_prefix()?);
    }
    Ok(match items.len() {
        0 => GrammarExpr::Empty,
        1 => items.remove(0),
        _ => GrammarExpr::Sequence(items),
    })
}

struct SequenceParser<'tokens, 'rule> {
    tokens: &'tokens [Token],
    rule: Option<&'rule str>,
    cursor: usize,
}

impl<'tokens, 'rule> SequenceParser<'tokens, 'rule> {
    const fn new(tokens: &'tokens [Token], rule: Option<&'rule str>) -> Self {
        Self {
            tokens,
            rule,
            cursor: 0,
        }
    }

    const fn is_done(&self) -> bool {
        self.cursor >= self.tokens.len()
    }

    fn parse_prefix(&mut self) -> Result<GrammarExpr, GrammarSurfaceError> {
        if self.consume_atom("&") {
            return self.parse_prefix().map(GrammarExpr::and);
        }
        if self.consume_atom("!") {
            return self.parse_prefix().map(GrammarExpr::not);
        }

        let mut expr = self.parse_atom()?;
        loop {
            if self.consume_atom("?") {
                expr = GrammarExpr::optional(expr);
            } else if self.consume_atom("*") {
                expr = GrammarExpr::zero_or_more(expr);
            } else if self.consume_atom("+") {
                expr = GrammarExpr::one_or_more(expr);
            } else if let Some((min, max)) = self.peek_repeat_bounds()? {
                self.cursor += 1;
                expr = GrammarExpr::repeat(expr, min, max);
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn parse_atom(&mut self) -> Result<GrammarExpr, GrammarSurfaceError> {
        let Some(token) = self.tokens.get(self.cursor) else {
            return Err(lowering_error(self.rule, "expected expression"));
        };
        self.cursor += 1;
        match token {
            Token::Quoted {
                value,
                quote: QuoteKind::Backtick,
            } => Ok(GrammarExpr::terminal_insensitive(value.clone())),
            Token::Quoted { value, .. } => Ok(GrammarExpr::terminal(value.clone())),
            Token::Atom(value) if value == "." => Ok(GrammarExpr::AnyChar),
            Token::Atom(value) if operator_atom(value) => Err(lowering_error(
                self.rule,
                format!("operator {value:?} is missing an operand"),
            )),
            Token::Atom(value) if valid_name(value) => Ok(GrammarExpr::non_terminal(value.clone())),
            Token::Atom(value) => Err(lowering_error(
                self.rule,
                format!("invalid bare reference {value:?}"),
            )),
            Token::Group {
                delimiter: Delimiter::Round,
                tokens,
            } => lower_expr(tokens, self.rule),
            Token::Group {
                delimiter: Delimiter::Square,
                tokens,
            } => lower_char_group(tokens, self.rule),
            Token::Group {
                delimiter: Delimiter::Brace,
                tokens,
            } => lower_capture_group(tokens, self.rule),
        }
    }

    fn consume_atom(&mut self, expected: &str) -> bool {
        if self
            .tokens
            .get(self.cursor)
            .is_some_and(|token| token.is_atom(expected))
        {
            self.cursor += 1;
            true
        } else {
            false
        }
    }

    fn peek_repeat_bounds(&self) -> Result<Option<(usize, Option<usize>)>, GrammarSurfaceError> {
        let Some(Token::Group {
            delimiter: Delimiter::Brace,
            tokens,
        }) = self.tokens.get(self.cursor)
        else {
            return Ok(None);
        };
        parse_repeat_bounds(tokens, self.rule)
    }
}

fn parse_repeat_bounds(
    tokens: &[Token],
    rule: Option<&str>,
) -> Result<Option<(usize, Option<usize>)>, GrammarSurfaceError> {
    let bounds = match tokens {
        [Token::Atom(min), comma, Token::Atom(max)] if comma.is_atom(",") => {
            Some((parse_usize(min, rule)?, Some(parse_usize(max, rule)?)))
        }
        [Token::Atom(min), comma] if comma.is_atom(",") => Some((parse_usize(min, rule)?, None)),
        _ => None,
    };
    if let Some((min, Some(max))) = bounds {
        if max < min {
            return Err(lowering_error(
                rule,
                format!("repeat maximum {max} is less than minimum {min}"),
            ));
        }
    }
    Ok(bounds)
}

fn lower_capture_group(
    tokens: &[Token],
    rule: Option<&str>,
) -> Result<GrammarExpr, GrammarSurfaceError> {
    if let [Token::Atom(label), colon, rest @ ..] = tokens {
        if colon.is_atom(":") {
            if !valid_name(label) {
                return Err(lowering_error(
                    rule,
                    format!("invalid capture label {label:?}"),
                ));
            }
            return Ok(GrammarExpr::capture(label.clone(), lower_expr(rest, rule)?));
        }
    }
    lower_expr(tokens, rule).map(GrammarExpr::capture_unlabeled)
}

fn lower_char_group(
    tokens: &[Token],
    rule: Option<&str>,
) -> Result<GrammarExpr, GrammarSurfaceError> {
    let (negated, items) = if tokens.first().is_some_and(|token| token.is_atom("^")) {
        (true, &tokens[1..])
    } else {
        (false, tokens)
    };
    if items.is_empty() {
        return Err(lowering_error(rule, "character class cannot be empty"));
    }

    if !negated {
        if let [Token::Atom(range)] = items {
            if let Some((start, end)) = parse_atom_range(range) {
                return Ok(GrammarExpr::char_range(start, end));
            }
        }
        if let [first, second] = items {
            if matches!(first, Token::Quoted { .. }) && matches!(second, Token::Quoted { .. }) {
                return Ok(GrammarExpr::char_range(
                    token_char(first, rule)?,
                    token_char(second, rule)?,
                ));
            }
        }
    }

    let class_items = items
        .iter()
        .map(|item| char_class_item(item, rule))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(GrammarExpr::char_class(negated, class_items))
}

fn char_class_item(
    token: &Token,
    rule: Option<&str>,
) -> Result<CharClassItem, GrammarSurfaceError> {
    if let Token::Atom(value) = token {
        if let Some((start, end)) = parse_atom_range(value) {
            return Ok(CharClassItem::range(start, end));
        }
    }
    token_char(token, rule).map(CharClassItem::char)
}

fn token_char(token: &Token, rule: Option<&str>) -> Result<char, GrammarSurfaceError> {
    let value = match token {
        Token::Atom(value) | Token::Quoted { value, .. } => value,
        Token::Group { .. } => {
            return Err(lowering_error(
                rule,
                "character class items must be single characters or ranges",
            ));
        }
    };
    let mut chars = value.chars();
    let Some(character) = chars.next() else {
        return Err(lowering_error(rule, "character class item cannot be empty"));
    };
    if chars.next().is_some() {
        return Err(lowering_error(
            rule,
            format!("character class item {value:?} must be one character"),
        ));
    }
    Ok(character)
}

fn parse_atom_range(value: &str) -> Option<(char, char)> {
    let mut chars = value.chars();
    let start = chars.next()?;
    if chars.next()? != '-' {
        return None;
    }
    let end = chars.next()?;
    chars.next().is_none().then_some((start, end))
}

fn parse_usize(value: &str, rule: Option<&str>) -> Result<usize, GrammarSurfaceError> {
    value.parse().map_err(|error: std::num::ParseIntError| {
        lowering_error(rule, format!("invalid repeat bound {value:?}: {error}"))
    })
}

fn valid_name(value: &str) -> bool {
    !value.is_empty() && !operator_atom(value)
}

fn operator_atom(value: &str) -> bool {
    matches!(
        value,
        ":" | "," | "?" | "*" | "+" | "/" | "|" | "&" | "!" | "^"
    )
}
