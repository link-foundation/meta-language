// Generated from complete js/src/rust-project-source-rules.js. Do not edit by hand.
use super::MlTemporaryReadRustSourceProjectCommandPlanLoop12Result;
use super::accept_rust_project_source_path;
use super::group_end;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::number_column;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::read_rust_project_module_directory;
use super::read_rust_project_module_plan;
use super::source_ml_temporary_expand_rust_project_source_events_loop9;
use super::source_ml_temporary_read_rust_source_project_command_plan_loop12;
use super::text_column;

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
                            let file = 0f64;
                            {
                                let plans_2 = self::source_ml_temporary_expand_rust_project_source_events_loop8(paths, kinds, values, starts, ends, plans, file);
                                {
                                    let root = self::ml_array::index_of(
                                        paths,
                                        entry,
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    );
                                    if root < 0f64 {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    109, 105, 115, 115, 105, 110, 103, 32, 82, 117,
                                                    115, 116, 32, 112, 114, 111, 106, 101, 99, 116,
                                                    32, 101, 110, 116, 114, 121, 32, 115, 111, 117,
                                                    114, 99, 101,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    } else {
                                        {
                                            let frames = vec![root, 0f64, 0f64, 0f64];
                                            {
                                                let directories =
                                                    vec![self::read_rust_project_module_directory(
                                                        entry, true,
                                                    )];
                                                {
                                                    let owners =
                                                        vec![self::ml_string::Text::from_units(
                                                            vec![99, 114, 97, 116, 101],
                                                        )];
                                                    {
                                                        let events =
                                                            Vec::<self::ml_string::Text>::new();
                                                        {
                                                            let expansions = 0f64;
                                                            {
                                                                let output_units = 0f64;
                                                                {
                                                                    self::source_ml_temporary_expand_rust_project_source_events_loop9(paths, sources, values, starts, ends, depth_limit, expansion_limit, output_unit_limit, &(plans_2), frames, directories, owners, events, expansions, output_units)
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
                                        let source_ml_source1_s16 = self::source_ml_temporary_read_rust_source_project_command_plan_loop12(args, entry, target, binary, library, position);
                                        match source_ml_source1_s16 {
                                            self::MlTemporaryReadRustSourceProjectCommandPlanLoop12Result::MlTemporaryReadRustSourceProjectCommandPlanLoop12Done(entry_2, target_2, binary_2, library_2) => {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_project_module_plan_loop2(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut plan: Vec<f64>,
    mut parents: Vec<f64>,
    mut closes: Vec<f64>,
    mut position: f64,
    mut parent: f64,
) -> Vec<f64> {
    loop {
        return if position < (values.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, position);
                {
                    let value = self::read_required_text_fact(values, position);
                    if ((closes.len() as f64) > 0f64)
                        && (position
                            == (self::read_required_number_fact(
                                &(closes),
                                ((closes.len() as f64) - 1f64),
                            )))
                    {
                        {
                            let closes_2 = self::ml_array::slice(&closes, 0f64, -1f64);
                            {
                                let parent_2 = self::read_required_number_fact(
                                    &(parents),
                                    ((parents.len() as f64) - 1f64),
                                );
                                {
                                    let parents_2 = self::ml_array::slice(&parents, 0f64, -1f64);
                                    {
                                        let position_2 = position + 1f64;
                                        {
                                            (parents, closes, position, parent) =
                                                (parents_2, closes_2, position_2, parent_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    {
                        if (kind
                            == (self::ml_string::Text::from_units(vec![
                                107, 101, 121, 119, 111, 114, 100,
                            ])))
                            && (value == (self::ml_string::Text::from_units(vec![109, 111, 100])))
                        {
                            if ((position + 2f64) >= (values.len() as f64))
                                || ((self::read_required_text_fact(kinds, (position + 1f64)))
                                    != (self::ml_string::Text::from_units(vec![
                                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                                    ])))
                            {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        105, 110, 118, 97, 108, 105, 100, 32, 82, 117, 115, 116,
                                        32, 112, 114, 111, 106, 101, 99, 116, 32, 109, 111, 100,
                                        117, 108, 101, 32, 110, 97, 109, 101,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            } else {
                                {
                                    let delimiter =
                                        self::read_required_text_fact(values, (position + 2f64));
                                    if ((self::read_required_text_fact(kinds, (position + 2f64)))
                                        != (self::ml_string::Text::from_units(vec![
                                            112, 117, 110, 99, 116,
                                        ])))
                                        || ((delimiter
                                            != (self::ml_string::Text::from_units(vec![59])))
                                            && (delimiter
                                                != (self::ml_string::Text::from_units(vec![123]))))
                                    {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    105, 110, 118, 97, 108, 105, 100, 32, 82, 117,
                                                    115, 116, 32, 112, 114, 111, 106, 101, 99, 116,
                                                    32, 109, 111, 100, 117, 108, 101, 32, 98, 111,
                                                    117, 110, 100, 97, 114, 121,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    } else {
                                        {
                                            let after = if delimiter
                                                == (self::ml_string::Text::from_units(vec![59]))
                                            {
                                                position + 3f64
                                            } else {
                                                self::group_end(kinds, values, (position + 2f64))
                                            };
                                            {
                                                let record = (plan.len() as f64) / 5f64;
                                                {
                                                    let plan_2 = {
                                                        let (ml_append_right, ml_append_left) = (
                                                            vec![
                                                                position,
                                                                (position + 1f64),
                                                                (position + 2f64),
                                                                after,
                                                                parent,
                                                            ],
                                                            plan,
                                                        );
                                                        self::ml_array::append(
                                                            ml_append_left,
                                                            ml_append_right,
                                                        )
                                                    };
                                                    {
                                                        let source_ml_source1_s17 = if delimiter
                                                            == (self::ml_string::Text::from_units(
                                                                vec![123],
                                                            )) {
                                                            {
                                                                let parents_3 = {
                                                                    let (
                                                                        ml_append_right,
                                                                        ml_append_left,
                                                                    ) = (vec![parent], parents);
                                                                    self::ml_array::append(
                                                                        ml_append_left,
                                                                        ml_append_right,
                                                                    )
                                                                };
                                                                {
                                                                    let closes_3 = {
                                                                        let (
                                                                            ml_append_right,
                                                                            ml_append_left,
                                                                        ) = (
                                                                            vec![(after - 1f64)],
                                                                            closes,
                                                                        );
                                                                        self::ml_array::append(
                                                                            ml_append_left,
                                                                            ml_append_right,
                                                                        )
                                                                    };
                                                                    {
                                                                        let parent_3 = record;
                                                                        {
                                                                            let position_3 =
                                                                                position + 3f64;
                                                                            self::MlTemporaryReadRustProjectModulePlanJoin3::MlTemporaryReadRustProjectModulePlanJoin3Next(parents_3, closes_3, position_3, parent_3)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                let position_4 = after;
                                                                self::MlTemporaryReadRustProjectModulePlanJoin3::MlTemporaryReadRustProjectModulePlanJoin3Next(parents, closes, position_4, parent)
                                                            }
                                                        };
                                                        match source_ml_source1_s17 {
                                                            self::MlTemporaryReadRustProjectModulePlanJoin3::MlTemporaryReadRustProjectModulePlanJoin3Next(parents_4, closes_4, position_5, parent_4) => {
                                                                {
                                                                    (plan, parents, closes, position, parent) = (plan_2, parents_4, closes_4, position_5, parent_4);
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
                        } else if (kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && (((value == (self::ml_string::Text::from_units(vec![123])))
                                || (value == (self::ml_string::Text::from_units(vec![91]))))
                                || (value == (self::ml_string::Text::from_units(vec![40]))))
                        {
                            {
                                let source_ml_source1_s18 = if ((value
                                    == (self::ml_string::Text::from_units(vec![91])))
                                    && (position > 0f64))
                                    && (((self::read_required_text_fact(
                                        values,
                                        (position - 1f64),
                                    )) == (self::ml_string::Text::from_units(vec![35])))
                                        || (((position > 1f64)
                                            && ((self::read_required_text_fact(
                                                values,
                                                (position - 1f64),
                                            )) == (self::ml_string::Text::from_units(vec![
                                                33,
                                            ]))))
                                            && ((self::read_required_text_fact(
                                                values,
                                                (position - 2f64),
                                            )) == (self::ml_string::Text::from_units(vec![
                                                35,
                                            ]))))) {
                                    {
                                        let attribute = if (position + 1f64) < (values.len() as f64)
                                        {
                                            self::read_required_text_fact(values, (position + 1f64))
                                        } else {
                                            self::ml_string::Text::from_units(vec![])
                                        };
                                        if (((attribute
                                            == (self::ml_string::Text::from_units(vec![
                                                99, 102, 103,
                                            ])))
                                            || (attribute
                                                == (self::ml_string::Text::from_units(vec![
                                                    99, 102, 103, 95, 97, 116, 116, 114,
                                                ]))))
                                            || (attribute
                                                == (self::ml_string::Text::from_units(vec![
                                                    112, 97, 116, 104,
                                                ]))))
                                            || (attribute
                                                == (self::ml_string::Text::from_units(vec![
                                                    109, 97, 99, 114, 111, 95, 117, 115, 101,
                                                ])))
                                        {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        82, 117, 115, 116, 32, 112, 114, 111, 106,
                                                        101, 99, 116, 32, 109, 111, 100, 117, 108,
                                                        101, 32, 97, 116, 116, 114, 105, 98, 117,
                                                        116, 101, 115, 32, 110, 101, 101, 100, 32,
                                                        97, 110, 32, 101, 120, 112, 108, 105, 99,
                                                        105, 116, 32, 99, 111, 110, 102, 105, 103,
                                                        117, 114, 97, 116, 105, 111, 110, 32, 105,
                                                        110, 116, 101, 114, 112, 114, 101, 116, 97,
                                                        116, 105, 111, 110,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        } else {
                                            self::MlTemporaryReadRustProjectModulePlanJoin4::MlTemporaryReadRustProjectModulePlanJoin4Next
                                        }
                                    }
                                } else {
                                    self::MlTemporaryReadRustProjectModulePlanJoin4::MlTemporaryReadRustProjectModulePlanJoin4Next
                                };
                                match source_ml_source1_s18 {
                                        self::MlTemporaryReadRustProjectModulePlanJoin4::MlTemporaryReadRustProjectModulePlanJoin4Next => {
                                            {
                                                let position_6 = self::group_end(kinds, values, position);
                                                {
                                                    position = position_6;
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                            }
                        } else if (kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && (((value == (self::ml_string::Text::from_units(vec![125])))
                                || (value == (self::ml_string::Text::from_units(vec![93]))))
                                || (value == (self::ml_string::Text::from_units(vec![41]))))
                        {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    117, 110, 101, 120, 112, 101, 99, 116, 101, 100, 32, 82, 117,
                                    115, 116, 32, 112, 114, 111, 106, 101, 99, 116, 32, 99, 108,
                                    111, 115, 105, 110, 103, 32, 103, 114, 111, 117, 112,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        } else {
                            {
                                let position_7 = position + 1f64;
                                {
                                    position = position_7;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            plan
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRustProjectSourcePathLoop6Result {
    MlTemporaryAcceptRustProjectSourcePathLoop6Done,
    MlTemporaryAcceptRustProjectSourcePathLoop6Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_rust_project_source_path_loop6(
    source_ml_temporary_values5: &[self::ml_string::Text],
    mut source_ml_temporary_index5: self::ml::Big,
) -> self::MlTemporaryAcceptRustProjectSourcePathLoop6Result {
    loop {
        return if source_ml_temporary_index5
            < (self::ml::Big::from_u128(source_ml_temporary_values5.len() as u128))
        {
            {
                let part = self::ml_array::at(
                    source_ml_temporary_values5,
                    source_ml_temporary_index5.to_index(),
                );
                if (!(!part.0.is_empty())
                    || (part == (self::ml_string::Text::from_units(vec![46]))))
                    || (part == (self::ml_string::Text::from_units(vec![46, 46])))
                {
                    self::MlTemporaryAcceptRustProjectSourcePathLoop6Result::MlTemporaryAcceptRustProjectSourcePathLoop6Return(false)
                } else {
                    {
                        let source_ml_temporary_index5_2 =
                            source_ml_temporary_index5.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index5 = source_ml_temporary_index5_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRustProjectSourcePathLoop6Result::MlTemporaryAcceptRustProjectSourcePathLoop6Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_module_ancestors_loop7(
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
pub fn source_ml_temporary_expand_rust_project_source_events_loop8(
    paths: &[self::ml_string::Text],
    kinds: &[Vec<self::ml_string::Text>],
    values: &[Vec<self::ml_string::Text>],
    starts: &[Vec<f64>],
    ends: &[Vec<f64>],
    mut plans: Vec<Vec<f64>>,
    mut file: f64,
) -> Vec<Vec<f64>> {
    loop {
        return if file < (paths.len() as f64) {
            if (self::accept_rust_project_source_path(
                &(self::read_required_text_fact(paths, file)),
            )) {
                {
                    let types = self::text_column(kinds, file);
                    {
                        let tokens = self::text_column(values, file);
                        {
                            let first = self::number_column(starts, file);
                            {
                                let last = self::number_column(ends, file);
                                if ((types.len() as f64) != (first.len() as f64))
                                    || ((types.len() as f64) != (last.len() as f64))
                                {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (self::ml_string::Text::from_units(vec![
                                            117, 110, 97, 108, 105, 103, 110, 101, 100, 32, 82,
                                            117, 115, 116, 32, 112, 114, 111, 106, 101, 99, 116,
                                            32, 116, 111, 107, 101, 110, 32, 114, 97, 110, 103,
                                            101, 115,
                                        ]))
                                        .0,
                                        message_present: true,
                                    })
                                    .raise()
                                } else {
                                    {
                                        let plans_2 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![self::read_rust_project_module_plan(
                                                    &(types),
                                                    &(tokens),
                                                )],
                                                plans,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let file_2 = file + 1f64;
                                            {
                                                (plans, file) = (plans_2, file_2);
                                                continue;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        105, 110, 118, 97, 108, 105, 100, 32, 82, 117, 115, 116, 32, 112, 114, 111,
                        106, 101, 99, 116, 32, 115, 111, 117, 114, 99, 101, 32, 112, 97, 116, 104,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
            }
        } else {
            plans
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryExpandRustProjectSourceEventsJoin10 {
    MlTemporaryExpandRustProjectSourceEventsJoin10Next(Vec<self::ml_string::Text>, f64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryExpandRustProjectSourceEventsLoop11Result {
    MlTemporaryExpandRustProjectSourceEventsLoop11Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_expand_rust_project_source_events_loop11(
    frames: &[f64],
    provider: &self::ml_string::Text,
    mut child: f64,
    mut active: f64,
) -> self::MlTemporaryExpandRustProjectSourceEventsLoop11Result {
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
            self::MlTemporaryExpandRustProjectSourceEventsLoop11Result::MlTemporaryExpandRustProjectSourceEventsLoop11Done
        };
    }
}
