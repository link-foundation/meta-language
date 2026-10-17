//! Physical field discovery and owned witnesses for the shared field plan.
//! This host adapter never resolves a grammar or traverses a decoded plane.

use serde_json::{Map, Value, json};

use super::executor::Run;
use super::operations::Abort;
use super::selection_source_mapping::{MappingBudget, MappingFailure, numeric};
use super::text::{hex_of, text_of};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::grammar_selection_field_range_rules::{
    accept_grammar_selection_field_range_match, accept_grammar_selection_field_range_selector,
    read_grammar_selection_field_range_children, read_grammar_selection_field_range_plan,
};
use crate::translation::grammar_selection_mapping_rules::accept_grammar_selection_physical_span;

#[path = "field_range_source/physical.rs"]
mod physical;
pub(super) use physical::physical_children;

#[derive(Debug)]
pub(super) struct FieldRangeView {
    pub(super) ranges: Vec<[f64; 2]>,
    pub(super) occurrences: Value,
    pub(super) witness: Value,
}

enum ViewFailure {
    Invalid(usize),
    Aborted(Abort),
}

impl From<Abort> for ViewFailure {
    fn from(error: Abort) -> Self {
        Self::Aborted(error)
    }
}

type ViewResult<T> = Result<T, ViewFailure>;

fn number(value: &Value) -> f64 {
    value.as_f64().unwrap_or(f64::NAN)
}

fn position(value: &Value) -> Option<usize> {
    number_index(number(value))
}

fn text(value: Option<&Value>) -> Option<&str> {
    value.and_then(Value::as_str)
}

// Ordinary JSON values retain JavaScript's primitive truthiness here. This
// only transports existing public-tree flags; it never coerces a value to text.
fn metadata_truthy(value: Option<&Value>) -> bool {
    match value {
        None | Some(Value::Null) => false,
        Some(Value::Bool(value)) => *value,
        Some(Value::String(value)) => !value.is_empty(),
        Some(Value::Number(value)) => value.as_f64().is_some_and(|value| {
            value
                .partial_cmp(&0.0)
                .is_some_and(|order| order != std::cmp::Ordering::Equal)
        }),
        Some(Value::Array(_) | Value::Object(_)) => true,
    }
}

fn depth(budget: &impl MappingBudget, level: usize, remaining: usize) -> ViewResult<()> {
    budget.mapping_step()?;
    if level > remaining {
        return Err(ViewFailure::Aborted(Abort::NestingTooDeep));
    }
    Ok(())
}

fn copy_string(value: &str, budget: &impl MappingBudget) -> Run<String> {
    for _ in value.encode_utf16() {
        budget.mapping_step()?;
    }
    budget.mapping_retain(value.len().checked_add(1).ok_or(Abort::MemoryBudget)?)?;
    Ok(value.to_owned())
}

fn copy_metadata(
    value: &Value,
    budget: &impl MappingBudget,
    level: usize,
    remaining: usize,
) -> ViewResult<Value> {
    depth(budget, level, remaining)?;
    match value {
        Value::String(value) => Ok(Value::String(copy_string(value, budget)?)),
        Value::Array(values) => {
            budget.mapping_retain(values.len().checked_add(2).ok_or(Abort::MemoryBudget)?)?;
            let mut copy = Vec::with_capacity(values.len());
            for value in values {
                budget.mapping_step()?;
                copy.push(copy_metadata(value, budget, level + 1, remaining)?);
            }
            Ok(Value::Array(copy))
        }
        Value::Object(values) => {
            budget.mapping_retain(values.len().checked_add(1).ok_or(Abort::MemoryBudget)?)?;
            let mut copy = Map::new();
            for (key, value) in values {
                budget.mapping_step()?;
                let key = copy_string(key, budget)?;
                budget.mapping_retain(5)?;
                copy.insert(key, copy_metadata(value, budget, level + 1, remaining)?);
            }
            Ok(Value::Object(copy))
        }
        _ => {
            budget.mapping_retain(1)?;
            Ok(value.clone())
        }
    }
}

