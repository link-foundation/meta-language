//! Hand-written pest PEG importer, a port of
//! `js/src/grammar-importers/pest.js`.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::common::{
    Cursor, canonical_repeat, choice, collect_references, decimal, default_escapes,
    grammar_from_rules, is_space, sequence,
};
use super::pest_policy::allow_pest_implicit_trivia;
use super::{GrammarImportError, parse_error, unsupported_error};
use crate::grammar::{
    FeatureExpr, FeatureForm, FieldValue, Grammar, GrammarExpr, GrammarFormat, GrammarRule,
    RuleKind, parse_grammar_links,
};

const FORMAT: GrammarFormat = GrammarFormat::Peg;

const BUILT_IN_RULES: &str = include_str!("../../data/pest-built-ins.lino");
const STACK_RULES: &[&str] = &[
    "PUSH",
    "PUSH_LITERAL",
    "PEEK",
    "PEEK_ALL",
    "POP",
    "POP_ALL",
    "DROP",
];
const TRIVIA_RULES: &[&str] = &["WHITESPACE", "COMMENT"];

pub(crate) fn pest_built_in_source(name: &str) -> Option<&'static str> {
    BUILT_IN_RULES.lines().find(|line| {
        line.strip_prefix("(rule ")
            .and_then(|rule| rule.split_once(' '))
            .is_some_and(|(rule, _)| rule == name)
    })
}

/// Parses PEG `.pest` grammar text into the grammar IR.
///
/// Rules are `name = modifier? { expression }` with the modifiers `_`
/// (silent), `@` (atomic), `$` (token), and `!` (normal). Expressions use
/// ordered choice `|`, sequence `~`, the predicates `&` and `!`, the postfix
/// operators `?`, `*`, `+`, and counted repetition `{n}`, `{n,}`, `{,m}`,
/// `{n,m}`, over strings, `^"…"` case-insensitive strings, `'a'..'z'` ranges,
/// and rule references, where `ANY` matches any character. `//` and `/* */`
/// comments are skipped.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the pest grammar cannot be parsed, when
/// a parsed construct (such as `PUSH(…)` or `PEEK[…]`) cannot be represented
/// in the grammar IR, or when a non-terminal reference does not resolve to a
/// local rule or pest built-in.
pub fn import_pest(text: &str) -> Result<Grammar, GrammarImportError> {
    let mut parser = PestParser {
        cursor: Cursor::with_comments(text, FORMAT),
    };
    let mut definitions = parser.rules()?;
    add_built_in_rules(&mut definitions)?;
    let rules = lower_implicit_trivia(&definitions)?;
    grammar_from_rules(FORMAT, rules, &[])
}

struct PestRule {
    rule: GrammarRule,
    modifier: Option<char>,
}

fn add_built_in_rules(definitions: &mut Vec<PestRule>) -> Result<(), GrammarImportError> {
    let mut references = Vec::new();
    for definition in definitions.iter() {
        collect_references(definition.rule.expr(), &mut references);
    }
    let local: BTreeSet<_> = definitions
        .iter()
        .map(|definition| definition.rule.name())
        .collect();
    references.retain(|name| !local.contains(name.as_str()));
    references.sort_unstable();
    let Some(first) = references.first() else {
        return Ok(());
    };
    let built_in: BTreeMap<_, _> = BUILT_IN_RULES
        .lines()
        .filter_map(|line| {
            let (name, _) = line.strip_prefix("(rule ")?.split_once(' ')?;
            Some((name, line))
        })
        .collect();
    let mut links = format!("(grammar (start {first}))");
    for name in references {
        let source = built_in
            .get(name.as_str())
            .ok_or_else(|| parse_error(FORMAT, format!("undefined non-terminal {name}")))?;
        links.push('\n');
        links.push_str(source);
    }
    let grammar = parse_grammar_links(&links)?;
    definitions.extend(grammar.rules().iter().cloned().map(|rule| PestRule {
        rule,
        modifier: None,
    }));
    Ok(())
}

