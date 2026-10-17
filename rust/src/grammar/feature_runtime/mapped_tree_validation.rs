//! Inspectable ordinary-data adapter for generated source-mapping decisions.
//! Generic Links decoding alone does not authenticate mapping or grammar CSTs.

use std::cell::Cell;
use std::rc::Rc;

use serde_json::Value;

use super::operations::Abort;
use super::selection_source_mapping::{MappingBudget, SourceRegistry, numeric};
use super::{DEFAULT_MAX_DEPTH, DEFAULT_MEMORY_LIMIT, FeatureParseOptions, GrammarRuntimeError};
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::grammar_selection_mapping_rules::{
    accept_grammar_selection_mapping_descriptor, accept_grammar_selection_mapping_occurrence,
    accept_grammar_selection_mapping_profile, accept_grammar_selection_mapping_reference,
    accept_grammar_selection_physical_span, accept_grammar_selection_source_definition,
    accept_grammar_selection_source_derivation, accept_grammar_selection_source_interval,
    accept_grammar_selection_source_root, read_grammar_selection_mapping_field_order,
    read_grammar_selection_tree_children,
};

fn invalid(detail: &str) -> GrammarRuntimeError {
    GrammarRuntimeError {
        reason: "selection",
        message: format!("mapped grammar CST: {detail}"),
    }
}

fn aborted(abort: Abort) -> GrammarRuntimeError {
    let reason = match abort {
        Abort::StepLimit => "stepLimit",
        Abort::MemoryBudget => "memoryBudget",
        Abort::NestingTooDeep => "nestingDepth",
        Abort::Operation => "operation",
    };
    GrammarRuntimeError {
        reason,
        message: format!("mapped grammar CST: {reason} resource refusal"),
    }
}

fn number(value: &Value) -> f64 {
    value.as_f64().unwrap_or(f64::NAN)
}
fn offset(value: &Value) -> Option<usize> {
    number_index(number(value))
}

struct ValidationBudget {
    steps: Cell<usize>,
    cells: Cell<usize>,
    step_limit: usize,
    memory_limit: usize,
    max_depth: usize,
}

impl MappingBudget for ValidationBudget {
    fn mapping_step(&self) -> Result<(), Abort> {
        let steps = self.steps.get().checked_add(1).ok_or(Abort::StepLimit)?;
        self.steps.set(steps);
        if steps > self.step_limit {
            Err(Abort::StepLimit)
        } else {
            Ok(())
        }
    }
    fn mapping_retain(&self, count: usize) -> Result<(), Abort> {
        let cells = self
            .cells
            .get()
            .checked_add(count)
            .ok_or(Abort::MemoryBudget)?;
        self.cells.set(cells);
        if cells > self.memory_limit {
            Err(Abort::MemoryBudget)
        } else {
            Ok(())
        }
    }
}

