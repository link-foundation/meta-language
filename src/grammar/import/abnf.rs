use std::char;

use ::abnf::types::{
    Kind as AbnfKind, Node as AbnfNode, Repeat as AbnfRepeat, Rule as AbnfRule,
    StringLiteral as AbnfStringLiteral, TerminalValues as AbnfTerminalValues,
};

use super::{parse_error, unsupported_error, GrammarImportError};
use crate::grammar::{CharClassItem, Grammar, GrammarExpr, GrammarFormat, GrammarRule};

/// Parses Augmented Backus-Naur Form text into the grammar IR.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the ABNF text cannot be parsed, when a
/// parsed construct cannot be represented, or when a non-terminal reference
/// does not resolve to an imported or RFC 5234 core rule.
pub fn import_abnf(text: &str) -> Result<Grammar, GrammarImportError> {
    let normalized = normalize_input(text);
    let parsed = ::abnf::rulelist(&normalized)
        .map_err(|error| parse_error(GrammarFormat::Abnf, error.to_string()))?;
    let mut grammar = lower_grammar(&parsed)?;
    inject_core_rules(&mut grammar);
    validate_references(&grammar)?;
    Ok(grammar)
}

fn normalize_input(text: &str) -> String {
    let mut normalized = text.trim().to_string();
    if !normalized.ends_with('\n') {
        normalized.push('\n');
    }
    normalized
}

fn lower_grammar(parsed: &[AbnfRule]) -> Result<Grammar, GrammarImportError> {
    let mut rules = Vec::new();
    for rule in parsed {
        let expr = lower_node(rule.node())?;
        merge_rule(&mut rules, rule.name(), rule.kind(), expr)?;
    }

    let mut grammar = Grammar::new().with_source_format(GrammarFormat::Abnf);
    for rule in rules {
        grammar.add_rule(rule);
    }
    Ok(grammar)
}

fn merge_rule(
    rules: &mut Vec<GrammarRule>,
    name: &str,
    kind: AbnfKind,
    expr: GrammarExpr,
) -> Result<(), GrammarImportError> {
    match kind {
        AbnfKind::Basic => {
            if rules.iter().any(|rule| rule.name() == name) {
                return Err(parse_error(
                    GrammarFormat::Abnf,
                    format!("duplicate rule {name}"),
                ));
            }
            rules.push(GrammarRule::new(name, expr));
            Ok(())
        }
        AbnfKind::Incremental => {
            let Some(rule) = rules.iter_mut().find(|rule| rule.name() == name) else {
                return Err(parse_error(
                    GrammarFormat::Abnf,
                    format!("incremental alternative for undefined rule {name}"),
                ));
            };
            append_choice_alternative(&mut rule.expr, expr);
            Ok(())
        }
    }
}

fn append_choice_alternative(expr: &mut GrammarExpr, alternative: GrammarExpr) {
    if let GrammarExpr::Choice {
        ordered: false,
        alternatives,
    } = expr
    {
        push_choice_alternative(alternatives, alternative);
    } else {
        let previous = std::mem::replace(expr, GrammarExpr::Empty);
        let mut alternatives = Vec::new();
        push_choice_alternative(&mut alternatives, previous);
        push_choice_alternative(&mut alternatives, alternative);
        *expr = GrammarExpr::Choice {
            ordered: false,
            alternatives,
        };
    }
}

fn lower_node(node: &AbnfNode) -> Result<GrammarExpr, GrammarImportError> {
    match node {
        AbnfNode::Alternatives(nodes) => lower_choice(nodes.iter().map(lower_node)),
        AbnfNode::Concatenation(nodes) => lower_sequence(nodes.iter().map(lower_node)),
        AbnfNode::Repetition { repeat, node } => lower_repetition(repeat, node),
        AbnfNode::Rulename(name) => Ok(GrammarExpr::NonTerminal(name.clone())),
        AbnfNode::Group(node) => lower_node(node),
        AbnfNode::Optional(node) => lower_node(node).map(GrammarExpr::optional),
        AbnfNode::String(literal) => Ok(lower_string(literal)),
        AbnfNode::TerminalValues(values) => lower_terminal_values(values),
        AbnfNode::Prose(_) => Err(unsupported_error(GrammarFormat::Abnf, "prose-val")),
    }
}

fn lower_string(literal: &AbnfStringLiteral) -> GrammarExpr {
    let value = literal.as_str().to_string();
    if literal.is_case_sensitive() {
        GrammarExpr::Terminal(value)
    } else {
        GrammarExpr::TerminalInsensitive(value)
    }
}

fn lower_terminal_values(
    terminal_values: &AbnfTerminalValues,
) -> Result<GrammarExpr, GrammarImportError> {
    match terminal_values {
        AbnfTerminalValues::Range(start, end) if start == end => {
            decode_terminal(*start).map(GrammarExpr::Terminal)
        }
        AbnfTerminalValues::Range(start, end) => {
            let start = decode_char(*start)?;
            let end = decode_char(*end)?;
            Ok(GrammarExpr::CharRange(start, end))
        }
        AbnfTerminalValues::Concatenation(values) => {
            let mut terminal = String::new();
            for value in values {
                terminal.push(decode_char(*value)?);
            }
            if terminal.is_empty() {
                Ok(GrammarExpr::Empty)
            } else {
                Ok(GrammarExpr::Terminal(terminal))
            }
        }
    }
}

fn decode_terminal(value: u32) -> Result<String, GrammarImportError> {
    decode_char(value).map(|character| character.to_string())
}

fn decode_char(value: u32) -> Result<char, GrammarImportError> {
    char::from_u32(value).ok_or_else(|| {
        unsupported_error(
            GrammarFormat::Abnf,
            format!("numeric terminal value U+{value:04X}"),
        )
    })
}

