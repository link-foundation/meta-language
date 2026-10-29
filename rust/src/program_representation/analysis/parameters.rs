//! Actual CST coordinates and token roles transported to shared definitions.
use super::{BTreeSet, ProgramSourceMapping, SemanticToken, TokenKind};
use crate::translation::source_parameter_binding_rules as rules;

pub(super) fn binding_positions(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    first: usize,
    last: usize,
) -> Vec<usize> {
    positions(tokens, syntax, first, last, false, None)
}
pub(super) fn variable_positions(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    index: usize,
) -> Option<Vec<usize>> {
    let declaration = syntax.iter().find(|node| {
        matches!(
            node.term.as_str(),
            "lexical_declaration" | "variable_declaration"
        ) && node.range.start == tokens[index].range.start
    })?;
    let last = tokens.partition_point(|token| token.range.start < declaration.range.end);
    Some(positions(
        tokens,
        syntax,
        index + 1,
        last,
        true,
        Some(declaration.range.end),
    ))
}
fn positions(
    tokens: &[SemanticToken],
    syntax: &[ProgramSourceMapping],
    first: usize,
    last: usize,
    variables: bool,
    boundary: Option<usize>,
) -> Vec<usize> {
    if first >= last || last > tokens.len() {
        return Vec::new();
    }
    let selected = &tokens[first..last];
    let begin = selected[0].range.start;
    let end = boundary.unwrap_or_else(|| {
        tokens.get(last).map_or(
            selected.last().expect("nonempty formal interval").range.end,
            |token| token.range.start,
        )
    });
    let lower = |offset| selected.partition_point(|token| token.range.start < offset);
    let identifiers = syntax
        .iter()
        .filter(|node| rules::accept_java_script_source_parameter_identifier_term(&node.term))
        .map(|node| (node.range.start, node.range.end))
        .collect::<BTreeSet<_>>();
    let mut defaults = Vec::new();
    let mut computed = Vec::new();
    for node in syntax {
        if node.range.start < begin || node.range.start >= end || node.range.end > end {
            continue;
        }
        let interval = (lower(node.range.start), lower(node.range.end));
        if matches!(
            node.term.as_str(),
            "assignment_pattern" | "object_assignment_pattern"
        ) {
            defaults.push(interval);
        }
        if node.term == "computed_property_name" {
            computed.push(interval);
        }
        if variables && node.term == "variable_declarator" {
            let root = syntax
                .iter()
                .filter(|child| {
                    rules::accept_java_script_source_binding_root_term(&child.term)
                        && child.range.start == node.range.start
                        && child.range.end <= node.range.end
                })
                .max_by_key(|child| child.range.end);
            if root
                .and_then(|root| selected.get(lower(root.range.end)))
                .is_some_and(|token| token.kind == TokenKind::Punctuation && token.text == "=")
            {
                defaults.push(interval);
            }
        }
    }
    defaults.sort_by_key(|(first, last)| last - first);
    let count = |value: usize| u32::try_from(value).map_or(f64::INFINITY, f64::from);
    let kinds = selected
        .iter()
        .map(|token| {
            match token.kind {
                TokenKind::Identifier => "identifier",
                TokenKind::Keyword => "keyword",
                TokenKind::Punctuation => "punct",
            }
            .to_string()
        })
        .collect::<Vec<_>>();
    let values = selected
        .iter()
        .map(|token| token.text.clone())
        .collect::<Vec<_>>();
    let candidates = selected
        .iter()
        .map(|token| identifiers.contains(&(token.range.start, token.range.end)))
        .collect::<Vec<_>>();
    let plan = rules::plan_java_script_source_parameter_bindings(
        &kinds,
        &values,
        &candidates,
        &defaults
            .iter()
            .map(|(first, _)| count(*first))
            .collect::<Vec<_>>(),
        &defaults
            .iter()
            .map(|(_, last)| count(*last))
            .collect::<Vec<_>>(),
        &computed
            .iter()
            .map(|(first, _)| count(*first))
            .collect::<Vec<_>>(),
        &computed
            .iter()
            .map(|(_, last)| count(*last))
            .collect::<Vec<_>>(),
    );
    if plan == [-1.0] {
        return Vec::new();
    }
    let mut result = Vec::new();
    for value in plan {
        if !value.is_finite()
            || value < 0.0
            || value >= count(selected.len())
            || value.fract() != 0.0
        {
            return Vec::new();
        }
        let Ok(index) = value.to_string().parse::<usize>() else {
            return Vec::new();
        };
        result.push(first + index);
    }
    result
}