struct ImplicitTrivia<'rules> {
    rules: BTreeMap<&'rules str, &'rules PestRule>,
    trivia: GrammarExpr,
    variants: BTreeMap<String, String>,
    pending: VecDeque<(String, &'rules PestRule)>,
}

fn lower_implicit_trivia(definitions: &[PestRule]) -> Result<Vec<GrammarRule>, GrammarImportError> {
    let rules: BTreeMap<_, _> = definitions
        .iter()
        .map(|definition| (definition.rule.name(), definition))
        .collect();
    let spaces = rules
        .contains_key("WHITESPACE")
        .then(|| GrammarExpr::zero_or_more(GrammarExpr::NonTerminal("WHITESPACE".to_owned())));
    let comments = rules.contains_key("COMMENT").then(|| {
        GrammarExpr::zero_or_more(sequence(
            std::iter::once(GrammarExpr::NonTerminal("COMMENT".to_owned()))
                .chain(spaces.iter().cloned())
                .collect(),
        ))
    });
    if spaces.is_none() && comments.is_none() {
        return Ok(definitions
            .iter()
            .map(|definition| definition.rule.clone())
            .collect());
    }
    let trivia = GrammarExpr::feature(FeatureExpr::Form(FeatureForm::new(
        "token",
        vec![FieldValue::Expression(sequence(
            spaces.into_iter().chain(comments).collect(),
        ))],
    )));
    let mut lowering = ImplicitTrivia {
        rules,
        trivia,
        variants: BTreeMap::new(),
        pending: VecDeque::new(),
    };
    let mut lowered = Vec::new();
    for definition in definitions {
        lowered.push(lowering.rule(definition.rule.name(), definition, true)?);
    }
    while let Some((name, definition)) = lowering.pending.pop_front() {
        lowered.push(lowering.rule(&name, definition, false)?);
    }
    Ok(lowered)
}

impl ImplicitTrivia<'_> {
    fn referenced_rule(&mut self, name: &str, allows_trivia: bool) -> String {
        let Some(&definition) = self.rules.get(name) else {
            return name.to_owned();
        };
        if allows_trivia
            || definition.modifier.is_none()
            || matches!(definition.modifier, Some('@' | '$' | '!'))
            || TRIVIA_RULES.contains(&name)
        {
            return name.to_owned();
        }
        if let Some(variant) = self.variants.get(name) {
            return variant.clone();
        }
        let mut variant = format!("{name}_pest_atomic");
        while self.rules.contains_key(variant.as_str()) {
            variant.push('_');
        }
        self.variants.insert(name.to_owned(), variant.clone());
        self.pending.push_back((variant.clone(), definition));
        variant
    }

    fn rule(
        &mut self,
        name: &str,
        definition: &PestRule,
        inherited_trivia: bool,
    ) -> Result<GrammarRule, GrammarImportError> {
        let allows_trivia = allow_pest_implicit_trivia(
            inherited_trivia,
            TRIVIA_RULES.contains(&definition.rule.name()),
            matches!(definition.modifier, Some('@' | '$')),
            definition.modifier == Some('!'),
        );
        let expr = self.expression(definition.rule.expr(), allows_trivia)?;
        let mut rule = GrammarRule::new(name, expr).with_kind(definition.rule.kind());
        if name != definition.rule.name() && rule.kind() != RuleKind::Silent {
            rule.attributes.action = Some(vec![FeatureForm::new(
                "buildNode",
                vec![FieldValue::Name(definition.rule.name.clone())],
            )]);
        }
        Ok(rule)
    }

    fn expression(
        &mut self,
        expr: &GrammarExpr,
        allows_trivia: bool,
    ) -> Result<GrammarExpr, GrammarImportError> {
        let lowered = match expr {
            GrammarExpr::NonTerminal(name) => {
                GrammarExpr::NonTerminal(self.referenced_rule(name, allows_trivia))
            }
            GrammarExpr::Sequence(items) => {
                let mut interleaved = Vec::new();
                for (index, item) in items.iter().enumerate() {
                    if allows_trivia && index > 0 {
                        interleaved.push(self.trivia.clone());
                    }
                    interleaved.push(self.expression(item, allows_trivia)?);
                }
                sequence(interleaved)
            }
            GrammarExpr::Choice {
                ordered,
                alternatives,
            } => {
                let items = alternatives
                    .iter()
                    .map(|item| self.expression(item, allows_trivia))
                    .collect::<Result<_, _>>()?;
                choice(items, *ordered)
            }
            GrammarExpr::Optional(item) => {
                GrammarExpr::optional(self.expression(item, allows_trivia)?)
            }
            GrammarExpr::And(item) => GrammarExpr::and(self.expression(item, allows_trivia)?),
            GrammarExpr::Not(item) => GrammarExpr::not(self.expression(item, allows_trivia)?),
            GrammarExpr::ZeroOrMore(item) => self.repetition(item, 0, None, allows_trivia)?,
            GrammarExpr::OneOrMore(item) => self.repetition(item, 1, None, allows_trivia)?,
            GrammarExpr::Repeat { expr, min, max } => {
                self.repetition(expr, *min, *max, allows_trivia)?
            }
            _ => expr.clone(),
        };
        Ok(lowered)
    }

    fn repetition(
        &mut self,
        item: &GrammarExpr,
        min: usize,
        max: Option<usize>,
        allows_trivia: bool,
    ) -> Result<GrammarExpr, GrammarImportError> {
        let item = self.expression(item, allows_trivia)?;
        if !allows_trivia {
            return canonical_repeat(FORMAT, item, min as u64, max.map(|count| count as u64));
        }
        if max == Some(0) {
            return Ok(GrammarExpr::Empty);
        }
        let rest = if max == Some(1) {
            GrammarExpr::Empty
        } else {
            canonical_repeat(
                FORMAT,
                sequence(vec![self.trivia.clone(), item.clone()]),
                min.saturating_sub(1) as u64,
                max.map(|count| count as u64 - 1),
            )?
        };
        let repeated = sequence(vec![item, rest]);
        Ok(if min == 0 {
            GrammarExpr::optional(repeated)
        } else {
            repeated
        })
    }
}

