// Generated from complete js/src/translation/record-closure-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30;
use super::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32;
use super::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33;
use super::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result;
use super::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result;
use super::accept_java_script_record_parameter_bound_name;
use super::continues_java_script_directive_expression;
use super::find_java_script_pattern_default_expression_end;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_required_boolean_fact;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop31(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut ordinary: bool,
    mut defaults: bool,
    mut at: f64,
    mut names: Vec<self::ml_string::Text>,
    mut pairs: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result {
    loop {
        return if (at < (kinds.len() as f64))
            && !(((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![125]))))
        {
            if ((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![46, 46, 46])))
            {
                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Return(vec![-6f64, at])
            } else {
                let ml_library_inequality_1 = (self::read_required_text_fact(kinds, at))
                    != (self::ml_string::Text::from_units(vec![
                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                    ]));
                if ml_library_inequality_1 {
                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Return(vec![-7f64, at])
                } else {
                    {
                        let property = at;
                        {
                            let at_2 = at + 1f64;
                            if at_2 >= (kinds.len() as f64) {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Return(vec![-1f64, at_2])
                            } else {
                                {
                                    let binding = property;
                                    {
                                        let source_ml_source2_s160 =
                                            if ((self::read_required_text_fact(kinds, at_2))
                                                == (self::ml_string::Text::from_units(vec![
                                                    112, 117, 110, 99, 116,
                                                ])))
                                                && ((self::read_required_text_fact(values, at_2))
                                                    == (self::ml_string::Text::from_units(vec![
                                                        58,
                                                    ])))
                                            {
                                                {
                                                    let at_3 = at_2 + 1f64;
                                                    if at_3 >= (kinds.len() as f64) {
                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32Return(vec![-1f64, at_3])
                                                } else if ((self::read_required_text_fact(kinds, at_3)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (((self::read_required_text_fact(values, at_3)) == (self::ml_string::Text::from_units(vec![123]))) || ((self::read_required_text_fact(values, at_3)) == (self::ml_string::Text::from_units(vec![91])))) {
                                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32Return(vec![-5f64, at_3])
                                                    } else {
                                                        {
                                                            let binding_2 = at_3;
                                                            {
                                                                let at_4 = at_3 + 1f64;
                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32Next(at_4, binding_2)
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32Next(at_2, binding)
                                            };
                                        match source_ml_source2_s160 {
                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32Next(at_5, binding_3) => {
                                                {
                                                    let name = self::read_required_text_fact(values, binding_3);
                                                    if (self::accept_java_script_record_parameter_bound_name(&(self::read_required_text_fact(kinds, binding_3)), &(name), &(names), reserved, ordinary)) {
                                                        {
                                                            let names_2 = { let (ml_append_right, ml_append_left) = (vec![name.clone()], names); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            if at_5 >= (kinds.len() as f64) {
                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Return(vec![-1f64, at_5])
                                                            } else {
                                                                {
                                                                    let first = -1f64;
                                                                    {
                                                                        let last = -1f64;
                                                                        {
                                                                            let source_ml_source2_s161 = if ((self::read_required_text_fact(kinds, at_5)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_5)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                                if defaults {
                                                                                    {
                                                                                        let first_2 = at_5 + 1f64;
                                                                                        {
                                                                                            let last_2 = self::find_java_script_pattern_default_expression_end(kinds, values, first_2, &(self::ml_string::Text::from_units(vec![125])));
                                                                                            if last_2 < 0f64 {
                                                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Return(vec![-1f64, first_2])
                                                                                            } else {
                                                                                                {
                                                                                                    let at_6 = last_2;
                                                                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Next(at_6, first_2, last_2)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Return(vec![-2f64, at_5])
                                                                                }
                                                                            } else {
                                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Next(at_5, first, last)
                                                                            };
                                                                            match source_ml_source2_s161 {
                                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Next(at_7, first_3, last_3) => {
                                                                                    {
                                                                                        let pairs_2 = {
                                                                                            let source_ml_source2_concat_value162 = pairs;
                                                                                            {
                                                                                                let source_ml_source2_concat_value163 = if defaults {
                                                                                                    vec![property, binding_3, first_3, last_3]
                                                                                                } else {
                                                                                                    vec![property, binding_3]
                                                                                                };
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value162), source_ml_source2_concat_value163)
                                                                                            }
                                                                                        };
                                                                                        if ((self::read_required_text_fact(kinds, at_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_7)) == (self::ml_string::Text::from_units(vec![44]))) {
                                                                                            {
                                                                                                let at_8 = at_7 + 1f64;
                                                                                                {
                                                                                                    (at, names, pairs) = (at_8, names_2, pairs_2);
                                                                                                    continue;
                                                                                                }
                                                                                            }
                                                                                        }
{
                                                                                            if (((self::read_required_text_fact(kinds, at_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_7)) == (self::ml_string::Text::from_units(vec![125])))) {
                                                                                                {
                                                                                                    (at, names, pairs) = (at_7, names_2, pairs_2);
                                                                                                    continue;
                                                                                                }
                                                                                            }
{
                                                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Return(vec![-1f64, at_7])
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Return(source_ml_source1_temporary_result33) => {
                                                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Return(source_ml_source1_temporary_result33)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Return(vec![-8f64, binding_3])
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin32Return(source_ml_source1_temporary_result33_2) => {
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Return(source_ml_source1_temporary_result33_2)
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
            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Done(at, names, pairs)
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
pub fn source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop29(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut ordinary: bool,
    mut defaults: bool,
    mut at: f64,
    mut slot: f64,
    mut names: Vec<self::ml_string::Text>,
    mut instructions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result {
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
                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Return(vec![-6f64, at])
            } else if ((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![91])))
            {
                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Return(vec![-3f64, at])
            } else {
                {
                    let source_ml_source2_s171 = if ((self::read_required_text_fact(kinds, at))
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
                                        let source_ml_source2_s172 = self::source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop31(kinds, values, reserved, ordinary, defaults, at_2, names, pairs);
                                        match source_ml_source2_s172 {
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Done(at_3, names_2, pairs_2) => {
                                                    if (pairs_2.len() as f64) == 0f64 {
                                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30Return(vec![-4f64, open])
                                                    } else if (at_3 >= (kinds.len() as f64)) || !(((self::read_required_text_fact(kinds, at_3)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_3)) == (self::ml_string::Text::from_units(vec![125])))) {
                                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30Return(vec![-1f64, at_3])
                                                        } else {
                                                            {
                                                                let instructions_2 = {
                                                                    let source_ml_source2_concat_value175 = {
                                                                        let source_ml_source2_concat_value173 = instructions;
                                                                        {
                                                                            let source_ml_source2_concat_value174 = vec![1f64, slot, open, at_3, ((pairs_2.len() as f64) / if defaults {
                                                                                4f64
                                                                            } else {
                                                                                2f64
                                                                            })];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value173), source_ml_source2_concat_value174)
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_concat_value176 = pairs_2;
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value175), source_ml_source2_concat_value176)
                                                                    }
                                                                };
                                                                {
                                                                    let at_4 = at_3 + 1f64;
                                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30Next(at_4, names_2, instructions_2)
                                                                }
                                                            }
                                                        }
                                                }
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop31Return(source_ml_source1_temporary_result31) => {
                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30Return(source_ml_source1_temporary_result31)
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
                                            let source_ml_source2_concat_value177 = instructions;
                                            {
                                                let source_ml_source2_concat_value178 =
                                                    vec![0f64, slot, at];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value177,
                                                    ),
                                                    source_ml_source2_concat_value178,
                                                )
                                            }
                                        };
                                        {
                                            let at_5 = at + 1f64;
                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30Next(at_5, names_3, instructions_3)
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30Return(vec![-8f64, at])
                            }
                        }
                    };
                    match source_ml_source2_s171 {
                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30Next(at_6, names_4, instructions_4) => {
                                if at_6 >= (kinds.len() as f64) {
                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Return(vec![-1f64, at_6])
                                } else if ((self::read_required_text_fact(kinds, at_6)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_6)) == (self::ml_string::Text::from_units(vec![61]))) {
                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Return(vec![-2f64, at_6])
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
                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Return(vec![-1f64, at_6])
                                                }
                                            }
                                        }
                                    }
                            }
                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin30Return(source_ml_source1_temporary_result30) => {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Return(source_ml_source1_temporary_result30)
                            }
                        }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop29Done(at, slot, instructions)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop35Result {
    MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop35Done(f64, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_record_formal_default_binding_positions_loop35(
    plan: &[f64],
    mut cursor: f64,
    mut names: Vec<f64>,
    mut count: f64,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop35Result {
    loop {
        if index < count {
            {
                let names_2 = {
                    let source_ml_source2_concat_value179 = names;
                    {
                        let source_ml_source2_concat_value180 =
                            vec![self::read_required_number_fact(plan, (cursor + 1f64))];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value179,
                            ),
                            source_ml_source2_concat_value180,
                        )
                    }
                };
                {
                    let cursor_2 = cursor + 4f64;
                    {
                        let index_2 = index + 1f64;
                        {
                            (cursor, names, index) = (cursor_2, names_2, index_2);
                            continue;
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop35Result::MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop35Done(cursor, names)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_record_formal_default_binding_positions_loop34(
    plan: &[f64],
    mut cursor: f64,
    mut names: Vec<f64>,
) -> Vec<f64> {
    loop {
        return if cursor < (plan.len() as f64) {
            {
                let form = self::read_required_number_fact(plan, cursor);
                {
                    let cursor_2 = cursor + 2f64;
                    if form == 0f64 {
                        {
                            let names_2 = {
                                let source_ml_source2_concat_value181 = names;
                                {
                                    let source_ml_source2_concat_value182 =
                                        vec![self::read_required_number_fact(plan, cursor_2)];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value181,
                                        ),
                                        source_ml_source2_concat_value182,
                                    )
                                }
                            };
                            {
                                let cursor_3 = cursor_2 + 1f64;
                                {
                                    (cursor, names) = (cursor_3, names_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        if form == 1f64 {
                            {
                                let count =
                                    self::read_required_number_fact(plan, (cursor_2 + 2f64));
                                {
                                    let cursor_4 = cursor_2 + 3f64;
                                    {
                                        let index = 0f64;
                                        {
                                            let source_ml_source2_s183 = self::source_ml_source1_temporary_read_java_script_record_formal_default_binding_positions_loop35(plan, cursor_4, names, count, index);
                                            match source_ml_source2_s183 {
                                                self::MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop35Result::MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop35Done(cursor_5, names_3) => {
                                                    {
                                                        (cursor, names) = (cursor_5, names_3);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("RangeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    97, 32, 114, 101, 99, 111, 114, 100, 32, 105, 110, 105, 116,
                                    105, 97, 108, 105, 122, 101, 114, 32, 99, 101, 110, 115, 117,
                                    115, 32, 110, 101, 101, 100, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 102, 111, 114, 109, 97, 108, 32, 105, 110, 115, 116,
                                    114, 117, 99, 116, 105, 111, 110, 115,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                }
            }
        } else {
            names
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result {
    MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Done,
    MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_record_parameter_strict_directive_loop36(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
    line_breaks: &[bool],
    mut at: f64,
) -> self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result {
    loop {
        return if at < (kinds.len() as f64) {
            {
                let current = self::read_required_text_fact(kinds, at);
                if current == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(-2f64)
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
                                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(-2f64)
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
                                                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(at)
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
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(if strict {
                                                            at
                                                        } else {
                                                            -1f64
                                                        })
                                            } else if kind
                                                == (self::ml_string::Text::from_units(vec![
                                                    101, 111, 102,
                                                ]))
                                            {
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(-2f64)
                                            } else if (self::read_required_boolean_fact(
                                                line_breaks,
                                                next,
                                            )) {
                                                if self::continues_java_script_directive_expression(&(kind), &(value)) {
                                                                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(-1f64)
                                                                } else if ((kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || (kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) || ((kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (((value == (self::ml_string::Text::from_units(vec![123]))) || (value == (self::ml_string::Text::from_units(vec![43, 43])))) || (value == (self::ml_string::Text::from_units(vec![45, 45]))))) {
                                                                        if strict {
                                                                            self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(at)
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
                                                                        self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(-2f64)
                                                                    }
                                            } else {
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(-1f64)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Return(-1f64)
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Result::MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop36Done
        };
    }
}