impl ValidationBudget {
    fn depth(&self, depth: usize) -> Result<(), GrammarRuntimeError> {
        self.mapping_step().map_err(aborted)?;
        if depth > self.max_depth {
            Err(aborted(Abort::NestingTooDeep))
        } else {
            Ok(())
        }
    }
    fn retain(&self, count: usize) -> Result<(), GrammarRuntimeError> {
        self.mapping_retain(count).map_err(aborted)
    }
    fn step(&self) -> Result<(), GrammarRuntimeError> {
        self.mapping_step().map_err(aborted)
    }
    fn plain(&self, value: &Value, depth: usize) -> Result<(), GrammarRuntimeError> {
        self.depth(depth)?;
        match value {
            Value::Array(items) => {
                self.retain(items.len() + 2)?;
                for item in items {
                    self.step()?;
                    self.plain(item, depth + 1)?;
                }
                // The JS ordinary-data snapshot visits the own length
                // descriptor after its dense array elements.
                self.step()?;
            }
            Value::Object(fields) => {
                self.retain(fields.len() + 1)?;
                for item in fields.values() {
                    self.step()?;
                    self.plain(item, depth + 1)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

fn exact_keys(value: &Value, role: &str) -> bool {
    let Some(fields) = value.as_object() else {
        return false;
    };
    let plan = read_grammar_selection_mapping_field_order(role);
    fields.keys().map(String::as_str).eq(plan.split('|'))
}

fn physical(
    nodes: &[Value],
    begin: usize,
    end: usize,
    depth: usize,
    budget: &ValidationBudget,
) -> Result<Vec<u8>, GrammarRuntimeError> {
    let mut bytes = Vec::new();
    let mut cursor = begin;
    for node in nodes {
        physical_node(node, &mut cursor, &mut bytes, depth, budget)?;
    }
    if cursor != end {
        return Err(invalid("incomplete physical source plane"));
    }
    Ok(bytes)
}

fn physical_node(
    node: &Value,
    cursor: &mut usize,
    bytes: &mut Vec<u8>,
    depth: usize,
    budget: &ValidationBudget,
) -> Result<(), GrammarRuntimeError> {
    budget.depth(depth)?;
    budget.retain(1)?;
    if !node.is_object() {
        return Err(invalid("invalid physical tree"));
    }
    let begin = *cursor;
    let role = read_grammar_selection_tree_children(
        node["type"].as_str().unwrap_or_default(),
        node.get("sourceChildren").is_some(),
    );
    match role.as_str() {
        "leaf" => {
            let count = if let Some(text) = node["text"].as_str() {
                text.len()
            } else if node.get("text").is_some_and(Value::is_null) {
                let Some(hex) = node["hex"].as_str().filter(|hex| {
                    hex.len().is_multiple_of(2)
                        && hex
                            .bytes()
                            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
                }) else {
                    return Err(invalid("invalid physical leaf bytes"));
                };
                hex.len() / 2
            } else {
                return Err(invalid("invalid physical leaf bytes"));
            };
            if !accept_grammar_selection_physical_span(
                number(&node["start"]),
                number(&node["end"]),
                numeric(*cursor),
                numeric(count),
                false,
            ) {
                return Err(invalid("physical leaf span differs"));
            }
            budget.retain(count + 1)?;
            if let Some(text) = node["text"].as_str() {
                for byte in text.as_bytes() {
                    budget.step()?;
                    bytes.push(*byte);
                }
            } else {
                let hex = node["hex"]
                    .as_str()
                    .ok_or_else(|| invalid("invalid physical leaf bytes"))?;
                for word in hex.as_bytes().as_chunks::<2>().0 {
                    budget.step()?;
                    let word = std::str::from_utf8(word)
                        .map_err(|_| invalid("invalid physical leaf bytes"))?;
                    bytes.push(
                        u8::from_str_radix(word, 16)
                            .map_err(|_| invalid("invalid physical leaf bytes"))?,
                    );
                }
            }
            *cursor = offset(&node["end"]).ok_or_else(|| invalid("physical leaf span differs"))?;
        }
        "missing" => {
            if !accept_grammar_selection_physical_span(
                number(&node["start"]),
                number(&node["end"]),
                numeric(*cursor),
                0.0,
                true,
            ) {
                return Err(invalid("invalid missing span"));
            }
        }
        "children" | "sourceChildren" => {
            let field = if role == "children" {
                "children"
            } else {
                "sourceChildren"
            };
            let children = node[field]
                .as_array()
                .ok_or_else(|| invalid("missing concrete children"))?;
            for child in children {
                physical_node(child, cursor, bytes, depth + 1, budget)?;
            }
        }
        "root" => physical_node(&node["root"], cursor, bytes, depth + 1, budget)?,
        _ => return Err(invalid("unknown concrete node kind")),
    }
    if role != "leaf"
        && role != "missing"
        && !accept_grammar_selection_physical_span(
            number(&node["start"]),
            number(&node["end"]),
            numeric(begin),
            numeric(*cursor - begin),
            false,
        )
    {
        return Err(invalid("physical container span differs"));
    }
    Ok(())
}

struct Validator<'a> {
    definitions: &'a [Value],
    planes: Vec<Rc<[u8]>>,
    registry: SourceRegistry,
    budget: ValidationBudget,
}

impl Validator<'_> {
    fn walk(
        &mut self,
        node: &Value,
        plane: usize,
        depth: usize,
    ) -> Result<(), GrammarRuntimeError> {
        self.budget.depth(depth)?;
        if node["type"] == "node" {
            let children = node["children"]
                .as_array()
                .ok_or_else(|| invalid("missing concrete children"))?;
            for child in children {
                self.walk(child, plane, depth + 1)?;
            }
            return Ok(());
        }
        if node["type"] != "embed" {
            return Ok(());
        }
        let selection = &node["selection"];
        let profile = match selection.get("mapping") {
            None | Some(Value::Null) => "identity",
            Some(Value::String(profile)) => profile,
            _ => return Err(invalid("unknown mapping profile")),
        };
        if !accept_grammar_selection_mapping_profile(
            profile,
            node.get("sourceChildren").is_some(),
            selection.get("sourceMapping").is_some(),
        ) {
            return Err(invalid("unknown mapping profile"));
        }
        let alternatives = selection["alternatives"].as_array();
        if profile != "json-string" {
            self.walk(&node["root"], plane, depth + 1)?;
            if let Some(alternatives) = alternatives {
                for alternative in alternatives {
                    self.walk(&alternative["root"], plane, depth + 1)?;
                }
            }
            return Ok(());
        }
        let mapping = &selection["sourceMapping"];
        if !exact_keys(mapping, "mapping")
            || !accept_grammar_selection_mapping_descriptor(
                mapping["kind"].as_str().unwrap_or_default(),
                mapping["operation"].as_str().unwrap_or_default(),
            )
            || !node["sourceChildren"].is_array()
            || mapping["input"] == mapping["output"]
            || !accept_grammar_selection_mapping_occurrence(
                number(&mapping["input"]),
                numeric(plane),
                number(&selection["start"]),
                number(&node["start"]),
                number(&selection["end"]),
                number(&node["end"]),
            )
        {
            return Err(invalid("invalid mapped boundary"));
        }
        if !accept_grammar_selection_mapping_reference(
            number(&mapping["input"]),
            numeric(plane),
            number(&mapping["output"]),
            numeric(self.planes.len()),
        ) {
            return Err(invalid("unowned source-plane reference"));
        }
        let output =
            offset(&mapping["output"]).ok_or_else(|| invalid("unowned source-plane reference"))?;
        let origin = &self.definitions[output]["derivation"];
        if !accept_grammar_selection_mapping_occurrence(
            number(&origin["input"]),
            number(&mapping["input"]),
            number(&origin["start"]),
            number(&node["start"]),
            number(&origin["end"]),
            number(&node["end"]),
        ) {
            return Err(invalid("mapping belongs to another source occurrence"));
        }
        let start = offset(&node["start"]).ok_or_else(|| invalid("invalid mapped boundary"))?;
        let end = offset(&node["end"]).ok_or_else(|| invalid("invalid mapped boundary"))?;
        let derived = self
            .registry
            .map(&self.planes[plane], start, end, &self.budget)
            .map_err(aborted)?
            .map_err(|failed| invalid(failed.reason))?;
        let actual = mapping["intervals"]
            .as_array()
            .ok_or_else(|| invalid("ordered source intervals differ"))?;
        let expected = derived.mapping["intervals"]
            .as_array()
            .ok_or_else(|| invalid("ordered source intervals differ"))?;
        if actual.len() != expected.len() {
            return Err(invalid("ordered source intervals differ"));
        }
        for (row, expected) in actual.iter().zip(expected) {
            let Some(row) = row.as_array() else {
                return Err(invalid("ordered source intervals differ"));
            };
            let Some(expected) = expected.as_array() else {
                return Err(invalid("ordered source intervals differ"));
            };
            let row = row.iter().map(number).collect::<Vec<_>>();
            let expected = expected.iter().map(number).collect::<Vec<_>>();
            if !accept_grammar_selection_source_interval(&row, &expected) {
                return Err(invalid("ordered source intervals differ"));
            }
        }
        if offset(&derived.mapping["output"]) != offset(&mapping["output"]) {
            return Err(invalid(
                "source reference differs from its actual derivation",
            ));
        }
        for child in node["sourceChildren"]
            .as_array()
            .ok_or_else(|| invalid("invalid physical source children"))?
        {
            self.walk(child, plane, depth + 1)?;
        }
        let alternatives = alternatives.ok_or_else(|| invalid("invalid selection witnesses"))?;
        for root in std::iter::once(&node["root"])
            .chain(alternatives.iter().map(|alternative| &alternative["root"]))
        {
            let stored = physical(
                std::slice::from_ref(root),
                0,
                derived.bytes.len(),
                depth + 1,
                &self.budget,
            )?;
            if stored.as_slice() != &*derived.bytes {
                return Err(invalid("decoded tree bytes differ"));
            }
            self.walk(root, output, depth + 1)?;
        }
        Ok(())
    }
}

/// Validate source correspondence in a complete mapped CST plain-data bundle.
/// Returns the same borrowed value; grammar membership still requires parsing
/// with its actual grammar. No generic decoding or numeric ID proves ownership.
///
/// # Errors
/// Returns a selection error for malformed maps, source definitions or CST
/// bytes, and the unchanged resource reason when the shared bounds are reached.
pub fn check_grammar_selection_mapped_tree<'a>(
    bundle: &'a Value,
    options: &FeatureParseOptions,
) -> Result<&'a Value, GrammarRuntimeError> {
    let tree = bundle
        .get("tree")
        .filter(|tree| tree.is_object())
        .ok_or_else(|| invalid("a complete physical root CST is required"))?;
    let end = offset(&tree["end"])
        .filter(|end| u32::try_from(*end).is_ok())
        .ok_or_else(|| invalid("a complete physical root CST is required"))?;
    let budget = ValidationBudget {
        steps: Cell::new(0),
        cells: Cell::new(0),
        step_limit: options.step_limit.unwrap_or(
            (100_000 + 1000 * end)
                * if options.error_recovery == Some(true) {
                    2
                } else {
                    1
                },
        ),
        memory_limit: options.memory_limit.unwrap_or(DEFAULT_MEMORY_LIMIT),
        max_depth: options.max_depth.unwrap_or(DEFAULT_MAX_DEPTH),
    };
    budget.plain(bundle, 0)?;
    let definitions = bundle["sourcePlanes"]
        .as_array()
        .filter(|rows| !rows.is_empty())
        .ok_or_else(|| invalid("the source-plane table is required"))?;
    let bytes = physical(std::slice::from_ref(tree), 0, end, 0, &budget)?;
    let mut position = 0;
    while position < bytes.len() {
        budget.step()?;
        let (scalar, width) = super::text::decode_at(&bytes, position, bytes.len());
        if scalar.is_none() {
            return Err(invalid("the root source plane is not UTF-8"));
        }
        position += width;
    }
    let mut validator = Validator {
        definitions,
        planes: Vec::new(),
        registry: SourceRegistry::default(),
        budget,
    };
    let root: Rc<[u8]> = Rc::from(bytes);
    for (identity, definition) in definitions.iter().enumerate() {
        validator.budget.step()?;
        validator.budget.retain(1)?;
        if !exact_keys(definition, "source") {
            return Err(invalid("invalid ordered source definition"));
        }
        if !accept_grammar_selection_source_root(
            numeric(identity),
            definition["derivation"].is_null(),
        ) {
            return Err(invalid("invalid source derivation presence"));
        }
        let bytes = if identity == 0 {
            root.clone()
        } else {
            let origin = &definition["derivation"];
            if !exact_keys(origin, "derivation") {
                return Err(invalid("unowned source derivation"));
            }
            let input = offset(&origin["input"])
                .and_then(|index| validator.planes.get(index))
                .ok_or_else(|| invalid("unowned source derivation"))?;
            if !accept_grammar_selection_source_derivation(
                origin["operation"].as_str().unwrap_or_default(),
                number(&origin["input"]),
                numeric(identity),
                number(&origin["start"]),
                number(&origin["end"]),
                numeric(input.len()),
            ) {
                return Err(invalid("unowned source derivation"));
            }
            let start =
                offset(&origin["start"]).ok_or_else(|| invalid("unowned source derivation"))?;
            let end = offset(&origin["end"]).ok_or_else(|| invalid("unowned source derivation"))?;
            let derived = validator
                .registry
                .map(input, start, end, &validator.budget)
                .map_err(aborted)?
                .map_err(|_| invalid("invalid source derivation"))?;
            if offset(&derived.mapping["output"]) != Some(identity) {
                return Err(invalid("invalid source derivation"));
            }
            derived.bytes
        };
        if !accept_grammar_selection_source_definition(
            definition["kind"].as_str().unwrap_or_default(),
            number(&definition["identity"]),
            numeric(identity),
            definition["encoding"].as_str().unwrap_or_default(),
            number(&definition["byteLength"]),
            numeric(bytes.len()),
        ) {
            return Err(invalid("invalid source definition"));
        }
        validator.planes.push(bytes);
    }
    validator.walk(tree, 0, 0)?;
    Ok(bundle)
}
