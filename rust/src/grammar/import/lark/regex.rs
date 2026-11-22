use super::{GrammarImportError, Lexer, error_at, finish_sequence};
use crate::grammar::GrammarExpr;
use crate::grammar::feature::{
    FeatureExpr, FeatureForm, FieldValue, UnicodeClassItem, class_expression,
};

mod policy;

use UnicodeClassItem::{Category, Char, Range};
use policy::{
    accept_lark_regex_flag, accept_lark_regex_literal_escape, accept_lark_regex_meta_character,
    accept_lark_regex_quantifier,
};

pub(super) fn lower_regex(
    value: &str,
    flags: &str,
    offset: usize,
) -> Result<GrammarExpr, GrammarImportError> {
    let base = offset + 1;
    for (index, flag) in flags.char_indices() {
        if !accept_lark_regex_flag(flag.encode_utf8(&mut [0; 4])) {
            return Err(error_at(
                base + value.len() + 1 + index,
                format!("unsupported regex flag {flag}"),
            ));
        }
    }

    let mut stream = Lexer::new(value);
    let scoped_case = stream.starts_with("(?i:");
    let scoped = scoped_case || stream.starts_with("(?:");
    if scoped {
        stream.cursor += if scoped_case { 4 } else { 3 };
    }
    let insensitive = scoped_case || flags.contains('i');
    let dot_all = flags.contains('s');
    let mut items = Vec::new();
    let mut variable = false;
    let mut required = false;
    while !stream.is_end() && !(scoped && stream.peek_char() == Some(')')) {
        let position = base + stream.cursor;
        let mut item = read_atom(&mut stream, base, position, insensitive, dot_all)?;
        if let Some(operator) = stream
            .peek_char()
            .filter(|character| accept_lark_regex_quantifier(character.encode_utf8(&mut [0; 4])))
        {
            if variable {
                return Err(error_at(
                    base + stream.cursor,
                    "unsupported second regex quantifier",
                ));
            }
            variable = true;
            stream.advance_char();
            item = match operator {
                '+' => GrammarExpr::one_or_more(item),
                '*' => GrammarExpr::zero_or_more(item),
                _ => GrammarExpr::optional(item),
            };
            required |= operator == '+';
        } else {
            required = true;
        }
        items.push(item);
    }
    if scoped && (stream.advance_char() != Some(')') || !stream.is_end()) {
        return Err(error_at(
            base + stream.cursor,
            "unsupported regex group boundary",
        ));
    }
    if scoped && items.is_empty() {
        return Ok(GrammarExpr::Empty);
    }
    if !required {
        return Err(error_at(offset, "regex terminal must consume input"));
    }
    if items.len() == 1 && !variable {
        return Ok(items.remove(0));
    }

    Ok(GrammarExpr::feature(FeatureExpr::Form(FeatureForm::new(
        "token",
        vec![FieldValue::Expression(finish_sequence(items))],
    ))))
}

fn read_atom(
    stream: &mut Lexer<'_>,
    base: usize,
    position: usize,
    insensitive: bool,
    dot_all: bool,
) -> Result<GrammarExpr, GrammarImportError> {
    let head = stream.advance_char().expect("a regex atom is available");
    match head {
        '[' => read_class(stream, base, position, insensitive),
        '.' => Ok(if dot_all {
            class_expression(false, vec![Range('\0', char::MAX)])
        } else {
            class_expression(false, vec![Range('\0', '\x09'), Range('\x0b', char::MAX)])
        }),
        '\\' => match read_escape(stream, base, position, false)? {
            ClassItem::Character(value) => literal(value, insensitive, position),
            ClassItem::Class(items, negated) => Ok(class_expression(negated, items)),
        },
        _ if accept_lark_regex_meta_character(head.encode_utf8(&mut [0; 4])) => Err(error_at(
            position,
            format!("unsupported regex construct {head}"),
        )),
        _ => literal(head, insensitive, position),
    }
}

enum ClassItem {
    Character(char),
    Class(Vec<UnicodeClassItem>, bool),
}

fn read_class(
    stream: &mut Lexer<'_>,
    base: usize,
    position: usize,
    insensitive: bool,
) -> Result<GrammarExpr, GrammarImportError> {
    let negated = stream.peek_char() == Some('^');
    if negated {
        stream.advance_char();
    }
    let mut items = Vec::new();
    while !stream.is_end() && stream.peek_char() != Some(']') {
        let start_position = base + stream.cursor;
        let start = read_class_item(stream, base, start_position)?;
        if stream.starts_with("-")
            && !matches!(
                stream.text[stream.cursor + 1..].chars().next(),
                None | Some(']')
            )
        {
            stream.advance_char();
            let end = read_class_item(stream, base, base + stream.cursor)?;
            let (ClassItem::Character(start), ClassItem::Character(end)) = (start, end) else {
                return Err(error_at(
                    start_position,
                    "invalid regex character class range",
                ));
            };
            if start > end {
                return Err(error_at(
                    start_position,
                    "invalid regex character class range",
                ));
            }
            items.extend(case_items(start, end, insensitive, start_position, true)?);
        } else {
            match start {
                ClassItem::Character(value) => items.extend(case_items(
                    value,
                    value,
                    insensitive,
                    start_position,
                    false,
                )?),
                ClassItem::Class(additional, _) => items.extend(additional),
            }
        }
    }
    if items.is_empty() || stream.advance_char() != Some(']') {
        return Err(error_at(
            position,
            "empty or unterminated regex character class",
        ));
    }
    Ok(class_expression(negated, items))
}

