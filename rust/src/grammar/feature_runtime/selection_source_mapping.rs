//! Source-buffer storage and interval transport for generated mapping plans.
//! JSON/Unicode, source-reference and interval decisions remain shared rules.

use std::collections::HashMap;
use std::rc::Rc;

use serde_json::{Value, json};

use crate::structured_data_links::into_json_number;

use super::executor::{Executor, Run};
use super::field_range_source::{copy_field_value, physical_children, read_field_range_view};
use super::operations::Abort;
use super::results::Children;
use super::text::decode_at;
use crate::translation::frontend_rules::ml_array::number_index;
use crate::translation::grammar_selection_included_range_rules::{
    accept_grammar_selection_included_range_byte_boundary,
    accept_grammar_selection_included_range_derivation,
    accept_grammar_selection_included_range_source_range,
    read_grammar_selection_included_range_part, read_grammar_selection_included_range_plan,
    read_grammar_selection_included_range_reservation,
};
use crate::translation::grammar_selection_mapping_rules::{
    read_grammar_selection_json_string_part, read_grammar_selection_mapped_cover,
    read_grammar_selection_mapping_reservation,
};
use crate::translation::source_text_byte_rules::read_source_text_scalar_byte_width;

pub(super) trait MappingBudget {
    fn mapping_step(&self) -> Run<()>;
    fn mapping_retain(&self, count: usize) -> Run<()>;
}

impl MappingBudget for Executor<'_> {
    fn mapping_step(&self) -> Run<()> {
        self.step()
    }
    fn mapping_retain(&self, count: usize) -> Run<()> {
        self.retain(count)
    }
}

/// All grammar byte coordinates fit the shared unsigned 32-bit source range.
pub(super) fn numeric(value: usize) -> f64 {
    u32::try_from(value).map_or(f64::NAN, f64::from)
}

#[derive(Clone, Debug)]
pub(super) struct MappingFailure {
    pub(super) reason: &'static str,
    pub(super) offset: usize,
}

#[derive(Clone, Debug)]
pub(super) struct MappedSource {
    pub(super) bytes: Rc<[u8]>,
    pub(super) mapping: Value,
}

