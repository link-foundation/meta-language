// Generated from the complete checked JavaScript source and recorded dependencies.
// Do not edit by hand.
use super::MlFindJavaScriptRecordParameterPatternLoop9Result;
use super::accept_java_script_record_parameter_binding;
use super::continues_java_script_directive_expression;
use super::ml_array;
use super::ml_string;
use super::read_required_boolean_fact;
use super::read_required_text_fact;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_find_java_script_record_parameter_pattern_loop9(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut depth: f64,
    mut binding: bool,
    mut at: f64,
) -> self::MlFindJavaScriptRecordParameterPatternLoop9Result {
    loop {
        return if at < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, at);
                if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlFindJavaScriptRecordParameterPatternLoop9Result::MlFindJavaScriptRecordParameterPatternLoop9Return(-1f64)
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
                                self::MlFindJavaScriptRecordParameterPatternLoop9Result::MlFindJavaScriptRecordParameterPatternLoop9Return(-1f64)
                            } else if ((depth == 0f64) && binding)
                                && (value == (self::ml_string::Text::from_units(vec![123])))
                            {
                                self::MlFindJavaScriptRecordParameterPatternLoop9Result::MlFindJavaScriptRecordParameterPatternLoop9Return(at)
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
                                                self::MlFindJavaScriptRecordParameterPatternLoop9Result::MlFindJavaScriptRecordParameterPatternLoop9Return(-1f64)
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
            self::MlFindJavaScriptRecordParameterPatternLoop9Result::MlFindJavaScriptRecordParameterPatternLoop9Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_accept_java_script_record_parameter_binding_loop10(
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
pub enum MlPlanJavaScriptRecordParametersLoop11Result {
    MlPlanJavaScriptRecordParametersLoop11Done(f64, f64, Vec<f64>),
    MlPlanJavaScriptRecordParametersLoop11Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanJavaScriptRecordParametersJoin12 {
    MlPlanJavaScriptRecordParametersJoin12Next(f64, Vec<self::ml_string::Text>, Vec<f64>),
    MlPlanJavaScriptRecordParametersJoin12Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanJavaScriptRecordParametersLoop13Result {
    MlPlanJavaScriptRecordParametersLoop13Done(f64, Vec<self::ml_string::Text>, Vec<f64>),
    MlPlanJavaScriptRecordParametersLoop13Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlPlanJavaScriptRecordParametersJoin14 {
    MlPlanJavaScriptRecordParametersJoin14Next(f64, f64),
    MlPlanJavaScriptRecordParametersJoin14Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_java_script_record_parameters_loop13(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut at: f64,
    mut names: Vec<self::ml_string::Text>,
    mut pairs: Vec<f64>,
) -> self::MlPlanJavaScriptRecordParametersLoop13Result {
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
                self::MlPlanJavaScriptRecordParametersLoop13Result::MlPlanJavaScriptRecordParametersLoop13Return(vec![-6f64, at])
            } else {
                let ml_library_inequality_1 = (self::read_required_text_fact(kinds, at))
                    != (self::ml_string::Text::from_units(vec![
                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                    ]));
                if ml_library_inequality_1 {
                    self::MlPlanJavaScriptRecordParametersLoop13Result::MlPlanJavaScriptRecordParametersLoop13Return(vec![-7f64, at])
                } else {
                    {
                        let property = at;
                        {
                            let at_2 = at + 1f64;
                            if at_2 >= (kinds.len() as f64) {
                                self::MlPlanJavaScriptRecordParametersLoop13Result::MlPlanJavaScriptRecordParametersLoop13Return(vec![-1f64, at_2])
                            } else {
                                {
                                    let binding = property;
                                    {
                                        let ml_s12 = if ((self::read_required_text_fact(
                                            kinds, at_2,
                                        )) == (self::ml_string::Text::from_units(
                                            vec![112, 117, 110, 99, 116],
                                        ))) && ((self::read_required_text_fact(
                                            values, at_2,
                                        ))
                                            == (self::ml_string::Text::from_units(vec![58])))
                                        {
                                            {
                                                let at_3 = at_2 + 1f64;
                                                if at_3 >= (kinds.len() as f64) {
                                                    self::MlPlanJavaScriptRecordParametersJoin14::MlPlanJavaScriptRecordParametersJoin14Return(vec![-1f64, at_3])
                                                } else if ((self::read_required_text_fact(
                                                    kinds, at_3,
                                                ))
                                                    == (self::ml_string::Text::from_units(vec![
                                                        112, 117, 110, 99, 116,
                                                    ])))
                                                    && (((self::read_required_text_fact(
                                                        values, at_3,
                                                    )) == (self::ml_string::Text::from_units(
                                                        vec![123],
                                                    ))) || ((self::read_required_text_fact(
                                                        values, at_3,
                                                    ))
                                                        == (self::ml_string::Text::from_units(
                                                            vec![91],
                                                        ))))
                                                {
                                                    self::MlPlanJavaScriptRecordParametersJoin14::MlPlanJavaScriptRecordParametersJoin14Return(vec![-5f64, at_3])
                                                } else {
                                                    {
                                                        let binding_2 = at_3;
                                                        {
                                                            let at_4 = at_3 + 1f64;
                                                            self::MlPlanJavaScriptRecordParametersJoin14::MlPlanJavaScriptRecordParametersJoin14Next(at_4, binding_2)
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlPlanJavaScriptRecordParametersJoin14::MlPlanJavaScriptRecordParametersJoin14Next(at_2, binding)
                                        };
                                        match ml_s12 {
                                            self::MlPlanJavaScriptRecordParametersJoin14::MlPlanJavaScriptRecordParametersJoin14Next(at_5, binding_3) => {
                                                {
                                                    let name = self::read_required_text_fact(values, binding_3);
                                                    if (self::accept_java_script_record_parameter_binding(&(self::read_required_text_fact(kinds, binding_3)), &(name), &(names), reserved)) {
                                                        {
                                                            let names_2 = { let (ml_append_right, ml_append_left) = (vec![name.clone()], names); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            {
                                                                let pairs_2 = { let (ml_append_right, ml_append_left) = (vec![property, binding_3], pairs); self::ml_array::append(ml_append_left, ml_append_right) };
                                                                if at_5 >= (kinds.len() as f64) {
                                                                    self::MlPlanJavaScriptRecordParametersLoop13Result::MlPlanJavaScriptRecordParametersLoop13Return(vec![-1f64, at_5])
                                                                } else if ((self::read_required_text_fact(kinds, at_5)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_5)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                        self::MlPlanJavaScriptRecordParametersLoop13Result::MlPlanJavaScriptRecordParametersLoop13Return(vec![-2f64, at_5])
                                                                    } else {
                                                                        if ((self::read_required_text_fact(kinds, at_5)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_5)) == (self::ml_string::Text::from_units(vec![44]))) {
                                                                            {
                                                                                let at_6 = at_5 + 1f64;
                                                                                {
                                                                                    (at, names, pairs) = (at_6, names_2, pairs_2);
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
{
                                                                            if (((self::read_required_text_fact(kinds, at_5)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_5)) == (self::ml_string::Text::from_units(vec![125])))) {
                                                                                {
                                                                                    (at, names, pairs) = (at_5, names_2, pairs_2);
                                                                                    continue;
                                                                                }
                                                                            }
{
                                                                                self::MlPlanJavaScriptRecordParametersLoop13Result::MlPlanJavaScriptRecordParametersLoop13Return(vec![-1f64, at_5])
                                                                            }
                                                                        }
                                                                    }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlPlanJavaScriptRecordParametersLoop13Result::MlPlanJavaScriptRecordParametersLoop13Return(vec![-8f64, binding_3])
                                                    }
                                                }
                                            }
                                            self::MlPlanJavaScriptRecordParametersJoin14::MlPlanJavaScriptRecordParametersJoin14Return(ml_result14) => {
                                                self::MlPlanJavaScriptRecordParametersLoop13Result::MlPlanJavaScriptRecordParametersLoop13Return(ml_result14)
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
            self::MlPlanJavaScriptRecordParametersLoop13Result::MlPlanJavaScriptRecordParametersLoop13Done(at, names, pairs)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_plan_java_script_record_parameters_loop11(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut at: f64,
    mut slot: f64,
    mut names: Vec<self::ml_string::Text>,
    mut instructions: Vec<f64>,
) -> self::MlPlanJavaScriptRecordParametersLoop11Result {
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
                self::MlPlanJavaScriptRecordParametersLoop11Result::MlPlanJavaScriptRecordParametersLoop11Return(vec![-6f64, at])
            } else if ((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![91])))
            {
                self::MlPlanJavaScriptRecordParametersLoop11Result::MlPlanJavaScriptRecordParametersLoop11Return(vec![-3f64, at])
            } else {
                {
                    let ml_s14 = if ((self::read_required_text_fact(kinds, at))
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
                                        let ml_s15 =
                                            self::ml_plan_java_script_record_parameters_loop13(
                                                kinds, values, reserved, at_2, names, pairs,
                                            );
                                        match ml_s15 {
                                                self::MlPlanJavaScriptRecordParametersLoop13Result::MlPlanJavaScriptRecordParametersLoop13Done(at_3, names_2, pairs_2) => {
                                                    if (pairs_2.len() as f64) == 0f64 {
                                                        self::MlPlanJavaScriptRecordParametersJoin12::MlPlanJavaScriptRecordParametersJoin12Return(vec![-4f64, open])
                                                    } else if (at_3 >= (kinds.len() as f64)) || !(((self::read_required_text_fact(kinds, at_3)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_3)) == (self::ml_string::Text::from_units(vec![125])))) {
                                                            self::MlPlanJavaScriptRecordParametersJoin12::MlPlanJavaScriptRecordParametersJoin12Return(vec![-1f64, at_3])
                                                        } else {
                                                            {
                                                                let instructions_2 = self::ml_array::append({ let (ml_append_right, ml_append_left) = (vec![1f64, slot, open, at_3, ((pairs_2.len() as f64) / 2f64)], instructions); self::ml_array::append(ml_append_left, ml_append_right) }, pairs_2);
                                                                {
                                                                    let at_4 = at_3 + 1f64;
                                                                    self::MlPlanJavaScriptRecordParametersJoin12::MlPlanJavaScriptRecordParametersJoin12Next(at_4, names_2, instructions_2)
                                                                }
                                                            }
                                                        }
                                                }
                                                self::MlPlanJavaScriptRecordParametersLoop13Result::MlPlanJavaScriptRecordParametersLoop13Return(ml_result13) => {
                                                    self::MlPlanJavaScriptRecordParametersJoin12::MlPlanJavaScriptRecordParametersJoin12Return(ml_result13)
                                                }
                                            }
                                    }
                                }
                            }
                        }
                    } else {
                        {
                            let name = self::read_required_text_fact(values, at);
                            if (self::accept_java_script_record_parameter_binding(
                                &(self::read_required_text_fact(kinds, at)),
                                &(name),
                                &(names),
                                reserved,
                            )) {
                                {
                                    let names_3 = {
                                        let (ml_append_right, ml_append_left) =
                                            (vec![name.clone()], names);
                                        self::ml_array::append(ml_append_left, ml_append_right)
                                    };
                                    {
                                        let instructions_3 = {
                                            let (ml_append_right, ml_append_left) =
                                                (vec![0f64, slot, at], instructions);
                                            self::ml_array::append(ml_append_left, ml_append_right)
                                        };
                                        {
                                            let at_5 = at + 1f64;
                                            self::MlPlanJavaScriptRecordParametersJoin12::MlPlanJavaScriptRecordParametersJoin12Next(at_5, names_3, instructions_3)
                                        }
                                    }
                                }
                            } else {
                                self::MlPlanJavaScriptRecordParametersJoin12::MlPlanJavaScriptRecordParametersJoin12Return(vec![-8f64, at])
                            }
                        }
                    };
                    match ml_s14 {
                            self::MlPlanJavaScriptRecordParametersJoin12::MlPlanJavaScriptRecordParametersJoin12Next(at_6, names_4, instructions_4) => {
                                if at_6 >= (kinds.len() as f64) {
                                    self::MlPlanJavaScriptRecordParametersLoop11Result::MlPlanJavaScriptRecordParametersLoop11Return(vec![-1f64, at_6])
                                } else if ((self::read_required_text_fact(kinds, at_6)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_6)) == (self::ml_string::Text::from_units(vec![61]))) {
                                        self::MlPlanJavaScriptRecordParametersLoop11Result::MlPlanJavaScriptRecordParametersLoop11Return(vec![-2f64, at_6])
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
                                                    self::MlPlanJavaScriptRecordParametersLoop11Result::MlPlanJavaScriptRecordParametersLoop11Return(vec![-1f64, at_6])
                                                }
                                            }
                                        }
                                    }
                            }
                            self::MlPlanJavaScriptRecordParametersJoin12::MlPlanJavaScriptRecordParametersJoin12Return(ml_result12) => {
                                self::MlPlanJavaScriptRecordParametersLoop11Result::MlPlanJavaScriptRecordParametersLoop11Return(ml_result12)
                            }
                        }
                }
            }
        } else {
            self::MlPlanJavaScriptRecordParametersLoop11Result::MlPlanJavaScriptRecordParametersLoop11Done(at, slot, instructions)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result {
    MlFindJavaScriptRecordParameterStrictDirectiveLoop15Done,
    MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn ml_find_java_script_record_parameter_strict_directive_loop15(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    raws: &[self::ml_string::Text],
    line_breaks: &[bool],
    mut at: f64,
) -> self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result {
    loop {
        return if at < (kinds.len() as f64) {
            {
                let current = self::read_required_text_fact(kinds, at);
                if current == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(-2f64)
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
                                    self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(-2f64)
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
                                                    self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(at)
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
                                                self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(if strict {
                                                            at
                                                        } else {
                                                            -1f64
                                                        })
                                            } else if kind
                                                == (self::ml_string::Text::from_units(vec![
                                                    101, 111, 102,
                                                ]))
                                            {
                                                self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(-2f64)
                                            } else if (self::read_required_boolean_fact(
                                                line_breaks,
                                                next,
                                            )) {
                                                if self::continues_java_script_directive_expression(&(kind), &(value)) {
                                                                    self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(-1f64)
                                                                } else if ((kind == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) || (kind == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])))) || ((kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (((value == (self::ml_string::Text::from_units(vec![123]))) || (value == (self::ml_string::Text::from_units(vec![43, 43])))) || (value == (self::ml_string::Text::from_units(vec![45, 45]))))) {
                                                                        if strict {
                                                                            self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(at)
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
                                                                        self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(-2f64)
                                                                    }
                                            } else {
                                                self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(-1f64)
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Return(-1f64)
                }
            }
        } else {
            self::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Result::MlFindJavaScriptRecordParameterStrictDirectiveLoop15Done
        };
    }
}
