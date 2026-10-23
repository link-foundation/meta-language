// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result;
use super::MlPlanJavaScriptRecordParametersLoop11Result;
use super::accept_java_script_array_binding_name;
use super::ml_accept_java_script_record_parameter_binding_loop10;
use super::ml_array;
use super::ml_find_java_script_record_parameter_pattern_loop9;
use super::ml_find_java_script_record_parameter_strict_directive_loop15;
use super::ml_math;
use super::ml_plan_java_script_record_parameters_loop11;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_binding_fact_field(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    field: &self::ml_string::Text,
) -> f64 {
    if ((node < 0f64) || (node >= (starts.len() as f64))) || (node >= (counts.len() as f64)) {
        -1f64
    } else {
        {
            let at = self::read_required_number_fact(starts, node);
            {
                let end = at + self::read_required_number_fact(counts, node);
                {
                    let ml_s22 = self::ml_read_array_binding_fact_field_loop2(
                        labels, targets, field, at, end,
                    );
                    match ml_s22 {
                        self::MlReadArrayBindingFactFieldLoop2Result::MlReadArrayBindingFactFieldLoop2Done => {
                            -1f64
                        }
                        self::MlReadArrayBindingFactFieldLoop2Result::MlReadArrayBindingFactFieldLoop2Return(ml_result2) => {
                            ml_result2
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_array_binding_initializer_end(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
) -> f64 {
    {
        let index = start;
        {
            let closes = Vec::<self::ml_string::Text>::new();
            {
                let ml_s33 = self::ml_read_array_binding_initializer_end_loop3(
                    kinds, values, start, index, closes,
                );
                match ml_s33 {
                    self::MlReadArrayBindingInitializerEndLoop3Result::MlReadArrayBindingInitializerEndLoop3Done => {
                        -1f64
                    }
                    self::MlReadArrayBindingInitializerEndLoop3Result::MlReadArrayBindingInitializerEndLoop3Return(ml_result3) => {
                        ml_result3
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_array_binding_names(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
) -> Vec<f64> {
    if ((((start < 0f64) || ((start % 1f64) != 0f64)) || (start >= (kinds.len() as f64)))
        || (start >= (values.len() as f64)))
        || (((self::read_required_text_fact(kinds, start))
            != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
            || ((self::read_required_text_fact(values, start))
                != (self::ml_string::Text::from_units(vec![91]))))
    {
        vec![-1f64]
    } else {
        {
            let index = start + 1f64;
            {
                let positions = Vec::<f64>::new();
                {
                    let ml_s44 = self::ml_plan_java_script_array_binding_names_loop5(
                        kinds, values, index, positions,
                    );
                    match ml_s44 {
                            self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Done => {
                                vec![-1f64]
                            }
                            self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(ml_result5) => {
                                ml_result5
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_record_parameter_pattern(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
) -> f64 {
    if ((((kinds.len() as f64) != (values.len() as f64)) || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64))
    {
        -1f64
    } else {
        {
            let depth = 0f64;
            {
                let binding = true;
                {
                    let at = start;
                    {
                        let ml_s55 = self::ml_find_java_script_record_parameter_pattern_loop9(
                            kinds, values, depth, binding, at,
                        );
                        match ml_s55 {
                            self::MlFindJavaScriptRecordParameterPatternLoop9Result::MlFindJavaScriptRecordParameterPatternLoop9Done => {
                                -1f64
                            }
                            self::MlFindJavaScriptRecordParameterPatternLoop9Result::MlFindJavaScriptRecordParameterPatternLoop9Return(ml_result9) => {
                                ml_result9
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
pub fn accept_java_script_record_parameter_binding(
    kind: &self::ml_string::Text,
    name: &self::ml_string::Text,
    names: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
) -> bool {
    {
        let duplicate = false;
        {
            let index = 0f64;
            {
                let duplicate_2 = self::ml_accept_java_script_record_parameter_binding_loop10(
                    name, names, duplicate, index,
                );
                self::accept_java_script_array_binding_name(kind, name, duplicate_2, reserved)
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_record_parameters(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    start: f64,
    reserved: &[self::ml_string::Text],
) -> Vec<f64> {
    if ((((kinds.len() as f64) != (values.len() as f64)) || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64))
    {
        vec![-1f64, start]
    } else {
        {
            let at = start;
            {
                let slot = 0f64;
                {
                    let names = Vec::<self::ml_string::Text>::new();
                    {
                        let instructions = Vec::<f64>::new();
                        {
                            let ml_s66 = self::ml_plan_java_script_record_parameters_loop11(
                                kinds,
                                values,
                                reserved,
                                at,
                                slot,
                                names,
                                instructions,
                            );
                            match ml_s66 {
                                self::MlPlanJavaScriptRecordParametersLoop11Result::MlPlanJavaScriptRecordParametersLoop11Done(at_2, slot_2, instructions_2) => {
                                    if (at_2 >= (kinds.len() as f64)) || !(((self::read_required_text_fact(kinds, at_2)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_2)) == (self::ml_string::Text::from_units(vec![41])))) {
                                        vec![-1f64, at_2]
                                    } else {
                                        self::ml_array::append(vec![(at_2 + 1f64), slot_2], instructions_2)
                                    }
                                }
                                self::MlPlanJavaScriptRecordParametersLoop11Result::MlPlanJavaScriptRecordParametersLoop11Return(ml_result11) => {
                                    ml_result11
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
pub fn accept_java_script_record_parameter_profile(
    context: &self::ml_string::Text,
    asynchronous: bool,
    documentation_parameters: f64,
    bound_writes: bool,
    variable_redeclarations: bool,
) -> bool {
    (((((*context)
        == (self::ml_string::Text::from_units(vec![
            111, 114, 100, 105, 110, 97, 114, 121, 32, 110, 97, 109, 101, 100, 32, 102, 117, 110,
            99, 116, 105, 111, 110, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110,
        ])))
        && !asynchronous)
        && (documentation_parameters == 0f64))
        && !bound_writes)
        && !variable_redeclarations
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_standard_builtin_owner(
    name: &self::ml_string::Text,
    shadowed: bool,
) -> bool {
    (((((((((((((((((*name)
        == (self::ml_string::Text::from_units(vec![77, 97, 112])))
        || ((*name) == (self::ml_string::Text::from_units(vec![83, 101, 116]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 110, 115, 111, 108, 101,
            ]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![77, 97, 116, 104]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![78, 117, 109, 98, 101, 114]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
            ]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                84, 121, 112, 101, 69, 114, 114, 111, 114,
            ]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![78, 97, 78]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![73, 110, 102, 105, 110, 105, 116, 121]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![
                117, 110, 100, 101, 102, 105, 110, 101, 100,
            ]))))
        || ((*name)
            == (self::ml_string::Text::from_units(vec![105, 115, 70, 105, 110, 105, 116, 101]))))
        || ((*name) == (self::ml_string::Text::from_units(vec![105, 115, 78, 97, 78]))))
        && !shadowed
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_record_parameter_strict_directive(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
    line_breaks: &[bool],
    start: f64,
) -> f64 {
    if ((((((kinds.len() as f64) != (values.len() as f64))
        || ((kinds.len() as f64) != (raws.len() as f64)))
        || ((kinds.len() as f64) != (line_breaks.len() as f64)))
        || !(self::ml_math::is_integer(start)))
        || (start < 0f64))
        || (start >= (kinds.len() as f64))
    {
        -2f64
    } else {
        {
            let at = start;
            {
                let ml_s77 = self::ml_find_java_script_record_parameter_strict_directive_loop15(
                    kinds,
                    values,
                    raws,
                    line_breaks,
                    at,
                );
                match ml_s77 {
                    self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Done => {
                        -2f64
                    }
                    self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(ml_result15) => {
                        ml_result15
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continues_java_script_directive_expression(
    kind: &self::ml_string::Text,
    value: &self::ml_string::Text,
) -> bool {
    if ((*kind) == (self::ml_string::Text::from_units(vec![116, 101, 109, 112, 108, 97, 116, 101])))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![
                105, 110, 116, 101, 114, 112, 111, 108, 97, 116, 105, 111, 110,
            ])))
    {
        true
    } else if (*kind)
        == (self::ml_string::Text::from_units(vec![
            105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
        ]))
    {
        ((*value) == (self::ml_string::Text::from_units(vec![105, 110])))
            || ((*value)
                == (self::ml_string::Text::from_units(vec![
                    105, 110, 115, 116, 97, 110, 99, 101, 111, 102,
                ])))
    } else {
        let ml_library_inequality_1 =
            (*kind) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]));
        if ml_library_inequality_1 {
            false
        } else if (((((((((((*value) == (self::ml_string::Text::from_units(vec![40])))
            || ((*value) == (self::ml_string::Text::from_units(vec![91]))))
            || ((*value) == (self::ml_string::Text::from_units(vec![46]))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![63, 46])))
                || ((*value) == (self::ml_string::Text::from_units(vec![43]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![45])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![42])))
                || ((*value) == (self::ml_string::Text::from_units(vec![47]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![37])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![42, 42])))
                || ((*value) == (self::ml_string::Text::from_units(vec![60, 60]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![62, 62])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![62, 62, 62])))
                || ((*value) == (self::ml_string::Text::from_units(vec![60]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![62])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![60, 61])))
                || ((*value) == (self::ml_string::Text::from_units(vec![62, 61]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![61, 61])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![33, 61])))
                || ((*value) == (self::ml_string::Text::from_units(vec![61, 61, 61]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![33, 61, 61])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![38])))
                || ((*value) == (self::ml_string::Text::from_units(vec![94]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![124])))))
            || ((((*value) == (self::ml_string::Text::from_units(vec![38, 38])))
                || ((*value) == (self::ml_string::Text::from_units(vec![124, 124]))))
                || ((*value) == (self::ml_string::Text::from_units(vec![63, 63]))))
        {
            true
        } else {
            ((*value) == (self::ml_string::Text::from_units(vec![63])))
                || ((*value) == (self::ml_string::Text::from_units(vec![44])))
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlAcceptJavaScriptArrayBindingNameLoop1Result {
    MlAcceptJavaScriptArrayBindingNameLoop1Done,
    MlAcceptJavaScriptArrayBindingNameLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_java_script_array_binding_name_loop1(
    name: &self::ml_string::Text,
    reserved: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlAcceptJavaScriptArrayBindingNameLoop1Result {
    loop {
        return if index < (reserved.len() as f64) {
            if (self::read_required_text_fact(reserved, index)) == (*name) {
                self::MlAcceptJavaScriptArrayBindingNameLoop1Result::MlAcceptJavaScriptArrayBindingNameLoop1Return(false)
            } else {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlAcceptJavaScriptArrayBindingNameLoop1Result::MlAcceptJavaScriptArrayBindingNameLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadArrayBindingFactFieldLoop2Result {
    MlReadArrayBindingFactFieldLoop2Done,
    MlReadArrayBindingFactFieldLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_array_binding_fact_field_loop2(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    field: &self::ml_string::Text,
    mut at: f64,
    mut end: f64,
) -> self::MlReadArrayBindingFactFieldLoop2Result {
    loop {
        return if at < end {
            if (self::read_required_text_fact(labels, at)) == (*field) {
                self::MlReadArrayBindingFactFieldLoop2Result::MlReadArrayBindingFactFieldLoop2Return(
                    self::read_required_number_fact(targets, at),
                )
            } else {
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlReadArrayBindingFactFieldLoop2Result::MlReadArrayBindingFactFieldLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadArrayBindingInitializerEndLoop3Result {
    MlReadArrayBindingInitializerEndLoop3Done,
    MlReadArrayBindingInitializerEndLoop3Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlReadArrayBindingInitializerEndJoin4 {
    MlReadArrayBindingInitializerEndJoin4Next(Vec<self::ml_string::Text>),
    MlReadArrayBindingInitializerEndJoin4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_read_array_binding_initializer_end_loop3(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut start: f64,
    mut index: f64,
    mut closes: Vec<self::ml_string::Text>,
) -> self::MlReadArrayBindingInitializerEndLoop3Result {
    loop {
        return if (index < (values.len() as f64)) && (index < (kinds.len() as f64)) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                        self::MlReadArrayBindingInitializerEndLoop3Result::MlReadArrayBindingInitializerEndLoop3Return(-1f64)
                    } else {
                        {
                            let ml_s88 = if kind
                                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))
                            {
                                if ((closes.len() as f64) == 0f64)
                                    && ((value == (self::ml_string::Text::from_units(vec![44])))
                                        || (value == (self::ml_string::Text::from_units(vec![93]))))
                                {
                                    self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Return(if index > start {
                                        index
                                    } else {
                                        -1f64
                                    })
                                } else if ((value == (self::ml_string::Text::from_units(vec![40])))
                                    || (value == (self::ml_string::Text::from_units(vec![91]))))
                                    || (value == (self::ml_string::Text::from_units(vec![123])))
                                {
                                    {
                                        let closes_2 = {
                                            let (ml_append_right, ml_append_left) = (
                                                vec![if value
                                                    == (self::ml_string::Text::from_units(vec![40]))
                                                {
                                                    self::ml_string::Text::from_units(vec![41])
                                                } else if value
                                                    == (self::ml_string::Text::from_units(vec![91]))
                                                {
                                                    self::ml_string::Text::from_units(vec![93])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![125])
                                                }],
                                                closes,
                                            );
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Next(closes_2)
                                    }
                                } else if ((value == (self::ml_string::Text::from_units(vec![41])))
                                    || (value == (self::ml_string::Text::from_units(vec![93]))))
                                    || (value == (self::ml_string::Text::from_units(vec![125])))
                                {
                                    if ((closes.len() as f64) == 0f64)
                                        || ((self::read_required_text_fact(
                                            &(closes),
                                            ((closes.len() as f64) - 1f64),
                                        )) != value)
                                    {
                                        self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Return(-1f64)
                                    } else {
                                        {
                                            let closes_3 = self::ml_array::slice(
                                                &closes,
                                                0f64,
                                                ((closes.len() as f64) - 1f64),
                                            );
                                            self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Next(closes_3)
                                        }
                                    }
                                } else {
                                    self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Next(closes)
                                }
                            } else {
                                self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Next(closes)
                            };
                            match ml_s88 {
                                self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Next(closes_4) => {
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (index, closes) = (index_2, closes_4);
                                            continue;
                                        }
                                    }
                                }
                                self::MlReadArrayBindingInitializerEndJoin4::MlReadArrayBindingInitializerEndJoin4Return(ml_result4) => {
                                    self::MlReadArrayBindingInitializerEndLoop3Result::MlReadArrayBindingInitializerEndLoop3Return(ml_result4)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlReadArrayBindingInitializerEndLoop3Result::MlReadArrayBindingInitializerEndLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanJavaScriptArrayBindingNamesLoop5Result {
    MlPlanJavaScriptArrayBindingNamesLoop5Done,
    MlPlanJavaScriptArrayBindingNamesLoop5Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanJavaScriptArrayBindingNamesJoin8 {
    MlPlanJavaScriptArrayBindingNamesJoin8Next(f64),
    MlPlanJavaScriptArrayBindingNamesJoin8Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanJavaScriptArrayBindingNamesJoin7 {
    MlPlanJavaScriptArrayBindingNamesJoin7Next(f64, Vec<f64>),
    MlPlanJavaScriptArrayBindingNamesJoin7Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanJavaScriptArrayBindingNamesJoin6 {
    MlPlanJavaScriptArrayBindingNamesJoin6Next(f64, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_java_script_array_binding_names_loop5(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut index: f64,
    mut positions: Vec<f64>,
) -> self::MlPlanJavaScriptArrayBindingNamesLoop5Result {
    loop {
        return if (index < (values.len() as f64)) && (index < (kinds.len() as f64)) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if (kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && (value == (self::ml_string::Text::from_units(vec![93])))
                    {
                        self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(self::ml_array::append(vec![index], positions))
                    } else {
                        if (kind
                            == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                            && (value == (self::ml_string::Text::from_units(vec![44])))
                        {
                            {
                                let index_2 = index + 1f64;
                                {
                                    index = index_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let rest = false;
                                {
                                    let ml_s99 = if (kind
                                        == (self::ml_string::Text::from_units(vec![
                                            112, 117, 110, 99, 116,
                                        ])))
                                        && (value
                                            == (self::ml_string::Text::from_units(vec![
                                                46, 46, 46,
                                            ]))) {
                                        {
                                            let rest_2 = true;
                                            {
                                                let index_3 = index + 1f64;
                                                self::MlPlanJavaScriptArrayBindingNamesJoin6::MlPlanJavaScriptArrayBindingNamesJoin6Next(index_3, rest_2)
                                            }
                                        }
                                    } else {
                                        self::MlPlanJavaScriptArrayBindingNamesJoin6::MlPlanJavaScriptArrayBindingNamesJoin6Next(index, rest)
                                    };
                                    match ml_s99 {
                                        self::MlPlanJavaScriptArrayBindingNamesJoin6::MlPlanJavaScriptArrayBindingNamesJoin6Next(index_4, rest_3) => {
                                            if (index_4 >= (values.len() as f64)) || (index_4 >= (kinds.len() as f64)) {
                                                self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                            } else {
                                                {
                                                    let ml_s1100 = if ((self::read_required_text_fact(kinds, index_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_4)) == (self::ml_string::Text::from_units(vec![91]))) {
                                                        if rest_3 {
                                                            self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let nested = self::plan_java_script_array_binding_names(kinds, values, index_4);
                                                                if (self::read_required_number_fact(&(nested), 0f64)) < 0f64 {
                                                                    self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                                } else {
                                                                    {
                                                                        let positions_2 = { let (ml_append_right, ml_append_left) = (self::ml_array::slice(&nested, 1f64, f64::INFINITY), positions); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                        {
                                                                            let index_5 = self::read_required_number_fact(&(nested), 0f64) + 1f64;
                                                                            self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Next(index_5, positions_2)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
let ml_library_inequality_3 = (self::read_required_text_fact(kinds, index_4)) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])) ;
if ml_library_inequality_3 {
                                                            self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let positions_3 = { let (ml_append_right, ml_append_left) = (vec![index_4], positions); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                {
                                                                    let index_6 = index_4 + 1f64;
                                                                    self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Next(index_6, positions_3)
                                                                }
                                                            }
                                                        }
};
                                                    match ml_s1100 {
                                                        self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Next(index_7, positions_4) => {
                                                            if (index_7 >= (values.len() as f64)) || (index_7 >= (kinds.len() as f64)) {
                                                                self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                                            } else if rest_3 {
                                                                    self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![93]))) {
                                                                        self::ml_array::append(vec![index_7], positions_4)
                                                                    } else {
                                                                        vec![-1f64]
                                                                    })
                                                                } else {
                                                                    {
                                                                        let ml_s1111 = if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                            {
                                                                                let index_8 = self::read_array_binding_initializer_end(kinds, values, (index_7 + 1f64));
                                                                                if index_8 < 0f64 {
                                                                                    self::MlPlanJavaScriptArrayBindingNamesJoin8::MlPlanJavaScriptArrayBindingNamesJoin8Return(vec![-1f64])
                                                                                } else {
                                                                                    self::MlPlanJavaScriptArrayBindingNamesJoin8::MlPlanJavaScriptArrayBindingNamesJoin8Next(index_8)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlPlanJavaScriptArrayBindingNamesJoin8::MlPlanJavaScriptArrayBindingNamesJoin8Next(index_7)
                                                                        };
                                                                        match ml_s1111 {
                                                                            self::MlPlanJavaScriptArrayBindingNamesJoin8::MlPlanJavaScriptArrayBindingNamesJoin8Next(index_9) => {
                                                                                {
let ml_library_inequality_2 = (self::read_required_text_fact(kinds, index_9)) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) ;
if ml_library_inequality_2 {
                                                                                    self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                                                                } else {
                                                                                    {
                                                                                        let next = self::read_required_text_fact(values, index_9);
                                                                                        if next == (self::ml_string::Text::from_units(vec![93])) {
                                                                                            self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(self::ml_array::append(vec![index_9], positions_4))
                                                                                        } else {
let ml_library_inequality_1 = next != (self::ml_string::Text::from_units(vec![44])) ;
if ml_library_inequality_1 {
                                                                                                self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                                                                            } else {
                                                                                                {
                                                                                                    let index_10 = index_9 + 1f64;
                                                                                                    {
                                                                                                        (index, positions) = (index_10, positions_4);
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
}
                                                                                    }
                                                                                }
}
                                                                            }
                                                                            self::MlPlanJavaScriptArrayBindingNamesJoin8::MlPlanJavaScriptArrayBindingNamesJoin8Return(ml_result8) => {
                                                                                self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(ml_result8)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                        }
                                                        self::MlPlanJavaScriptArrayBindingNamesJoin7::MlPlanJavaScriptArrayBindingNamesJoin7Return(ml_result8_2) => {
                                                            self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Return(ml_result8_2)
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
            self::MlPlanJavaScriptArrayBindingNamesLoop5Result::MlPlanJavaScriptArrayBindingNamesLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindJavaScriptRecordParameterPatternLoop9Result {
    MlFindJavaScriptRecordParameterPatternLoop9Done,
    MlFindJavaScriptRecordParameterPatternLoop9Return(f64),
}
