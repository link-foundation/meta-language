//! Closed source-view transport before positional codecs can omit fields.

use serde_json::{Value, json};

use super::GrammarExpr;
use super::feature::{FeatureExpr, FeatureForm, FieldValue};
use crate::translation::grammar_selection_field_range_rules::{
    accept_grammar_selection_field_range_candidate, accept_grammar_selection_field_range_selector,
    read_grammar_selection_field_range_field_order,
};
use crate::translation::grammar_selection_included_range_rules::{
    accept_grammar_selection_included_range_candidate,
    accept_grammar_selection_included_range_source_range,
    read_grammar_selection_included_range_field_order,
};

fn check_field_count(form: &FeatureForm, role: &str) -> Result<(), String> {
    if form.fields.len() + 1
        != read_grammar_selection_field_range_field_order(role)
            .split('|')
            .count()
    {
        return Err("a field source view needs exactly its declared fields".to_owned());
    }
    Ok(())
}

fn read_selector_form(form: &FeatureForm) -> Result<Value, String> {
    check_field_count(form, "selector")?;
    let [
        FieldValue::Name(label),
        FieldValue::Word(mode),
        FieldValue::Integer(first),
        FieldValue::Integer(count),
    ] = form.fields.as_slice()
    else {
        return Err("a field selector needs its exact kind, label, mode and finite nonempty occurrence window".to_owned());
    };
    let number = |value| u32::try_from(value).map_or(f64::NAN, f64::from);
    if !accept_grammar_selection_field_range_selector(
        &form.head,
        label,
        mode,
        number(*first),
        number(*count),
    ) {
        return Err("a field selector needs its exact kind, label, mode and finite nonempty occurrence window".to_owned());
    }
    Ok(json!({"kind":form.head,"label":label,"mode":mode,"first":first,"count":count}))
}

pub(super) fn read_selection_field_selector(candidate: &FeatureForm) -> Result<Value, String> {
    let Some(GrammarExpr::Feature(feature)) = candidate.expression("selector") else {
        return Err("a field source view needs ordinary data records".to_owned());
    };
    let FeatureExpr::Form(selector) = &**feature else {
        return Err("a field source view needs exactly its declared fields".to_owned());
    };
    read_selector_form(selector)
}

pub(super) fn check_grammar_selection_field_range_form(form: &FeatureForm) -> Result<bool, String> {
    match form.head.as_str() {
        "source-field-selector" => {
            read_selector_form(form)?;
        }
        "grammar-field-candidate" => {
            check_field_count(form, "candidate")?;
            let [
                FieldValue::Name(name),
                FieldValue::Name(mapping),
                FieldValue::Expression(_),
            ] = form.fields.as_slice()
            else {
                return Err(
                    "a field candidate needs its name and explicit field-ranges mapping".to_owned(),
                );
            };
            if name.is_empty()
                || !accept_grammar_selection_field_range_candidate(&form.head, mapping)
            {
                return Err(
                    "a field candidate needs its name and explicit field-ranges mapping".to_owned(),
                );
            }
            read_selection_field_selector(form)?;
        }
        _ => return Ok(false),
    }
    Ok(true)
}

fn check_field_data_fields(value: &Value, role: &str) -> Result<(), String> {
    let fields = value
        .as_object()
        .ok_or("a field source view needs ordinary data records")?;
    let names = read_grammar_selection_field_range_field_order(role);
    if fields.len() != names.split('|').count()
        || fields
            .keys()
            .any(|name| !names.split('|').any(|expected| expected == name))
    {
        return Err("a field source view needs exactly its declared fields".to_owned());
    }
    Ok(())
}

fn check_selector_data(value: &Value) -> Result<(), String> {
    check_field_data_fields(value, "selector")?;
    if !accept_grammar_selection_field_range_selector(
        value["kind"].as_str().unwrap_or_default(),
        value["label"].as_str().unwrap_or_default(),
        value["mode"].as_str().unwrap_or_default(),
        value["first"].as_f64().unwrap_or(f64::NAN),
        value["count"].as_f64().unwrap_or(f64::NAN),
    ) {
        return Err("a field selector needs its exact kind, label, mode and finite nonempty occurrence window".to_owned());
    }
    Ok(())
}

fn check_field_data_form(value: &Value) -> Result<(), String> {
    match value["kind"].as_str().unwrap_or_default() {
        "source-field-selector" => check_selector_data(value)?,
        "grammar-field-candidate" => {
            check_field_data_fields(value, "candidate")?;
            if value["name"].as_str().is_none_or(str::is_empty)
                || !accept_grammar_selection_field_range_candidate(
                    "grammar-field-candidate",
                    value["mapping"].as_str().unwrap_or_default(),
                )
            {
                return Err(
                    "a field candidate needs its name and explicit field-ranges mapping".to_owned(),
                );
            }
            check_selector_data(&value["selector"])?;
        }
        _ => {}
    }
    Ok(())
}

/// Inspect every original raw value before a reader can omit unknown fields.
pub(super) fn check_grammar_selection_included_range_data(root: &Value) -> Result<(), String> {
    let exact = |value: &Value, role| {
        let names = read_grammar_selection_included_range_field_order(role);
        value.as_object().is_some_and(|fields| {
            fields.len() == names.split('|').count()
                && fields
                    .keys()
                    .all(|name| names.split('|').any(|expected| expected == name))
        })
    };
    let range = |value: &Value| -> Result<(), String> {
        if !value.is_object() {
            return Err("an included source view needs ordinary data records".to_owned());
        }
        if !exact(value, "range") {
            return Err("an included source view needs exactly its declared fields".to_owned());
        }
        if !accept_grammar_selection_included_range_source_range(
            value["kind"].as_str().unwrap_or_default(),
            value["start"].as_f64().unwrap_or(f64::NAN),
            value["end"].as_f64().unwrap_or(f64::NAN),
        ) {
            return Err(
                "an included source range needs its kind and nonempty integer byte bounds"
                    .to_owned(),
            );
        }
        Ok(())
    };
    let mut pending = vec![root];
    while let Some(value) = pending.pop() {
        if let Some(fields) = value.as_object() {
            check_field_data_form(value)?;
            match value["kind"].as_str().unwrap_or_default() {
                "source-range" => range(value)?,
                "grammar-range-candidate" => {
                    if !exact(value, "candidate") {
                        return Err(
                            "an included source view needs exactly its declared fields".to_owned()
                        );
                    }
                    if value["name"].as_str().is_none_or(str::is_empty)
                        || !accept_grammar_selection_included_range_candidate(
                            "grammar-range-candidate",
                            value["mapping"].as_str().unwrap_or_default(),
                        )
                    {
                        return Err("an included range candidate needs its name and explicit included-ranges mapping".to_owned());
                    }
                    let ranges = value["ranges"]
                        .as_array()
                        .ok_or("an included range candidate needs an ordinary range array")?;
                    if ranges.is_empty() {
                        return Err(
                            "an included range candidate needs a nonempty dense range array"
                                .to_owned(),
                        );
                    }
                    for row in ranges {
                        range(row)?;
                    }
                }
                _ => {}
            }
            pending.extend(fields.values());
        } else if let Some(items) = value.as_array() {
            pending.extend(items);
        }
    }
    Ok(())
}