struct View<'a, B> {
    bytes: &'a [u8],
    budget: &'a B,
    start: usize,
    end: usize,
    cursor: usize,
    remaining: usize,
    label: &'a str,
    window: (usize, usize),
    matching: usize,
    spans: Vec<f64>,
    paths: Vec<Value>,
    path: Vec<Value>,
}

impl<B: MappingBudget> View<'_, B> {
    fn children(
        &mut self,
        values: &Value,
        role: &str,
        level: usize,
        select: bool,
    ) -> ViewResult<Value> {
        let values = values.as_array().ok_or(ViewFailure::Invalid(self.cursor))?;
        self.budget
            .mapping_retain(values.len().checked_add(2).ok_or(Abort::MemoryBudget)?)?;
        let mut copy = Vec::with_capacity(values.len());
        for (index, child) in values.iter().enumerate() {
            self.budget.mapping_step()?;
            self.budget.mapping_retain(3)?;
            self.path
                .push(Value::String(copy_string(role, self.budget)?));
            self.path.push(json!(index));
            copy.push(self.node(child, level + 1, select)?);
            self.path.truncate(self.path.len() - 2);
        }
        Ok(Value::Array(copy))
    }

    fn leaf(&self, node: &Value, copy: &mut Value, begin: usize, end: usize) -> ViewResult<()> {
        let bytes = self
            .bytes
            .get(begin..end)
            .ok_or(ViewFailure::Invalid(begin))?;
        self.budget
            .mapping_retain(bytes.len().checked_add(1).ok_or(Abort::MemoryBudget)?)?;
        for _ in bytes {
            self.budget.mapping_step()?;
        }
        copy["text"] = text_of(self.bytes, begin, end).map_or(Value::Null, Value::String);
        if copy["text"].is_null() {
            self.budget.mapping_retain(
                bytes
                    .len()
                    .checked_mul(4)
                    .and_then(|n| n.checked_add(1))
                    .ok_or(Abort::MemoryBudget)?,
            )?;
            copy["hex"] = Value::String(hex_of(self.bytes, begin, end));
        }
        if node.get("text").is_some_and(|value| value != &copy["text"])
            || node
                .get("hex")
                .is_some_and(|value| Some(value) != copy.get("hex"))
        {
            return Err(ViewFailure::Invalid(begin));
        }
        Ok(())
    }

    fn stored_metadata(
        &self,
        node: &Value,
        copy: &mut Value,
        key: &str,
        level: usize,
    ) -> ViewResult<()> {
        if let Some(value) = node.get(key).filter(|value| !value.is_null()) {
            if !value.is_object() {
                return Err(ViewFailure::Invalid(self.cursor));
            }
            self.budget.mapping_retain(1)?;
            copy[key] = copy_metadata(value, self.budget, level + 1, self.remaining)?;
        }
        Ok(())
    }

    fn node(&mut self, node: &Value, level: usize, select: bool) -> ViewResult<Value> {
        depth(self.budget, level, self.remaining)?;
        let fields = node.as_object().ok_or(ViewFailure::Invalid(self.cursor))?;
        let (Some(kind), Some(begin), Some(end)) = (
            text(node.get("type")),
            node.get("start").and_then(position),
            node.get("end").and_then(position),
        ) else {
            return Err(ViewFailure::Invalid(self.cursor));
        };
        let role = read_grammar_selection_field_range_children(
            kind,
            fields.contains_key("sourceChildren"),
        );
        if role.is_empty()
            || !accept_grammar_selection_physical_span(
                numeric(begin),
                numeric(end),
                numeric(self.cursor),
                numeric(end.saturating_sub(begin)),
                kind == "missing",
            )
            || end > self.end
        {
            return Err(ViewFailure::Invalid(begin));
        }
        if matches!(kind, "embed" | "error" | "missing")
            && (fields.contains_key("field") || fields.contains_key("trivia"))
        {
            return Err(ViewFailure::Invalid(begin));
        }
        let matched = select
            && accept_grammar_selection_field_range_match(
                text(node.get("field")).unwrap_or_default(),
                self.label,
                node.get("trivia").and_then(Value::as_bool).unwrap_or(false),
            );
        if matched {
            self.budget.mapping_retain(2)?;
            self.spans.extend([numeric(begin), numeric(end)]);
            if self.matching >= self.window.0 && self.matching < self.window.1 {
                self.budget
                    .mapping_retain(self.path.len().checked_add(2).ok_or(Abort::MemoryBudget)?)?;
                let mut path = Vec::with_capacity(self.path.len());
                for component in &self.path {
                    path.push(match component {
                        Value::String(role) => Value::String(copy_string(role, self.budget)?),
                        _ => {
                            self.budget.mapping_retain(1)?;
                            component.clone()
                        }
                    });
                }
                self.paths.push(Value::Array(path));
            }
            self.matching += 1;
        }
        self.budget.mapping_retain(6)?;
        let copy = match kind {
            "node" | "token" => {
                let projected =
                    if kind == "token" && metadata_truthy(node.get("projectionAnonymous")) {
                        Value::Null
                    } else {
                        let projected = node
                            .get("projectionName")
                            .filter(|value| !value.is_null())
                            .or_else(|| node.get("kind"))
                            .ok_or(ViewFailure::Invalid(begin))?;
                        if projected.is_null() {
                            Value::Null
                        } else {
                            Value::String(copy_string(
                                projected.as_str().ok_or(ViewFailure::Invalid(begin))?,
                                self.budget,
                            )?)
                        }
                    };
                let mut copy = json!({"type":kind,"kind":projected});
                if let Some(field) = node.get("field") {
                    if !field.is_null() && !field.is_string() {
                        return Err(ViewFailure::Invalid(begin));
                    }
                    self.budget.mapping_retain(1)?;
                    copy["field"] = if field.is_null() {
                        Value::Null
                    } else {
                        Value::String(copy_string(
                            field.as_str().ok_or(ViewFailure::Invalid(begin))?,
                            self.budget,
                        )?)
                    };
                }
                if let Some(trivia) = node.get("trivia") {
                    let trivia = trivia.as_bool().ok_or(ViewFailure::Invalid(begin))?;
                    if trivia {
                        self.budget.mapping_retain(1)?;
                        copy["trivia"] = Value::Bool(true);
                    }
                }
                copy["start"] = json!(begin);
                copy["end"] = json!(end);
                if role == "children" {
                    copy["children"] =
                        self.children(&node["children"], &role, level, select && !matched)?;
                } else {
                    self.cursor = end;
                }
                self.stored_metadata(node, &mut copy, "attributes", level)?;
                if kind == "token" {
                    self.stored_metadata(node, &mut copy, "lexical", level)?;
                    self.leaf(node, &mut copy, begin, end)?;
                }
                copy
            }
            "error" => {
                let mut copy = json!({"type":kind,"start":begin,"end":end});
                self.leaf(node, &mut copy, begin, end)?;
                if metadata_truthy(node.get("reason")) {
                    let reason =
                        text(node.get("reason")).ok_or(ViewFailure::Invalid(self.start))?;
                    self.budget.mapping_retain(1)?;
                    copy["reason"] = Value::String(copy_string(reason, self.budget)?);
                }
                self.cursor = end;
                copy
            }
            "missing" => {
                let name = text(node.get("kind")).ok_or(ViewFailure::Invalid(begin))?;
                let mut copy = json!({"type":kind,"kind":copy_string(name,self.budget)?,"start":begin,"end":end});
                if metadata_truthy(node.get("literal")) {
                    self.budget.mapping_retain(1)?;
                    copy["literal"] = Value::Bool(true);
                }
                copy
            }
            _ => {
                let language = text(node.get("language")).ok_or(ViewFailure::Invalid(begin))?;
                let mut copy = json!({"type":kind,"language":copy_string(language,self.budget)?,"start":begin,"end":end});
                if role == "sourceChildren" {
                    copy["sourceChildren"] =
                        self.children(&node["sourceChildren"], &role, level, select && !matched)?;
                } else {
                    copy["root"] = self.node(&node["root"], level + 1, false)?;
                }
                copy
            }
        };
        if !accept_grammar_selection_physical_span(
            numeric(begin),
            numeric(end),
            numeric(begin),
            numeric(self.cursor.saturating_sub(begin)),
            kind == "missing",
        ) {
            return Err(ViewFailure::Invalid(begin));
        }
        // Retain insertion order; this is the same public physical data shape.
        Ok(copy)
    }
}

