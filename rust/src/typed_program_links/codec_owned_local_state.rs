//! Preserve the selected proof's original property-order comparisons.
//! The semantic validator still owns values, types and source occurrence facts.
use super::super::{Result, error, field_of, typed_program_links_schema};
use crate::translation::rust::owned_local_state_rules::{
    accept_rust_owned_local_record_value, accept_rust_owned_local_source_span,
};
use crate::{LinkId, LinkNetwork, StructuredDataValue};
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

type Fields = HashMap<LinkId, Vec<(String, LinkId)>>;

fn policy() -> &'static StructuredDataValue {
    static POLICY: LazyLock<StructuredDataValue> = LazyLock::new(|| {
        crate::read_structured_data_lino_ordered(include_str!(
            "../data/rust-owned-local-state.lino"
        ))
        .expect("the shared ordered owned local state policy is valid")
    });
    &POLICY
}

fn member<'a>(value: &'a StructuredDataValue, name: &str) -> Result<&'a StructuredDataValue> {
    let StructuredDataValue::Object(entries) = value else {
        return Err(error(
            "the shared owned local state policy needs an ordered record",
        ));
    };
    entries
        .iter()
        .find(|(key, _)| key.iter().copied().eq(name.encode_utf16()))
        .map(|(_, value)| value)
        .ok_or_else(|| error("the shared owned local state policy is missing a role"))
}

fn field(fields: &Fields, record: LinkId, name: &str) -> Option<LinkId> {
    fields
        .get(&record)?
        .iter()
        .find(|(label, _)| field_of(label) == name)
        .map(|(_, value)| *value)
}

fn names(fields: &Fields, record: LinkId, excluded: &[&str]) -> Vec<String> {
    fields
        .get(&record)
        .into_iter()
        .flatten()
        .map(|(name, _)| field_of(name))
        .filter(|name| !excluded.contains(&name.as_str()))
        .collect()
}

fn compare_policy_order(
    fields: &Fields,
    record: LinkId,
    expected: &StructuredDataValue,
    excluded: &[&str],
) -> Result<()> {
    let StructuredDataValue::Object(entries) = expected else {
        return Ok(());
    };
    let expected_names = entries
        .iter()
        .map(|(name, _)| String::from_utf16(name).expect("policy keys are scalar text"))
        .collect::<Vec<_>>();
    if names(fields, record, excluded) != expected_names {
        return Err(error(
            "Rust owned local state proof: original owned property order differs from its shared interpretation",
        ));
    }
    for ((_, value), name) in entries.iter().zip(expected_names) {
        if let Some(child) = field(fields, record, &name) {
            compare_policy_order(fields, child, value, &[])?;
        }
    }
    Ok(())
}

fn occurrence(
    network: &LinkNetwork,
    fields: &Fields,
    body: LinkId,
    path: LinkId,
) -> Result<LinkId> {
    let path = network
        .link(path)
        .ok_or_else(|| error("an owned state occurrence path is missing"))?
        .references();
    if path.len() > 256 {
        return Err(error(
            "an owned state occurrence path exceeds its traversal limit",
        ));
    }
    let mut node = body;
    for part in path {
        let part = network
            .link(*part)
            .ok_or_else(|| error("an owned state occurrence path is missing"))?;
        let text = part
            .metadata()
            .definition()
            .ok_or_else(|| error("an owned state occurrence path is invalid"))?;
        node = if part.metadata().term()
            == typed_program_links_schema()["scalars"]["string"].as_str()
            && fields.contains_key(&node)
        {
            field(fields, node, text)
                .ok_or_else(|| error("an owned state occurrence path is invalid"))?
        } else if part.metadata().term()
            == typed_program_links_schema()["scalars"]["number"].as_str()
        {
            let list = network
                .link(node)
                .filter(|node| {
                    node.metadata().term()
                        == typed_program_links_schema()["containers"]["array"].as_str()
                })
                .ok_or_else(|| error("an owned state occurrence path is invalid"))?
                .references();
            let value: Value = serde_json::from_str(text)
                .map_err(|_| error("an owned state occurrence path is invalid"))?;
            let length = serde_json::Number::from(list.len())
                .as_f64()
                .ok_or_else(|| error("an owned state occurrence path is invalid"))?;
            let index =
                crate::translation::owned_local_state::read_owned_local_position(&value, length)
                    .map_err(|failure| error(failure.to_string()))?;
            list.get(index)
                .copied()
                .ok_or_else(|| error("an owned state occurrence path is invalid"))?
        } else {
            return Err(error("an owned state occurrence path is invalid"));
        };
    }
    Ok(node)
}

