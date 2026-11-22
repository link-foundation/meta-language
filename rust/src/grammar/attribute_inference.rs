//! Concrete trace validation and native grammar execution for attribute inference.
use std::collections::BTreeSet;

use super::feature_runtime::{
    FeatureParseOptions, OperationValue, SyntaxAttributes, SyntaxTree, compile_feature_grammar,
};
use super::{Grammar, parse_grammar_links, percent_encode_links_text};
use crate::grammar_attribute_inference_rules as rules;

/// The selected rule, its output attribute and optional observed dependencies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GrammarAttributeInferenceRequest<'a> {
    /// The rule to enrich.
    pub rule: &'a str,
    /// The attribute assigned by the inferred action.
    pub attribute: &'a str,
    /// A named capture available as an operand.
    pub field: Option<&'a str>,
    /// An existing attribute on occurrences of the named capture.
    pub child_attribute: Option<&'a str>,
}

/// One instrumented observation selected by actual CST path and UTF-8 span.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarAttributeObservation {
    /// Exact source text supplied to the independent parser.
    pub source: String,
    /// Child indexes from the root, including trivia in the count.
    pub path: Vec<usize>,
    /// First byte of the observed rule instance.
    pub start: usize,
    /// Byte immediately following the observed instance.
    pub end: usize,
    /// Output seen by the independent instrumented parser.
    pub value: OperationValue,
}

/// All replay-surviving definitions and the uniquely selected grammar, if any.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GrammarAttributeInference {
    /// Native candidate value expressions in stable search order.
    pub candidates: Vec<String>,
    /// A grammar with the executable action when exactly one candidate survives.
    pub grammar: Option<Grammar>,
    /// Why inference needs a wider action vocabulary or more observations.
    pub diagnostic: Option<&'static str>,
}

fn fail(reason: &str) -> String {
    format!("grammar attribute inference: {reason}")
}

fn valid_name(value: &str) -> bool {
    !value.is_empty() && value.len() <= 4096
}

fn select_trace_node<'a>(
    mut node: &'a SyntaxTree,
    row: &GrammarAttributeObservation,
    rule: &str,
) -> Option<&'a SyntaxTree> {
    for &index in &row.path {
        let SyntaxTree::Node { children, .. } = node else {
            return None;
        };
        node = children.get(index)?;
    }
    if node.start() != row.start || node.end() != row.end {
        return None;
    }
    match node {
        SyntaxTree::Node {
            kind,
            trivia: false,
            ..
        }
        | SyntaxTree::Token {
            kind: Some(kind),
            trivia: false,
            ..
        } if kind == rule => Some(node),
        _ => None,
    }
}

fn trace_attributes(node: &SyntaxTree) -> Option<&SyntaxAttributes> {
    match node {
        SyntaxTree::Node { attributes, .. } | SyntaxTree::Token { attributes, .. } => {
            attributes.as_ref()
        }
        _ => None,
    }
}

fn action_grammar(
    grammar: &Grammar,
    rule: &str,
    attribute: &str,
    candidate: &str,
) -> Result<Grammar, String> {
    let attribute = percent_encode_links_text(attribute);
    let text = format!(
        "(grammar (start action))\n(rule action normal empty (action (setAttribute {attribute} {candidate})))\n"
    );
    let definition = parse_grammar_links(&text).map_err(|error| fail(&error.to_string()))?;
    let action = definition
        .rules
        .into_iter()
        .next()
        .and_then(|rule| rule.attributes.action);
    let mut output = grammar.clone();
    let owner = output
        .rules
        .iter_mut()
        .find(|entry| entry.name == rule)
        .ok_or_else(|| fail("expected an existing rule without an authored action"))?;
    owner.attributes.action = action;
    Ok(output)
}