fn read_class_item(
    stream: &mut Lexer<'_>,
    base: usize,
    position: usize,
) -> Result<ClassItem, GrammarImportError> {
    match stream
        .advance_char()
        .expect("a regex class item is available")
    {
        '[' => Err(error_at(
            position,
            "unsupported nested regex character class",
        )),
        '\\' => read_escape(stream, base, position, true),
        value => Ok(ClassItem::Character(value)),
    }
}

fn read_escape(
    stream: &mut Lexer<'_>,
    base: usize,
    position: usize,
    inside_class: bool,
) -> Result<ClassItem, GrammarImportError> {
    let Some(escape) = stream.advance_char() else {
        return Err(error_at(position, "unterminated regex escape"));
    };
    let lower = escape.to_ascii_lowercase();
    let items = match lower {
        'd' => vec![Category("Nd".into())],
        's' => vec![
            Range('\x09', '\x0d'),
            Range('\x1c', ' '),
            Char('\u{85}'),
            Char('\u{a0}'),
            Char('\u{1680}'),
            Range('\u{2000}', '\u{200a}'),
            Range('\u{2028}', '\u{2029}'),
            Char('\u{202f}'),
            Char('\u{205f}'),
            Char('\u{3000}'),
        ],
        'w' => vec![Category("L".into()), Category("N".into()), Char('_')],
        _ => {
            return read_literal_escape(stream, base, position, inside_class, escape)
                .map(ClassItem::Character);
        }
    };
    if escape != lower && inside_class {
        return Err(error_at(
            position,
            format!("unsupported complemented class escape \\{escape}"),
        ));
    }
    Ok(ClassItem::Class(items, escape != lower))
}

fn read_literal_escape(
    stream: &mut Lexer<'_>,
    base: usize,
    position: usize,
    inside_class: bool,
    escape: char,
) -> Result<char, GrammarImportError> {
    match escape {
        'a' => Ok('\x07'),
        'v' => Ok('\x0b'),
        'n' => Ok('\n'),
        'r' => Ok('\r'),
        't' => Ok('\t'),
        'f' => Ok('\x0c'),
        'b' if inside_class => Ok('\x08'),
        'x' | 'u' | 'U' => {
            let length = match escape {
                'x' => 2,
                'u' => 4,
                _ => 8,
            };
            let mut value = 0;
            for _ in 0..length {
                let digit_position = base + stream.cursor;
                let Some(character) = stream.advance_char() else {
                    return Err(error_at(position, "unterminated hexadecimal escape"));
                };
                let Some(digit) = character.to_digit(16) else {
                    return Err(error_at(
                        digit_position,
                        "hexadecimal escape requires hexadecimal digits",
                    ));
                };
                value = value * 16 + digit;
            }
            char::from_u32(value).ok_or_else(|| error_at(position, "invalid hexadecimal escape"))
        }
        _ if accept_lark_regex_literal_escape(f64::from(u32::from(escape))) => Ok(escape),
        _ => Err(error_at(
            position,
            format!("unsupported regex escape \\{escape}"),
        )),
    }
}

fn literal(
    value: char,
    insensitive: bool,
    position: usize,
) -> Result<GrammarExpr, GrammarImportError> {
    if !insensitive {
        return Ok(GrammarExpr::terminal(value));
    }
    Ok(class_expression(
        false,
        case_items(value, value, true, position, false)?,
    ))
}

fn case_items(
    start: char,
    end: char,
    insensitive: bool,
    position: usize,
    explicit_range: bool,
) -> Result<Vec<UnicodeClassItem>, GrammarImportError> {
    let original = if explicit_range {
        Range(start, end)
    } else {
        Char(start)
    };
    let mut items = vec![original];
    if !insensitive {
        return Ok(items);
    }
    if !end.is_ascii() {
        return Err(error_at(
            position,
            "unsupported non-ASCII case-insensitive regex range or literal",
        ));
    }
    for (first, last, delta) in [(b'A', b'Z', 32), (b'a', b'z', -32)] {
        let lower = (start as u8).max(first);
        let upper = (end as u8).min(last);
        if lower <= upper {
            let lower = char::from(lower.wrapping_add_signed(delta));
            let upper = char::from(upper.wrapping_add_signed(delta));
            items.push(if lower == upper {
                Char(lower)
            } else {
                Range(lower, upper)
            });
        }
    }
    for (first, second, additional) in [('I', 'i', "İı"), ('K', 'k', "K"), ('S', 's', "ſ")] {
        if (start..=end).contains(&first) || (start..=end).contains(&second) {
            items.extend(additional.chars().map(Char));
        }
    }
    Ok(items)
}
