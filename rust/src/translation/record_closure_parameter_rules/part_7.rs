// Generated from complete js/src/translation/record-closure-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result;
use super::accept_java_script_record_parameter_bound_name;
use super::find_java_script_pattern_default_expression_end;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_object_pattern_bindings;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_record_parameter_pattern_loop26(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut depth: f64,
    mut binding: bool,
    mut at: f64,
) -> self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result {
    loop {
        return if at < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, at);
                if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Return(-1f64)
                } else {
                    if kind != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) {
                        {
                            let binding_3 = if depth == 0f64 { { false } } else { binding };
                            {
                                let at_2 = at + 1f64;
                                {
                                    (binding, at) = (binding_3, at_2);
                                    continue;
                                }
                            }
                        }
                    }
                    {
                        {
                            let value = self::read_required_text_fact(values, at);
                            if (depth == 0f64)
                                && (value == (self::ml_string::Text::from_units(vec![41])))
                            {
                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Return(-1f64)
                            } else if ((depth == 0f64) && binding)
                                && (value == (self::ml_string::Text::from_units(vec![123])))
                            {
                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Return(at)
                            } else {
                                if ((value == (self::ml_string::Text::from_units(vec![40])))
                                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                                    || (value == (self::ml_string::Text::from_units(vec![123])))
                                {
                                    {
                                        let depth_2 = depth + 1f64;
                                        {
                                            let binding_4 = false;
                                            {
                                                let at_3 = at + 1f64;
                                                {
                                                    (depth, binding, at) =
                                                        (depth_2, binding_4, at_3);
                                                    continue;
                                                }
                                            }
                                        }
                                    }
                                }
                                {
                                    if ((value == (self::ml_string::Text::from_units(vec![41])))
                                        || (value == (self::ml_string::Text::from_units(vec![93]))))
                                        || (value == (self::ml_string::Text::from_units(vec![125])))
                                    {
                                        {
                                            let depth_3 = depth - 1f64;
                                            if depth_3 < 0f64 {
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Return(-1f64)
                                            } else {
                                                {
                                                    let at_4 = at + 1f64;
                                                    {
                                                        (depth, at) = (depth_3, at_4);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (depth == 0f64)
                                            && (value
                                                == (self::ml_string::Text::from_units(vec![44])))
                                        {
                                            {
                                                let binding_5 = true;
                                                {
                                                    let at_5 = at + 1f64;
                                                    {
                                                        (binding, at) = (binding_5, at_5);
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        {
                                            if (depth == 0f64)
                                                && (value
                                                    != (self::ml_string::Text::from_units(vec![
                                                        44,
                                                    ])))
                                            {
                                                {
                                                    let binding_6 = false;
                                                    {
                                                        let at_6 = at + 1f64;
                                                        {
                                                            (binding, at) = (binding_6, at_6);
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                            {
                                                {
                                                    let at_7 = at + 1f64;
                                                    {
                                                        at = at_7;
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
            self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop26Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_record_parameter_binding_loop27(
    name: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    mut duplicate: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (names.len() as f64) {
            if (self::read_required_text_fact(names, index)) == (*name) {
                {
                    let duplicate_2 = true;
                    {
                        let index_2 = index + 1f64;
                        {
                            (duplicate, index) = (duplicate_2, index_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        }
        return { duplicate };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_record_parameter_bound_name_loop28(
    name: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    mut duplicate: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (names.len() as f64) {
            if (self::read_required_text_fact(names, index)) == (*name) {
                {
                    let duplicate_2 = true;
                    {
                        let index_2 = index + 1f64;
                        {
                            (duplicate, index) = (duplicate_2, index_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let index_3 = index + 1f64;
                    {
                        index = index_3;
                        continue;
                    }
                }
            }
        }
        return { duplicate };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result {
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Done(f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31 {
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31Next(f64, f64, f64),
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30 {
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30Next(f64, f64),
    MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_object_pattern_bindings_loop29(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut profile: f64,
    mut at: f64,
    mut names: Vec<self::ml_string::Text>,
    mut pairs: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result {
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
                {
                    let ml_library_inequality_2 = profile != 3f64;
                    if ml_library_inequality_2 {
                        self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(vec![-6f64, at])
                    } else {
                        {
                            let spread = at;
                            {
                                let binding = at + 1f64;
                                if binding >= (kinds.len() as f64) {
                                    self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(vec![-1f64, binding])
                                } else {
                                    {
                                        let name = self::read_required_text_fact(values, binding);
                                        if (self::accept_java_script_record_parameter_bound_name(
                                            &(self::read_required_text_fact(kinds, binding)),
                                            &(name),
                                            &(names),
                                            reserved,
                                            true,
                                        )) {
                                            {
                                                let at_2 = binding + 1f64;
                                                if ((at_2 >= (kinds.len() as f64))
                                                    || ((self::read_required_text_fact(
                                                        kinds, at_2,
                                                    )) != (self::ml_string::Text::from_units(
                                                        vec![112, 117, 110, 99, 116],
                                                    ))))
                                                    || ((self::read_required_text_fact(
                                                        values, at_2,
                                                    )) != (self::ml_string::Text::from_units(
                                                        vec![125],
                                                    )))
                                                {
                                                    self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(vec![-6f64, at_2])
                                                } else {
                                                    {
                                                        let pairs_2 = {
                                                            let source_ml_source2_concat_value163 =
                                                                pairs;
                                                            {
                                                                let source_ml_source2_concat_value164 = vec![
                                                                    spread, binding, -2f64, -2f64,
                                                                ];
                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value163), source_ml_source2_concat_value164)
                                                            }
                                                        };
                                                        self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Done(at_2, pairs_2)
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(vec![-8f64, binding])
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                let ml_library_inequality_1 = (self::read_required_text_fact(kinds, at))
                    != (self::ml_string::Text::from_units(vec![
                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                    ]));
                if ml_library_inequality_1 {
                    self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(vec![-7f64, at])
                } else {
                    {
                        let property = at;
                        {
                            let at_3 = at + 1f64;
                            if at_3 >= (kinds.len() as f64) {
                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(vec![-1f64, at_3])
                            } else {
                                {
                                    let binding_2 = property;
                                    {
                                        let source_ml_source2_s165 =
                                            if ((self::read_required_text_fact(kinds, at_3))
                                                == (self::ml_string::Text::from_units(vec![
                                                    112, 117, 110, 99, 116,
                                                ])))
                                                && ((self::read_required_text_fact(values, at_3))
                                                    == (self::ml_string::Text::from_units(vec![
                                                        58,
                                                    ])))
                                            {
                                                {
                                                    let at_4 = at_3 + 1f64;
                                                    if at_4 >= (kinds.len() as f64) {
                                                    self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30Return(vec![-1f64, at_4])
                                                } else if ((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![123]))) || ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![91])))) {
                                                        self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30Return(vec![-5f64, at_4])
                                                    } else {
                                                        {
                                                            let binding_3 = at_4;
                                                            {
                                                                let at_5 = at_4 + 1f64;
                                                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30Next(at_5, binding_3)
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30Next(at_3, binding_2)
                                            };
                                        match source_ml_source2_s165 {
                                            self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30Next(at_6, binding_4) => {
                                                {
                                                    let name_2 = self::read_required_text_fact(values, binding_4);
                                                    if (self::accept_java_script_record_parameter_bound_name(&(self::read_required_text_fact(kinds, binding_4)), &(name_2), &(names), reserved, (profile > 0f64))) {
                                                        {
                                                            let names_2 = {
                                                                let source_ml_source2_concat_value166 = names;
                                                                {
                                                                    let source_ml_source2_concat_value167 = vec![name_2.clone()];
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::ml_string::Text>::new(), source_ml_source2_concat_value166), source_ml_source2_concat_value167)
                                                                }
                                                            };
                                                            if at_6 >= (kinds.len() as f64) {
                                                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(vec![-1f64, at_6])
                                                            } else {
                                                                {
                                                                    let first = -1f64;
                                                                    {
                                                                        let last = -1f64;
                                                                        {
                                                                            let source_ml_source2_s168 = if ((self::read_required_text_fact(kinds, at_6)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_6)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                                if profile < 2f64 {
                                                                                    self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31Return(vec![-2f64, at_6])
                                                                                } else {
                                                                                    {
                                                                                        let first_2 = at_6 + 1f64;
                                                                                        {
                                                                                            let last_2 = self::find_java_script_pattern_default_expression_end(kinds, values, first_2, &(self::ml_string::Text::from_units(vec![125])));
                                                                                            if last_2 < 0f64 {
                                                                                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31Return(vec![-1f64, first_2])
                                                                                            } else {
                                                                                                {
                                                                                                    let at_7 = last_2;
                                                                                                    self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31Next(at_7, first_2, last_2)
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31Next(at_6, first, last)
                                                                            };
                                                                            match source_ml_source2_s168 {
                                                                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31Next(at_8, first_3, last_3) => {
                                                                                    {
                                                                                        let pairs_3 = {
                                                                                            let source_ml_source2_concat_value169 = pairs;
                                                                                            {
                                                                                                let source_ml_source2_concat_value170 = if profile >= 2f64 {
                                                                                                    vec![property, binding_4, first_3, last_3]
                                                                                                } else {
                                                                                                    vec![property, binding_4]
                                                                                                };
                                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value169), source_ml_source2_concat_value170)
                                                                                            }
                                                                                        };
                                                                                        if ((self::read_required_text_fact(kinds, at_8)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_8)) == (self::ml_string::Text::from_units(vec![44]))) {
                                                                                            {
                                                                                                let at_9 = at_8 + 1f64;
                                                                                                {
                                                                                                    (at, names, pairs) = (at_9, names_2, pairs_3);
                                                                                                    continue;
                                                                                                }
                                                                                            }
                                                                                        }
{
                                                                                            if (((self::read_required_text_fact(kinds, at_8)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_8)) == (self::ml_string::Text::from_units(vec![125])))) {
                                                                                                {
                                                                                                    (at, names, pairs) = (at_8, names_2, pairs_3);
                                                                                                    continue;
                                                                                                }
                                                                                            }
{
                                                                                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(vec![-1f64, at_8])
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin31Return(source_ml_source1_temporary_result31) => {
                                                                                    self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(source_ml_source1_temporary_result31)
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(vec![-8f64, binding_4])
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsJoin30Return(source_ml_source1_temporary_result31_2) => {
                                                self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Return(source_ml_source1_temporary_result31_2)
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
            self::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Result::MlSource1TemporaryPlanJavaScriptObjectPatternBindingsLoop29Done(at, pairs)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result {
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Done(f64, f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33 {
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Next(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop34(
    values: &[self::ml_string::Text],
    mut names: Vec<self::ml_string::Text>,
    pattern: &[f64],
    mut count: f64,
    mut width: f64,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < count {
            {
                let names_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::read_required_text_fact(
                            values,
                            self::read_required_number_fact(
                                pattern,
                                ((2f64 + (index * width)) + 1f64),
                            ),
                        )],
                        names,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (names, index) = (names_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { names };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop32(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut ordinary: bool,
    mut defaults: bool,
    mut rest: bool,
    mut at: f64,
    mut slot: f64,
    mut names: Vec<self::ml_string::Text>,
    mut instructions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result {
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
                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Return(vec![-6f64, at])
            } else if ((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![91])))
            {
                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Return(vec![-3f64, at])
            } else {
                {
                    let source_ml_source2_s176 = if ((self::read_required_text_fact(kinds, at))
                        == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && ((self::read_required_text_fact(values, at))
                            == (self::ml_string::Text::from_units(vec![123])))
                    {
                        {
                            let open = at;
                            {
                                let profile = if rest {
                                    3f64
                                } else if defaults {
                                    2f64
                                } else if ordinary {
                                    1f64
                                } else {
                                    0f64
                                };
                                {
                                    let pattern = self::plan_java_script_object_pattern_bindings(
                                        kinds,
                                        values,
                                        at,
                                        reserved,
                                        &(names),
                                        profile,
                                    );
                                    if (self::read_required_number_fact(&(pattern), 0f64)) < 0f64 {
                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Return(pattern)
                                    } else {
                                        {
                                            let at_2 =
                                                self::read_required_number_fact(&(pattern), 0f64);
                                            {
                                                let count = self::read_required_number_fact(
                                                    &(pattern),
                                                    1f64,
                                                );
                                                {
                                                    let width = if defaults { 4f64 } else { 2f64 };
                                                    {
                                                        let instructions_2 = {
                                                            let source_ml_source2_concat_value177 =
                                                                instructions;
                                                            {
                                                                let source_ml_source2_concat_value178 = vec![
                                                                    1f64,
                                                                    slot,
                                                                    open,
                                                                    (at_2 - 1f64),
                                                                    count,
                                                                ];
                                                                {
                                                                    let source_ml_source2_concat_value179 =
                                                                        self::ml_array::slice(
                                                                            &pattern,
                                                                            2f64,
                                                                            f64::INFINITY,
                                                                        );
                                                                    self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value177), source_ml_source2_concat_value178), source_ml_source2_concat_value179)
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let index = 0f64;
                                                            {
                                                                let names_2 = self::source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop34(values, names, &(pattern), count, width, index);
                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Next(at_2, names_2, instructions_2)
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
                                            let source_ml_source2_concat_value180 = instructions;
                                            {
                                                let source_ml_source2_concat_value181 =
                                                    vec![0f64, slot, at];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value180,
                                                    ),
                                                    source_ml_source2_concat_value181,
                                                )
                                            }
                                        };
                                        {
                                            let at_3 = at + 1f64;
                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Next(at_3, names_3, instructions_3)
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Return(vec![-8f64, at])
                            }
                        }
                    };
                    match source_ml_source2_s176 {
                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Next(at_4, names_4, instructions_4) => {
                                if at_4 >= (kinds.len() as f64) {
                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Return(vec![-1f64, at_4])
                                } else if ((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![61]))) {
                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Return(vec![-2f64, at_4])
                                    } else {
                                        {
                                            let slot_2 = slot + 1f64;
                                            if ((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![44]))) {
                                                {
                                                    let at_5 = at_4 + 1f64;
                                                    {
                                                        (at, slot, names, instructions) = (at_5, slot_2, names_4, instructions_4);
                                                        continue;
                                                    }
                                                }
                                            }
{
                                                if (((self::read_required_text_fact(kinds, at_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_4)) == (self::ml_string::Text::from_units(vec![41])))) {
                                                    {
                                                        (at, slot, names, instructions) = (at_4, slot_2, names_4, instructions_4);
                                                        continue;
                                                    }
                                                }
{
                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Return(vec![-1f64, at_4])
                                                }
                                            }
                                        }
                                    }
                            }
                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin33Return(source_ml_source1_temporary_result33) => {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Return(source_ml_source1_temporary_result33)
                            }
                        }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop32Done(at, slot, instructions)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_pattern_binding_positions_loop35(
    plan: &[f64],
    mut count: f64,
    mut positions: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let positions_2 = {
                    let source_ml_source2_concat_value182 = positions;
                    {
                        let source_ml_source2_concat_value183 =
                            vec![self::read_required_number_fact(
                                plan,
                                (3f64 + (index * 4f64)),
                            )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value182,
                            ),
                            source_ml_source2_concat_value183,
                        )
                    }
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (positions, index) = (positions_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { positions };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop37Result {
    MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop37Done(f64, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_record_formal_default_binding_positions_loop37(
    plan: &[f64],
    mut cursor: f64,
    mut names: Vec<f64>,
    mut count: f64,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop37Result {
    loop {
        if index < count {
            {
                let names_2 = {
                    let source_ml_source2_concat_value184 = names;
                    {
                        let source_ml_source2_concat_value185 =
                            vec![self::read_required_number_fact(plan, (cursor + 1f64))];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source2_concat_value184,
                            ),
                            source_ml_source2_concat_value185,
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
            self::MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop37Result::MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop37Done(cursor, names)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_record_formal_default_binding_positions_loop36(
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
                                let source_ml_source2_concat_value186 = names;
                                {
                                    let source_ml_source2_concat_value187 =
                                        vec![self::read_required_number_fact(plan, cursor_2)];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source2_concat_value186,
                                        ),
                                        source_ml_source2_concat_value187,
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
                                            let source_ml_source2_s188 = self::source_ml_source1_temporary_read_java_script_record_formal_default_binding_positions_loop37(plan, cursor_4, names, count, index);
                                            match source_ml_source2_s188 {
                                                self::MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop37Result::MlSource1TemporaryReadJavaScriptRecordFormalDefaultBindingPositionsLoop37Done(cursor_5, names_3) => {
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
