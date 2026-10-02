//! Hand-written RFC 5234 ABNF importer, a port of
//! `js/src/grammar-importers/abnf.js`.

use super::common::{
    Cursor, canonical_repeat, choice, collect_references, decimal, default_escapes,
    grammar_from_rules, is_line_terminator, is_space, json_quote, sequence, split_lines,
    strip_line_comment, trim,
};
use super::{GrammarImportError, parse_error, unsupported_error};
use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule};

const FORMAT: GrammarFormat = GrammarFormat::Abnf;

/// Parses Augmented Backus-Naur Form text into the grammar IR.
///
/// Rule names are case-insensitive: `=/` extends the rule of the same name,
/// references take the defining rule's spelling, and RFC 5234 core rules such
/// as `ALPHA` and `DIGIT` are appended when referenced but not defined.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the ABNF text cannot be parsed, when a
/// parsed construct cannot be represented, or when a non-terminal reference
/// does not resolve to an imported or RFC 5234 core rule.
pub fn import_abnf(text: &str) -> Result<Grammar, GrammarImportError> {
    let mut rules: Vec<GrammarRule> = Vec::new();
    for line in logical_lines(text)? {
        let Some((name, incremental, rhs)) = split_rule(&line) else {
            return Err(parse_error(
                FORMAT,
                format!("invalid rule {}", json_quote(&line)),
            ));
        };
        let expr = AbnfParser::new(rhs).parse()?;
        let existing = rules
            .iter_mut()
            .find(|rule| rule.name.eq_ignore_ascii_case(name));
        match (incremental, existing) {
            (true, Some(existing)) => {
                let previous = std::mem::replace(&mut existing.expr, GrammarExpr::Empty);
                existing.expr = choice(vec![previous, expr], false);
            }
            (true, None) => {
                return Err(parse_error(
                    FORMAT,
                    format!("incremental alternative for undefined rule {name}"),
                ));
            }
            (false, Some(_)) => {
                return Err(parse_error(FORMAT, format!("duplicate rule {name}")));
            }
            (false, None) => rules.push(GrammarRule::new(name, expr)),
        }
    }

    canonicalize_references(&mut rules);
    inject_core_rules(&mut rules);
    grammar_from_rules(FORMAT, rules, &[])
}

/// Joins indented continuation lines onto their rule after dropping `;`
/// comments and blank lines.
fn logical_lines(source: &str) -> Result<Vec<String>, GrammarImportError> {
    let mut lines: Vec<String> = Vec::new();
    for physical in split_lines(source) {
        let line = strip_line_comment(physical, ';', true).trim_end_matches(is_space);
        if line.is_empty() {
            continue;
        }
        if line.starts_with([' ', '\t']) {
            let Some(previous) = lines.last_mut() else {
                return Err(parse_error(FORMAT, "continuation without a rule"));
            };
            previous.push(' ');
            previous.push_str(trim(line));
        } else {
            lines.push(line.to_string());
        }
    }
    Ok(lines)
}

/// Splits `name = rhs` or `name =/ rhs`, as the JavaScript pattern
/// `^([A-Za-z][A-Za-z0-9-]*)\s*(=\/|=)\s*(.*)$` does.
fn split_rule(line: &str) -> Option<(&str, bool, &str)> {
    if !line.starts_with(|character: char| character.is_ascii_alphabetic()) {
        return None;
    }
    let name_end = line
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '-'))
        .unwrap_or(line.len());
    let (name, rest) = line.split_at(name_end);
    let rest = rest.trim_start_matches(is_space).strip_prefix('=')?;
    let (incremental, rest) = rest
        .strip_prefix('/')
        .map_or((false, rest), |rest| (true, rest));
    let rhs = rest.trim_start_matches(is_space);
    (!rhs.contains(is_line_terminator)).then_some((name, incremental, rhs))
}

struct AbnfParser<'source> {
    cursor: Cursor<'source>,
}

impl<'source> AbnfParser<'source> {
    const fn new(source: &'source str) -> Self {
        Self {
            cursor: Cursor::new(source, FORMAT),
        }
    }

    fn parse(mut self) -> Result<GrammarExpr, GrammarImportError> {
        let expr = self.alternation(None)?;
        self.cursor.skip_space()?;
        if !self.cursor.eof() {
            return Err(self.cursor.error("unexpected ABNF input"));
        }
        Ok(expr)
    }

    fn alternation(&mut self, close: Option<char>) -> Result<GrammarExpr, GrammarImportError> {
        let mut alternatives = vec![self.concatenation(close)?];
        while self.cursor.try_consume("/")? {
            alternatives.push(self.concatenation(close)?);
        }
        Ok(choice(alternatives, false))
    }

