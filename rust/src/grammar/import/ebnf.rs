//! Hand-written ISO-style EBNF importer, a port of `importEbnf` in
//! `js/src/grammar-importers/bnf-ebnf.js`.

use super::common::{Cursor, alternatives_choice, grammar_from_rules, sequence, terminal_or_empty};
use super::{GrammarImportError, parse_error, unsupported_error};
use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarRule};

const FORMAT: GrammarFormat = GrammarFormat::Ebnf;

/// Parses Extended Backus-Naur Form text into the grammar IR.
///
/// The parser accepts the ISO-style `name = expression ;` spelling used by the
/// issue fixtures and the `::=` spelling. Concatenation (`,` or juxtaposition)
/// binds tighter than alternation (`|`); `( )`, `[ ]`, and `{ }` group,
/// make optional, and repeat; a postfix `?`, `*`, or `+` follows an element.
/// ISO `(* ... *)` comments outside string literals are skipped, as in the
/// JavaScript importer.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the EBNF text cannot be parsed, when a
/// parsed construct cannot be represented, or when a non-terminal reference does
/// not resolve to a rule in the imported grammar.
pub fn import_ebnf(text: &str) -> Result<Grammar, GrammarImportError> {
    let source = remove_comments(text)?;
    let mut cursor = Cursor::new(&source, FORMAT);
    let mut rules: Vec<GrammarRule> = Vec::new();
    loop {
        cursor.skip_space()?;
        if cursor.eof() {
            break;
        }
        let name = cursor.identifier()?;
        if rules.iter().any(|rule| rule.name == name) {
            return Err(parse_error(FORMAT, format!("duplicate rule {name}")));
        }
        if !cursor.try_consume("::=")? {
            cursor.consume("=")?;
        }
        let expr = alternation(&mut cursor, ';')?;
        cursor.consume(";")?;
        rules.push(GrammarRule::new(name, expr));
    }
    grammar_from_rules(FORMAT, rules, &[])
}

fn alternation(cursor: &mut Cursor<'_>, close: char) -> Result<GrammarExpr, GrammarImportError> {
    let mut alternatives = vec![concatenation(cursor, close)?];
    while cursor.try_consume("|")? {
        alternatives.push(concatenation(cursor, close)?);
    }
    Ok(alternatives_choice(alternatives))
}

fn concatenation(cursor: &mut Cursor<'_>, close: char) -> Result<GrammarExpr, GrammarImportError> {
    let mut items = Vec::new();
    loop {
        cursor.skip_space()?;
        match cursor.peek() {
            None | Some('|' | ')' | ']' | '}') => break,
            Some(character) if character == close => break,
            Some(',') => cursor.advance(1),
            Some(_) => items.push(postfix(cursor)?),
        }
    }
    Ok(sequence(items))
}

fn postfix(cursor: &mut Cursor<'_>) -> Result<GrammarExpr, GrammarImportError> {
    let expr = atom(cursor)?;
    Ok(if cursor.try_consume("?")? {
        GrammarExpr::optional(expr)
    } else if cursor.try_consume("*")? {
        GrammarExpr::zero_or_more(expr)
    } else if cursor.try_consume("+")? {
        GrammarExpr::one_or_more(expr)
    } else {
        expr
    })
}

fn atom(cursor: &mut Cursor<'_>) -> Result<GrammarExpr, GrammarImportError> {
    cursor.skip_space()?;
    if matches!(cursor.peek(), Some('"' | '\'')) {
        return cursor.quoted(Some(decode_escape)).map(terminal_or_empty);
    }
    for (open, close) in [("(", ')'), ("[", ']'), ("{", '}')] {
        if cursor.try_consume(open)? {
            let expr = alternation(cursor, close)?;
            cursor.consume(&close.to_string())?;
            return Ok(match close {
                ']' => GrammarExpr::optional(expr),
                '}' => GrammarExpr::zero_or_more(expr),
                _ => expr,
            });
        }
    }
    match cursor.peek() {
        Some('?') => Err(unsupported_error(FORMAT, "special sequence")),
        Some('#') => Err(unsupported_error(FORMAT, "inline regex")),
        _ => Ok(GrammarExpr::NonTerminal(cursor.identifier()?)),
    }
}

/// Decodes the escapes `\t \b \n \r \f \/ \\` and an escaped quote inside a
/// quoted terminal, rejecting any other escape.
fn decode_escape(
    escaped: char,
    quote: char,
    format: GrammarFormat,
) -> Result<String, GrammarImportError> {
    let decoded = match escaped {
        _ if escaped == quote => quote,
        't' => '\t',
        'b' => '\u{8}',
        'n' => '\n',
        'r' => '\r',
        'f' => '\u{c}',
        '/' => '/',
        '\\' => '\\',
        _ => {
            return Err(parse_error(
                format,
                format!("unsupported string escape \\{escaped}"),
            ));
        }
    };
    Ok(decoded.to_string())
}

/// Replaces every `(* ... *)` comment outside a string literal with one space.
fn remove_comments(source: &str) -> Result<String, GrammarImportError> {
    let mut result = String::with_capacity(source.len());
    let mut quote = None;
    let mut index = 0;
    while let Some(character) = source[index..].chars().next() {
        if let Some(open) = quote {
            result.push(character);
            index += character.len_utf8();
            if character == '\\'
                && let Some(next) = source[index..].chars().next()
            {
                result.push(next);
                index += next.len_utf8();
            } else if character == open {
                quote = None;
            }
            continue;
        }
        if matches!(character, '"' | '\'') {
            quote = Some(character);
        } else if source[index..].starts_with("(*") {
            let Some(end) = source[index + 2..].find("*)") else {
                return Err(parse_error(FORMAT, "unterminated comment"));
            };
            result.push(' ');
            index += 2 + end + 2;
            continue;
        }
        result.push(character);
        index += character.len_utf8();
    }
    if quote.is_some() {
        return Err(parse_error(FORMAT, "unterminated string literal"));
    }
    Ok(result)
}