fn compare_occurrence_span_order(
    network: &LinkNetwork,
    fields: &Fields,
    body: LinkId,
    fact: LinkId,
) -> Result<()> {
    let Some(path) = field(fields, fact, "path") else {
        return Ok(());
    };
    let actual = occurrence(network, fields, body, path)?;
    let actual = field(fields, actual, "span");
    let recorded = field(fields, fact, "span");
    if let (Some(actual), Some(recorded)) = (actual, recorded)
        && names(fields, actual, &[]) != names(fields, recorded, &[])
    {
        return Err(error(
            "Rust owned local state proof: original source span property order differs from its actual occurrence",
        ));
    }
    Ok(())
}

fn source_offset(network: &LinkNetwork, id: Option<LinkId>) -> Option<f64> {
    let link = network.link(id?)?;
    if link.metadata().term() != typed_program_links_schema()["scalars"]["number"].as_str() {
        return None;
    }
    serde_json::from_str::<Value>(link.metadata().definition()?)
        .ok()?
        .as_f64()
}

fn validate_original_shapes(network: &LinkNetwork, fields: &Fields, body: LinkId) -> Result<()> {
    let policy = crate::translation::owned_local_state::policy();
    let main = field(fields, body, "main");
    let mut pending = vec![body];
    let mut visited = HashSet::new();
    while let Some(id) = pending.pop() {
        if !visited.insert(id) {
            continue;
        }
        if visited.len() > 100_000 {
            return Err(error(
                "owned state record shapes exceed their traversal limit",
            ));
        }
        let link = network
            .link(id)
            .ok_or_else(|| error("an owned state value is missing"))?;
        pending.extend_from_slice(link.references());
        let context = if id == body {
            "program"
        } else if main == Some(id) {
            "main"
        } else {
            field(fields, id, "k")
                .and_then(|id| network.link(id))
                .and_then(|link| link.metadata().definition())
                .unwrap_or_default()
        };
        for (label, child) in fields.get(&id).into_iter().flatten() {
            pending.push(*child);
            let name = field_of(label);
            let child_link = network
                .link(*child)
                .ok_or_else(|| error("an owned state value is missing"))?;
            let required = policy["recordBooleanFields"][context]
                .as_array()
                .is_some_and(|names| {
                    names
                        .iter()
                        .any(|field| field.as_str() == Some(name.as_str()))
                });
            let boolean = child_link.metadata().term()
                == typed_program_links_schema()["scalars"]["boolean"].as_str()
                && matches!(child_link.metadata().definition(), Some("true" | "false"));
            if !accept_rust_owned_local_record_value(required, boolean) {
                return Err(error(
                    "Rust owned local state proof: a contextual flag has a non-Boolean value",
                ));
            }
            if name != "span" {
                continue;
            }
            let present = child_link.metadata().term()
                != typed_program_links_schema()["scalars"]["null"].as_str();
            let mut keys = names(fields, *child, &[]);
            keys.sort();
            let shape = keys.iter().map(String::as_str).eq(["end", "start"]);
            let offsets = match (
                source_offset(network, field(fields, *child, "start")),
                source_offset(network, field(fields, *child, "end")),
            ) {
                (Some(start), Some(end)) => {
                    super::super::source_span_rules::accept_source_span_offset(
                        start,
                        start == 0.0 && start.is_sign_negative(),
                    ) && super::super::source_span_rules::accept_source_span_offset(
                        end,
                        end == 0.0 && end.is_sign_negative(),
                    ) && super::super::source_span_rules::accept_source_span_interval(start, end)
                }
                _ => false,
            };
            if !accept_rust_owned_local_source_span(present, shape, offsets) {
                return Err(error(
                    "Rust owned local state proof: a source span has an invalid owned shape or offset",
                ));
            }
        }
    }
    Ok(())
}

/// Inspect retained original relations before projecting proof records to values.
pub(super) fn validate_order(
    network: &LinkNetwork,
    fields: &Fields,
    declarations: &[LinkId],
    body: LinkId,
) -> Result<()> {
    if !declarations
        .iter()
        .any(|id| field(fields, *id, "sourceLocalState").is_some())
    {
        return Ok(());
    }
    validate_original_shapes(network, fields, body)?;
    for declaration in declarations {
        let Some(proof) = field(fields, *declaration, "sourceLocalState") else {
            continue;
        };
        if let Some(interpretation) = field(fields, proof, "interpretation") {
            compare_policy_order(
                fields,
                interpretation,
                member(policy(), "interpretation")?,
                &[],
            )?;
        }
        let body = field(fields, *declaration, "body")
            .ok_or_else(|| error("an owned state declaration has no actual body"))?;
        for role in ["reads", "literalConstructors"] {
            let Some(array) = field(fields, proof, role) else {
                continue;
            };
            let facts = network
                .link(array)
                .ok_or_else(|| error("an owned state fact list is missing"))?;
            for fact in facts.references() {
                if role == "literalConstructors" {
                    compare_policy_order(
                        fields,
                        *fact,
                        member(policy(), "constructor")?,
                        &["span", "callSpan", "path"],
                    )?;
                }
                compare_occurrence_span_order(network, fields, body, *fact)?;
            }
        }
    }
    Ok(())
}

