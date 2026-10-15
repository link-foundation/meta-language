//! Preserve the selected proof's original property-order comparisons.
//! The semantic validator still owns values, types and source occurrence facts.
use super::super::{Result, error, field_of};
use crate::{LinkId, LinkNetwork, StructuredDataValue};
use serde_json::{Map, Value};
use std::collections::HashMap;
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
        node = if fields.contains_key(&node) {
            field(fields, node, text)
                .ok_or_else(|| error("an owned state occurrence path is invalid"))?
        } else {
            let index = text
                .parse::<usize>()
                .map_err(|_| error("an owned state occurrence path is invalid"))?;
            network
                .link(node)
                .and_then(|node| node.references().get(index))
                .copied()
                .ok_or_else(|| error("an owned state occurrence path is invalid"))?
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

/// Inspect retained original relations before projecting proof records to values.
pub(super) fn validate_order(
    network: &LinkNetwork,
    fields: &Fields,
    declarations: &[LinkId],
) -> Result<()> {
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
            part.as_u64()
                .and_then(|index| usize::try_from(index).ok())
                .and_then(|index| value.get(index))
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
