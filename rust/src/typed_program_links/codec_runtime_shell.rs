//! Identity certificates for the complete runtime graph before tree projection.
//! References are checked against real owned LinkIds; equal record copies fail.
use super::super::{Result, error};
use super::{Data, Decoder};
use crate::LinkId;
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};

/// Native owned-vector references reattach the same public graph nodes.
pub(super) fn reference_path(value: &Value) -> Result<Option<String>> {
    let Some(record) = value.as_object() else {
        return Ok(None);
    };
    if record.len() != 1 {
        return Ok(None);
    }
    for (field, prefix) in [
        ("declarationPosition", "/declarations"),
        ("tokenPosition", "/sourceRuntimeShell/tokens"),
        ("bindingPosition", "/sourceRuntimeShell/tokens"),
    ] {
        if let Some(index) = record.get(field) {
            let index = index
                .as_u64()
                .ok_or_else(|| error("a runtime structural reference has no owned position"))?;
            return Ok(Some(format!("{prefix}/{index}")));
        }
    }
    if let Some(path) = record.get("operationPosition") {
        let mut result = "/sourceRuntimeShell/operations".to_owned();
        for index in path
            .as_array()
            .ok_or_else(|| error("a runtime operation reference has no owned path"))?
        {
            let index = index
                .as_u64()
                .ok_or_else(|| error("a runtime operation reference has no owned position"))?;
            result.push_str("/parts/");
            result.push_str(&index.to_string());
        }
        return Ok(Some(result));
    }
    Ok(None)
}
struct Graph<'a, 'b> {
    decoder: &'a Decoder<'b>,
    tokens: HashMap<LinkId, usize>,
    operations: HashMap<LinkId, Vec<usize>>,
    remaining: usize,
}
impl Graph<'_, '_> {
    fn operation(&mut self, id: LinkId, raw: &Value, path: &mut Vec<usize>) -> Result<Value> {
        self.remaining = self
            .remaining
            .checked_sub(1)
            .ok_or_else(|| error("the runtime operation graph exceeds its traversal limit"))?;
        if let Some(index) = self.tokens.get(&id) {
            return Ok(json!({"tokenPosition":index}));
        }
        if self.decoder.tag(id, "kind") != "source operation"
            || !matches!(self.decoder.values.get(&id), Some(Data::Record(_)))
        {
            return Err(error(
                "a runtime operation part is not an actual owned source occurrence",
            ));
        }
        if self.operations.insert(id, path.clone()).is_some() {
            return Err(error("a runtime operation has multiple structural owners"));
        }
        let parts = self
            .decoder
            .field(id, "parts")
            .ok_or_else(|| error("an owned runtime operation has no parts"))?;
        let children = self.decoder.array(parts)?.to_vec();
        let raw_parts = raw["parts"]
            .as_array()
            .ok_or_else(|| error("an owned runtime operation has no ordered parts"))?;
        if children.len() != raw_parts.len() {
            return Err(error(
                "the rendered runtime operation lost an actual source occurrence",
            ));
        }
        let mut normalized = raw.clone();
        let mut parts = Vec::new();
        for (index, (child, raw)) in children.iter().zip(raw_parts).enumerate() {
            path.push(index);
            parts.push(self.operation(*child, raw, path)?);
            path.pop();
        }
        normalized["parts"] = Value::Array(parts);
        Ok(normalized)
    }
}
/// Validate actual public graph ownership before Serde can lose shared identity.
pub(super) fn normalize(
    decoder: &Decoder<'_>,
    body: LinkId,
    declarations: &[LinkId],
    value: &mut Value,
) -> Result<()> {
    let Some(root) = decoder.field(body, "sourceRuntimeShell") else {
        return Ok(());
    };
    let tokens_id = decoder
        .field(root, "tokens")
        .ok_or_else(|| error("the complete runtime requires its actual ordered source tokens"))?;
    let tokens = decoder.array(tokens_id)?.to_vec();
    if tokens.is_empty() || tokens.len() > 100_000 {
        return Err(error(
            "the complete runtime requires bounded actual source tokens",
        ));
    }
    let positions = tokens
        .iter()
        .enumerate()
        .map(|(index, id)| (*id, index))
        .collect::<HashMap<_, _>>();
    if positions.len() != tokens.len() {
        return Err(error(
            "each actual runtime source occurrence requires its distinct token identity",
        ));
    }
    let declaration_positions = declarations
        .iter()
        .enumerate()
        .map(|(index, id)| (*id, index))
        .collect::<HashMap<_, _>>();
    let mut proof = value["sourceRuntimeShell"].clone();
    let owned_tokens = proof["tokens"]
        .as_array_mut()
        .ok_or_else(|| error("the rendered runtime lost its ordered source tokens"))?;
    if owned_tokens.len() != tokens.len() {
        return Err(error(
            "the rendered runtime changed the source occurrence count",
        ));
    }
    for (index, id) in tokens.iter().enumerate() {
        if !matches!(decoder.values.get(id), Some(Data::Record(_))) {
            return Err(error(
                "an actual runtime source occurrence requires its owned record",
            ));
        }
        if let Some(target) = decoder.field(*id, "declaration") {
            let position = declaration_positions.get(&target).ok_or_else(|| {
                error("a runtime source occurrence references an unowned declaration identity")
            })?;
            owned_tokens[index]["declaration"] = json!({"declarationPosition":position});
        }
        if let Some(target) = decoder.field(*id, "binding") {
            let position = positions.get(&target).ok_or_else(|| {
                error("a runtime source occurrence references an unowned lexical token identity")
            })?;
            owned_tokens[index]["binding"] = json!({"bindingPosition":position});
        }
    }
    let operation_id = decoder
        .field(root, "operations")
        .ok_or_else(|| error("the complete runtime has no owned operation tree"))?;
    let mut graph = Graph {
        decoder,
        tokens: positions,
        operations: HashMap::new(),
        remaining: 100_000,
    };
    proof["operations"] = graph.operation(operation_id, &proof["operations"], &mut Vec::new())?;
    let assumptions = decoder
        .field(root, "assumptions")
        .ok_or_else(|| error("the complete runtime has no owned conditional assumptions"))?;
    let assumption_ids = decoder.array(assumptions)?;
    let rows = proof["assumptions"]
        .as_array_mut()
        .ok_or_else(|| error("the complete runtime assumptions require an ordered list"))?;
    if rows.len() != assumption_ids.len() {
        return Err(error(
            "the rendered runtime changed its conditional assumptions",
        ));
    }
    let mut owners = HashSet::new();
    for (row, id) in rows.iter_mut().zip(assumption_ids) {
        if !owners.insert(*id) {
            return Err(error(
                "each runtime assumption requires its actual owned record",
            ));
        }
        let target = decoder.field(*id, "operationNode").ok_or_else(|| {
            error("a conditional runtime assumption has no actual operation link")
        })?;
        let path = graph.operations.get(&target).ok_or_else(|| {
            error("a conditional runtime assumption references an unowned operation identity")
        })?;
        row["operationNode"] = json!({"operationPosition":path});
    }
    value["sourceRuntimeShell"] = proof;
    Ok(())
}
