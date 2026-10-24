// Generated from complete js/src/translation/javascript-record-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16;
use super::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result;
use super::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result;
use super::accept_java_script_record_parameter_bound_name;
use super::continues_java_script_directive_expression;
use super::ml_array;
use super::ml_string;
use super::read_required_boolean_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop17;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop15(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut ordinary: bool,
    mut at: f64,
    mut slot: f64,
    mut names: Vec<self::ml_string::Text>,
    mut instructions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result {
    loop {
        return if (at < (kinds.len() as f64))
            && !(((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![41]))))
        {
            if ((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![46, 46, 46])))
            {
                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Return(vec![-6f64, at])
            } else if ((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![91])))
            {
                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Return(vec![-3f64, at])
            } else {
                {
                    let source_ml_source2_s44 = if ((self::read_required_text_fact(kinds, at))
                        == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && ((self::read_required_text_fact(values, at))
                            == (self::ml_string::Text::from_units(vec![123])))
                    {
                        {
                            let open = at;
                            {
                                let at_2 = at + 1f64;
                                {
                                    let pairs = Vec::<f64>::new();
                                    {
                                        let source_ml_source2_s45 = self::source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop17(kinds, values, reserved, ordinary, at_2, names, pairs);
                                        match source_ml_source2_s45 {
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Done(at_3, names_2, pairs_2) => {
                                                    if (pairs_2.len() as f64) == 0f64 {
                                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16Return(vec![-4f64, open])
                                                    } else if (at_3 >= (kinds.len() as f64)) || !(((self::read_required_text_fact(kinds, at_3)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_3)) == (self::ml_string::Text::from_units(vec![125])))) {
                                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16Return(vec![-1f64, at_3])
                                                        } else {
                                                            {
                                                                let instructions_2 = {
                                                                    let source_ml_source2_concat_value48 = {
                                                                        let source_ml_source2_concat_value46 = instructions;
                                                                        {
                                                                            let source_ml_source2_concat_value47 = vec![1f64, slot, open, at_3, ((pairs_2.len() as f64) / 2f64)];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value46), source_ml_source2_concat_value47)
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_concat_value49 = pairs_2;
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value48), source_ml_source2_concat_value49)
                                                                    }
                                                                };
                                                                {
                                                                    let at_4 = at_3 + 1f64;
                                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16Next(at_4, names_2, instructions_2)
                                                                }
                                                            }
                                                        }
                                                }
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop17Return(source_ml_source1_temporary_result17) => {
                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16Return(source_ml_source1_temporary_result17)
                                                }
                                            }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let name = self::read_required_text_fact(values, at);
                            if (self::accept_java_script_record_parameter_bound_name(
                                &(self::read_required_text_fact(kinds, at)),
                                &(name),
                                &(names),
                                reserved,
                                ordinary,
                            )) {
                                {
                                    let names_3 = {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![name.clone()], names);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let instructions_3 = {
                                            let source_ml_source2_concat_value50 = instructions;
                                            {
                                                let source_ml_source2_concat_value51 =
                                                    vec![0f64, slot, at];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value50,
                                                    ),
                                                    source_ml_source2_concat_value51,
                                                )
                                            }
                                        };
                                        {
                                            let at_5 = at + 1f64;
                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16Next(at_5, names_3, instructions_3)
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16Return(vec![-8f64, at])
                            }
                        }
                    };
                    match source_ml_source2_s44 {
                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16Next(at_6, names_4, instructions_4) => {
                                if at_6 >= (kinds.len() as f64) {
                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Return(vec![-1f64, at_6])
                                } else if ((self::read_required_text_fact(kinds, at_6)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_6)) == (self::ml_string::Text::from_units(vec![61]))) {
                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Return(vec![-2f64, at_6])
                                    } else {
                                        {
                                            let slot_2 = slot + 1f64;
                                            if ((self::read_required_text_fact(kinds, at_6)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_6)) == (self::ml_string::Text::from_units(vec![44]))) {
                                                {
                                                    let at_7 = at_6 + 1f64;
                                                    {
                                                        (at, slot, names, instructions) = (at_7, slot_2, names_4, instructions_4);
                                                        continue;
                                                    }
                                                }
                                            }
{
                                                if (((self::read_required_text_fact(kinds, at_6)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_6)) == (self::ml_string::Text::from_units(vec![41])))) {
                                                    {
                                                        (at, slot, names, instructions) = (at_6, slot_2, names_4, instructions_4);
                                                        continue;
                                                    }
                                                }
{
                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Return(vec![-1f64, at_6])
                                                }
                                            }
                                        }
                                    }
                            }
                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin16Return(source_ml_source1_temporary_result16) => {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Return(source_ml_source1_temporary_result16)
                            }
                        }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop15Done(at, slot, instructions)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result {
    MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Done,
    MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_record_parameter_strict_directive_loop19(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
    line_breaks: &[bool],
    mut at: f64,
) -> self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result {
    loop {
        return if at < (kinds.len() as f64) {
            {
                let current = self::read_required_text_fact(kinds, at);
                if current == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(-2f64)
                } else if current
                    == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))
                {
                    {
                        let raw_ = self::read_required_text_fact(raws, at);
                        {
                            let strict = (raw_
                                == (self::ml_string::Text::from_units(vec![
                                    34, 117, 115, 101, 32, 115, 116, 114, 105, 99, 116, 34,
                                ])))
                                || (raw_
                                    == (self::ml_string::Text::from_units(vec![
                                        39, 117, 115, 101, 32, 115, 116, 114, 105, 99, 116, 39,
                                    ])));
                            {
                                let next = at + 1f64;
                                if next >= (kinds.len() as f64) {
                                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(-2f64)
                                } else {
                                    {
                                        let kind = self::read_required_text_fact(kinds, next);
                                        {
                                            let value = self::read_required_text_fact(values, next);
                                            if (kind
                                                == (self::ml_string::Text::from_units(vec![
                                                    112, 117, 110, 99, 116,
                                                ])))
                                                && (value
                                                    == (self::ml_string::Text::from_units(vec![
                                                        59,
                                                    ])))
                                            {
                                                if strict {
                                                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(at)
                                                } else {
                                                    {
                                                        let at_2 = next + 1f64;
                                                        {
                                                            at = at_2;
                                                            continue;
                                                        }
                                                    }
                                                }
                                            } else if (kind
                                                == (self::ml_string::Text::from_units(vec![
                                                    112, 117, 110, 99, 116,
                                                ])))
                                                && (value
                                                    == (self::ml_string::Text::from_units(vec![
                                                        125,
                                                    ])))
                                            {
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(if strict {
                                                            at
                                                        } else {
                                                            -1f64
                                                        })
                                            } else if kind
                                                == (self::ml_string::Text::from_units(vec![
                                                    101, 111, 102,
                                                ]))
                                            {
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(-2f64)
                                            } else if (self::read_required_boolean_fact(
                                                line_breaks,
                                                next,
                                            )) {
                                                if self::continues_java_script_directive_expression(&(kind), &(value)) {
                                                                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(-1f64)
                                                                } else if ((kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || (kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) || ((kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (((value == (self::ml_string::Text::from_units(vec![123]))) || (value == (self::ml_string::Text::from_units(vec![43, 43])))) || (value == (self::ml_string::Text::from_units(vec![45, 45]))))) {
                                                                        if strict {
                                                                            self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(at)
                                                                        } else {
                                                                            {
                                                                                let at_3 = next;
                                                                                {
                                                                                    at = at_3;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(-2f64)
                                                                    }
                                            } else {
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(-1f64)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Return(-1f64)
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop19Done
        };
    }
}
