//! Authenticate comparison-sensitive const proof order before value projection.
use super::super::{Result, error, field_of};
use crate::translation::source_const_functions::policy;
use crate::{LinkId, LinkNetwork};
use serde_json::{Map, Value};
use std::collections::{HashMap, HashSet};

type Fields = HashMap<LinkId, Vec<(String, LinkId)>>;

fn field(fields: &Fields, owner: LinkId, name: &str) -> Option<LinkId> {
    fields
        .get(&owner)?
        .iter()
        .find(|(label, _)| field_of(label) == name)
        .map(|(_, child)| *child)
}

fn compare_order(
    network: &LinkNetwork,
    fields: &Fields,
    actual: LinkId,
    expected: &Value,
    depth: usize,
) -> Result<()> {
    if depth > 256 {
        return Err(error(
            "Rust const function proof: interpretation order exceeds its traversal limit",
        ));
    }
    match expected {
        Value::Object(expected) => {
            let original = fields.get(&actual).map(Vec::as_slice).unwrap_or_default();
            if original.len() != expected.len()
                || !original
                    .iter()
                    .zip(expected.keys())
                    .all(|((label, _), name)| field_of(label) == *name)
            {
                return Err(error(
                    "Rust const function proof: the actual const source qualification or observation boundary changed",
                ));
            }
            for ((_, child), expected) in original.iter().zip(expected.values()) {
                compare_order(network, fields, *child, expected, depth + 1)?;
            }
        }
        Value::Array(expected) => {
            let children = network
                .link(actual)
                .ok_or_else(|| error("a const interpretation value is missing"))?
                .references();
            if children.len() != expected.len() {
                return Err(error(
                    "Rust const function proof: the actual const source qualification or observation boundary changed",
                ));
            }
            for (child, expected) in children.iter().zip(expected) {
                compare_order(network, fields, *child, expected, depth + 1)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn selected(network: &LinkNetwork, fields: &Fields, body: LinkId) -> Result<bool> {
    let mut pending = vec![body];
    let mut visited = HashSet::new();
    while let Some(id) = pending.pop() {
        if !visited.insert(id) {
            continue;
        }
        if visited.len() > 100_000 {
            return Err(error(
                "Rust const function proof: interpretation order exceeds its traversal limit",
            ));
        }
        let link = network
            .link(id)
            .ok_or_else(|| error("a const program value is missing"))?;
        pending.extend_from_slice(link.references());
        for (label, child) in fields.get(&id).into_iter().flatten() {
            if field_of(label) == "sourceConstFunction" {
                return Ok(true);
            }
            pending.push(*child);
        }
    }
    Ok(false)
}

/// Check original declaration identity/order and shared interpretation key order.
pub(super) fn validate_order(
    network: &LinkNetwork,
    fields: &Fields,
    mapping: LinkId,
    body: LinkId,
) -> Result<()> {
    if !selected(network, fields, body)? {
        return Ok(());
    }
    let mapping = network
        .link(mapping)
        .ok_or_else(|| error("a const program declaration mapping is missing"))?;
    let mut declarations = Vec::with_capacity(mapping.references().len());
    for pair in mapping.references() {
        let pair = network
            .link(*pair)
            .ok_or_else(|| error("a const declaration entry is missing"))?;
        let [_, declaration] = pair.references() else {
            return Err(error("a const declaration entry is malformed"));
        };
        declarations.push(*declaration);
    }
    let items = field(fields, body, "items").and_then(|items| network.link(items)).ok_or_else(|| error("Rust const function proof: the actual ordered declarations differ from the program items"))?;
    if items.references() != declarations.as_slice() {
        return Err(error(
            "Rust const function proof: the actual ordered declarations differ from the program items",
        ));
    }
    for declaration in declarations {
        let Some(interpretation) = field(fields, declaration, "sourceConstFunction")
            .and_then(|proof| field(fields, proof, "interpretation"))
        else {
            continue;
        };
        compare_order(
            network,
            fields,
            interpretation,
            &policy()["interpretation"],
            0,
        )?;
    }
    Ok(())
}

fn restore_order(value: &mut Value, expected: &Value) -> Result<()> {
    let Some(expected) = expected.as_object() else {
        return Ok(());
    };
    let Some(record) = value.as_object_mut() else {
        return Ok(());
    };
    if record.len() != expected.len() || expected.keys().any(|name| !record.contains_key(name)) {
        return Err(error(
            "Rust const function proof: the actual const source qualification or observation boundary changed",
        ));
    }
    let mut original = std::mem::take(record);
    let mut ordered = Map::new();
    for (name, expected) in expected {
        let mut child = original
            .remove(name)
            .ok_or_else(|| error("a const interpretation role is missing"))?;
        restore_order(&mut child, expected)?;
        ordered.insert(name.clone(), child);
    }
    *record = ordered;
    Ok(())
}

/// Restore only the interpretation order authenticated on original relations.
pub(super) fn prepare_order(program: &mut Value) -> Result<()> {
    let Some(declarations) = program["declarations"].as_array_mut() else {
        return Ok(());
    };
    for declaration in declarations {
        if let Some(proof) = declaration.get_mut("sourceConstFunction")
            && let Some(interpretation) = proof.get_mut("interpretation")
        {
            restore_order(interpretation, &policy()["interpretation"])?;
        }
    }
    Ok(())
}
