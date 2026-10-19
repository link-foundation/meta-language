//! Budgeted transport of actual executor trees into physical witness records.

use serde_json::{Map, Value, json};

use super::super::executor::Run;
use super::super::operations::{Abort, OperationValue};
use super::super::results::{Children, Tree, TreeType};
use super::super::selection_source_mapping::{MappingBudget, MappingFailure};
use super::super::text::{hex_of, text_of};
use super::{ViewFailure, ViewResult, copy_metadata, copy_string, depth};

fn children(
    bytes: &[u8],
    values: &Children,
    budget: &impl MappingBudget,
    level: usize,
    remaining: usize,
) -> ViewResult<Value> {
    budget.mapping_retain(values.count.checked_add(2).ok_or(Abort::MemoryBudget)?)?;
    let mut copy = Vec::with_capacity(values.count);
    for child in values.iter() {
        budget.mapping_step()?;
        copy.push(node(bytes, child, budget, level + 1, remaining)?);
    }
    Ok(Value::Array(copy))
}

fn leaf(
    bytes: &[u8],
    node: &Tree,
    copy: &mut Value,
    budget: &impl MappingBudget,
) -> ViewResult<()> {
    let input = bytes
        .get(node.start..node.end)
        .ok_or(ViewFailure::Invalid(node.start))?;
    budget.mapping_retain(input.len().checked_add(1).ok_or(Abort::MemoryBudget)?)?;
    for _ in input {
        budget.mapping_step()?;
    }
    copy["text"] = text_of(bytes, node.start, node.end).map_or(Value::Null, Value::String);
    if copy["text"].is_null() {
        budget.mapping_retain(
            input
                .len()
                .checked_mul(4)
                .and_then(|n| n.checked_add(1))
                .ok_or(Abort::MemoryBudget)?,
        )?;
        copy["hex"] = Value::String(hex_of(bytes, node.start, node.end));
    }
    Ok(())
}

fn node(
    bytes: &[u8],
    tree: &Tree,
    budget: &impl MappingBudget,
    level: usize,
    remaining: usize,
) -> ViewResult<Value> {
    depth(budget, level, remaining)?;
    if tree.start > tree.end || tree.end > bytes.len() {
        return Err(ViewFailure::Invalid(tree.start));
    }
    if matches!(
        tree.ty,
        TreeType::Embed | TreeType::Error | TreeType::Missing
    ) && (tree.field.is_some() || tree.trivia)
    {
        return Err(ViewFailure::Invalid(tree.start));
    }
    budget.mapping_retain(6)?;
    let copy = match tree.ty {
        TreeType::Node | TreeType::Token => {
            let kind = if tree.ty == TreeType::Token && tree.projection_anonymous {
                Value::Null
            } else {
                tree.projection_name
                    .as_deref()
                    .or(tree.kind.as_deref())
                    .map_or(Ok(Value::Null), |name| {
                        copy_string(name, budget).map(Value::String)
                    })?
            };
            let mut copy =
                json!({"type":if tree.ty==TreeType::Node {"node"} else {"token"},"kind":kind});
            if let Some(field) = &tree.field {
                budget.mapping_retain(1)?;
                copy["field"] = Value::String(copy_string(field, budget)?);
            }
            if tree.trivia {
                budget.mapping_retain(1)?;
                copy["trivia"] = Value::Bool(true);
            }
            copy["start"] = json!(tree.start);
            copy["end"] = json!(tree.end);
            if tree.ty == TreeType::Node {
                copy["children"] = children(bytes, &tree.children, budget, level, remaining)?;
            }
            if let Some(attributes) = &tree.attributes {
                depth(budget, level + 1, remaining)?;
                budget
                    .mapping_retain(attributes.len().checked_add(2).ok_or(Abort::MemoryBudget)?)?;
                let mut values = Map::new();
                for (key, value) in super::super::attributes::attribute_entries(tree)
                    .expect("the physical tree retains its attribute values")
                {
                    budget.mapping_step()?;
                    let key = copy_string(key, budget)?;
                    budget.mapping_retain(6)?;
                    depth(budget, level + 2, remaining)?;
                    let value = match value {
                        OperationValue::Integer(value) => json!(value),
                        OperationValue::Text(value) => Value::String(copy_string(value, budget)?),
                    };
                    values.insert(key, value);
                }
                copy["attributes"] = Value::Object(values);
            }
            if tree.ty == TreeType::Token {
                if let Some(lexical) = &tree.lexical {
                    if !lexical.is_object() {
                        return Err(ViewFailure::Invalid(tree.start));
                    }
                    budget.mapping_retain(1)?;
                    copy["lexical"] = copy_metadata(lexical, budget, level + 1, remaining)?;
                }
                leaf(bytes, tree, &mut copy, budget)?;
            }
            copy
        }
        TreeType::Error => {
            let mut copy = json!({"type":"error","start":tree.start,"end":tree.end});
            leaf(bytes, tree, &mut copy, budget)?;
            copy
        }
        TreeType::Missing => {
            let kind = tree.kind.as_deref().map_or(Ok(Value::Null), |kind| {
                copy_string(kind, budget).map(Value::String)
            })?;
            let mut copy = json!({"type":"missing","kind":kind,"start":tree.start,"end":tree.end});
            if tree.literal {
                budget.mapping_retain(1)?;
                copy["literal"] = Value::Bool(true);
            }
            copy
        }
        TreeType::Embed => {
            let language = tree
                .language
                .as_deref()
                .ok_or(ViewFailure::Invalid(tree.start))?;
            let mut copy = json!({"type":"embed","language":copy_string(language,budget)?,"start":tree.start,"end":tree.end});
            if let Some(physical) = &tree.source_children {
                copy["sourceChildren"] = children(bytes, physical, budget, level, remaining)?;
            } else {
                let root = tree
                    .root
                    .as_deref()
                    .ok_or(ViewFailure::Invalid(tree.start))?;
                copy["root"] = node(bytes, root, budget, level + 1, remaining)?;
            }
            copy
        }
    };
    Ok(copy)
}

/// Only physical children are exposed; programs, decoded roots and alternative
/// interpretations remain outside the source witness and its budgeted copy.
pub(in super::super) fn physical_children(
    bytes: &[u8],
    values: &Children,
    budget: &impl MappingBudget,
    remaining_depth: usize,
) -> Run<Result<Value, MappingFailure>> {
    match children(bytes, values, budget, 0, remaining_depth) {
        Ok(value) => Ok(Ok(value)),
        Err(ViewFailure::Aborted(error)) => Err(error),
        Err(ViewFailure::Invalid(offset)) => Ok(Err(MappingFailure {
            reason: "invalidEmbeddingMapping",
            offset,
        })),
    }
}
