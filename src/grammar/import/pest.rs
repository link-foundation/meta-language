use std::collections::BTreeSet;

use pest_meta::ast::{Expr as PestExpr, Rule as PestRule, RuleType as PestRuleType};
use pest_meta::{
    parser::{self, Rule as PestParserRule},
    validator,
};

use super::{parse_error, unsupported_error, GrammarImportError};
use crate::grammar::{Grammar, GrammarExpr, GrammarFormat, GrammarRule, RuleKind};

/// Parses PEG `.pest` grammar text into the grammar IR.
///
/// # Errors
///
/// Returns [`GrammarImportError`] when the pest grammar cannot be parsed or
/// validated, when a parsed construct cannot be represented in the grammar IR,
/// or when a non-terminal reference does not resolve to a local rule or pest
/// built-in.
pub fn import_pest(text: &str) -> Result<Grammar, GrammarImportError> {
    let pairs = parser::parse(PestParserRule::grammar_rules, text)
        .map_err(|error| parse_error(GrammarFormat::Peg, error.to_string()))?;
    let used_builtins = validator::validate_pairs(pairs.clone())
        .map_err(|errors| parse_error(GrammarFormat::Peg, format_errors(errors)))?;
    let parsed = parser::consume_rules(pairs)
        .map_err(|errors| parse_error(GrammarFormat::Peg, format_errors(errors)))?;
    let grammar = lower_grammar(parsed)?;
    validate_references(&grammar, &used_builtins)?;
    Ok(grammar)
}

fn lower_grammar(parsed: Vec<PestRule>) -> Result<Grammar, GrammarImportError> {
    let mut grammar = Grammar::new().with_source_format(GrammarFormat::Peg);
    for rule in parsed {
        grammar.add_rule(
            GrammarRule::new(rule.name, lower_expr(rule.expr)?).with_kind(lower_rule_type(rule.ty)),
        );
    }
    Ok(grammar)
}

fn lower_rule_type(rule_type: PestRuleType) -> RuleKind {
    match rule_type {
        PestRuleType::Normal | PestRuleType::NonAtomic => RuleKind::Normal,
        PestRuleType::Silent => RuleKind::Silent,
        PestRuleType::Atomic | PestRuleType::CompoundAtomic => RuleKind::Atomic,
    }
}

fn lower_expr(expr: PestExpr) -> Result<GrammarExpr, GrammarImportError> {
    match expr {
        PestExpr::Str(value) => Ok(GrammarExpr::Terminal(value)),
        PestExpr::Insens(value) => Ok(GrammarExpr::TerminalInsensitive(value)),
        PestExpr::Range(start, end) => Ok(GrammarExpr::CharRange(
            single_char(&start, "range start")?,
            single_char(&end, "range end")?,
        )),
        PestExpr::Ident(name) if name == "ANY" => Ok(GrammarExpr::AnyChar),
        PestExpr::Ident(name) => Ok(GrammarExpr::NonTerminal(name)),
        PestExpr::PeekSlice(_, _) => Err(unsupported_error(GrammarFormat::Peg, "PeekSlice")),
        PestExpr::PosPred(inner) => lower_expr(*inner).map(GrammarExpr::and),
        PestExpr::NegPred(inner) => lower_expr(*inner).map(GrammarExpr::not),
        PestExpr::Seq(left, right) => lower_sequence([lower_expr(*left), lower_expr(*right)]),
        PestExpr::Choice(left, right) => lower_choice([lower_expr(*left), lower_expr(*right)]),
        PestExpr::Opt(inner) => lower_expr(*inner).map(GrammarExpr::optional),
        PestExpr::Rep(inner) => lower_expr(*inner).map(GrammarExpr::zero_or_more),
        PestExpr::RepOnce(inner) => lower_expr(*inner).map(GrammarExpr::one_or_more),
        PestExpr::RepExact(inner, count) => {
            let count = repetition_count(count)?;
            lower_expr(*inner).map(|expr| GrammarExpr::repeat(expr, count, Some(count)))
        }
        PestExpr::RepMin(inner, min) => {
            let min = repetition_count(min)?;
            lower_expr(*inner).map(|expr| GrammarExpr::repeat(expr, min, None))
        }
        PestExpr::RepMax(inner, max) => {
            let max = repetition_count(max)?;
            lower_expr(*inner).map(|expr| GrammarExpr::repeat(expr, 0, Some(max)))
        }
        PestExpr::RepMinMax(inner, min, max) => {
            let min = repetition_count(min)?;
            let max = repetition_count(max)?;
            lower_expr(*inner).map(|expr| GrammarExpr::repeat(expr, min, Some(max)))
        }
        PestExpr::Skip(_) => Err(unsupported_error(GrammarFormat::Peg, "Skip")),
        PestExpr::Push(_) => Err(unsupported_error(GrammarFormat::Peg, "Push")),
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

    Ok(match lowered.len() {
        0 => GrammarExpr::Empty,
        1 => lowered.remove(0),
        _ => GrammarExpr::Choice {
            ordered: true,
            alternatives: lowered,
        },
    })
}

fn push_choice_alternative(alternatives: &mut Vec<GrammarExpr>, alternative: GrammarExpr) {
    match alternative {
        GrammarExpr::Choice {
            ordered: true,
            alternatives: nested,
        } => alternatives.extend(nested),
        alternative => alternatives.push(alternative),
    }
}

fn single_char(value: &str, role: &str) -> Result<char, GrammarImportError> {
    let mut chars = value.chars();
    let Some(character) = chars.next() else {
        return Err(parse_error(
            GrammarFormat::Peg,
            format!("{role} must contain exactly one character"),
        ));
    };
    if chars.next().is_some() {
        return Err(parse_error(
            GrammarFormat::Peg,
            format!("{role} {value:?} must contain exactly one character"),
        ));
    }
    Ok(character)
}

fn repetition_count(count: u32) -> Result<usize, GrammarImportError> {
    usize::try_from(count).map_err(|_| {
        unsupported_error(
            GrammarFormat::Peg,
            format!("repetition count {count} exceeds usize"),
        )
    })
}

fn validate_references(
    grammar: &Grammar,
    used_builtins: &[&str],
) -> Result<(), GrammarImportError> {
    let used_builtins = used_builtins.iter().copied().collect::<BTreeSet<_>>();
    if let Some(name) = grammar
        .undefined_nonterminals()
        .into_iter()
        .find(|name| !used_builtins.contains(name.as_str()))
    {
        return Err(parse_error(
            GrammarFormat::Peg,
            format!("undefined non-terminal {name}"),
        ));
    }
    Ok(())
}

fn format_errors<E>(errors: impl IntoIterator<Item = E>) -> String
where
    E: ToString,
{
    errors
        .into_iter()
        .map(|error| error.to_string())
        .collect::<Vec<_>>()
        .join("\n\n")
}
