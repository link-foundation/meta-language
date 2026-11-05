// Generated from complete js/src/rust-project-source-rules.js. Do not edit by hand.
use super::MlTemporaryExpandRustProjectSourceEventsLoop21Result;
use super::MlTemporaryModuleSourceContextLoop16Result;
use super::MlTemporaryReadRustSourceProjectCommandPlanLoop29Result;
use super::MlTemporaryReadSourceIntervalsLoop17Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::ml_utf16_text;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_temporary_expand_rust_project_source_events_loop21;
use super::source_ml_temporary_expand_rust_project_source_events_loop22;
use super::source_ml_temporary_module_ancestors_loop18;
use super::source_ml_temporary_module_source_context_loop15;
use super::source_ml_temporary_module_source_context_loop16;
use super::source_ml_temporary_read_rust_source_project_command_plan_loop29;
use super::source_ml_temporary_read_source_intervals_loop17;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn module_source_context(
    directory: &self::ml_string::Text,
    relative: &self::ml_string::Text,
    plan: &[f64],
    attributes: &[f64],
    values: &[self::ml_string::Text],
    record: f64,
) -> Vec<self::ml_string::Text> {
    {
        let parent = self::read_required_number_fact(plan, ((record * 5f64) + 4f64));
        {
            let ancestors = Vec::<f64>::new();
            {
                let ancestors_2 =
                    self::source_ml_temporary_module_source_context_loop15(plan, parent, ancestors);
                {
                    let current = directory.clone();
                    {
                        let pending = relative.clone();
                        {
                            let source_ml_temporary_values14 = ancestors_2;
                            {
                                let source_ml_temporary_index14 = self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source1_s15 =
                                        self::source_ml_temporary_module_source_context_loop16(
                                            plan,
                                            attributes,
                                            values,
                                            current,
                                            pending,
                                            &(source_ml_temporary_values14),
                                            source_ml_temporary_index14,
                                        );
                                    match source_ml_source1_s15 {
                                        self::MlTemporaryModuleSourceContextLoop16Result::MlTemporaryModuleSourceContextLoop16Done(current_2, pending_2) => {
                                            vec![current_2, pending_2]
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_source_intervals(
    source: &self::ml_string::Text,
    path: &self::ml_string::Text,
    from: f64,
    to: f64,
    owner: &self::ml_string::Text,
    plan: &[f64],
    attributes: &[f64],
    values: &[self::ml_string::Text],
    starts: &[f64],
    ends: &[f64],
) -> Vec<self::ml_string::Text> {
    {
        let offset = from;
        {
            let events = Vec::<self::ml_string::Text>::new();
            {
                let record = 0f64;
                {
                    let source_ml_source1_s16 =
                        self::source_ml_temporary_read_source_intervals_loop17(
                            source, path, from, to, owner, plan, attributes, values, starts, ends,
                            offset, events, record,
                        );
                    match source_ml_source1_s16 {
                        self::MlTemporaryReadSourceIntervalsLoop17Result::MlTemporaryReadSourceIntervalsLoop17Done(offset_2, events_2) => {
                            {
                                { let (ml_append_right, ml_append_left) = (vec![self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101]), path.clone(), {
                                    let source_ml_source1_string_argument17 = offset_2;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument17))
                                }, {
                                    let source_ml_source1_string_argument18 = to;
                                    self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source1_string_argument18))
                                }, {
                                    let source_ml_source1_string_range_operand19 = source.clone();
                                    {
                                        let source_ml_source1_string_range_operand20 = offset_2;
                                        {
                                            let source_ml_source1_string_range_operand21 = to;
                                            {
                                                let source_ml_source1_string_range_position22 = source_ml_source1_string_range_operand20;
                                                {
                                                    let source_ml_source1_string_range_position23 = source_ml_source1_string_range_operand21;
                                                    source_ml_source1_string_range_operand19.slice(source_ml_source1_string_range_position22, source_ml_source1_string_range_position23)
                                                }
                                            }
                                        }
                                    }
                                }, owner.clone(), self::ml_string::Text::from_units(vec![])], events_2); self::ml_array::append(ml_append_left, ml_append_right) }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_project_module_directory(
    path: &self::ml_string::Text,
    root: bool,
) -> self::ml_string::Text {
    {
        let parts = {
            let source_ml_source1_string_unit_operand24 = path.clone();
            {
                let source_ml_source1_string_unit_operand25 =
                    self::ml_string::Text::from_units(vec![47]);
                {
                    let source_ml_source1_string_split_limit26 = 4_294_967_295f64;
                    {
                        let source_ml_source1_string_split_separator27 =
                            source_ml_source1_string_unit_operand25;
                        (|ml_text: &self::ml_string::Text,
                          ml_separator: Option<&self::ml_string::Text>,
                          ml_limit: f64| {
                            self::ml_utf16_text::split_utf16_text(
                                &(ml_text
                                    .0
                                    .iter()
                                    .map(|ml_unit| f64::from(*ml_unit))
                                    .collect::<Vec<f64>>()),
                                &(ml_separator
                                    .map(|ml_value| {
                                        ml_value
                                            .0
                                            .iter()
                                            .map(|ml_unit| f64::from(*ml_unit))
                                            .collect::<Vec<f64>>()
                                    })
                                    .unwrap_or_default()),
                                ml_limit,
                                ml_separator.is_none(),
                            )
                            .into_iter()
                            .map(|ml_units| {
                                self::ml_string::Text::from_units(
                                    ml_units.into_iter().map(|ml_unit| ml_unit as u16).collect(),
                                )
                            })
                            .collect::<Vec<_>>()
                        })(
                            &(source_ml_source1_string_unit_operand24),
                            Some(&source_ml_source1_string_split_separator27),
                            source_ml_source1_string_split_limit26,
                        )
                    }
                }
            }
        };
        {
            let file = self::read_required_text_fact(&(parts), ((parts.len() as f64) - 1f64));
            {
                let parent = self::ml_array::slice(&parts, 0f64, -1f64);
                self::ml_string::join(
                    &{
                        let (ml_append_right, ml_append_left) = (
                            if !root
                                && (file
                                    != (self::ml_string::Text::from_units(vec![
                                        109, 111, 100, 46, 114, 115,
                                    ])))
                            {
                                vec![{
                                    let source_ml_source1_string_range_operand28 = file.clone();
                                    {
                                        let source_ml_source1_string_range_operand29 = 0f64;
                                        {
                                            let source_ml_source1_string_range_operand30 = -3f64;
                                            {
                                                let source_ml_source1_string_range_position31 =
                                                    source_ml_source1_string_range_operand29;
                                                {
                                                    let source_ml_source1_string_range_position32 =
                                                        source_ml_source1_string_range_operand30;
                                                    source_ml_source1_string_range_operand28.slice(
                                                        source_ml_source1_string_range_position31,
                                                        source_ml_source1_string_range_position32,
                                                    )
                                                }
                                            }
                                        }
                                    }
                                }]
                            } else {
                                Vec::<self::ml_string::Text>::new()
                            },
                            parent,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    },
                    &self::ml_string::Text::from_units(vec![47]),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn select_rust_project_module_source(
    directory: &self::ml_string::Text,
    name: &self::ml_string::Text,
    available: &[self::ml_string::Text],
) -> self::ml_string::Text {
    {
        let prefix = if directory.0.is_empty() {
            self::ml_string::Text::from_units(vec![])
        } else {
            directory.concat(&self::ml_string::Text::from_units(vec![47]))
        };
        {
            let first = (prefix.concat(name))
                .concat(&self::ml_string::Text::from_units(vec![46, 114, 115]));
            {
                let second =
                    (prefix.concat(name)).concat(&self::ml_string::Text::from_units(vec![
                        47, 109, 111, 100, 46, 114, 115,
                    ]));
                {
                    let found_first =
                        self::ml_array::includes(available, &first.clone(), 0f64, false, |a, b| {
                            a == b
                        });
                    {
                        let found_second = self::ml_array::includes(
                            available,
                            &second.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        );
                        if found_first && found_second {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: ((((self::ml_string::Text::from_units(vec![
                                    97, 109, 98, 105, 103, 117, 111, 117, 115, 32, 82, 117, 115,
                                    116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100,
                                    117, 108, 101, 32, 102, 105, 108, 101, 115, 58, 32,
                                ]))
                                .concat(&first))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    32, 97, 110, 100, 32,
                                ])))
                                .concat(&second))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        } else if ((found_first) || (found_second)) {
                            if found_first {
                                first.clone()
                            } else {
                                second.clone()
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: ((((self::ml_string::Text::from_units(vec![
                                    109, 105, 115, 115, 105, 110, 103, 32, 82, 117, 115, 116, 32,
                                    112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100, 117, 108,
                                    101, 32, 102, 105, 108, 101, 58, 32,
                                ]))
                                .concat(&first))
                                .concat(&self::ml_string::Text::from_units(vec![
                                    32, 111, 114, 32,
                                ])))
                                .concat(&second))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn module_ancestors(
    plan: &[f64],
    values: &[self::ml_string::Text],
    record: f64,
) -> Vec<self::ml_string::Text> {
    {
        let parent = self::read_required_number_fact(plan, ((record * 5f64) + 4f64));
        {
            let ordered = Vec::<self::ml_string::Text>::new();
            { self::source_ml_temporary_module_ancestors_loop18(plan, values, parent, ordered) }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn expand_rust_project_source_events(
    paths: &[self::ml_string::Text],
    sources: &[self::ml_string::Text],
    kinds: &[Vec<self::ml_string::Text>],
    values: &[Vec<self::ml_string::Text>],
    starts: &[Vec<f64>],
    ends: &[Vec<f64>],
    entry: &self::ml_string::Text,
    depth_limit: f64,
    expansion_limit: f64,
    output_unit_limit: f64,
) -> Vec<self::ml_string::Text> {
    if (((((paths.len() as f64) != (sources.len() as f64))
        || ((paths.len() as f64) != (kinds.len() as f64)))
        || ((paths.len() as f64) != (values.len() as f64)))
        || ((paths.len() as f64) != (starts.len() as f64)))
        || ((paths.len() as f64) != (ends.len() as f64))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                117, 110, 97, 108, 105, 103, 110, 101, 100, 32, 82, 117, 115, 116, 32, 112, 114,
                111, 106, 101, 99, 116, 32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 118, 101,
                110, 116, 111, 114, 121,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else if ((((!(self::ml_math::is_safe_integer(depth_limit)) || (depth_limit < 1f64))
        || !(self::ml_math::is_safe_integer(expansion_limit)))
        || (expansion_limit < 1f64))
        || !(self::ml_math::is_safe_integer(output_unit_limit)))
        || (output_unit_limit < 1f64)
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("RangeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 82, 117, 115, 116, 32, 112, 114, 111, 106,
                101, 99, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 108, 105, 109, 105,
                116,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let unique = self::ml_array::set(paths.to_vec(), |a, b| a == b);
            {
                let ml_library_inequality_1 = (unique.len() as f64) != (paths.len() as f64);
                if ml_library_inequality_1 {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 82, 117, 115, 116, 32,
                            112, 114, 111, 106, 101, 99, 116, 32, 115, 111, 117, 114, 99, 101, 32,
                            112, 97, 116, 104,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                } else {
                    {
                        let plans = Vec::<Vec<f64>>::new();
                        {
                            let path_attributes = Vec::<Vec<f64>>::new();
                            {
                                let file = 0f64;
                                {
                                    let source_ml_source1_s33 = self::source_ml_temporary_expand_rust_project_source_events_loop21(paths, kinds, values, starts, ends, plans, path_attributes, file);
                                    match source_ml_source1_s33 {
                                        self::MlTemporaryExpandRustProjectSourceEventsLoop21Result::MlTemporaryExpandRustProjectSourceEventsLoop21Done(plans_2, path_attributes_2) => {
                                            {
                                                let root = self::ml_array::index_of(paths, entry, 0f64, false, |a, b| a == b);
                                                if root < 0f64 {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![109, 105, 115, 115, 105, 110, 103, 32, 82, 117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 101, 110, 116, 114, 121, 32, 115, 111, 117, 114, 99, 101])).0, message_present: true }).raise()
                                                } else {
                                                    {
                                                        let frames = vec![root, 0f64, 0f64, 0f64];
                                                        {
                                                            let directories = vec![self::read_rust_project_module_directory(entry, true)];
                                                            {
                                                                let relative_directories = vec![self::ml_string::Text::from_units(vec![])];
                                                                {
                                                                    let owners = vec![self::ml_string::Text::from_units(vec![99, 114, 97, 116, 101])];
                                                                    {
                                                                        let events = Vec::<self::ml_string::Text>::new();
                                                                        {
                                                                            let expansions = 0f64;
                                                                            {
                                                                                let output_units = 0f64;
                                                                                {
                                                                                    self::source_ml_temporary_expand_rust_project_source_events_loop22(paths, sources, values, starts, ends, depth_limit, expansion_limit, output_unit_limit, &(plans_2), &(path_attributes_2), frames, directories, relative_directories, owners, events, expansions, output_units)
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_rust_source_project_command_plan(
    args: &[self::ml_string::Text],
) -> Vec<self::ml_string::Text> {
    if ((args.len() as f64) == 0f64)
        || (((args.len() as f64) == 1f64)
            && (((self::read_required_text_fact(args, 0f64))
                == (self::ml_string::Text::from_units(vec![104, 101, 108, 112])))
                || ((self::read_required_text_fact(args, 0f64))
                    == (self::ml_string::Text::from_units(vec![45, 45, 104, 101, 108, 112])))))
    {
        Vec::<self::ml_string::Text>::new()
    } else {
        {
            let input = self::read_required_text_fact(args, 0f64);
            if !(!input.0.is_empty())
                || input.starts_with(&self::ml_string::Text::from_units(vec![45]))
            {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        101, 120, 112, 101, 99, 116, 101, 100, 32, 97, 32, 76, 105, 110, 107, 115,
                        32, 115, 111, 117, 114, 99, 101, 32, 105, 110, 118, 101, 110, 116, 111,
                        114, 121, 32, 102, 105, 108, 101,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
            } else {
                {
                    let entry = self::ml_string::Text::from_units(vec![]);
                    {
                        let target = self::ml_string::Text::from_units(vec![]);
                        {
                            let binary = false;
                            {
                                let library = false;
                                {
                                    let position = 1f64;
                                    {
                                        let source_ml_source1_s34 = self::source_ml_temporary_read_rust_source_project_command_plan_loop29(args, entry, target, binary, library, position);
                                        match source_ml_source1_s34 {
                                            self::MlTemporaryReadRustSourceProjectCommandPlanLoop29Result::MlTemporaryReadRustSourceProjectCommandPlanLoop29Done(entry_2, target_2, binary_2, library_2) => {
                                                if entry_2 == (self::ml_string::Text::from_units(vec![])) {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 111, 117, 114, 99, 101, 32, 112, 114, 111, 106, 101, 99, 116, 32, 101, 110, 116, 114, 121, 32, 105, 115, 32, 114, 101, 113, 117, 105, 114, 101, 100])).0, message_present: true }).raise()
                                                } else if binary_2 && (target_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121, 32, 103, 114, 97, 112, 104, 32, 111, 117, 116, 112, 117, 116, 32, 97, 110, 100, 32, 116, 97, 114, 103, 101, 116, 32, 115, 111, 117, 114, 99, 101, 32, 97, 114, 101, 32, 115, 101, 112, 97, 114, 97, 116, 101, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111, 110, 115])).0, message_present: true }).raise()
                                                    } else {
                                                        vec![input.clone(), entry_2, target_2, if binary_2 {
                                                            self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                                                        }, if library_2 {
                                                            self::ml_string::Text::from_units(vec![116, 114, 117, 101])
                                                        } else {
                                                            self::ml_string::Text::from_units(vec![102, 97, 108, 115, 101])
                                                        }]
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryGroupEndLoop1Result {
    MlTemporaryGroupEndLoop1Done,
    MlTemporaryGroupEndLoop1Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_group_end_loop1(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut stack: Vec<self::ml_string::Text>,
    mut position: f64,
) -> self::MlTemporaryGroupEndLoop1Result {
    loop {
        return if position < (values.len() as f64) {
            if (self::read_required_text_fact(kinds, position))
                != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))
            {
                {
                    let position_2 = position + 1f64;
                    {
                        position = position_2;
                        continue;
                    }
                }
            }
            {
                {
                    let value = self::read_required_text_fact(values, position);
                    if ((value == (self::ml_string::Text::from_units(vec![123])))
                        || (value == (self::ml_string::Text::from_units(vec![91]))))
                        || (value == (self::ml_string::Text::from_units(vec![40])))
                    {
                        {
                            let stack_2 = {
                                let (ml_append_right, ml_append_left) = (
                                    vec![
                                        if value == (self::ml_string::Text::from_units(vec![123])) {
                                            self::ml_string::Text::from_units(vec![125])
                                        } else if value
                                            == (self::ml_string::Text::from_units(vec![91]))
                                        {
                                            self::ml_string::Text::from_units(vec![93])
                                        } else {
                                            self::ml_string::Text::from_units(vec![41])
                                        },
                                    ],
                                    stack,
                                );
                                self::ml_array::append(ml_append_left, ml_append_right)
                            };
                            {
                                let position_3 = position + 1f64;
                                {
                                    (stack, position) = (stack_2, position_3);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if ((value == (self::ml_string::Text::from_units(vec![125])))
                            || (value == (self::ml_string::Text::from_units(vec![93]))))
                            || (value == (self::ml_string::Text::from_units(vec![41])))
                        {
                            if value
                                == (self::read_required_text_fact(
                                    &(stack),
                                    ((stack.len() as f64) - 1f64),
                                ))
                            {
                                {
                                    let stack_3 = self::ml_array::slice(&stack, 0f64, -1f64);
                                    if (stack_3.len() as f64) == 0f64 {
                                        self::MlTemporaryGroupEndLoop1Result::MlTemporaryGroupEndLoop1Return(position + 1f64)
                                    } else {
                                        {
                                            let position_4 = position + 1f64;
                                            {
                                                (stack, position) = (stack_3, position_4);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        117, 110, 98, 97, 108, 97, 110, 99, 101, 100, 32, 82, 117,
                                        115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 115,
                                        111, 117, 114, 99, 101, 32, 103, 114, 111, 117, 112,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        } else {
                            {
                                let position_5 = position + 1f64;
                                {
                                    position = position_5;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryGroupEndLoop1Result::MlTemporaryGroupEndLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRustProjectModulePlanJoin3 {
    MlTemporaryReadRustProjectModulePlanJoin3Next(Vec<f64>, Vec<f64>, f64, f64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustProjectModulePlanJoin4 {
    MlTemporaryReadRustProjectModulePlanJoin4Next,
}