fn lower_repetition(
    repeat: &AbnfRepeat,
    node: &AbnfNode,
) -> Result<GrammarExpr, GrammarImportError> {
    let expr = lower_node(node)?;
    let (min, max) = match repeat {
        AbnfRepeat::Specific(count) => (*count, Some(*count)),
        AbnfRepeat::Variable { min, max } => (min.unwrap_or(0), *max),
    };
    Ok(canonical_repeat(expr, min, max))
}

fn canonical_repeat(expr: GrammarExpr, min: usize, max: Option<usize>) -> GrammarExpr {
    match (min, max) {
        (0, None) => GrammarExpr::zero_or_more(expr),
        (1, None) => GrammarExpr::one_or_more(expr),
        (0, Some(1)) => GrammarExpr::optional(expr),
        _ => GrammarExpr::repeat(expr, min, max),
    }
}

fn lower_sequence<I>(items: I) -> Result<GrammarExpr, GrammarImportError>
where
    I: IntoIterator<Item = Result<GrammarExpr, GrammarImportError>>,
{
    let mut lowered = Vec::new();
    for item in items {
        push_sequence_item(&mut lowered, item?);
    }

    Ok(match lowered.len() {
        0 => GrammarExpr::Empty,
        1 => lowered.remove(0),
        _ => GrammarExpr::Sequence(lowered),
    })
}

fn push_sequence_item(items: &mut Vec<GrammarExpr>, item: GrammarExpr) {
    match item {
        GrammarExpr::Empty => {}
        GrammarExpr::Sequence(nested) => {
            for item in nested {
                push_sequence_item(items, item);
            }
        }
        item => items.push(item),
    }
}

fn lower_choice<I>(alternatives: I) -> Result<GrammarExpr, GrammarImportError>
where
    I: IntoIterator<Item = Result<GrammarExpr, GrammarImportError>>,
{
    let mut lowered = Vec::new();
    for alternative in alternatives {
        push_choice_alternative(&mut lowered, alternative?);
    }

    if lowered.iter().all(|expr| expr == &GrammarExpr::Empty) {
        return Ok(GrammarExpr::Empty);
    }

    Ok(match lowered.len() {
        0 => GrammarExpr::Empty,
        1 => lowered.remove(0),
        _ => GrammarExpr::Choice {
            ordered: false,
            alternatives: lowered,
        },
    })
}

fn push_choice_alternative(alternatives: &mut Vec<GrammarExpr>, alternative: GrammarExpr) {
    match alternative {
        GrammarExpr::Choice {
            ordered: false,
            alternatives: nested,
        } => alternatives.extend(nested),
        alternative => alternatives.push(alternative),
    }
}

fn inject_core_rules(grammar: &mut Grammar) {
    loop {
        let unresolved = grammar.undefined_nonterminals();
        let mut added = false;
        for name in unresolved {
            if let Some(rule) = core_rule(&name) {
                grammar.add_rule(rule);
                added = true;
            }
        }
        if !added {
            break;
        }
    }
}

fn core_rule(name: &str) -> Option<GrammarRule> {
    let canonical_name = name.to_ascii_uppercase();
    let expr = match canonical_name.as_str() {
        "ALPHA" => GrammarExpr::char_class(
            false,
            [
                CharClassItem::Range('A', 'Z'),
                CharClassItem::Range('a', 'z'),
            ],
        ),
        "DIGIT" => GrammarExpr::CharRange('0', '9'),
        "HEXDIG" => GrammarExpr::char_class(
            false,
            [
                CharClassItem::Range('0', '9'),
                CharClassItem::Range('A', 'F'),
                CharClassItem::Range('a', 'f'),
            ],
        ),
        "BIT" => GrammarExpr::CharRange('0', '1'),
        "CR" => GrammarExpr::Terminal("\r".into()),
        "LF" => GrammarExpr::Terminal("\n".into()),
        "CRLF" => crlf_expr(),
        "SP" => GrammarExpr::Terminal(" ".into()),
        "HTAB" => GrammarExpr::Terminal("\t".into()),
        "WSP" => wsp_expr(),
        "DQUOTE" => GrammarExpr::Terminal("\"".into()),
        "CHAR" => GrammarExpr::CharRange('\u{01}', '\u{7f}'),
        "CTL" => GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                GrammarExpr::CharRange('\u{00}', '\u{1f}'),
                GrammarExpr::CharRange('\u{7f}', '\u{7f}'),
            ],
        },
        "OCTET" => GrammarExpr::CharRange('\u{00}', '\u{ff}'),
        "VCHAR" => GrammarExpr::CharRange('\u{21}', '\u{7e}'),
        "LWSP" => GrammarExpr::zero_or_more(GrammarExpr::Choice {
            ordered: false,
            alternatives: vec![
                wsp_expr(),
                GrammarExpr::Sequence(vec![crlf_expr(), wsp_expr()]),
            ],
        }),
        _ => return None,
    };
    Some(GrammarRule::new(name, expr))
}

fn crlf_expr() -> GrammarExpr {
    GrammarExpr::Sequence(vec![
        GrammarExpr::Terminal("\r".into()),
        GrammarExpr::Terminal("\n".into()),
    ])
}

fn wsp_expr() -> GrammarExpr {
    GrammarExpr::char_class(false, [CharClassItem::Char(' '), CharClassItem::Char('\t')])
}

fn validate_references(grammar: &Grammar) -> Result<(), GrammarImportError> {
    if let Some(name) = grammar.undefined_nonterminals().into_iter().next() {
        return Err(parse_error(
            GrammarFormat::Abnf,
            format!("undefined non-terminal {name}"),
        ));
    }
    Ok(())
}
