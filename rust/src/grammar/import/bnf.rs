//! Hand-written classic BNF importer, a port of `importBnf` in
//! `js/src/grammar-importers/bnf-ebnf.js`.

use super::common::{
    Cursor, alternatives_choice, choice, grammar_from_rules, is_line_terminator, is_space,
    json_quote, sequence, split_lines, strip_line_comment, terminal_or_empty, trim,
};
use super::{GrammarImportError, parse_error};
use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarRule};

const FORMAT: GrammarFormat = GrammarFormat::Bnf;

/// Parses classic angle-bracket Backus-Naur Form text into the grammar IR.
///
/// Each line holds one `<name> ::= alternatives` production; a later
/// production for the same name adds alternatives to it. Terminals are single-
/// or double-quoted without escape sequences, and `;` starts a line comment.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the BNF text cannot be parsed or when
/// a non-terminal reference does not resolve to a rule.
pub fn import_bnf(text: &str) -> Result<Grammar, GrammarImportError> {
    let mut rules: Vec<GrammarRule> = Vec::new();
    for raw_line in split_lines(text) {
        let line = trim(strip_line_comment(raw_line, ';', false));
        if line.is_empty() {
            continue;
        }
        let Some((name, rhs)) = split_production(line) else {
            return Err(parse_error(
                FORMAT,
                format!("invalid production {}", json_quote(line)),
            ));
        };
        let expr = parse_alternatives(rhs)?;
        if let Some(existing) = rules.iter_mut().find(|rule| rule.name == name) {
            let previous = std::mem::replace(&mut existing.expr, GrammarExpr::Empty);
            existing.expr = choice(vec![previous, expr], false);
        } else {
            rules.push(GrammarRule::new(name, expr));
        }
    }
    grammar_from_rules(FORMAT, rules, &[])
}

/// Splits `<name> ::= rhs`, as the JavaScript pattern
/// `^<([^<>]+)>\s*::=\s*(.*)$` does, returning the trimmed name.
fn split_production(line: &str) -> Option<(&str, &str)> {
    let rest = line.strip_prefix('<')?;
    let close = rest.find(['<', '>'])?;
    if close == 0 || !rest[close..].starts_with('>') {
        return None;
    }
    let after = rest[close + 1..]
        .trim_start_matches(is_space)
        .strip_prefix("::=")?;
    let rhs = after.trim_start_matches(is_space);
    (!rhs.contains(is_line_terminator)).then(|| (trim(&rest[..close]), rhs))
}

fn parse_alternatives(source: &str) -> Result<GrammarExpr, GrammarImportError> {
    let mut cursor = Cursor::new(source, FORMAT);
    let mut alternatives = Vec::new();
    loop {
        let mut items = Vec::new();
        loop {
            cursor.skip_space()?;
            if matches!(cursor.peek(), None | Some('|')) {
                break;
            }
            items.push(atom(&mut cursor)?);
        }
        alternatives.push(sequence(items));
        if !cursor.try_consume("|")? {
            break;
        }
    }
    cursor.skip_space()?;
    if !cursor.eof() {
        return Err(cursor.error("unexpected BNF input"));
    }
    Ok(alternatives_choice(alternatives))
}

fn atom(cursor: &mut Cursor<'_>) -> Result<GrammarExpr, GrammarImportError> {
    cursor.skip_space()?;
    // Classic BNF terminals have no escape sequences.
    if matches!(cursor.peek(), Some('"' | '\'')) {
        return cursor.quoted(None).map(terminal_or_empty);
    }
    if cursor.try_consume("<")? {
        let Some(end) = cursor.rest().find('>') else {
            return Err(cursor.error("unterminated non-terminal"));
        };
        let name = trim(&cursor.rest()[..end]);
        cursor.advance(end + 1);
        if name.is_empty() {
            return Err(cursor.error("empty non-terminal"));
        }
        return Ok(GrammarExpr::NonTerminal(name.to_string()));
    }
    Err(cursor.error("expected literal or non-terminal"))
}