    fn concatenation(&mut self, close: Option<char>) -> Result<GrammarExpr, GrammarImportError> {
        let mut items = Vec::new();
        loop {
            self.cursor.skip_space()?;
            let next = self.cursor.peek();
            if next.is_none() || next == Some('/') || (close.is_some() && next == close) {
                break;
            }
            items.push(self.repetition()?);
        }
        Ok(sequence(items))
    }

    /// Reads an optional `n`, `n*m`, `n*`, `*m`, or `*` repeat prefix, then
    /// the repeated element.
    fn repetition(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        self.cursor.skip_space()?;
        let rest = self.cursor.rest();
        let low = leading(rest, |character| character.is_ascii_digit());
        let bounds = if let Some(after) = rest[low.len()..].strip_prefix('*') {
            let high = leading(after, |character| character.is_ascii_digit());
            self.cursor.advance(low.len() + 1 + high.len());
            Some((decimal(low), (!high.is_empty()).then(|| decimal(high))))
        } else if low.is_empty() {
            None
        } else {
            self.cursor.advance(low.len());
            Some((decimal(low), Some(decimal(low))))
        };
        let atom = self.atom()?;
        match bounds {
            Some((min, max)) => canonical_repeat(FORMAT, atom, min, max),
            None => Ok(atom),
        }
    }

    fn atom(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        self.cursor.skip_space()?;
        if self.cursor.eof() {
            return Err(self.cursor.error("expected ABNF element"));
        }
        if self.cursor.try_consume("(")? {
            let expr = self.alternation(Some(')'))?;
            self.cursor.consume(")")?;
            return Ok(expr);
        }
        if self.cursor.try_consume("[")? {
            let expr = self.alternation(Some(']'))?;
            self.cursor.consume("]")?;
            return Ok(GrammarExpr::optional(expr));
        }
        match self.cursor.peek() {
            Some('"') => Ok(GrammarExpr::TerminalInsensitive(
                self.cursor.quoted(Some(default_escapes))?,
            )),
            Some('<') => Err(unsupported_error(FORMAT, "prose-val")),
            Some('%') => self.percent_value(),
            _ => Ok(GrammarExpr::NonTerminal(self.cursor.identifier_with(
                |character| character.is_ascii_alphabetic(),
                |character| character.is_ascii_alphanumeric() || character == '-',
            )?)),
        }
    }

    /// Reads `%s"…"`, `%i"…"`, or a `%b`, `%d`, or `%x` numeric value, range,
    /// or dotted series.
    fn percent_value(&mut self) -> Result<GrammarExpr, GrammarImportError> {
        self.cursor.consume("%")?;
        let kind = self.cursor.take().map(|kind| kind.to_ascii_lowercase());
        if matches!(kind, Some('s' | 'i')) && self.cursor.peek() == Some('"') {
            let value = self.cursor.quoted(Some(default_escapes))?;
            return Ok(if kind == Some('s') {
                GrammarExpr::Terminal(value)
            } else {
                GrammarExpr::TerminalInsensitive(value)
            });
        }
        let radix = match kind {
            Some('b') => 2,
            Some('d') => 10,
            Some('x') => 16,
            _ => return Err(self.cursor.error("invalid percent value")),
        };
        let is_digit = |character: char| character.is_digit(radix);
        let rest = self.cursor.rest();
        let first = leading(rest, is_digit);
        if first.is_empty() {
            return Err(self.cursor.error("invalid numeric terminal"));
        }
        let after = &rest[first.len()..];
        if let Some(end) = after
            .strip_prefix('-')
            .map(|after| leading(after, is_digit))
            .filter(|end| !end.is_empty())
        {
            self.cursor.advance(first.len() + 1 + end.len());
            return Ok(GrammarExpr::CharRange(
                decode_code_point(first, radix)?,
                decode_code_point(end, radix)?,
            ));
        }
        let mut values = vec![first];
        let mut length = first.len();
        while let Some(value) = rest[length..]
            .strip_prefix('.')
            .map(|after| leading(after, is_digit))
            .filter(|value| !value.is_empty())
        {
            values.push(value);
            length += 1 + value.len();
        }
        self.cursor.advance(length);
        let mut literal = String::new();
        for value in values {
            literal.push(decode_code_point(value, radix)?);
        }
        Ok(GrammarExpr::Terminal(literal))
    }
}

