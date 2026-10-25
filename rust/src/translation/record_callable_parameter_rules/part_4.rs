// Generated from complete js/src/translation/record-callable-parameter-rules.js. Do not edit by hand.
use super::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11;
use super::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result;
use super::accept_java_script_record_parameter_bound_name;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_binding_names;
use super::read_array_binding_fact_field;
use super::read_array_binding_initializer_end;
use super::read_required_number_fact;
use super::read_required_text_fact;

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10 {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Next(f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9 {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9Next(f64, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_binding_names_loop8(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut index: f64,
    mut positions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result {
    loop {
        return if (index < (values.len() as f64)) && (index < (kinds.len() as f64)) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if (kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && (value == (self::ml_string::Text::from_units(vec![93])))
                    {
                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return({
                            let source_ml_source2_concat_value99 = vec![index];
                            {
                                let source_ml_source2_concat_value100 = positions;
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value99), source_ml_source2_concat_value100)
                            }
                        })
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
                                    let source_ml_source2_s101 = if (kind
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
                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9Next(index_3, rest_2)
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9Next(index, rest)
                                    };
                                    match source_ml_source2_s101 {
                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin9Next(index_4, rest_3) => {
                                            if (index_4 >= (values.len() as f64)) || (index_4 >= (kinds.len() as f64)) {
                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(vec![-1f64])
                                            } else {
                                                {
                                                    let source_ml_source2_s106 = if ((self::read_required_text_fact(kinds, index_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_4)) == (self::ml_string::Text::from_units(vec![91]))) {
                                                        if rest_3 {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let nested = self::plan_java_script_array_binding_names(kinds, values, index_4);
                                                                if (self::read_required_number_fact(&(nested), 0f64)) < 0f64 {
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Return(vec![-1f64])
                                                                } else {
                                                                    {
                                                                        let positions_2 = {
                                                                            let source_ml_source2_concat_value107 = positions;
                                                                            {
                                                                                let source_ml_source2_concat_value108 = self::ml_array::slice(&nested, 1f64, f64::INFINITY);
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value107), source_ml_source2_concat_value108)
                                                                            }
                                                                        };
                                                                        {
                                                                            let index_5 = self::read_required_number_fact(&(nested), 0f64) + 1f64;
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Next(index_5, positions_2)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
let ml_library_inequality_3 = (self::read_required_text_fact(kinds, index_4)) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])) ;
if ml_library_inequality_3 {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let positions_3 = {
                                                                    let source_ml_source2_concat_value109 = positions;
                                                                    {
                                                                        let source_ml_source2_concat_value110 = vec![index_4];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value109), source_ml_source2_concat_value110)
                                                                    }
                                                                };
                                                                {
                                                                    let index_6 = index_4 + 1f64;
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Next(index_6, positions_3)
                                                                }
                                                            }
                                                        }
};
                                                    match source_ml_source2_s106 {
                                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Next(index_7, positions_4) => {
                                                            if (index_7 >= (values.len() as f64)) || (index_7 >= (kinds.len() as f64)) {
                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(vec![-1f64])
                                                            } else if rest_3 {
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![93]))) {
                                                                        {
                                                                            let source_ml_source2_concat_value111 = vec![index_7];
                                                                            {
                                                                                let source_ml_source2_concat_value112 = positions_4;
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value111), source_ml_source2_concat_value112)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        vec![-1f64]
                                                                    })
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_s113 = if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                            {
                                                                                let index_8 = self::read_array_binding_initializer_end(kinds, values, (index_7 + 1f64));
                                                                                if index_8 < 0f64 {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Return(vec![-1f64])
                                                                                } else {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Next(index_8)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Next(index_7)
                                                                        };
                                                                        match source_ml_source2_s113 {
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Next(index_9) => {
                                                                                {
let ml_library_inequality_2 = (self::read_required_text_fact(kinds, index_9)) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) ;
if ml_library_inequality_2 {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(vec![-1f64])
                                                                                } else {
                                                                                    {
                                                                                        let next = self::read_required_text_fact(values, index_9);
                                                                                        if next == (self::ml_string::Text::from_units(vec![93])) {
                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return({
                                                                                                let source_ml_source2_concat_value114 = vec![index_9];
                                                                                                {
                                                                                                    let source_ml_source2_concat_value115 = positions_4;
                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value114), source_ml_source2_concat_value115)
                                                                                                }
                                                                                            })
                                                                                        } else {
let ml_library_inequality_1 = next != (self::ml_string::Text::from_units(vec![44])) ;
if ml_library_inequality_1 {
                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(vec![-1f64])
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
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin11Return(source_ml_source1_temporary_result11) => {
                                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(source_ml_source1_temporary_result11)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                        }
                                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin10Return(source_ml_source1_temporary_result11_2) => {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Return(source_ml_source1_temporary_result11_2)
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
            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop8Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Result {
    MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Done(f64, self::ml_string::Text, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_binding_default_term_loop12(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut node: f64,
    mut absence: self::ml_string::Text,
    mut kind: f64,
) -> self::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Result {
    loop {
        if (kind >= 0f64)
            && ((self::read_required_text_fact(texts, kind))
                == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
        {
            {
                let field = self::read_array_binding_fact_field(
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 99, 101])),
                );
                {
                    let current = if field < 0f64 {
                        self::ml_string::Text::from_units(vec![98, 111, 116, 104])
                    } else {
                        self::read_required_text_fact(texts, field)
                    };
                    {
                        let absence_2 = if (absence == (self::ml_string::Text::from_units(vec![])))
                            || (absence == current)
                        {
                            current
                        } else {
                            self::ml_string::Text::from_units(vec![98, 111, 116, 104])
                        };
                        {
                            let node_2 = self::read_array_binding_fact_field(
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![
                                    101, 108, 101, 109, 101, 110, 116,
                                ])),
                            );
                            {
                                let kind_2 = self::read_array_binding_fact_field(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    node_2,
                                    &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
                                );
                                {
                                    (node, absence, kind) = (node_2, absence_2, kind_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Result::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop12Done(node, absence, kind)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Result {
    MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Done,
    MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_record_parameter_pattern_loop13(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut depth: f64,
    mut binding: bool,
    mut at: f64,
) -> self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Result {
    loop {
        return if at < (kinds.len() as f64) {
            {
                let kind = self::read_required_text_fact(kinds, at);
                if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                    self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Return(-1f64)
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
                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Return(-1f64)
                            } else if ((depth == 0f64) && binding)
                                && (value == (self::ml_string::Text::from_units(vec![123])))
                            {
                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Return(at)
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
                                                self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Return(-1f64)
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
            self::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Result::MlSource1TemporaryFindJavaScriptRecordParameterPatternLoop13Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_record_parameter_binding_loop14(
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
pub fn source_ml_source1_temporary_accept_java_script_record_parameter_bound_name_loop15(
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
pub enum MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Result {
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Done(f64, f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17 {
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17Next(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result {
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Done(
        f64,
        Vec<self::ml_string::Text>,
        Vec<f64>,
    ),
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19 {
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19Next(f64, f64),
    MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop18(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut ordinary: bool,
    mut at: f64,
    mut names: Vec<self::ml_string::Text>,
    mut pairs: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result {
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
                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Return(vec![-6f64, at])
            } else {
                let ml_library_inequality_1 = (self::read_required_text_fact(kinds, at))
                    != (self::ml_string::Text::from_units(vec![
                        105, 100, 101, 110, 116, 105, 102, 105, 101, 114,
                    ]));
                if ml_library_inequality_1 {
                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Return(vec![-7f64, at])
                } else {
                    {
                        let property = at;
                        {
                            let at_2 = at + 1f64;
                            if at_2 >= (kinds.len() as f64) {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Return(vec![-1f64, at_2])
                            } else {
                                {
                                    let binding = property;
                                    {
                                        let source_ml_source2_s116 =
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
                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19Return(vec![-1f64, at_3])
                                                } else if ((self::read_required_text_fact(kinds, at_3)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && (((self::read_required_text_fact(values, at_3)) == (self::ml_string::Text::from_units(vec![123]))) || ((self::read_required_text_fact(values, at_3)) == (self::ml_string::Text::from_units(vec![91])))) {
                                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19Return(vec![-5f64, at_3])
                                                    } else {
                                                        {
                                                            let binding_2 = at_3;
                                                            {
                                                                let at_4 = at_3 + 1f64;
                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19Next(at_4, binding_2)
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19Next(at_2, binding)
                                            };
                                        match source_ml_source2_s116 {
                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19Next(at_5, binding_3) => {
                                                {
                                                    let name = self::read_required_text_fact(values, binding_3);
                                                    if (self::accept_java_script_record_parameter_bound_name(&(self::read_required_text_fact(kinds, binding_3)), &(name), &(names), reserved, ordinary)) {
                                                        {
                                                            let names_2 = { let (ml_append_right, ml_append_left) = (vec![name.clone()], names); self::ml_array::append(ml_append_left, ml_append_right) };
                                                            {
                                                                let pairs_2 = {
                                                                    let source_ml_source2_concat_value117 = pairs;
                                                                    {
                                                                        let source_ml_source2_concat_value118 = vec![property, binding_3];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value117), source_ml_source2_concat_value118)
                                                                    }
                                                                };
                                                                if at_5 >= (kinds.len() as f64) {
                                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Return(vec![-1f64, at_5])
                                                                } else if ((self::read_required_text_fact(kinds, at_5)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_5)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Return(vec![-2f64, at_5])
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
                                                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Return(vec![-1f64, at_5])
                                                                            }
                                                                        }
                                                                    }
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Return(vec![-8f64, binding_3])
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin19Return(source_ml_source1_temporary_result19) => {
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Return(source_ml_source1_temporary_result19)
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
            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Done(at, names, pairs)
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
pub fn source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop16(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    reserved: &[self::ml_string::Text],
    mut ordinary: bool,
    mut at: f64,
    mut slot: f64,
    mut names: Vec<self::ml_string::Text>,
    mut instructions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Result {
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
                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Return(vec![-6f64, at])
            } else if ((self::read_required_text_fact(kinds, at))
                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                && ((self::read_required_text_fact(values, at))
                    == (self::ml_string::Text::from_units(vec![91])))
            {
                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Return(vec![-3f64, at])
            } else {
                {
                    let source_ml_source2_s126 = if ((self::read_required_text_fact(kinds, at))
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
                                        let source_ml_source2_s127 = self::source_ml_source1_temporary_plan_java_script_record_parameter_bindings_loop18(kinds, values, reserved, ordinary, at_2, names, pairs);
                                        match source_ml_source2_s127 {
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Done(at_3, names_2, pairs_2) => {
                                                    if (pairs_2.len() as f64) == 0f64 {
                                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17Return(vec![-4f64, open])
                                                    } else if (at_3 >= (kinds.len() as f64)) || !(((self::read_required_text_fact(kinds, at_3)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_3)) == (self::ml_string::Text::from_units(vec![125])))) {
                                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17Return(vec![-1f64, at_3])
                                                        } else {
                                                            {
                                                                let instructions_2 = {
                                                                    let source_ml_source2_concat_value130 = {
                                                                        let source_ml_source2_concat_value128 = instructions;
                                                                        {
                                                                            let source_ml_source2_concat_value129 = vec![1f64, slot, open, at_3, ((pairs_2.len() as f64) / 2f64)];
                                                                            self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value128), source_ml_source2_concat_value129)
                                                                        }
                                                                    };
                                                                    {
                                                                        let source_ml_source2_concat_value131 = pairs_2;
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value130), source_ml_source2_concat_value131)
                                                                    }
                                                                };
                                                                {
                                                                    let at_4 = at_3 + 1f64;
                                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17Next(at_4, names_2, instructions_2)
                                                                }
                                                            }
                                                        }
                                                }
                                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop18Return(source_ml_source1_temporary_result18) => {
                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17Return(source_ml_source1_temporary_result18)
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
                                            let source_ml_source2_concat_value132 = instructions;
                                            {
                                                let source_ml_source2_concat_value133 =
                                                    vec![0f64, slot, at];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value132,
                                                    ),
                                                    source_ml_source2_concat_value133,
                                                )
                                            }
                                        };
                                        {
                                            let at_5 = at + 1f64;
                                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17Next(at_5, names_3, instructions_3)
                                        }
                                    }
                                }
                            } else {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17Return(vec![-8f64, at])
                            }
                        }
                    };
                    match source_ml_source2_s126 {
                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17Next(at_6, names_4, instructions_4) => {
                                if at_6 >= (kinds.len() as f64) {
                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Return(vec![-1f64, at_6])
                                } else if ((self::read_required_text_fact(kinds, at_6)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, at_6)) == (self::ml_string::Text::from_units(vec![61]))) {
                                        self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Return(vec![-2f64, at_6])
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
                                                    self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Return(vec![-1f64, at_6])
                                                }
                                            }
                                        }
                                    }
                            }
                            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsJoin17Return(source_ml_source1_temporary_result17) => {
                                self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Return(source_ml_source1_temporary_result17)
                            }
                        }
                }
            }
        } else {
            self::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Result::MlSource1TemporaryPlanJavaScriptRecordParameterBindingsLoop16Done(at, slot, instructions)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop20Result {
    MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop20Done,
    MlSource1TemporaryFindJavaScriptRecordParameterStrictDirectiveLoop20Return(f64),
}