/// Reconstruct the physical view from plain decoded records using the same
/// generated field decisions as the live tree adapter.
pub(super) fn read_field_range_view(
    bytes: &[u8],
    start: usize,
    end: usize,
    selector: &Value,
    children: &Value,
    budget: &impl MappingBudget,
    remaining_depth: usize,
) -> Run<Result<FieldRangeView, MappingFailure>> {
    let result = (|| -> ViewResult<FieldRangeView> {
        budget.mapping_retain(1)?;
        let (Some(kind), Some(label), Some(mode)) = (
            text(selector.get("kind")),
            text(selector.get("label")),
            text(selector.get("mode")),
        ) else {
            return Err(ViewFailure::Invalid(start));
        };
        let first = number(&selector["first"]);
        let count = number(&selector["count"]);
        if !accept_grammar_selection_field_range_selector(kind, label, mode, first, count)
            || start >= end
            || end > bytes.len()
        {
            return Err(ViewFailure::Invalid(start));
        }
        let (Some(window_start), Some(window_count)) = (number_index(first), number_index(count))
        else {
            return Err(ViewFailure::Invalid(start));
        };
        let following = window_start
            .checked_add(window_count)
            .ok_or(ViewFailure::Invalid(start))?;
        budget.mapping_retain(4)?;
        let mut view = View {
            bytes,
            budget,
            start,
            end,
            cursor: start,
            remaining: remaining_depth,
            label,
            window: (window_start, following),
            matching: 0,
            spans: Vec::new(),
            paths: Vec::new(),
            path: Vec::new(),
        };
        let physical = view.children(children, "children", 0, true)?;
        if view.cursor != end {
            return Err(ViewFailure::Invalid(view.cursor));
        }
        budget.mapping_retain(view.spans.len().checked_add(2).ok_or(Abort::MemoryBudget)?)?;
        for _ in &view.spans {
            budget.mapping_step()?;
        }
        let plan = read_grammar_selection_field_range_plan(
            &view.spans,
            numeric(start),
            numeric(end),
            first,
            count,
            mode,
        );
        if plan.is_empty() {
            return Err(ViewFailure::Invalid(start));
        }
        budget.mapping_retain((plan.len() / 2).checked_add(3).ok_or(Abort::MemoryBudget)?)?;
        let mut ranges = Vec::new();
        for pair in plan.as_chunks::<2>().0 {
            budget.mapping_step()?;
            budget.mapping_retain(3)?;
            ranges.push(*pair);
        }
        let mut occurrences = Vec::new();
        for (index, path) in view.paths.into_iter().enumerate() {
            budget.mapping_step()?;
            budget.mapping_retain(4)?;
            let at = window_start
                .checked_add(index)
                .and_then(|n| n.checked_mul(2))
                .ok_or(ViewFailure::Invalid(start))?;
            let (Some(begin), Some(end)) = (
                view.spans.get(at).copied().and_then(number_index),
                view.spans.get(at + 1).copied().and_then(number_index),
            ) else {
                return Err(ViewFailure::Invalid(start));
            };
            occurrences.push(json!({"path":path,"start":begin,"end":end}));
        }
        budget.mapping_retain(4)?;
        Ok(FieldRangeView {
            ranges,
            occurrences: Value::Array(occurrences),
            witness: json!({"start":start,"end":end,"children":physical}),
        })
    })();
    match result {
        Ok(view) => Ok(Ok(view)),
        Err(ViewFailure::Invalid(offset)) => Ok(Err(MappingFailure {
            reason: "invalidEmbeddingMapping",
            offset,
        })),
        Err(ViewFailure::Aborted(error)) => Err(error),
    }
}

pub(super) fn copy_field_value(
    value: &Value,
    budget: &impl MappingBudget,
    remaining_depth: usize,
) -> Run<Result<Value, MappingFailure>> {
    match copy_metadata(value, budget, 0, remaining_depth) {
        Ok(value) => Ok(Ok(value)),
        Err(ViewFailure::Aborted(error)) => Err(error),
        Err(ViewFailure::Invalid(offset)) => Ok(Err(MappingFailure {
            reason: "invalidEmbeddingMapping",
            offset,
        })),
    }
}