struct PestParser<'source> {
    cursor: Cursor<'source>,
}

impl PestParser<'_> {
    fn rules(&mut self) -> Result<Vec<PestRule>, GrammarImportError> {
        let mut rules: Vec<PestRule> = Vec::new();
        loop {
            self.cursor.skip_space()?;
            if self.cursor.eof() {
                return Ok(rules);
            }
            let name = self.cursor.identifier()?;
            if rules.iter().any(|definition| definition.rule.name == name) {
                return Err(parse_error(FORMAT, format!("duplicate rule {name}")));
            }
            self.cursor.consume("=")?;
            self.cursor.skip_space()?;
            let modifier = self
                .cursor
                .peek()
                .filter(|value| matches!(value, '_' | '@' | '$' | '!'));
            if modifier.is_some() {
                self.cursor.take();
            }
            let kind = match modifier {
                Some('_') => RuleKind::Silent,
                Some('@') => RuleKind::Atomic,
                Some('$') => RuleKind::Token,
                _ => RuleKind::Normal,
            };
            self.cursor.consume("{")?;
            let expr = self.alternation()?;
            self.cursor.consume("}")?;
            rules.push(PestRule {
                rule: GrammarRule::new(name, expr).with_kind(kind),
                modifier: Some(modifier.unwrap_or(' ')),
            });
        }
    }

    fn alternation(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        let mut alternatives = vec![self.sequence()?];
        while self.cursor.try_consume("|")? {
            alternatives.push(self.sequence()?);
        }
        Ok(choice(alternatives, true))
    }

    fn sequence(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        let mut items = vec![self.prefix()?];
        while self.cursor.try_consume("~")? {
            items.push(self.prefix()?);
        }
        Ok(sequence(items))
    }

    fn prefix(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        if self.cursor.try_consume("&")? {
            return self.prefix().map(GrammarExpr::and);
        }
        if self.cursor.try_consume("!")? {
            return self.prefix().map(GrammarExpr::not);
        }
        self.postfix()
    }

    fn postfix(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        let expr = self.atom()?;
        if self.cursor.try_consume("?")? {
            return Ok(GrammarExpr::optional(expr));
        }
        if self.cursor.try_consume("*")? {
            return Ok(GrammarExpr::zero_or_more(expr));
        }
        if self.cursor.try_consume("+")? {
            return Ok(GrammarExpr::one_or_more(expr));
        }
        self.cursor.skip_space()?;
        match counted_repetition(self.cursor.rest()) {
            Some((length, min, max)) => {
                self.cursor.advance(length);
                canonical_repeat(FORMAT, expr, min, max)
            }
            None => Ok(expr),
        }
    }

    fn atom(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        self.cursor.skip_space()?;
        if self.cursor.try_consume("(")? {
            let expr = self.alternation()?;
            self.cursor.consume(")")?;
            return Ok(expr);
        }
        match self.cursor.peek() {
            Some('^') => {
                self.cursor.advance(1);
                if self.cursor.peek() != Some('"') {
                    return Err(self
                        .cursor
                        .error("case-insensitive marker must precede a string"));
                }
                return Ok(GrammarExpr::TerminalInsensitive(
                    self.cursor.quoted(Some(default_escapes))?,
                ));
            }
            Some('"') => {
                return Ok(GrammarExpr::Terminal(
                    self.cursor.quoted(Some(default_escapes))?,
                ));
            }
            Some('\'') => {
                let start = self.cursor.quoted(Some(default_escapes))?;
                self.cursor.consume("..")?;
                let end = self.cursor.quoted(Some(default_escapes))?;
                return match (single_char(&start), single_char(&end)) {
                    (Some(start), Some(end)) => Ok(GrammarExpr::CharRange(start, end)),
                    _ => Err(parse_error(
                        FORMAT,
                        "character range endpoints must be one character",
                    )),
                };
            }
            _ => {}
        }
        let name = self.cursor.identifier()?;
        self.cursor.skip_space()?;
        let next = self.cursor.peek();
        if name == "PUSH" && next == Some('(') {
            return Err(unsupported_error(FORMAT, "Push"));
        }
        if name == "PEEK" && next == Some('[') {
            return Err(unsupported_error(FORMAT, "PeekSlice"));
        }
        if STACK_RULES.contains(&name.as_str()) {
            return Err(unsupported_error(FORMAT, format!("{name} stack rule")));
        }
        Ok(if name == "ANY" {
            GrammarExpr::AnyChar
        } else {
            GrammarExpr::NonTerminal(name)
        })
    }
}