type IncludedRangeKey = (usize, usize, Vec<[usize; 2]>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum FieldRangeOwner {
    Runtime(usize),
    Replay(usize),
}

type FieldRangeKey = (FieldRangeOwner, usize, usize, Vec<u8>);
type RangeSourceData = (Rc<[u8]>, Vec<Value>);

#[derive(Debug)]
struct SourceRecord {
    definition: Value,
    // Successful parent caches retain every derived buffer. Private buffer
    // keys cannot be reused during this parse; no public identity uses them.
    parent: Option<usize>,
    mapping: Option<Value>,
    mappings: HashMap<(usize, usize), Result<MappedSource, MappingFailure>>,
    included_mappings: HashMap<IncludedRangeKey, Result<MappedSource, MappingFailure>>,
    field_mappings: HashMap<FieldRangeKey, Result<MappedSource, MappingFailure>>,
}

#[derive(Debug, Default)]
pub(super) struct SourceRegistry {
    records: Vec<SourceRecord>,
    by_buffer: HashMap<(usize, usize), usize>,
    invalid: HashMap<(usize, usize), MappingFailure>,
    field_owners: HashMap<usize, Children>,
}

fn buffer_key(bytes: &[u8]) -> (usize, usize) {
    (bytes.as_ptr().addr(), bytes.len())
}

// A private injective cache encoding: Number bits retain signed zero and all
// containers carry type/length tags. It never establishes a public identity.
fn field_key(
    value: &Value,
    key: &mut Vec<u8>,
    budget: &impl MappingBudget,
    level: usize,
    remaining: usize,
) -> Run<()> {
    budget.mapping_step()?;
    if level > remaining {
        return Err(Abort::NestingTooDeep);
    }
    budget.mapping_retain(9)?;
    match value {
        Value::Null => key.push(b'n'),
        Value::Bool(value) => key.extend([b'b', u8::from(*value)]),
        Value::Number(value) => {
            key.push(b'd');
            key.extend(value.as_f64().unwrap_or(f64::NAN).to_bits().to_le_bytes());
        }
        Value::String(value) => {
            budget.mapping_retain(value.len())?;
            for _ in value.encode_utf16() {
                budget.mapping_step()?;
            }
            key.push(b's');
            key.extend(
                u64::try_from(value.len())
                    .map_err(|_| Abort::MemoryBudget)?
                    .to_le_bytes(),
            );
            key.extend(value.as_bytes());
        }
        Value::Array(values) => {
            key.push(b'a');
            key.extend(
                u64::try_from(values.len())
                    .map_err(|_| Abort::MemoryBudget)?
                    .to_le_bytes(),
            );
            for value in values {
                field_key(value, key, budget, level + 1, remaining)?;
            }
        }
        Value::Object(values) => {
            key.push(b'o');
            key.extend(
                u64::try_from(values.len())
                    .map_err(|_| Abort::MemoryBudget)?
                    .to_le_bytes(),
            );
            for (name, value) in values {
                budget.mapping_retain(name.len().checked_add(9).ok_or(Abort::MemoryBudget)?)?;
                for _ in name.encode_utf16() {
                    budget.mapping_step()?;
                }
                key.extend(
                    u64::try_from(name.len())
                        .map_err(|_| Abort::MemoryBudget)?
                        .to_le_bytes(),
                );
                key.extend(name.as_bytes());
                field_key(value, key, budget, level + 1, remaining)?;
            }
        }
    }
    Ok(())
}

fn execute_range_source(
    bytes: &[u8],
    region: (usize, usize),
    ranges: &[[f64; 2]],
    bounds: &[[usize; 2]],
    budget: &impl MappingBudget,
    reservation: usize,
    cell_count: f64,
) -> Run<Result<RangeSourceData, MappingFailure>> {
    let (start, end) = region;
    let length = end - start;
    let failed = |offset| MappingFailure {
        reason: "invalidEmbeddingMapping",
        offset,
    };
    budget.mapping_retain(reservation)?;
    let Some(steps) = number_index(read_grammar_selection_included_range_reservation(
        numeric(length),
        cell_count,
        true,
    )) else {
        return Ok(Err(failed(start)));
    };
    for _ in 0..steps {
        budget.mapping_step()?;
    }
    let columns = ranges.iter().flatten().copied().collect::<Vec<_>>();
    let plan = read_grammar_selection_included_range_plan(&columns, numeric(length));
    if plan.is_empty() {
        return Ok(Err(failed(start)));
    }
    for relative in std::iter::once(0)
        .chain(std::iter::once(length))
        .chain(bounds.iter().flatten().copied())
    {
        let position = start + relative;
        let byte = bytes.get(position).map_or(-1.0, |byte| f64::from(*byte));
        if !accept_grammar_selection_included_range_byte_boundary(byte, position == bytes.len()) {
            return Ok(Err(failed(position)));
        }
    }
    let Some(output_length) = plan.last().copied().and_then(number_index) else {
        return Ok(Err(failed(start)));
    };
    let mut output_bytes = vec![0; output_length];
    let mut intervals = Vec::new();
    let (mut row, mut position, mut output) = (0, 0, 0);
    loop {
        let at = start + position;
        // decodeAt reports one invalid unit at EOF; the shared part plan
        // ignores that width for a final point or excluded gap.
        let width = if at < bytes.len() {
            decode_at(bytes, at, end).1
        } else {
            1
        };
        let part = read_grammar_selection_included_range_part(
            &plan,
            numeric(row),
            numeric(position),
            numeric(output),
            numeric(width),
        );
        let opcode = part.first().copied().and_then(number_index);
        if opcode == Some(1) {
            break;
        }
        if part.len() != 8 || opcode != Some(0) {
            return Ok(Err(failed(at)));
        }
        let mut cells = [0; 8];
        for (cell, value) in cells.iter_mut().zip(part) {
            let Some(value) = number_index(value) else {
                return Ok(Err(failed(at)));
            };
            *cell = value;
        }
        let [
            _,
            next_row,
            next_position,
            next_output,
            input_start,
            input_end,
            output_start,
            output_end,
        ] = cells;
        intervals.push(json!([
            start + input_start,
            start + input_end,
            output_start,
            output_end
        ]));
        if output_end > output_start {
            output_bytes[output_start..output_end]
                .copy_from_slice(&bytes[start + input_start..start + input_end]);
        }
        (row, position, output) = (next_row, next_position, next_output);
    }
    let bytes: Rc<[u8]> = Rc::from(output_bytes);
    Ok(Ok((bytes, intervals)))
}

impl SourceRegistry {
    pub(super) fn definition(&self, source: usize) -> Option<&Value> {
        self.records.get(source).map(|record| &record.definition)
    }

    pub(super) fn map_field(
        &mut self,
        bytes: &[u8],
        region: (usize, usize),
        selector: &Value,
        children: &Children,
        budget: &impl MappingBudget,
        remaining_depth: usize,
    ) -> Run<Result<MappedSource, MappingFailure>> {
        if let Err(failed) = self.source(bytes, budget)? {
            return Ok(Err(failed));
        }
        let physical = match physical_children(bytes, children, budget, remaining_depth)? {
            Ok(physical) => physical,
            Err(failed) => return Ok(Err(failed)),
        };
        let owner = Rc::as_ptr(children).addr();
        if let std::collections::hash_map::Entry::Vacant(entry) = self.field_owners.entry(owner) {
            budget.mapping_retain(1)?;
            entry.insert(children.clone());
        }
        self.map_field_data(
            bytes,
            region,
            selector,
            (FieldRangeOwner::Runtime(owner), &physical),
            budget,
            remaining_depth,
        )
    }

    pub(super) fn map_field_value(
        &mut self,
        bytes: &[u8],
        region: (usize, usize),
        selector: &Value,
        owner: (usize, &Value),
        budget: &impl MappingBudget,
        remaining_depth: usize,
    ) -> Run<Result<MappedSource, MappingFailure>> {
        self.map_field_data(
            bytes,
            region,
            selector,
            (FieldRangeOwner::Replay(owner.0), owner.1),
            budget,
            remaining_depth,
        )
    }

    fn map_field_data(
        &mut self,
        bytes: &[u8],
        region: (usize, usize),
        selector: &Value,
        owner: (FieldRangeOwner, &Value),
        budget: &impl MappingBudget,
        remaining_depth: usize,
    ) -> Run<Result<MappedSource, MappingFailure>> {
        use crate::translation::grammar_selection_field_range_rules::read_grammar_selection_field_range_field_order;
        let (start, end) = region;
        let input = match self.source(bytes, budget)? {
            Ok(input) => input,
            Err(failed) => return Ok(Err(failed)),
        };
        let view = match read_field_range_view(
            bytes,
            start,
            end,
            selector,
            owner.1,
            budget,
            remaining_depth,
        )? {
            Ok(view) => view,
            Err(failed) => return Ok(Err(failed)),
        };
        budget.mapping_retain(6)?;
        let mut parameters = serde_json::Map::new();
        for key in read_grammar_selection_field_range_field_order("selector").split('|') {
            let Some(value) = selector.get(key) else {
                return Ok(Err(MappingFailure {
                    reason: "invalidEmbeddingMapping",
                    offset: start,
                }));
            };
            let copy = match copy_field_value(value, budget, remaining_depth)? {
                Ok(copy) => copy,
                Err(failed) => return Ok(Err(failed)),
            };
            parameters.insert(key.to_owned(), copy);
        }
        let parameters = Value::Object(parameters);
        budget.mapping_retain(
            view.ranges
                .len()
                .checked_mul(7)
                .and_then(|n| n.checked_add(2))
                .ok_or(Abort::MemoryBudget)?,
        )?;
        let mut range_values = Vec::with_capacity(view.ranges.len());
        let mut bounds = Vec::with_capacity(view.ranges.len());
        for range in &view.ranges {
            budget.mapping_step()?;
            let (Some(left), Some(right)) = (number_index(range[0]), number_index(range[1])) else {
                return Ok(Err(MappingFailure {
                    reason: "invalidEmbeddingMapping",
                    offset: start,
                }));
            };
            bounds.push([left, right]);
            let values = range
                .iter()
                .map(|value| into_json_number(*value).map(Value::Number))
                .collect::<Result<Vec<_>, _>>();
            let Ok(values) = values else {
                return Ok(Err(MappingFailure {
                    reason: "invalidEmbeddingMapping",
                    offset: start,
                }));
            };
            range_values.push(Value::Array(values));
        }
        let range_values = Value::Array(range_values);
        budget.mapping_retain(1)?;
        let mut tag = Vec::new();
        for value in [&parameters, &view.occurrences, &range_values, &view.witness] {
            field_key(value, &mut tag, budget, 1, remaining_depth)?;
        }
        let key = (owner.0, start, end, tag);
        if let Some(cached) = self.records[input].field_mappings.get(&key) {
            return match cached {
                Ok(mapped) => match copy_field_value(&mapped.mapping, budget, remaining_depth)? {
                    Ok(mapping) => {
                        budget.mapping_retain(1)?;
                        Ok(Ok(MappedSource {
                            bytes: mapped.bytes.clone(),
                            mapping,
                        }))
                    }
                    Err(failed) => Ok(Err(failed)),
                },
                Err(failed) => {
                    budget.mapping_retain(3)?;
                    Ok(Err(failed.clone()))
                }
            };
        }
        budget.mapping_retain(key.3.len().checked_add(1).ok_or(Abort::MemoryBudget)?)?;
        let cells = view
            .ranges
            .len()
            .checked_mul(2)
            .ok_or(Abort::MemoryBudget)?;
        let cell_count = serde_json::Number::from(cells).as_f64().unwrap_or(f64::NAN);
        let length = end - start;
        let Some(reservation) = number_index(read_grammar_selection_included_range_reservation(
            numeric(length),
            cell_count,
            false,
        )) else {
            return Ok(Err(MappingFailure {
                reason: "invalidEmbeddingMapping",
                offset: start,
            }));
        };
        if !accept_grammar_selection_included_range_derivation(
            "included-ranges",
            numeric(input),
            numeric(self.records.len()),
            numeric(start),
            numeric(end),
            numeric(bytes.len()),
        ) {
            return Ok(Err(MappingFailure {
                reason: "invalidEmbeddingMapping",
                offset: start,
            }));
        }
        let (bytes, intervals) = match execute_range_source(
            bytes,
            region,
            &view.ranges,
            &bounds,
            budget,
            reservation,
            cell_count,
        )? {
            Ok(mapped) => mapped,
            Err(failed) => {
                self.records[input]
                    .field_mappings
                    .insert(key, Err(failed.clone()));
                return Ok(Err(failed));
            }
        };
        let source = self.records.len();
        let selector_copy = match copy_field_value(&parameters, budget, remaining_depth)? {
            Ok(value) => value,
            Err(failed) => return Ok(Err(failed)),
        };
        let occurrences_copy = match copy_field_value(&view.occurrences, budget, remaining_depth)? {
            Ok(value) => value,
            Err(failed) => return Ok(Err(failed)),
        };
        let ranges_copy = match copy_field_value(&range_values, budget, remaining_depth)? {
            Ok(value) => value,
            Err(failed) => return Ok(Err(failed)),
        };
        budget.mapping_retain(15)?;
        let mapping = json!({"kind":"source-mapping","operation":"field-ranges","input":input,"output":source,
            "selector":parameters,"occurrences":view.occurrences,"ranges":range_values,"intervals":intervals});
        let mapped_mapping = match copy_field_value(&mapping, budget, remaining_depth)? {
            Ok(value) => value,
            Err(failed) => return Ok(Err(failed)),
        };
        let cached_mapping = match copy_field_value(&mapping, budget, remaining_depth)? {
            Ok(value) => value,
            Err(failed) => return Ok(Err(failed)),
        };
        budget.mapping_retain(2)?;
        let mapped = MappedSource {
            bytes: bytes.clone(),
            mapping: mapped_mapping,
        };
        self.records.push(SourceRecord {
            definition:json!({"kind":"grammar-source","identity":source,"encoding":"UTF-8","byteLength":bytes.len(),
                "derivation":{"operation":"field-ranges","input":input,"start":start,"end":end,"selector":selector_copy,
                    "occurrences":occurrences_copy,"ranges":ranges_copy,"witness":view.witness}}),
            parent:Some(input),mapping:Some(mapping),mappings:HashMap::new(),included_mappings:HashMap::new(),field_mappings:HashMap::new(),
        });
        self.by_buffer.insert(buffer_key(&bytes), source);
        self.records[input].field_mappings.insert(
            key,
            Ok(MappedSource {
                bytes: bytes.clone(),
                mapping: cached_mapping,
            }),
        );
        Ok(Ok(mapped))
    }

    pub(super) fn definitions(&self) -> Option<Vec<Value>> {
        (!self.records.is_empty()).then(|| {
            self.records
                .iter()
                .map(|row| row.definition.clone())
                .collect()
        })
    }

    fn source(
        &mut self,
        bytes: &[u8],
        budget: &impl MappingBudget,
    ) -> Run<Result<usize, MappingFailure>> {
        let key = buffer_key(bytes);
        if let Some(source) = self.by_buffer.get(&key) {
            return Ok(Ok(*source));
        }
        if let Some(failed) = self.invalid.get(&key) {
            return Ok(Err(failed.clone()));
        }
        budget.mapping_retain(1)?;
        let mut position = 0;
        while position < bytes.len() {
            budget.mapping_step()?;
            let (scalar, width) = decode_at(bytes, position, bytes.len());
            if scalar.is_none() {
                let failed = MappingFailure {
                    reason: "unsupportedEmbeddingMapping",
                    offset: position,
                };
                self.invalid.insert(key, failed.clone());
                return Ok(Err(failed));
            }
            position += width;
        }
        let source = self.records.len();
        self.records.push(SourceRecord {
            definition: json!({"kind":"grammar-source","identity":source,
            "encoding":"UTF-8","byteLength":bytes.len(),"derivation":null}),
            parent: None,
            mapping: None,
            mappings: HashMap::new(),
            included_mappings: HashMap::new(),
            field_mappings: HashMap::new(),
        });
        self.by_buffer.insert(key, source);
        Ok(Ok(source))
    }

    pub(super) fn map(
        &mut self,
        bytes: &[u8],
        start: usize,
        end: usize,
        budget: &impl MappingBudget,
    ) -> Run<Result<MappedSource, MappingFailure>> {
        let input = match self.source(bytes, budget)? {
            Ok(source) => source,
            Err(failed) => return Ok(Err(failed)),
        };
        if let Some(cached) = self.records[input].mappings.get(&(start, end)) {
            return Ok(cached.clone());
        }
        let failed = |offset| MappingFailure {
            reason: "invalidEmbeddingMapping",
            offset,
        };
        if start > end || end > bytes.len() {
            return Ok(Err(failed(start.min(bytes.len()))));
        }
        let length = end - start;
        let Some(cells) = number_index(read_grammar_selection_mapping_reservation(
            numeric(length),
            false,
        )) else {
            return Ok(Err(failed(start)));
        };
        budget.mapping_retain(cells)?;
        let Some(steps) = number_index(read_grammar_selection_mapping_reservation(
            numeric(length),
            true,
        )) else {
            return Ok(Err(failed(start)));
        };
        for _ in 0..steps {
            budget.mapping_step()?;
        }
        let mut codes = Vec::new();
        let mut positions = Vec::new();
        let mut position = start;
        while position < end {
            let (scalar, width) = decode_at(bytes, position, end);
            let Some(scalar) = scalar else {
                let failed = failed(position);
                self.records[input]
                    .mappings
                    .insert((start, end), Err(failed.clone()));
                return Ok(Err(failed));
            };
            positions.push(position);
            codes.push(f64::from(scalar));
            position += width;
        }
        positions.push(end);
        let mut text = String::new();
        let mut intervals = vec![json!([start, start + 1, 0, 0])];
        let mut index = 1;
        let mut output_end = 0;
        loop {
            let part = read_grammar_selection_json_string_part(&codes, numeric(index));
            let opcode = part.first().copied().and_then(number_index);
            if opcode == Some(3) {
                break;
            }
            if opcode != Some(0) {
                let at = part
                    .get(1)
                    .copied()
                    .and_then(number_index)
                    .and_then(|at| positions.get(at))
                    .copied()
                    .unwrap_or(start);
                let failed = MappingFailure {
                    reason: if opcode == Some(2) {
                        "unsupportedEmbeddingMapping"
                    } else {
                        "invalidEmbeddingMapping"
                    },
                    offset: at,
                };
                self.records[input]
                    .mappings
                    .insert((start, end), Err(failed.clone()));
                return Ok(Err(failed));
            }
            let Some(code) = part
                .get(1)
                .copied()
                .and_then(number_index)
                .and_then(|code| u32::try_from(code).ok())
            else {
                return Ok(Err(failed(start)));
            };
            let Some(scalar) = char::from_u32(code) else {
                return Ok(Err(failed(start)));
            };
            let Some(next) = part.get(2).copied().and_then(number_index) else {
                return Ok(Err(failed(start)));
            };
            let Some(width) = number_index(read_source_text_scalar_byte_width(f64::from(code)))
            else {
                return Ok(Err(failed(start)));
            };
            let (Some(&from), Some(&to)) = (positions.get(index), positions.get(next)) else {
                return Ok(Err(failed(start)));
            };
            text.push(scalar);
            intervals.push(json!([from, to, output_end, output_end + width]));
            index = next;
            output_end += width;
        }
        intervals.push(json!([end - 1, end, output_end, output_end]));
        let bytes: Rc<[u8]> = Rc::from(text.into_bytes());
        let source = self.records.len();
        let mapping = json!({"kind":"source-mapping","operation":"json-string","input":input,
            "output":source,"intervals":intervals});
        let mapped = MappedSource {
            bytes: bytes.clone(),
            mapping: mapping.clone(),
        };
        self.records.push(SourceRecord { definition: json!({"kind":"grammar-source","identity":source,
            "encoding":"UTF-8","byteLength":bytes.len(),"derivation":{"operation":"json-string","input":input,"start":start,"end":end}}),
            parent: Some(input), mapping: Some(mapping), mappings: HashMap::new(),
            included_mappings: HashMap::new(), field_mappings: HashMap::new() });
        self.by_buffer.insert(buffer_key(&bytes), source);
        self.records[input]
            .mappings
            .insert((start, end), Ok(mapped.clone()));
        Ok(Ok(mapped))
    }

    /// Execute one explicit relative-range view on the actual parent source.
    /// Generated plans own validation, scalar copies, gaps and contraction.
    pub(super) fn map_included(
        &mut self,
        bytes: &[u8],
        start: usize,
        end: usize,
        ranges: &[[f64; 2]],
        budget: &impl MappingBudget,
    ) -> Run<Result<MappedSource, MappingFailure>> {
        let input = match self.source(bytes, budget)? {
            Ok(source) => source,
            Err(failed) => return Ok(Err(failed)),
        };
        let failed = |offset| MappingFailure {
            reason: "invalidEmbeddingMapping",
            offset,
        };
        if !accept_grammar_selection_included_range_derivation(
            "included-ranges",
            numeric(input),
            numeric(self.records.len()),
            numeric(start),
            numeric(end),
            numeric(bytes.len()),
        ) {
            return Ok(Err(failed(start)));
        }
        let Some(cells) = ranges.len().checked_mul(2) else {
            return Ok(Err(failed(start)));
        };
        let length = end - start;
        let cell_count = serde_json::Number::from(cells).as_f64().unwrap_or(f64::NAN);
        let Some(reservation) = number_index(read_grammar_selection_included_range_reservation(
            numeric(length),
            cell_count,
            false,
        )) else {
            return Ok(Err(failed(start)));
        };
        let mut parameters = Vec::with_capacity(ranges.len());
        for range in ranges {
            budget.mapping_step()?;
            if !accept_grammar_selection_included_range_source_range(
                "source-range",
                range[0],
                range[1],
            ) {
                return Ok(Err(failed(start)));
            }
            let (Some(from), Some(to)) = (number_index(range[0]), number_index(range[1])) else {
                return Ok(Err(failed(start)));
            };
            parameters.push([from, to]);
        }
        // JSON.stringify, used by the JavaScript cache, gives both signed
        // zeros the same coordinate key. Parameters in the output keep the
        // original finite Number values independently of this private key.
        let key = (start, end, parameters);
        if let Some(cached) = self.records[input].included_mappings.get(&key) {
            return Ok(cached.clone());
        }
        let refuse = |registry: &mut Self, offset| {
            let failure = failed(offset);
            registry.records[input]
                .included_mappings
                .insert(key.clone(), Err(failure.clone()));
            Ok(Err(failure))
        };
        let (bytes, intervals) = match execute_range_source(
            bytes,
            (start, end),
            ranges,
            &key.2,
            budget,
            reservation,
            cell_count,
        )? {
            Ok(mapped) => mapped,
            Err(failed) => return refuse(self, failed.offset),
        };
        let source = self.records.len();
        let parameters = ranges
            .iter()
            .map(|range| {
                range
                    .iter()
                    .map(|coordinate| into_json_number(*coordinate).map(Value::Number))
                    .collect::<Result<Vec<_>, _>>()
                    .map(Value::Array)
            })
            .collect::<Result<Vec<_>, _>>();
        let Ok(parameters) = parameters else {
            return refuse(self, start);
        };
        let parameters = Value::Array(parameters);
        let mapping = json!({"kind":"source-mapping","operation":"included-ranges","input":input,
            "output":source,"ranges":parameters,"intervals":intervals});
        let mapped = MappedSource {
            bytes: bytes.clone(),
            mapping: mapping.clone(),
        };
        self.records.push(SourceRecord {
            definition: json!({"kind":"grammar-source","identity":source,"encoding":"UTF-8","byteLength":bytes.len(),
                "derivation":{"operation":"included-ranges","input":input,"start":start,"end":end,"ranges":parameters}}),
            parent: Some(input), mapping: Some(mapping), mappings: HashMap::new(), included_mappings: HashMap::new(), field_mappings: HashMap::new(),
        });
        self.by_buffer.insert(buffer_key(&bytes), source);
        self.records[input]
            .included_mappings
            .insert(key, Ok(mapped.clone()));
        Ok(Ok(mapped))
    }

    pub(super) fn qualify(&self, bytes: &[u8], mut diagnostic: Value) -> Value {
        let Some(mut source) = self.by_buffer.get(&buffer_key(bytes)).copied() else {
            return diagnostic;
        };
        if self.records[source].parent.is_none() {
            return diagnostic;
        }
        let mut start = diagnostic["start"]
            .as_u64()
            .and_then(|value| usize::try_from(value).ok())
            .unwrap_or_default();
        let mut end = diagnostic["end"]
            .as_u64()
            .and_then(|value| usize::try_from(value).ok())
            .unwrap_or(start);
        if diagnostic.get("source").is_none() {
            diagnostic["source"] = json!(source);
        }
        if diagnostic.get("sourceStart").is_none() {
            diagnostic["sourceStart"] = json!(start);
        }
        if diagnostic.get("sourceEnd").is_none() {
            diagnostic["sourceEnd"] = json!(end);
        }
        let mut route = diagnostic["sourceRoute"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        while let Some(parent) = self.records[source].parent {
            let Some(mapping) = &self.records[source].mapping else {
                break;
            };
            let Some(cover) = mapped_cover(mapping, start, end) else {
                break;
            };
            route.push(
                json!({"output":source,"outputStart":start,"outputEnd":end,"input":parent,
                "inputStart":cover[1],"inputEnd":cover[2],"displayAnchor":cover[0]}),
            );
            start = cover[1];
            end = cover[2];
            source = parent;
        }
        diagnostic["start"] = json!(start);
        diagnostic["end"] = json!(end);
        diagnostic["sourceRoute"] = json!(route);
        diagnostic["offsetSemantics"] = json!("covering-source-spelling");
        diagnostic
    }

    pub(super) fn remember(
        &mut self,
        bytes: &[u8],
        mut diagnostic: Value,
        budget: &impl MappingBudget,
    ) -> Run<Value> {
        if let Ok(source) = self.source(bytes, budget)? {
            if diagnostic.get("source").is_none() {
                diagnostic["source"] = json!(source);
            }
            if diagnostic.get("sourceStart").is_none() {
                diagnostic["sourceStart"] = diagnostic["start"].clone();
            }
            if diagnostic.get("sourceEnd").is_none() {
                diagnostic["sourceEnd"] = diagnostic["end"].clone();
            }
            diagnostic = self.qualify(bytes, diagnostic);
        }
        budget.mapping_retain(1)?;
        Ok(diagnostic)
    }
}

pub(super) fn mapped_cover(mapping: &Value, start: usize, end: usize) -> Option<[usize; 3]> {
    let intervals = mapping["intervals"]
        .as_array()?
        .iter()
        .map(|row| {
            row.as_array().map(|cells| {
                cells
                    .iter()
                    .map(|cell| cell.as_f64().unwrap_or(f64::NAN))
                    .collect()
            })
        })
        .collect::<Option<Vec<Vec<f64>>>>()?;
    let cover = read_grammar_selection_mapped_cover(&intervals, numeric(start), numeric(end));
    if cover.len() != 3 {
        return None;
    }
    Some([
        number_index(cover[0])?,
        number_index(cover[1])?,
        number_index(cover[2])?,
    ])
}
