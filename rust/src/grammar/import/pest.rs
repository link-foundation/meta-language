//! Hand-written pest PEG importer, a port of
//! `js/src/grammar-importers/pest.js`.

use super::common::{
    Cursor, canonical_repeat, choice, decimal, default_escapes, grammar_from_rules, is_space,
    sequence,
};
use super::{GrammarImportError, parse_error, unsupported_error};
use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleKind};

const FORMAT: GrammarFormat = GrammarFormat::Peg;

/// The pest built-in rules a grammar may reference without defining them.
const PEST_BUILTINS: &[&str] = &[
    "ANY",
    "SOI",
    "EOI",
    "ASCII_DIGIT",
    "ASCII_NONZERO_DIGIT",
    "ASCII_BIN_DIGIT",
    "ASCII_OCT_DIGIT",
    "ASCII_HEX_DIGIT",
    "ASCII_ALPHA_LOWER",
    "ASCII_ALPHA_UPPER",
    "ASCII_ALPHA",
    "ASCII_ALPHANUMERIC",
    "ASCII",
    "NEWLINE",
    "UNICODE",
    "LETTER",
    "CASED_LETTER",
    "UPPERCASE_LETTER",
    "LOWERCASE_LETTER",
    "TITLECASE_LETTER",
    "MODIFIER_LETTER",
    "OTHER_LETTER",
    "MARK",
    "NUMBER",
    "PUNCTUATION",
    "SEPARATOR",
    "SYMBOL",
    "CONTROL",
    "XID_START",
    "XID_CONTINUE",
];

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
    let rules = parser.rules()?;
    grammar_from_rules(FORMAT, rules, PEST_BUILTINS)
}

struct PestParser<'source> {
    cursor: Cursor<'source>,
}

impl PestParser<'_> {
    fn rules(&mut self) -> Result<Vec<GrammarRule>, GrammarImportError> {
        let mut rules: Vec<GrammarRule> = Vec::new();
        loop {
            self.cursor.skip_space()?;
            if self.cursor.eof() {
                return Ok(rules);
            }
            let name = self.cursor.identifier()?;
            if rules.iter().any(|rule| rule.name == name) {
                return Err(parse_error(FORMAT, format!("duplicate rule {name}")));
            }
            self.cursor.consume("=")?;
            self.cursor.skip_space()?;
            let kind = match self.cursor.peek() {
                Some(modifier @ ('_' | '@' | '$' | '!')) => {
                    self.cursor.take();
                    match modifier {
                        '_' => RuleKind::Silent,
                        '@' => RuleKind::Atomic,
                        '$' => RuleKind::Token,
                        _ => RuleKind::Normal,
                    }
                }
                _ => RuleKind::Normal,
            };
            self.cursor.consume("{")?;
            let expr = self.alternation()?;
            self.cursor.consume("}")?;
            rules.push(GrammarRule::new(name, expr).with_kind(kind));
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
