//! Plain-fact transport for the shared field-view rules and retained witnesses.

use std::cmp::Ordering;

use serde_json::Value;

use super::super::field_range_source::FieldRangeView;
use super::{GrammarRuntimeError, ValidationBudget, invalid, number, read_included_range_columns};
use crate::translation::grammar_selection_field_range_rules::{
    accept_grammar_selection_field_range_selector, read_grammar_selection_field_range_field_order,
};
use crate::translation::grammar_selection_included_range_rules::{
    accept_grammar_selection_included_range_agreement,
    accept_grammar_selection_included_range_source_range,
};

pub(super) fn accept_field_range_keys(value: &Value, role: &str) -> bool {
    let Some(fields) = value.as_object() else {
        return false;
    };
    let plan = read_grammar_selection_field_range_field_order(role);
    fields.keys().map(String::as_str).eq(plan.split('|'))
}

pub(super) fn field_selector(
    selector: &Value,
    budget: &ValidationBudget,
) -> Result<(), GrammarRuntimeError> {
    budget.step()?;
    if !accept_field_range_keys(selector, "selector")
        || !accept_grammar_selection_field_range_selector(
            selector["kind"].as_str().unwrap_or_default(),
            selector["label"].as_str().unwrap_or_default(),
            selector["mode"].as_str().unwrap_or_default(),
            number(&selector["first"]),
            number(&selector["count"]),
        )
    {
        return Err(invalid("invalid source field selector"));
    }
    Ok(())
}

// JavaScript's ordinary fact equality compares numeric values, independently
// of their finite JSON spelling. Witness metadata below instead uses Object.is.
fn same_field_component(actual: &Value, expected: &Value) -> bool {
    match (actual, expected) {
        (Value::Number(_), Value::Number(_)) => {
            number(actual).partial_cmp(&number(expected)) == Some(Ordering::Equal)
        }
        _ => actual == expected,
    }
}

pub(super) fn field_facts_agree(
    actual: &Value,
    expected: &Value,
    budget: &ValidationBudget,
) -> Result<(), GrammarRuntimeError> {
    field_selector(&actual["selector"], budget)?;
    field_selector(&expected["selector"], budget)?;
    for key in read_grammar_selection_field_range_field_order("selector").split('|') {
        budget.step()?;
        if !same_field_component(&actual["selector"][key], &expected["selector"][key]) {
            return Err(invalid("source field selectors differ"));
        }
    }
    field_occurrences_agree(&actual["occurrences"], &expected["occurrences"], budget)?;
    let actual = read_included_range_columns(&actual["ranges"], budget)?;
    let expected = read_included_range_columns(&expected["ranges"], budget)?;
    if !accept_grammar_selection_included_range_agreement(&actual, &expected) {
        return Err(invalid("ordered field source ranges differ"));
    }
    Ok(())
}

fn field_occurrences_agree(
    actual: &Value,
    expected: &Value,
    budget: &ValidationBudget,
) -> Result<(), GrammarRuntimeError> {
    let occurrences = actual
        .as_array()
        .ok_or_else(|| invalid("ordered source field occurrences differ"))?;
    let others = expected
        .as_array()
        .filter(|others| others.len() == occurrences.len())
        .ok_or_else(|| invalid("ordered source field occurrences differ"))?;
    for (occurrence, other) in occurrences.iter().zip(others) {
        budget.step()?;
        if !accept_field_range_keys(occurrence, "occurrence")
            || !same_field_component(&occurrence["start"], &other["start"])
            || !same_field_component(&occurrence["end"], &other["end"])
        {
            return Err(invalid("ordered source field occurrences differ"));
        }
        let path = occurrence["path"]
            .as_array()
            .ok_or_else(|| invalid("ordered source field occurrences differ"))?;
        let other_path = other["path"]
            .as_array()
            .filter(|other| other.len() == path.len())
            .ok_or_else(|| invalid("ordered source field occurrences differ"))?;
        for (component, other) in path.iter().zip(other_path) {
            budget.step()?;
            if !same_field_component(component, other) {
                return Err(invalid("ordered source field occurrences differ"));
            }
        }
    }
    Ok(())
}

pub(super) fn field_view_agrees(
    view: &FieldRangeView,
    expected: &Value,
    budget: &ValidationBudget,
) -> Result<(), GrammarRuntimeError> {
    field_occurrences_agree(&view.occurrences, &expected["occurrences"], budget)?;
    budget.retain(view.ranges.len() * 2 + 1)?;
    let mut columns = Vec::new();
    for range in &view.ranges {
        budget.step()?;
        if !accept_grammar_selection_included_range_source_range("source-range", range[0], range[1])
        {
            return Err(invalid("invalid included source ranges"));
        }
        columns.extend(range);
    }
    let expected = read_included_range_columns(&expected["ranges"], budget)?;
    if !accept_grammar_selection_included_range_agreement(&columns, &expected) {
        return Err(invalid("ordered field source ranges differ"));
    }
    Ok(())
}

pub(super) fn same_bytes(
    actual: &[u8],
    expected: &[u8],
    begin: usize,
    budget: &ValidationBudget,
) -> Result<bool, GrammarRuntimeError> {
    for (index, byte) in actual.iter().enumerate() {
        budget.step()?;
        if expected.get(begin + index) != Some(byte) {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Exact typed ordered facts, matching the JavaScript Object.is scalar rule.
pub(super) fn same_field_witness(
    actual: &Value,
    expected: &Value,
    depth: usize,
    budget: &ValidationBudget,
) -> Result<bool, GrammarRuntimeError> {
    budget.depth(depth)?;
    match (actual, expected) {
        (Value::String(actual), Value::String(expected)) => {
            if actual.len() != expected.len() {
                return Ok(false);
            }
            for (unit, other) in actual.encode_utf16().zip(expected.encode_utf16()) {
                budget.step()?;
                if unit != other {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        (Value::Number(_), Value::Number(_)) => {
            Ok(number(actual).to_bits() == number(expected).to_bits())
        }
        (Value::Array(actual), Value::Array(expected)) => {
            if actual.len() != expected.len() {
                return Ok(false);
            }
            budget.retain(actual.len() + expected.len() + 2)?;
            for (item, other) in actual.iter().zip(expected) {
                budget.step()?;
                if !same_field_witness(item, other, depth + 1, budget)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        (Value::Object(actual), Value::Object(expected)) => {
            budget.retain(actual.len() + expected.len() + 2)?;
            if actual.len() != expected.len() {
                return Ok(false);
            }
            for ((key, item), (other_key, other)) in actual.iter().zip(expected) {
                budget.step()?;
                if key != other_key || !same_field_witness(item, other, depth + 1, budget)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        _ => Ok(actual == expected),
    }
}
