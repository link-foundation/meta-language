// Generated from complete js/src/rust-project-source-rules.js. Do not edit by hand.
use super::MlTemporaryModuleSourceContextLoop16Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::ml_utf16_text;
use super::module_ancestors;
use super::module_source_context;
use super::number_column;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_rust_project_module_directory;
use super::read_source_intervals;
use super::resolve_rust_project_literal_path;
use super::select_rust_project_module_source;
use super::text_column;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_module_source_context_loop16(
    plan: &[f64],
    attributes: &[f64],
    values: &[self::ml_string::Text],
    mut current: self::ml_string::Text,
    mut pending: self::ml_string::Text,
    source_ml_temporary_values14: &[f64],
    mut source_ml_temporary_index14: self::ml::Big,
) -> self::MlTemporaryModuleSourceContextLoop16Result {
    loop {
        if source_ml_temporary_index14
            < (self::ml::Big::from_u128(source_ml_temporary_values14.len() as u128))
        {
            {
                let ancestor = self::ml_array::at(
                    source_ml_temporary_values14,
                    source_ml_temporary_index14.to_index(),
                );
                {
                    let literal = self::read_required_number_fact(attributes, (ancestor * 3f64));
                    {
                        let current_4 = if literal >= 0f64 {
                            {
                                self::resolve_rust_project_literal_path(
                                    &(current),
                                    &(self::read_required_text_fact(values, literal)),
                                )
                            }
                        } else {
                            {
                                self::ml_string::join(
                                    &{
                                        let ml_values1 = vec![
                                            current.clone(),
                                            pending.clone(),
                                            self::read_required_text_fact(
                                                values,
                                                self::read_required_number_fact(
                                                    plan,
                                                    ((ancestor * 5f64) + 1f64),
                                                ),
                                            ),
                                        ];
                                        let ml_callback1 = std::rc::Rc::new(
                                            move |part: self::ml_string::Text| -> bool {
                                                part != (self::ml_string::Text::from_units(vec![]))
                                            },
                                        );
                                        ml_values1
                                            .iter()
                                            .cloned()
                                            .enumerate()
                                            .filter_map(|(ml_index1, ml_value1)| {
                                                if ml_callback1(ml_value1.clone()) {
                                                    Some(ml_value1)
                                                } else {
                                                    None
                                                }
                                            })
                                            .collect::<Vec<_>>()
                                    },
                                    &self::ml_string::Text::from_units(vec![47]),
                                )
                            }
                        };
                        {
                            let pending_2 = self::ml_string::Text::from_units(vec![]);
                            {
                                let source_ml_temporary_index14_2 =
                                    source_ml_temporary_index14.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, pending, source_ml_temporary_index14) =
                                        (current_4, pending_2, source_ml_temporary_index14_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlTemporaryModuleSourceContextLoop16Result::MlTemporaryModuleSourceContextLoop16Done(current.clone(), pending.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadSourceIntervalsLoop17Result {
    MlTemporaryReadSourceIntervalsLoop17Done(f64, Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_read_source_intervals_loop17(
    source: &self::ml_string::Text,
    path: &self::ml_string::Text,
    mut from: f64,
    mut to: f64,
    owner: &self::ml_string::Text,
    plan: &[f64],
    attributes: &[f64],
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
    mut offset: f64,
    mut events: Vec<self::ml_string::Text>,
    mut record: f64,
) -> self::MlTemporaryReadSourceIntervalsLoop17Result {
    loop {
        return if (record * 3f64) < (attributes.len() as f64) {
            {
                let literal = self::read_required_number_fact(attributes, (record * 3f64));
                if literal < 0f64 {
                    {
                        let record_2 = record + 1f64;
                        {
                            record = record_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let first = self::read_required_number_fact(
                            starts,
                            self::read_required_number_fact(attributes, ((record * 3f64) + 1f64)),
                        );
                        {
                            let last = self::read_required_number_fact(
                                ends,
                                (self::read_required_number_fact(
                                    attributes,
                                    ((record * 3f64) + 2f64),
                                ) - 1f64),
                            );
                            if (first < from) || (first >= to) {
                                {
                                    let record_3 = record + 1f64;
                                    {
                                        record = record_3;
                                        continue;
                                    }
                                }
                            }
                            {
                                if (first < offset) || (last > to) {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (self::ml_string::Text::from_units(vec![
                                            111, 118, 101, 114, 108, 97, 112, 112, 105, 110, 103,
                                            32, 82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99,
                                            116, 32, 105, 110, 116, 101, 114, 112, 114, 101, 116,
                                            101, 100, 32, 97, 116, 116, 114, 105, 98, 117, 116,
                                            101, 32, 105, 110, 116, 101, 114, 118, 97, 108,
                                        ]))
                                        .0,
                                        message_present: true,
                                    })
                                    .raise()
                                } else {
                                    {
                                        let events_2 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        115, 111, 117, 114, 99, 101,
                                                    ]),
                                                    path.clone(),
                                                    {
                                                        let source_ml_source1_string_argument37 =
                                                            offset;
                                                        self::ml_string::Text::from_text(
                                                            &self::ml_number::js_number(
                                                                source_ml_source1_string_argument37,
                                                            ),
                                                        )
                                                    },
                                                    {
                                                        let source_ml_source1_string_argument38 =
                                                            first;
                                                        self::ml_string::Text::from_text(
                                                            &self::ml_number::js_number(
                                                                source_ml_source1_string_argument38,
                                                            ),
                                                        )
                                                    },
                                                    {
                                                        let source_ml_source1_string_range_operand39 =
                                                            source.clone();
                                                        {
                                                            let source_ml_source1_string_range_operand40 =
                                                                offset;
                                                            {
                                                                let source_ml_source1_string_range_operand41 =
                                                                    first;
                                                                {
                                                                    let source_ml_source1_string_range_position42 = source_ml_source1_string_range_operand40;
                                                                    {
                                                                        let source_ml_source1_string_range_position43 = source_ml_source1_string_range_operand41;
                                                                        source_ml_source1_string_range_operand39.slice(source_ml_source1_string_range_position42, source_ml_source1_string_range_position43)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    },
                                                    owner.clone(),
                                                    self::ml_string::Text::from_units(vec![]),
                                                ],
                                                events,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let ancestors =
                                                self::module_ancestors(plan, values, record);
                                            {
                                                let name = self::read_required_text_fact(
                                                    values,
                                                    self::read_required_number_fact(
                                                        plan,
                                                        ((record * 5f64) + 1f64),
                                                    ),
                                                );
                                                {
                                                    let events_3 = {
                                                        let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![105, 110, 116, 101, 114, 112, 114, 101, 116, 101, 100, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101]), path.clone(), {
                                                        let source_ml_source1_string_argument44 = first;
                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument44))
                                                    }, {
                                                        let source_ml_source1_string_argument45 = last;
                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument45))
                                                    }, self::ml_string::Text::from_units(vec![]), (owner.concat(&self::ml_string::Text::from_units(vec![58, 58]))).concat(&self::ml_string::join(&{ let (ml_append_right, ml_append_left) = (vec![name], ancestors); self::ml_array::append(ml_append_left, ml_append_right) }, &self::ml_string::Text::from_units(vec![58, 58]))), self::read_required_text_fact(values, literal)], events_2);
                                                        self::ml_array::append(
                                                            ml_append_left,
                                                            ml_append_right,
                                                        )
                                                    };
                                                    {
                                                        let offset_2 = last;
                                                        {
                                                            let record_4 = record + 1f64;
                                                            {
                                                                (offset, events, record) =
                                                                    (offset_2, events_3, record_4);
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadSourceIntervalsLoop17Result::MlTemporaryReadSourceIntervalsLoop17Done(offset, events)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_module_ancestors_loop18(
    plan: &[f64],
    values: &[self::ml_string::Text],
    mut parent: f64,
    mut ordered: Vec<self::ml_string::Text>,
) -> Vec<self::ml_string::Text> {
    loop {
        if parent >= 0f64 {
            {
                let ordered_2 = self::ml_array::append(
                    vec![self::read_required_text_fact(
                        values,
                        self::read_required_number_fact(plan, ((parent * 5f64) + 1f64)),
                    )],
                    ordered,
                );
                {
                    let parent_2 = self::read_required_number_fact(plan, ((parent * 5f64) + 4f64));
                    {
                        (parent, ordered) = (parent_2, ordered_2);
                        continue;
                    }
                }
            }
        }
        return { ordered };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_expand_rust_project_source_events_loop23(
    mut output_units: f64,
    intervals: &[self::ml_string::Text],
    mut part: f64,
) -> f64 {
    loop {
        if part < (intervals.len() as f64) {
            {
                let output_units_2 =
                    output_units + (self::read_required_text_fact(intervals, part)).length();
                {
                    let part_2 = part + 7f64;
                    {
                        (output_units, part) = (output_units_2, part_2);
                        continue;
                    }
                }
            }
        }
        return { output_units };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_expand_rust_project_source_events_loop24(
    mut events: Vec<self::ml_string::Text>,
    source_ml_temporary_values19: &[self::ml_string::Text],
    mut source_ml_temporary_index19: self::ml::Big,
) -> Vec<self::ml_string::Text> {
    loop {
        if source_ml_temporary_index19
            < (self::ml::Big::from_u128(source_ml_temporary_values19.len() as u128))
        {
            {
                let part = self::ml_array::at(
                    source_ml_temporary_values19,
                    source_ml_temporary_index19.to_index(),
                );
                {
                    let events_2 = {
                        let (ml_append_right, ml_append_left) = (vec![part], events);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let source_ml_temporary_index19_2 =
                            source_ml_temporary_index19.add(&self::ml::Big::from_i128(1));
                        {
                            (events, source_ml_temporary_index19) =
                                (events_2, source_ml_temporary_index19_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { events };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryExpandRustProjectSourceEventsJoin25 {
    MlTemporaryExpandRustProjectSourceEventsJoin25Next(Vec<self::ml_string::Text>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_expand_rust_project_source_events_loop26(
    mut output_units: f64,
    intervals: &[self::ml_string::Text],
    mut part: f64,
) -> f64 {
    loop {
        if part < (intervals.len() as f64) {
            {
                let output_units_2 =
                    output_units + (self::read_required_text_fact(intervals, part)).length();
                {
                    let part_2 = part + 7f64;
                    {
                        (output_units, part) = (output_units_2, part_2);
                        continue;
                    }
                }
            }
        }
        return { output_units };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_expand_rust_project_source_events_loop27(
    mut events: Vec<self::ml_string::Text>,
    source_ml_temporary_values20: &[self::ml_string::Text],
    mut source_ml_temporary_index20: self::ml::Big,
) -> Vec<self::ml_string::Text> {
    loop {
        if source_ml_temporary_index20
            < (self::ml::Big::from_u128(source_ml_temporary_values20.len() as u128))
        {
            {
                let part = self::ml_array::at(
                    source_ml_temporary_values20,
                    source_ml_temporary_index20.to_index(),
                );
                {
                    let events_2 = {
                        let (ml_append_right, ml_append_left) = (vec![part], events);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let source_ml_temporary_index20_2 =
                            source_ml_temporary_index20.add(&self::ml::Big::from_i128(1));
                        {
                            (events, source_ml_temporary_index20) =
                                (events_2, source_ml_temporary_index20_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { events };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryExpandRustProjectSourceEventsLoop28Result {
    MlTemporaryExpandRustProjectSourceEventsLoop28Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_expand_rust_project_source_events_loop28(
    frames: &[f64],
    provider: &self::ml_string::Text,
    mut child: f64,
    mut active: f64,
) -> self::MlTemporaryExpandRustProjectSourceEventsLoop28Result {
    loop {
        return if active < (frames.len() as f64) {
            if (self::read_required_number_fact(frames, active)) == child {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: ((self::ml_string::Text::from_units(vec![
                        99, 121, 99, 108, 105, 99, 32, 82, 117, 115, 116, 32, 112, 114, 111, 106,
                        101, 99, 116, 32, 109, 111, 100, 117, 108, 101, 32, 102, 105, 108, 101, 58,
                        32,
                    ]))
                    .concat(provider))
                    .0,
                    message_present: true,
                })
                .raise()
            } else {
                {
                    let active_2 = active + 4f64;
                    {
                        active = active_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryExpandRustProjectSourceEventsLoop28Result::MlTemporaryExpandRustProjectSourceEventsLoop28Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_expand_rust_project_source_events_loop22(
    paths: &[self::ml_string::Text],
    sources: &[self::ml_string::Text],
    values: &[Vec<self::ml_string::Text>],
    starts: &[Vec<f64>],
    ends: &[Vec<f64>],
    mut depth_limit: f64,
    mut expansion_limit: f64,
    mut output_unit_limit: f64,
    plans: &[Vec<f64>],
    path_attributes: &[Vec<f64>],
    mut frames: Vec<f64>,
    mut directories: Vec<self::ml_string::Text>,
    mut relative_directories: Vec<self::ml_string::Text>,
    mut owners: Vec<self::ml_string::Text>,
    mut events: Vec<self::ml_string::Text>,
    mut expansions: f64,
    mut output_units: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        return {
            let ml_library_condition_2 = {
                let ml_value = frames.len() as f64;
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_2 {
                {
                    let at = (frames.len() as f64) - 4f64;
                    {
                        let file = self::read_required_number_fact(&(frames), at);
                        {
                            let next = self::read_required_number_fact(&(frames), (at + 1f64));
                            {
                                let offset =
                                    self::read_required_number_fact(&(frames), (at + 2f64));
                                {
                                    let depth =
                                        self::read_required_number_fact(&(frames), (at + 3f64));
                                    {
                                        let frames_2 = self::ml_array::slice(&frames, 0f64, at);
                                        {
                                            let directory = self::read_required_text_fact(
                                                &(directories),
                                                ((directories.len() as f64) - 1f64),
                                            );
                                            {
                                                let relative = self::read_required_text_fact(
                                                    &(relative_directories),
                                                    ((relative_directories.len() as f64) - 1f64),
                                                );
                                                {
                                                    let owner = self::read_required_text_fact(
                                                        &(owners),
                                                        ((owners.len() as f64) - 1f64),
                                                    );
                                                    {
                                                        let directories_2 = self::ml_array::slice(
                                                            &directories,
                                                            0f64,
                                                            -1f64,
                                                        );
                                                        {
                                                            let relative_directories_2 =
                                                                self::ml_array::slice(
                                                                    &relative_directories,
                                                                    0f64,
                                                                    -1f64,
                                                                );
                                                            {
                                                                let owners_2 =
                                                                    self::ml_array::slice(
                                                                        &owners, 0f64, -1f64,
                                                                    );
                                                                {
                                                                    let source = self::read_required_text_fact(sources, file);
                                                                    {
                                                                        let path = self::read_required_text_fact(paths, file);
                                                                        {
                                                                            let plan =
                                                                                self::number_column(
                                                                                    plans, file,
                                                                                );
                                                                            {
                                                                                let attributes = self::number_column(path_attributes, file);
                                                                                {
                                                                                    let tokens = self::text_column(values, file);
                                                                                    {
                                                                                        let first = self::number_column(starts, file);
                                                                                        {
                                                                                            let last = self::number_column(ends, file);
                                                                                            {
                                                                                                let events_3 = if (next == 0f64) && (offset == 0f64) {
                                                                                                {
                                                                                                    { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![102, 105, 108, 101]), path.clone(), self::ml_string::Text::from_units(vec![48]), {
                                                                                                        let source_ml_source1_string_argument46 = source.length();
                                                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument46))
                                                                                                    }, self::ml_string::Text::from_units(vec![]), owner.clone(), self::ml_string::Text::from_units(vec![])], events); self::ml_array::append(ml_append_left, ml_append_right) }
                                                                                                }
                                                                                            } else {
                                                                                                events
                                                                                            };
                                                                                                if (next * 5f64) >= (plan.len() as f64) {
                                                                                                {
                                                                                                    let intervals = self::read_source_intervals(&(source), &(path), offset, source.length(), &(owner), &(plan), &(attributes), &(tokens), &(first), &(last));
                                                                                                    {
                                                                                                        let part = 4f64;
                                                                                                        {
                                                                                                            let output_units_2 = self::source_ml_temporary_expand_rust_project_source_events_loop23(output_units, &(intervals), part);
                                                                                                            {
                                                                                                                let source_ml_temporary_values19 = intervals;
                                                                                                                {
                                                                                                                    let source_ml_temporary_index19 = self::ml::Big::from_i128(0);
                                                                                                                    {
                                                                                                                        let events_4 = self::source_ml_temporary_expand_rust_project_source_events_loop24(events_3, &(source_ml_temporary_values19), source_ml_temporary_index19);
                                                                                                                        if output_units_2 > output_unit_limit {
                                                                                                                            (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 99, 111, 109, 112, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])).0, message_present: true }).raise()
                                                                                                                        } else {
                                                                                                                            {
                                                                                                                                let source_ml_source1_s49 = if depth > 0f64 {
                                                                                                                                    if (output_units_2 + 2f64) > output_unit_limit {
                                                                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 99, 111, 109, 112, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])).0, message_present: true }).raise()
                                                                                                                                    } else {
                                                                                                                                        {
                                                                                                                                            let events_5 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 97, 114, 121]), path.clone(), {
                                                                                                                                                let source_ml_source1_string_argument50 = source.length();
                                                                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument50))
                                                                                                                                            }, {
                                                                                                                                                let source_ml_source1_string_argument51 = source.length();
                                                                                                                                                self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument51))
                                                                                                                                            }, self::ml_string::Text::from_units(vec![10, 125]), owner.clone(), self::ml_string::Text::from_units(vec![])], events_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                            {
                                                                                                                                                let output_units_3 = output_units_2 + 2f64;
                                                                                                                                                self::MlTemporaryExpandRustProjectSourceEventsJoin25::MlTemporaryExpandRustProjectSourceEventsJoin25Next(events_5, output_units_3)
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                } else {
                                                                                                                                    self::MlTemporaryExpandRustProjectSourceEventsJoin25::MlTemporaryExpandRustProjectSourceEventsJoin25Next(events_4, output_units_2)
                                                                                                                                };
                                                                                                                                match source_ml_source1_s49 {
                                                                                                                                    self::MlTemporaryExpandRustProjectSourceEventsJoin25::MlTemporaryExpandRustProjectSourceEventsJoin25Next(events_6, output_units_4) => {
                                                                                                                                        {
                                                                                                                                            (frames, directories, relative_directories, owners, events, output_units) = (frames_2, directories_2, relative_directories_2, owners_2, events_6, output_units_4);
                                                                                                                                            continue;
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            } else {
                                                                                                {
                                                                                                    let keyword = self::read_required_number_fact(&(plan), (next * 5f64));
                                                                                                    {
                                                                                                        let name = self::read_required_text_fact(&(tokens), self::read_required_number_fact(&(plan), ((next * 5f64) + 1f64)));
                                                                                                        {
                                                                                                            let delimiter = self::read_required_number_fact(&(plan), ((next * 5f64) + 2f64));
                                                                                                            {
                                                                                                                let ancestors = self::module_ancestors(&(plan), &(tokens), next);
                                                                                                                {
                                                                                                                    let qualified = ((owner.concat(&self::ml_string::Text::from_units(vec![58, 58]))).concat(&{
let ml_library_condition_1 =  { let ml_value = ancestors.len() as f64; ml_value != 0f64 && !ml_value.is_nan() };
if ml_library_condition_1 {
                                                                                                                        (self::ml_string::join(&ancestors, &self::ml_string::Text::from_units(vec![58, 58]))).concat(&self::ml_string::Text::from_units(vec![58, 58]))
                                                                                                                    } else {
                                                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                                                    }
})).concat(&name);
                                                                                                                    if ((depth + (ancestors.len() as f64)) + 1f64) > depth_limit {
                                                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100, 117, 108, 101, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])).0, message_present: true }).raise()
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let context = self::module_source_context(&(directory), &(relative), &(plan), &(attributes), &(tokens), next);
                                                                                                                            {
                                                                                                                                let literal = self::read_required_number_fact(&(attributes), (next * 3f64));
                                                                                                                                {
                                                                                                                                    let module_directory = self::ml_string::join(&{ let ml_values2 = context.clone(); let ml_callback2 = std::rc::Rc::new(move |part_2: self::ml_string::Text| -> bool { part_2 != (self::ml_string::Text::from_units(vec![])) }); ml_values2.iter().cloned().enumerate().filter_map(|(ml_index2, ml_value2)| if ml_callback2(ml_value2.clone()) { Some(ml_value2) } else { None }).collect::<Vec<_>>() }, &self::ml_string::Text::from_units(vec![47]));
                                                                                                                                    {
                                                                                                                                        let external = (self::read_required_text_fact(&(tokens), delimiter)) == (self::ml_string::Text::from_units(vec![59]));
                                                                                                                                        {
                                                                                                                                            let provider = if external {
                                                                                                                                                if literal >= 0f64 {
                                                                                                                                                    self::resolve_rust_project_literal_path(&(self::read_required_text_fact(&(context), 0f64)), &(self::read_required_text_fact(&(tokens), literal)))
                                                                                                                                                } else {
                                                                                                                                                    self::select_rust_project_module_source(&(module_directory), &(name), paths)
                                                                                                                                                }
                                                                                                                                            } else {
                                                                                                                                                path.clone()
                                                                                                                                            };
                                                                                                                                            if external && !(self::ml_array::includes(paths, &provider.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: ((self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100, 117, 108, 101, 32, 102, 105, 108, 101, 58, 32])).concat(&provider)).0, message_present: true }).raise()
                                                                                                                                            } else {
                                                                                                                                                {
                                                                                                                                                    let events_7 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![109, 111, 100, 117, 108, 101]), path.clone(), {
                                                                                                                                                        let source_ml_source1_string_argument52 = self::read_required_number_fact(&(first), keyword);
                                                                                                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument52))
                                                                                                                                                    }, {
                                                                                                                                                        let source_ml_source1_string_argument53 = self::read_required_number_fact(&(last), (self::read_required_number_fact(&(plan), ((next * 5f64) + 3f64)) - 1f64));
                                                                                                                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument53))
                                                                                                                                                    }, self::ml_string::Text::from_units(vec![]), qualified.clone(), provider.clone()], events_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                    if !external {
                                                                                                                                                        {
                                                                                                                                                            let frames_3 = { let (ml_append_right, ml_append_left) = (vec![file, (next + 1f64), offset, depth], frames_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                            {
                                                                                                                                                                let directories_3 = { let (ml_append_right, ml_append_left) = (vec![directory.clone()], directories_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                {
                                                                                                                                                                    let relative_directories_3 = { let (ml_append_right, ml_append_left) = (vec![relative.clone()], relative_directories_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                    {
                                                                                                                                                                        let owners_3 = { let (ml_append_right, ml_append_left) = (vec![owner.clone()], owners_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                        {
                                                                                                                                                                            (frames, directories, relative_directories, owners, events) = (frames_3, directories_3, relative_directories_3, owners_3, events_7);
                                                                                                                                                                            continue;
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
{
                                                                                                                                                        {
                                                                                                                                                            let expansions_2 = expansions + 1f64;
                                                                                                                                                            if expansions_2 > expansion_limit {
                                                                                                                                                                (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100, 117, 108, 101, 32, 101, 120, 112, 97, 110, 115, 105, 111, 110, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])).0, message_present: true }).raise()
                                                                                                                                                            } else {
                                                                                                                                                                {
                                                                                                                                                                    let beginning = self::read_required_number_fact(&(first), delimiter);
                                                                                                                                                                    {
                                                                                                                                                                        let after = self::read_required_number_fact(&(last), delimiter);
                                                                                                                                                                        if ((beginning < offset) || (after < beginning)) || (after > (source.length())) {
                                                                                                                                                                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100, 117, 108, 101, 32, 115, 111, 117, 114, 99, 101, 32, 114, 97, 110, 103, 101])).0, message_present: true }).raise()
                                                                                                                                                                        } else {
                                                                                                                                                                            {
                                                                                                                                                                                let intervals_2 = self::read_source_intervals(&(source), &(path), offset, beginning, &(owner), &(plan), &(attributes), &(tokens), &(first), &(last));
                                                                                                                                                                                {
                                                                                                                                                                                    let output_units_5 = output_units + 2f64;
                                                                                                                                                                                    {
                                                                                                                                                                                        let part_2 = 4f64;
                                                                                                                                                                                        {
                                                                                                                                                                                            let output_units_6 = self::source_ml_temporary_expand_rust_project_source_events_loop26(output_units_5, &(intervals_2), part_2);
                                                                                                                                                                                            {
                                                                                                                                                                                                let source_ml_temporary_values20 = intervals_2;
                                                                                                                                                                                                {
                                                                                                                                                                                                    let source_ml_temporary_index20 = self::ml::Big::from_i128(0);
                                                                                                                                                                                                    {
                                                                                                                                                                                                        let events_8 = self::source_ml_temporary_expand_rust_project_source_events_loop27(events_7, &(source_ml_temporary_values20), source_ml_temporary_index20);
                                                                                                                                                                                                        if output_units_6 > output_unit_limit {
                                                                                                                                                                                                            (self::ml_source_exception::SourceException { class_name: String::from("RangeError"), message_units: (self::ml_string::Text::from_units(vec![82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 99, 111, 109, 112, 111, 115, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 117, 110, 105, 116, 32, 108, 105, 109, 105, 116, 32, 101, 120, 99, 101, 101, 100, 101, 100])).0, message_present: true }).raise()
                                                                                                                                                                                                        } else {
                                                                                                                                                                                                            {
                                                                                                                                                                                                                let events_9 = { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 97, 114, 121]), path.clone(), {
                                                                                                                                                                                                                    let source_ml_source1_string_argument54 = beginning;
                                                                                                                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument54))
                                                                                                                                                                                                                }, {
                                                                                                                                                                                                                    let source_ml_source1_string_argument55 = after;
                                                                                                                                                                                                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument55))
                                                                                                                                                                                                                }, self::ml_string::Text::from_units(vec![123, 10]), qualified.clone(), provider.clone()], events_8); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                {
                                                                                                                                                                                                                    let child = self::ml_array::index_of(paths, &provider.clone(), 0f64, false, |a, b| a == b);
                                                                                                                                                                                                                    if child == file {
                                                                                                                                                                                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: ((self::ml_string::Text::from_units(vec![99, 121, 99, 108, 105, 99, 32, 82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100, 117, 108, 101, 32, 102, 105, 108, 101, 58, 32])).concat(&provider)).0, message_present: true }).raise()
                                                                                                                                                                                                                    } else {
                                                                                                                                                                                                                        {
                                                                                                                                                                                                                            let active = 0f64;
                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                let source_ml_source1_s56 = self::source_ml_temporary_expand_rust_project_source_events_loop28(&(frames_2), &(provider), child, active);
                                                                                                                                                                                                                                match source_ml_source1_s56 {
                                                                                                                                                                                                                                    self::MlTemporaryExpandRustProjectSourceEventsLoop28Result::MlTemporaryExpandRustProjectSourceEventsLoop28Done => {
                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                            let frames_4 = { let (ml_append_right, ml_append_left) = (vec![file, (next + 1f64), after, depth], frames_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                let directories_4 = { let (ml_append_right, ml_append_left) = (vec![directory.clone()], directories_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                    let relative_directories_4 = { let (ml_append_right, ml_append_left) = (vec![relative.clone()], relative_directories_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                                        let owners_4 = { let (ml_append_right, ml_append_left) = (vec![owner.clone()], owners_2); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                            let frames_5 = { let (ml_append_right, ml_append_left) = (vec![child, 0f64, 0f64, ((depth + (ancestors.len() as f64)) + 1f64)], frames_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                let directories_5 = { let (ml_append_right, ml_append_left) = (vec![self::read_rust_project_module_directory(&(provider), true)], directories_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                    let filename = self::read_required_text_fact(&({
                                                                                                                                                                                                                                                                        let source_ml_source1_string_unit_operand57 = provider.clone();
                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                            let source_ml_source1_string_unit_operand58 = self::ml_string::Text::from_units(vec![47]);
                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                let source_ml_source1_string_split_limit59 = 4_294_967_295f64;
                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                    let source_ml_source1_string_split_separator60 = source_ml_source1_string_unit_operand58;
                                                                                                                                                                                                                                                                                    (|ml_text:&self::ml_string::Text,ml_separator:Option<&self::ml_string::Text>,ml_limit:f64|{self::ml_utf16_text::split_utf16_text(&(ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()), &(ml_separator.map(|ml_value|ml_value.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()).unwrap_or_default()), ml_limit, ml_separator.is_none()).into_iter().map(|ml_units|self::ml_string::Text::from_units(ml_units.into_iter().map(|ml_unit|ml_unit as u16).collect())).collect::<Vec<_>>()})(&(source_ml_source1_string_unit_operand57), Some(&source_ml_source1_string_split_separator60), source_ml_source1_string_split_limit59)
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                    }), ((({
                                                                                                                                                                                                                                                                        let source_ml_source1_string_unit_operand61 = provider.clone();
                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                            let source_ml_source1_string_unit_operand62 = self::ml_string::Text::from_units(vec![47]);
                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                let source_ml_source1_string_split_limit63 = 4_294_967_295f64;
                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                    let source_ml_source1_string_split_separator64 = source_ml_source1_string_unit_operand62;
                                                                                                                                                                                                                                                                                    (|ml_text:&self::ml_string::Text,ml_separator:Option<&self::ml_string::Text>,ml_limit:f64|{self::ml_utf16_text::split_utf16_text(&(ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()), &(ml_separator.map(|ml_value|ml_value.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()).unwrap_or_default()), ml_limit, ml_separator.is_none()).into_iter().map(|ml_units|self::ml_string::Text::from_units(ml_units.into_iter().map(|ml_unit|ml_unit as u16).collect())).collect::<Vec<_>>()})(&(source_ml_source1_string_unit_operand61), Some(&source_ml_source1_string_split_separator64), source_ml_source1_string_split_limit63)
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                    }).len() as f64) - 1f64));
                                                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                                                        let relative_directories_5 = { let (ml_append_right, ml_append_left) = (vec![if (literal >= 0f64) || (filename == (self::ml_string::Text::from_units(vec![109, 111, 100, 46, 114, 115]))) {
                                                                                                                                                                                                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                                                                                                                                                                                                        } else {
                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                let source_ml_source1_string_range_operand65 = filename.clone();
                                                                                                                                                                                                                                                                                {
                                                                                                                                                                                                                                                                                    let source_ml_source1_string_range_operand66 = 0f64;
                                                                                                                                                                                                                                                                                    {
                                                                                                                                                                                                                                                                                        let source_ml_source1_string_range_operand67 = -3f64;
                                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                                            let source_ml_source1_string_range_position68 = source_ml_source1_string_range_operand66;
                                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                                let source_ml_source1_string_range_position69 = source_ml_source1_string_range_operand67;
                                                                                                                                                                                                                                                                                                source_ml_source1_string_range_operand65.slice(source_ml_source1_string_range_position68, source_ml_source1_string_range_position69)
                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                        }], relative_directories_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                                                                        {
                                                                                                                                                                                                                                                                            let owners_5 = { let (ml_append_right, ml_append_left) = (vec![qualified.clone()], owners_4); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                                                                                                                                                                                                                            {
                                                                                                                                                                                                                                                                                (frames, directories, relative_directories, owners, events, expansions, output_units) = (frames_5, directories_5, relative_directories_5, owners_5, events_9, expansions_2, output_units_6);
                                                                                                                                                                                                                                                                                continue;
                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                }
                                                                                                                                                                                                            }
                                                                                                                                                                                                        }
                                                                                                                                                                                                    }
                                                                                                                                                                                                }
                                                                                                                                                                                            }
                                                                                                                                                                                        }
                                                                                                                                                                                    }
                                                                                                                                                                                }
                                                                                                                                                                            }
                                                                                                                                                                        }
                                                                                                                                                                    }
                                                                                                                                                                }
                                                                                                                                                            }
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                events
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryExpandRustProjectSourceEventsLoop21Result {
    MlTemporaryExpandRustProjectSourceEventsLoop21Done(Vec<Vec<f64>>, Vec<Vec<f64>>),
}
