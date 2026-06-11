use crate::{LinkQuery, LinkType};

use super::{LinkRule, LinkRuleKind, LinkRuleParseError};

pub(super) fn parse_rule(source: &str) -> Result<LinkRule, LinkRuleParseError> {
    let mut parser = RuleParser::new(tokenize(source)?);
    let expression = parser.parse_expression()?;
    if !parser.is_at_end() {
        return Err(LinkRuleParseError::new(
            "rule may contain only one root expression",
        ));
    }
    rule_from_expression(&expression)
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RuleExpression {
    Atom(String),
    List(Vec<Self>),
}

fn rule_from_expression(expression: &RuleExpression) -> Result<LinkRule, LinkRuleParseError> {
    let RuleExpression::List(items) = expression else {
        return Err(LinkRuleParseError::new("rule expression must be a list"));
    };
    let Some((head, arguments)) = items.split_first() else {
        return Err(LinkRuleParseError::new("rule expression is empty"));
    };
    let operator = atom(head)?;
    match operator {
        "kind" | "term" => Ok(LinkRule::kind(required_atom(arguments, 0, operator)?)),
        "type" => Ok(LinkRule::link_type(parse_link_type(required_atom(
            arguments, 0, operator,
        )?)?)),
        "language" => Ok(LinkRule {
            kind: LinkRuleKind::Language(required_atom(arguments, 0, operator)?.to_string()),
        }),
        "named" => Ok(LinkRule {
            kind: LinkRuleKind::Named(parse_bool(required_atom(arguments, 0, operator)?)?),
        }),
        "query" => Ok(LinkRule::query(
            LinkQuery::from_sexpression(required_atom(arguments, 0, operator)?)
                .map_err(|error| LinkRuleParseError::new(error.to_string()))?,
        )),
        "capture" => Ok(LinkRule::capture(
            required_atom(arguments, 0, operator)?,
            rule_arg(arguments, 1, operator)?,
        )),
        "meta" => Ok(LinkRule::typed_metavariable(
            required_atom(arguments, 0, operator)?,
            required_atom(arguments, 1, operator)?,
        )),
        "inside" => Ok(LinkRule::inside(
            rule_arg(arguments, 0, operator)?,
            rule_arg(arguments, 1, operator)?,
        )),
        "has" => Ok(LinkRule::has(
            rule_arg(arguments, 0, operator)?,
            rule_arg(arguments, 1, operator)?,
        )),
        "precedes" => Ok(LinkRule::precedes(
            rule_arg(arguments, 0, operator)?,
            rule_arg(arguments, 1, operator)?,
        )),
        "follows" => Ok(LinkRule::follows(
            rule_arg(arguments, 0, operator)?,
            rule_arg(arguments, 1, operator)?,
        )),
        "all" => arguments
            .iter()
            .map(rule_from_expression)
            .collect::<Result<Vec<_>, _>>()
            .map(LinkRule::all),
        "any" => arguments
            .iter()
            .map(rule_from_expression)
            .collect::<Result<Vec<_>, _>>()
            .map(LinkRule::any),
        "not" => Ok(LinkRule::negate(rule_arg(arguments, 0, operator)?)),
        "ref" => Ok(LinkRule::named(required_atom(arguments, 0, operator)?)),
        "ellipsis" => Ok(LinkRule::ellipsis_gap(
            rule_arg(arguments, 0, operator)?,
            rule_arg(arguments, 1, operator)?,
        )),
        "text" => LinkRule::text(required_atom(arguments, 0, operator)?),
        other => Err(LinkRuleParseError::new(format!(
            "unknown rule operator `{other}`"
        ))),
    }
}

fn atom(expression: &RuleExpression) -> Result<&str, LinkRuleParseError> {
    match expression {
        RuleExpression::Atom(value) => Ok(value),
        RuleExpression::List(_) => Err(LinkRuleParseError::new("expected atom")),
    }
}

fn required_atom<'a>(
    arguments: &'a [RuleExpression],
    index: usize,
    operator: &str,
) -> Result<&'a str, LinkRuleParseError> {
    arguments
        .get(index)
        .ok_or_else(|| LinkRuleParseError::new(format!("`{operator}` is missing an argument")))
        .and_then(atom)
}