/// The longest prefix of `text` whose characters all satisfy `predicate`.
fn leading(text: &str, predicate: impl Fn(char) -> bool) -> &str {
    let end = text
        .find(|character: char| !predicate(character))
        .unwrap_or(text.len());
    &text[..end]
}

fn decode_code_point(value: &str, radix: u32) -> Result<char, GrammarImportError> {
    u32::from_str_radix(value, radix)
        .ok()
        .and_then(char::from_u32)
        .ok_or_else(|| unsupported_error(FORMAT, format!("numeric terminal value {value}")))
}

/// Rewrites every reference to the spelling of the rule it names, ignoring
/// ASCII case.
fn canonicalize_references(rules: &mut [GrammarRule]) {
    let names = rules
        .iter()
        .map(|rule| rule.name.clone())
        .collect::<Vec<_>>();
    for rule in rules {
        for_each_reference(&mut rule.expr, &mut |name| {
            if let Some(canonical) = names
                .iter()
                .find(|candidate| candidate.eq_ignore_ascii_case(name))
            {
                name.clone_from(canonical);
            }
        });
    }
}

fn for_each_reference(expr: &mut GrammarExpr, visit: &mut impl FnMut(&mut String)) {
    match expr {
        GrammarExpr::NonTerminal(name) => visit(name),
        GrammarExpr::Choice {
            alternatives: items,
            ..
        }
        | GrammarExpr::Sequence(items) => {
            for item in items {
                for_each_reference(item, visit);
            }
        }
        GrammarExpr::Optional(expr)
        | GrammarExpr::ZeroOrMore(expr)
        | GrammarExpr::OneOrMore(expr)
        | GrammarExpr::And(expr)
        | GrammarExpr::Not(expr)
        | GrammarExpr::Capture { expr, .. }
        | GrammarExpr::Repeat { expr, .. } => for_each_reference(expr, visit),
        GrammarExpr::Empty
        | GrammarExpr::Terminal(_)
        | GrammarExpr::TerminalInsensitive(_)
        | GrammarExpr::CharRange(_, _)
        | GrammarExpr::CharClass { .. }
        | GrammarExpr::AnyChar => {}
    }
}

/// Appends the RFC 5234 core rules the grammar references, in order of first
/// reference, under the spelling used by the reference.
fn inject_core_rules(rules: &mut Vec<GrammarRule>) {
    loop {
        let mut referenced = Vec::new();
        for rule in rules.iter() {
            collect_references(&rule.expr, &mut referenced);
        }
        let mut added = false;
        for name in referenced {
            if rules.iter().any(|rule| rule.name == name) {
                continue;
            }
            if let Some(expr) = core_expression(&name) {
                rules.push(GrammarRule::new(name, expr));
                added = true;
            }
        }
        if !added {
            return;
        }
    }
}

fn core_expression(name: &str) -> Option<GrammarExpr> {
    let wsp =
        || GrammarExpr::char_class(false, [CharClassItem::Char(' '), CharClassItem::Char('\t')]);
    let terminal = |value: &str| GrammarExpr::Terminal(value.to_string());
    Some(match name.to_ascii_uppercase().as_str() {
        "ALPHA" => GrammarExpr::char_class(
            false,
            [
                CharClassItem::Range('A', 'Z'),
                CharClassItem::Range('a', 'z'),
            ],
        ),
        "BIT" => GrammarExpr::CharRange('0', '1'),
        "CHAR" => GrammarExpr::CharRange('\u{01}', '\u{7f}'),
        "CR" => terminal("\r"),
        "CRLF" => sequence(vec![terminal("\r"), terminal("\n")]),
        "CTL" => choice(
            vec![
                GrammarExpr::CharRange('\u{00}', '\u{1f}'),
                GrammarExpr::CharRange('\u{7f}', '\u{7f}'),
            ],
            false,
        ),
        "DIGIT" => GrammarExpr::CharRange('0', '9'),
        "DQUOTE" => terminal("\""),
        "HEXDIG" => GrammarExpr::char_class(
            false,
            [
                CharClassItem::Range('0', '9'),
                CharClassItem::Range('A', 'F'),
                CharClassItem::Range('a', 'f'),
            ],
        ),
        "HTAB" => terminal("\t"),
        "LF" => terminal("\n"),
        "OCTET" => GrammarExpr::CharRange('\u{00}', '\u{ff}'),
        "SP" => terminal(" "),
        "VCHAR" => GrammarExpr::CharRange('!', '~'),
        "WSP" => wsp(),
        "LWSP" => GrammarExpr::zero_or_more(choice(
            vec![wsp(), sequence(vec![terminal("\r\n"), wsp()])],
            false,
        )),
        _ => return None,
    })
}