fn reorder(value: &mut Value, expected: &StructuredDataValue) -> Result<()> {
    let StructuredDataValue::Object(entries) = expected else {
        return Ok(());
    };
    let record = value
        .as_object_mut()
        .ok_or_else(|| error("an owned state role must remain an actual record"))?;
    let mut original = std::mem::take(record);
    let mut ordered = Map::new();
    for (name, child_policy) in entries {
        let name = String::from_utf16(name).expect("policy keys are scalar text");
        let mut child = original
            .remove(&name)
            .ok_or_else(|| error("an owned state role is missing a shared field"))?;
        reorder(&mut child, child_policy)?;
        ordered.insert(name, child);
    }
    ordered.extend(original);
    *record = ordered;
    Ok(())
}

fn value_occurrence<'a>(mut value: &'a Value, path: &Value) -> Result<&'a Value> {
    let path = path
        .as_array()
        .ok_or_else(|| error("an owned state occurrence needs its ordered path"))?;
    if path.len() > 256 {
        return Err(error(
            "an owned state occurrence path exceeds its traversal limit",
        ));
    }
    for part in path {
        value = if let Some(field) = part.as_str() {
            value.get(field)
        } else {
            let values = value
                .as_array()
                .ok_or_else(|| error("an owned state occurrence path is invalid"))?;
            let length = serde_json::Number::from(values.len())
                .as_f64()
                .ok_or_else(|| error("an owned state occurrence path is invalid"))?;
            let index =
                crate::translation::owned_local_state::read_owned_local_position(part, length)
                    .map_err(|failure| error(failure.to_string()))?;
            values.get(index)
        }
        .ok_or_else(|| error("an owned state occurrence path is invalid"))?;
    }
    Ok(value)
}

/// Restore only comparison-sensitive selected roles after checked IR projection.
pub(super) fn prepare_order(program: &mut Value) -> Result<()> {
    let Some(declarations) = program["declarations"].as_array_mut() else {
        return Ok(());
    };
    for declaration in declarations {
        if declaration.get("sourceLocalState").is_none() {
            continue;
        }
        let body = declaration["body"].clone();
        let Some(proof) = declaration.get_mut("sourceLocalState") else {
            continue;
        };
        reorder(
            &mut proof["interpretation"],
            member(policy(), "interpretation")?,
        )?;
        for role in ["reads", "literalConstructors"] {
            let facts = proof[role]
                .as_array_mut()
                .ok_or_else(|| error("owned state facts need their ordered roles"))?;
            for fact in facts {
                if role == "literalConstructors" {
                    reorder(fact, member(policy(), "constructor")?)?;
                }
                let actual = value_occurrence(&body, &fact["path"])?;
                // Actual spans serialize in the host Span's order. The raw guard
                // already checked their original relation-order agreement.
                if !actual["span"].is_null() {
                    fact["span"] = actual["span"].clone();
                }
            }
        }
    }
    Ok(())
}

fn normalize_span_order(value: &mut Value, depth: usize, remaining: &mut usize) -> Result<()> {
    *remaining = remaining
        .checked_sub(1)
        .ok_or_else(|| error("owned state span order exceeds its traversal limit"))?;
    if depth > 256 {
        return Err(error("owned state span order exceeds its traversal limit"));
    }
    if let Some(values) = value.as_array_mut() {
        for child in values {
            normalize_span_order(child, depth + 1, remaining)?;
        }
    } else if let Some(record) = value.as_object_mut() {
        if let Some(span) = record.get_mut("span").filter(|span| !span.is_null()) {
            let start = span["start"].clone();
            let end = span["end"].clone();
            let mut ordered = Map::new();
            ordered.insert("start".to_owned(), start);
            ordered.insert("end".to_owned(), end);
            *span = Value::Object(ordered);
        }
        for child in record.values_mut() {
            normalize_span_order(child, depth + 1, remaining)?;
        }
    }
    Ok(())
}

/// Canonicalize selected order only after original graph and value validation.
pub(super) fn prepare_decoded_order(program: &mut Value) -> Result<()> {
    if let Some(declarations) = program["declarations"].as_array_mut() {
        let mut remaining = 100_000;
        for declaration in declarations
            .iter_mut()
            .filter(|declaration| declaration.get("sourceLocalState").is_some())
        {
            normalize_span_order(&mut declaration["body"], 0, &mut remaining)?;
        }
    }
    prepare_order(program)
}