fn single_char(text: &str) -> Option<char> {
    let mut characters = text.chars();
    let character = characters.next()?;
    characters.next().is_none().then_some(character)
}

/// Matches `{n}`, `{n,}`, `{,m}`, or `{n,m}` at the start of `text`, as the
/// JavaScript pattern `^\{\s*(\d*)\s*(?:,\s*(\d*)\s*)?\}` does, returning the
/// matched length and the bounds. `{}` and `{ }` are not counted repetition.
fn counted_repetition(text: &str) -> Option<(usize, u64, Option<u64>)> {
    let digits = |text: &str| -> usize {
        text.find(|character: char| !character.is_ascii_digit())
            .unwrap_or(text.len())
    };
    let spaces = |text: &str, from: usize| -> usize {
        from + text[from..]
            .find(|character: char| !is_space(character))
            .unwrap_or(text.len() - from)
    };
    let mut index = spaces(text, text.strip_prefix('{').map(|_| 1)?);
    let low_end = index + digits(&text[index..]);
    let low = &text[index..low_end];
    index = spaces(text, low_end);
    let mut high = None;
    if text[index..].starts_with(',') {
        index = spaces(text, index + 1);
        let high_end = index + digits(&text[index..]);
        high = Some(&text[index..high_end]);
        index = spaces(text, high_end);
    }
    if !text[index..].starts_with('}') || (low.is_empty() && high.is_none()) {
        return None;
    }
    let min = decimal(low);
    let max = match high {
        None => Some(min),
        Some("") => None,
        Some(high) => Some(decimal(high)),
    };
    Some((index + 1, min, max))
}