fn rule_arg(
    arguments: &[RuleExpression],
    index: usize,
    operator: &str,
) -> Result<LinkRule, LinkRuleParseError> {
    arguments
        .get(index)
        .ok_or_else(|| LinkRuleParseError::new(format!("`{operator}` is missing a rule argument")))
        .and_then(rule_from_expression)
}

fn parse_bool(source: &str) -> Result<bool, LinkRuleParseError> {
    match source {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(LinkRuleParseError::new("expected `true` or `false`")),
    }
}

fn parse_link_type(source: &str) -> Result<LinkType, LinkRuleParseError> {
    match source {
        "link" => Ok(LinkType::Link),
        "reference" => Ok(LinkType::Reference),
        "relation" => Ok(LinkType::Relation),
        "language" => Ok(LinkType::Language),
        "grammar" => Ok(LinkType::Grammar),
        "type" => Ok(LinkType::Type),
        "concept" => Ok(LinkType::Concept),
        "syntax" => Ok(LinkType::Syntax),
        "field" => Ok(LinkType::Field),
        "trivia" => Ok(LinkType::Trivia),
        "token" => Ok(LinkType::Token),
        "document" => Ok(LinkType::Document),
        "semantic" => Ok(LinkType::Semantic),
        "region" => Ok(LinkType::Region),
        "object" => Ok(LinkType::Object),
        _ => Err(LinkRuleParseError::new(format!(
            "unknown link type `{source}`"
        ))),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum RuleToken {
    LParen,
    RParen,
    Atom(String),
}

struct RuleParser {
    tokens: Vec<RuleToken>,
    position: usize,
}

impl RuleParser {
    const fn new(tokens: Vec<RuleToken>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    fn parse_expression(&mut self) -> Result<RuleExpression, LinkRuleParseError> {
        match self.advance() {
            Some(RuleToken::Atom(value)) => Ok(RuleExpression::Atom(value)),
            Some(RuleToken::LParen) => {
                let mut items = Vec::new();
                while !matches!(self.peek(), Some(RuleToken::RParen)) {
                    if self.is_at_end() {
                        return Err(LinkRuleParseError::new("unterminated rule expression"));
                    }
                    items.push(self.parse_expression()?);
                }
                self.expect_rparen()?;
                Ok(RuleExpression::List(items))
            }
            Some(RuleToken::RParen) => Err(LinkRuleParseError::new("unexpected `)`")),
            None => Err(LinkRuleParseError::new("empty rule expression")),
        }
    }

    fn expect_rparen(&mut self) -> Result<(), LinkRuleParseError> {
        match self.advance() {
            Some(RuleToken::RParen) => Ok(()),
            _ => Err(LinkRuleParseError::new("expected `)`")),
        }
    }

    fn advance(&mut self) -> Option<RuleToken> {
        let token = self.tokens.get(self.position).cloned()?;
        self.position += 1;
        Some(token)
    }

    fn peek(&self) -> Option<&RuleToken> {
        self.tokens.get(self.position)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
}

fn tokenize(source: &str) -> Result<Vec<RuleToken>, LinkRuleParseError> {
    let mut tokens = Vec::new();
    let mut characters = source.chars().peekable();
    while let Some(character) = characters.peek().copied() {
        match character {
            whitespace if whitespace.is_whitespace() => {
                characters.next();
            }
            '(' => {
                characters.next();
                tokens.push(RuleToken::LParen);
            }
            ')' => {
                characters.next();
                tokens.push(RuleToken::RParen);
            }
            '"' => tokens.push(RuleToken::Atom(read_string(&mut characters)?)),
            _ => tokens.push(RuleToken::Atom(read_atom(&mut characters))),
        }
    }
    Ok(tokens)
}

fn read_atom(characters: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
    let mut atom = String::new();
    while let Some(character) = characters.peek().copied() {
        if character.is_whitespace() || matches!(character, '(' | ')' | '"') {
            break;
        }
        atom.push(character);
        characters.next();
    }
    atom
}

fn read_string(
    characters: &mut std::iter::Peekable<std::str::Chars<'_>>,
) -> Result<String, LinkRuleParseError> {
    let mut literal = String::new();
    characters.next();

    while let Some(character) = characters.next() {
        match character {
            '"' => return Ok(literal),
            '\\' => {
                let Some(escaped) = characters.next() else {
                    return Err(LinkRuleParseError::new("unterminated string escape"));
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

    Err(LinkRuleParseError::new("unterminated string literal"))
}
