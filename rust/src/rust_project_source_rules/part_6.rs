// Generated from complete js/src/rust-project-source-rules.js. Do not edit by hand.
use super::MlTemporaryExpandRustProjectSourceEventsLoop21Result;
use super::accept_rust_project_source_path;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::number_column;
use super::read_required_text_fact;
use super::read_rust_project_module_path_attributes;
use super::read_rust_project_module_plan;
use super::text_column;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_expand_rust_project_source_events_loop21(
    paths: &[self::ml_string::Text],
    kinds: &[Vec<self::ml_string::Text>],
    values: &[Vec<self::ml_string::Text>],
    starts: &[Vec<f64>],
    ends: &[Vec<f64>],
    mut plans: Vec<Vec<f64>>,
    mut path_attributes: Vec<Vec<f64>>,
    mut file: f64,
) -> self::MlTemporaryExpandRustProjectSourceEventsLoop21Result {
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
                                        let plan = self::read_rust_project_module_plan(
                                            &(types),
                                            &(tokens),
                                        );
                                        {
                                            let plans_2 = {
                                                let (ml_append_right, ml_append_left) =
                                                    (vec![plan.clone()], plans);
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            };
                                            {
                                                let path_attributes_2 = {
                                                    let (ml_append_right, ml_append_left) = (vec![self::read_rust_project_module_path_attributes(&(types), &(tokens), &(plan))], path_attributes);
                                                    self::ml_array::append(
                                                        ml_append_left,
                                                        ml_append_right,
                                                    )
                                                };
                                                {
                                                    let file_2 = file + 1f64;
                                                    {
                                                        (plans, path_attributes, file) =
                                                            (plans_2, path_attributes_2, file_2);
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
            self::MlTemporaryExpandRustProjectSourceEventsLoop21Result::MlTemporaryExpandRustProjectSourceEventsLoop21Done(plans, path_attributes)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustSourceProjectCommandPlanLoop29Result {
    MlTemporaryReadRustSourceProjectCommandPlanLoop29Done(
        self::ml_string::Text,
        self::ml_string::Text,
        bool,
        bool,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRustSourceProjectCommandPlanJoin30 {
    MlTemporaryReadRustSourceProjectCommandPlanJoin30Next(
        self::ml_string::Text,
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_rust_source_project_command_plan_loop29(
    args: &[self::ml_string::Text],
    mut entry: self::ml_string::Text,
    mut target: self::ml_string::Text,
    mut binary: bool,
    mut library: bool,
    mut position: f64,
) -> self::MlTemporaryReadRustSourceProjectCommandPlanLoop29Result {
    loop {
        return if position < (args.len() as f64) {
            {
                let option = self::read_required_text_fact(args, position);
                if option
                    == (self::ml_string::Text::from_units(vec![45, 45, 98, 105, 110, 97, 114, 121]))
                {
                    if binary {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (self::ml_string::Text::from_units(vec![
                                100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 45, 45, 98, 105,
                                110, 97, 114, 121,
                            ]))
                            .0,
                            message_present: true,
                        })
                        .raise()
                    } else {
                        {
                            let binary_2 = true;
                            {
                                let position_2 = position + 1f64;
                                {
                                    (binary, position) = (binary_2, position_2);
                                    continue;
                                }
                            }
                        }
                    }
                } else if option
                    == (self::ml_string::Text::from_units(vec![
                        45, 45, 108, 105, 98, 114, 97, 114, 121,
                    ]))
                {
                    if library {
                        (self::ml_source_exception::SourceException {
                            class_name: String::from("TypeError"),
                            message_units: (self::ml_string::Text::from_units(vec![
                                100, 117, 112, 108, 105, 99, 97, 116, 101, 32, 45, 45, 108, 105,
                                98, 114, 97, 114, 121,
                            ]))
                            .0,
                            message_present: true,
                        })
                        .raise()
                    } else {
                        {
                            let library_2 = true;
                            {
                                let position_3 = position + 1f64;
                                {
                                    (library, position) = (library_2, position_3);
                                    continue;
                                }
                            }
                        }
                    }
                } else if ((option
                    != (self::ml_string::Text::from_units(vec![45, 45, 101, 110, 116, 114, 121])))
                    && (option != (self::ml_string::Text::from_units(vec![45, 45, 116, 111]))))
                    || ((position + 1f64) >= (args.len() as f64))
                {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: ((self::ml_string::Text::from_units(vec![
                            105, 110, 118, 97, 108, 105, 100, 32, 115, 111, 117, 114, 99, 101, 32,
                            112, 114, 111, 106, 101, 99, 116, 32, 97, 114, 103, 117, 109, 101, 110,
                            116, 58, 32,
                        ]))
                        .concat(&option))
                        .0,
                        message_present: true,
                    })
                    .raise()
                } else {
                    {
                        let value = self::read_required_text_fact(args, (position + 1f64));
                        if !(!value.0.is_empty())
                            || value.starts_with(&self::ml_string::Text::from_units(vec![45, 45]))
                        {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: ((self::ml_string::Text::from_units(vec![
                                    109, 105, 115, 115, 105, 110, 103, 32, 115, 111, 117, 114, 99,
                                    101, 32, 112, 114, 111, 106, 101, 99, 116, 32, 111, 112, 116,
                                    105, 111, 110, 32, 118, 97, 108, 117, 101, 58, 32,
                                ]))
                                .concat(&option))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        } else {
                            {
                                let source_ml_source1_s70 = if option
                                    == (self::ml_string::Text::from_units(vec![
                                        45, 45, 101, 110, 116, 114, 121,
                                    ])) {
                                    {
                                        let ml_library_inequality_2 =
                                            entry != (self::ml_string::Text::from_units(vec![]));
                                        if ml_library_inequality_2 {
                                            (self::ml_source_exception::SourceException {
                                                class_name: String::from("TypeError"),
                                                message_units: (self::ml_string::Text::from_units(
                                                    vec![
                                                        100, 117, 112, 108, 105, 99, 97, 116, 101,
                                                        32, 45, 45, 101, 110, 116, 114, 121,
                                                    ],
                                                ))
                                                .0,
                                                message_present: true,
                                            })
                                            .raise()
                                        } else {
                                            {
                                                let entry_2 = value.clone();
                                                self::MlTemporaryReadRustSourceProjectCommandPlanJoin30::MlTemporaryReadRustSourceProjectCommandPlanJoin30Next(entry_2, target.clone())
                                            }
                                        }
                                    }
                                } else {
                                    let ml_library_inequality_1 =
                                        target != (self::ml_string::Text::from_units(vec![]));
                                    if ml_library_inequality_1 {
                                        (self::ml_source_exception::SourceException {
                                            class_name: String::from("TypeError"),
                                            message_units: (self::ml_string::Text::from_units(
                                                vec![
                                                    100, 117, 112, 108, 105, 99, 97, 116, 101, 32,
                                                    45, 45, 116, 111,
                                                ],
                                            ))
                                            .0,
                                            message_present: true,
                                        })
                                        .raise()
                                    } else {
                                        {
                                            let target_2 = value.clone();
                                            self::MlTemporaryReadRustSourceProjectCommandPlanJoin30::MlTemporaryReadRustSourceProjectCommandPlanJoin30Next(entry.clone(), target_2)
                                        }
                                    }
                                };
                                match source_ml_source1_s70 {
                                            self::MlTemporaryReadRustSourceProjectCommandPlanJoin30::MlTemporaryReadRustSourceProjectCommandPlanJoin30Next(entry_3, target_3) => {
                                                {
                                                    let position_4 = position + 1f64;
                                                    {
                                                        let position_5 = position_4 + 1f64;
                                                        {
                                                            (entry, target, position) = (entry_3.clone(), target_3.clone(), position_5);
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
        } else {
            self::MlTemporaryReadRustSourceProjectCommandPlanLoop29Result::MlTemporaryReadRustSourceProjectCommandPlanLoop29Done(entry.clone(), target.clone(), binary, library)
        };
    }
}