/// Search the native action vocabulary using concrete observations. An
/// inconclusive observation set retains every surviving alternative.
///
/// # Errors
///
/// Returns a diagnostic for invalid observations, resource bounds or a grammar
/// the native runtime cannot load.
pub fn infer_grammar_attribute_action(
    grammar: &Grammar,
    request: GrammarAttributeInferenceRequest<'_>,
    observations: &[GrammarAttributeObservation],
) -> Result<GrammarAttributeInference, String> {
    let GrammarAttributeInferenceRequest {
        rule,
        attribute,
        field,
        child_attribute,
    } = request;
    if !valid_name(rule)
        || !valid_name(attribute)
        || field.is_some_and(|value| !valid_name(value))
        || child_attribute.is_some_and(|value| field.is_none() || !valid_name(value))
    {
        return Err(fail("invalid rule, attribute, or field selection"));
    }
    if grammar
        .rule(rule)
        .is_none_or(|owner| owner.attributes.action.is_some())
    {
        return Err(fail("expected an existing rule without an authored action"));
    }
    let limit = crate::translation::frontend_rules::ml_array::number_index(
        rules::read_grammar_attribute_observation_limit(),
    )
    .unwrap_or_default();
    if !(2..=limit).contains(&observations.len()) {
        return Err(fail(
            "expected bounded observations from at least two sources",
        ));
    }
    let byte_limit = crate::translation::frontend_rules::ml_array::number_index(
        rules::read_grammar_attribute_observation_byte_limit(),
    )
    .unwrap_or_default();
    let options = FeatureParseOptions::default();
    let parser = compile_feature_grammar(grammar, None, options.clone())
        .map_err(|error| fail(&error.to_string()))?;
    let mut sources = BTreeSet::new();
    let mut bytes = 0usize;
    for row in observations {
        if row.end < row.start
            || row.path.len() > 64
            || matches!(
                &row.value,
                OperationValue::Integer(value) if !(-(1_i64 << 53) + 1..(1_i64 << 53)).contains(value)
            )
        {
            return Err(fail("invalid concrete trace observation"));
        }
        sources.insert(row.source.as_str());
        let value_bytes = match &row.value {
            OperationValue::Text(value) => value.len(),
            OperationValue::Integer(_) => 8,
        };
        bytes = bytes
            .saturating_add(row.source.len())
            .saturating_add(value_bytes);
        if bytes > byte_limit {
            return Err(fail("observation byte limit exceeded"));
        }
        let outcome = parser
            .parse_tree(row.source.as_bytes(), &options)
            .map_err(|error| fail(&error.to_string()))?;
        if !outcome.ok
            || !outcome.ambiguities.is_empty()
            || outcome
                .tree
                .as_ref()
                .and_then(|tree| select_trace_node(tree, row, rule))
                .is_none()
        {
            return Err(fail(
                "observation does not select an accepted unambiguous concrete rule occurrence",
            ));
        }
    }
    if sources.len() < 2 {
        return Err(fail(
            "expected bounded observations from at least two sources",
        ));
    }
    let candidates = rules::read_grammar_attribute_candidate_values(
        field
            .map(percent_encode_links_text)
            .as_deref()
            .unwrap_or(""),
        child_attribute
            .map(percent_encode_links_text)
            .as_deref()
            .unwrap_or(""),
    );
    let mut output = GrammarAttributeInference {
        candidates: Vec::new(),
        grammar: None,
        diagnostic: None,
    };
    'candidate: for candidate in candidates {
        let graph = action_grammar(grammar, rule, attribute, &candidate)?;
        let parser = compile_feature_grammar(&graph, None, options.clone())
            .map_err(|error| fail(&error.to_string()))?;
        for row in observations {
            let outcome = parser
                .parse_tree(row.source.as_bytes(), &options)
                .map_err(|error| fail(&error.to_string()))?;
            let value = outcome
                .tree
                .as_ref()
                .and_then(|tree| select_trace_node(tree, row, rule))
                .and_then(trace_attributes)
                .and_then(|values| values.get(attribute));
            if !outcome.ok || !outcome.ambiguities.is_empty() || value != Some(&row.value) {
                continue 'candidate;
            }
        }
        output.candidates.push(candidate);
        output.grammar = Some(graph);
    }
    if output.candidates.len() != 1 {
        output.grammar = None;
        output.diagnostic = Some(if output.candidates.is_empty() {
            "no native action in the selected vocabulary matches the observations"
        } else {
            "more observations are needed to distinguish the native actions"
        });
    }
    Ok(output)
}
