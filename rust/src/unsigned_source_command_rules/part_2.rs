// Generated from complete js/src/unsigned-source-command-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Result;
use super::accept_unsigned_source_command_identity;
use super::ml_array;
use super::ml_string;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_unsigned_source_command_identity_loop1(
    value: &self::ml_string::Text,
    mut first: f64,
    mut at: f64,
) -> self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Result {
    loop {
        return if at < (value.length()) {
            {
                let code = {
                    let source_ml_source1_string_unit_operand26 = value.clone();
                    {
                        let source_ml_source1_string_unit_operand27 = at;
                        (std::rc::Rc::new(
                            move |text: self::ml_string::Text, position: f64| -> f64 {
                                {
                                    let index = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index < 0f64) || (index >= (text.length())) {
                                        f64::NAN
                                    } else {
                                        ({
                                    let source_ml_source1_string_unit_operand33 = {
                                        let source_ml_source1_string_range_operand28 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand29 = index;
                                            {
                                                let source_ml_source1_string_range_operand30 = index + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position31 = source_ml_source1_string_range_operand29;
                                                    {
                                                        let source_ml_source1_string_range_position32 = source_ml_source1_string_range_operand30;
                                                        source_ml_source1_string_range_operand28.slice(source_ml_source1_string_range_position31, source_ml_source1_string_range_position32)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand34 = 0f64;
                                        source_ml_source1_string_unit_operand33.code_point_at(source_ml_source1_string_unit_operand34)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                    }
                                }
                            },
                        ))(
                            source_ml_source1_string_unit_operand26,
                            source_ml_source1_string_unit_operand27,
                        )
                    }
                };
                if (code < 48f64) || (code > 57f64) {
                    self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Return(false)
                } else {
                    if (first == at) && (code == 48f64) {
                        {
                            let first_2 = first + 1f64;
                            {
                                let at_2 = at + 1f64;
                                {
                                    (first, at) = (first_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let at_3 = at + 1f64;
                            {
                                at = at_3;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Result::MlTemporaryAcceptUnsignedSourceCommandIdentityLoop1Done(first)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanUnsignedSourceCommandLoop3Result {
    MlTemporaryPlanUnsignedSourceCommandLoop3Done(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        bool,
        bool,
        bool,
    ),
    MlTemporaryPlanUnsignedSourceCommandLoop3Return(Vec<self::ml_string::Text>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanUnsignedSourceCommandJoin4 {
    MlTemporaryPlanUnsignedSourceCommandJoin4Next(
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
        self::ml_string::Text,
    ),
    MlTemporaryPlanUnsignedSourceCommandJoin4Return(Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_temporary_plan_unsigned_source_command_loop3(
    args: &[self::ml_string::Text],
    mut file: self::ml_string::Text,
    mut language: self::ml_string::Text,
    mut first: self::ml_string::Text,
    mut target: self::ml_string::Text,
    mut version: self::ml_string::Text,
    mut typed: bool,
    mut owners: bool,
    mut binary: bool,
    mut ended: bool,
    mut at: f64,
) -> self::MlTemporaryPlanUnsignedSourceCommandLoop3Result {
    loop {
        return if at < (args.len() as f64) {
            {
                let arg = (self::ml_array::optional_at(args, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                if !ended
                    && (((arg
                        == (self::ml_string::Text::from_units(vec![45, 45, 104, 101, 108, 112])))
                        || (arg == (self::ml_string::Text::from_units(vec![45, 104]))))
                        || (arg == (self::ml_string::Text::from_units(vec![104, 101, 108, 112]))))
                {
                    self::MlTemporaryPlanUnsignedSourceCommandLoop3Result::MlTemporaryPlanUnsignedSourceCommandLoop3Return(vec![self::ml_string::Text::from_units(vec![104, 101, 108, 112])])
                } else {
                    if !ended && (arg == (self::ml_string::Text::from_units(vec![45, 45]))) {
                        {
                            let ended_2 = true;
                            {
                                let at_2 = at + 1f64;
                                {
                                    (ended, at) = (ended_2, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if !ended
                            && (arg
                                == (self::ml_string::Text::from_units(vec![
                                    45, 45, 98, 105, 110, 97, 114, 121,
                                ])))
                        {
                            if binary {
                                self::MlTemporaryPlanUnsignedSourceCommandLoop3Result::MlTemporaryPlanUnsignedSourceCommandLoop3Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                            } else {
                                {
                                    let binary_2 = true;
                                    {
                                        let at_3 = at + 1f64;
                                        {
                                            (binary, at) = (binary_2, at_3);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if !ended
                            && (arg
                                == (self::ml_string::Text::from_units(vec![
                                    45, 45, 116, 121, 112, 101, 100,
                                ])))
                        {
                            if typed {
                                self::MlTemporaryPlanUnsignedSourceCommandLoop3Result::MlTemporaryPlanUnsignedSourceCommandLoop3Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                            } else {
                                {
                                    let typed_2 = true;
                                    {
                                        let at_4 = at + 1f64;
                                        {
                                            (typed, at) = (typed_2, at_4);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if !ended
                            && (arg
                                == (self::ml_string::Text::from_units(vec![
                                    45, 45, 114, 117, 115, 116, 45, 115, 111, 117, 114, 99, 101,
                                    45, 111, 119, 110, 101, 114, 115,
                                ])))
                        {
                            if owners {
                                self::MlTemporaryPlanUnsignedSourceCommandLoop3Result::MlTemporaryPlanUnsignedSourceCommandLoop3Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                            } else {
                                {
                                    let owners_2 = true;
                                    {
                                        let at_5 = at + 1f64;
                                        {
                                            (owners, at) = (owners_2, at_5);
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else if !ended
                            && ((((arg
                                == (self::ml_string::Text::from_units(vec![
                                    45, 45, 108, 97, 110, 103, 117, 97, 103, 101,
                                ])))
                                || (arg
                                    == (self::ml_string::Text::from_units(vec![
                                        45, 45, 102, 105, 114, 115, 116, 45, 105, 100,
                                    ]))))
                                || (arg
                                    == (self::ml_string::Text::from_units(vec![
                                        45, 45, 116, 111,
                                    ]))))
                                || (arg
                                    == (self::ml_string::Text::from_units(vec![
                                        45, 45, 115, 111, 117, 114, 99, 101, 45, 118, 101, 114,
                                        115, 105, 111, 110,
                                    ]))))
                        {
                            {
                                let at_6 = at + 1f64;
                                {
                                    let value = (self::ml_array::optional_at(
                                        args,
                                        self::ml_array::number_index(at_6),
                                    ))
                                    .unwrap_or_else(|| self::ml_string::Text::from_units(vec![]));
                                    if (value == (self::ml_string::Text::from_units(vec![])))
                                        || value.starts_with(&self::ml_string::Text::from_units(
                                            vec![45],
                                        ))
                                    {
                                        self::MlTemporaryPlanUnsignedSourceCommandLoop3Result::MlTemporaryPlanUnsignedSourceCommandLoop3Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                                    } else {
                                        {
                                            let source_ml_source1_s35 = if arg
                                                == (self::ml_string::Text::from_units(vec![
                                                    45, 45, 108, 97, 110, 103, 117, 97, 103, 101,
                                                ])) {
                                                {
                                                    let ml_library_inequality_3 = language
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        ));
                                                    if ml_library_inequality_3 {
                                                        self::MlTemporaryPlanUnsignedSourceCommandJoin4::MlTemporaryPlanUnsignedSourceCommandJoin4Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                                                    } else {
                                                        {
                                                            let language_2 = value.clone();
                                                            self::MlTemporaryPlanUnsignedSourceCommandJoin4::MlTemporaryPlanUnsignedSourceCommandJoin4Next(language_2, first.clone(), target.clone(), version.clone())
                                                        }
                                                    }
                                                }
                                            } else if arg
                                                == (self::ml_string::Text::from_units(vec![
                                                    45, 45, 102, 105, 114, 115, 116, 45, 105, 100,
                                                ]))
                                            {
                                                if (first != (self::ml_string::Text::from_units(vec![]))) || !(self::accept_unsigned_source_command_identity(&(value))) {
                                                                    self::MlTemporaryPlanUnsignedSourceCommandJoin4::MlTemporaryPlanUnsignedSourceCommandJoin4Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                                                                } else {
                                                                    {
                                                                        let first_2 = value.clone();
                                                                        self::MlTemporaryPlanUnsignedSourceCommandJoin4::MlTemporaryPlanUnsignedSourceCommandJoin4Next(language.clone(), first_2, target.clone(), version.clone())
                                                                    }
                                                                }
                                            } else if arg
                                                == (self::ml_string::Text::from_units(vec![
                                                    45, 45, 116, 111,
                                                ]))
                                            {
                                                {
                                                    let ml_library_inequality_2 = target
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        ));
                                                    if ml_library_inequality_2 {
                                                        self::MlTemporaryPlanUnsignedSourceCommandJoin4::MlTemporaryPlanUnsignedSourceCommandJoin4Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                                                    } else {
                                                        {
                                                            let target_2 = value.clone();
                                                            self::MlTemporaryPlanUnsignedSourceCommandJoin4::MlTemporaryPlanUnsignedSourceCommandJoin4Next(language.clone(), first.clone(), target_2, version.clone())
                                                        }
                                                    }
                                                }
                                            } else {
                                                let ml_library_inequality_1 = version
                                                    != (self::ml_string::Text::from_units(vec![]));
                                                if ml_library_inequality_1 {
                                                    self::MlTemporaryPlanUnsignedSourceCommandJoin4::MlTemporaryPlanUnsignedSourceCommandJoin4Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                                                } else {
                                                    {
                                                        let version_2 = value.clone();
                                                        self::MlTemporaryPlanUnsignedSourceCommandJoin4::MlTemporaryPlanUnsignedSourceCommandJoin4Next(language.clone(), first.clone(), target.clone(), version_2)
                                                    }
                                                }
                                            };
                                            match source_ml_source1_s35 {
                                                            self::MlTemporaryPlanUnsignedSourceCommandJoin4::MlTemporaryPlanUnsignedSourceCommandJoin4Next(language_3, first_3, target_3, version_3) => {
                                                                {
                                                                    let at_7 = at_6 + 1f64;
                                                                    {
                                                                        (language, first, target, version, at) = (language_3.clone(), first_3.clone(), target_3.clone(), version_3.clone(), at_7);
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryPlanUnsignedSourceCommandJoin4::MlTemporaryPlanUnsignedSourceCommandJoin4Return(source_ml_temporary_result4) => {
                                                                self::MlTemporaryPlanUnsignedSourceCommandLoop3Result::MlTemporaryPlanUnsignedSourceCommandLoop3Return(source_ml_temporary_result4)
                                                            }
                                                        }
                                        }
                                    }
                                }
                            }
                        } else if ((file != (self::ml_string::Text::from_units(vec![])))
                            || (arg == (self::ml_string::Text::from_units(vec![]))))
                            || (!ended
                                && arg.starts_with(&self::ml_string::Text::from_units(vec![45])))
                        {
                            self::MlTemporaryPlanUnsignedSourceCommandLoop3Result::MlTemporaryPlanUnsignedSourceCommandLoop3Return(vec![self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])])
                        } else {
                            {
                                let file_2 = arg.clone();
                                {
                                    let at_8 = at + 1f64;
                                    {
                                        (file, at) = (file_2, at_8);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryPlanUnsignedSourceCommandLoop3Result::MlTemporaryPlanUnsignedSourceCommandLoop3Done(file.clone(), language.clone(), first.clone(), target.clone(), version.clone(), typed, owners, binary)
        };
    }
}
