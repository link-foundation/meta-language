// Generated from complete js/src/translation/array-binding-rules.js. Do not edit by hand.
use super::ml_array;
use super::ml_string;
use super::read_required_number_fact;
use super::read_required_text_fact;
use super::source_ml_source1_temporary_read_java_script_binding_default_term_loop9;

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
                    let source_ml_source2_s2 =
                        self::source_ml_source1_temporary_read_array_binding_fact_field_loop2(
                            labels, targets, field, at, end,
                        );
                    match source_ml_source2_s2 {
                        self::MlSource1TemporaryReadArrayBindingFactFieldLoop2Result::MlSource1TemporaryReadArrayBindingFactFieldLoop2Done => {
                            -1f64
                        }
                        self::MlSource1TemporaryReadArrayBindingFactFieldLoop2Result::MlSource1TemporaryReadArrayBindingFactFieldLoop2Return(source_ml_source1_temporary_result2) => {
                            source_ml_source1_temporary_result2
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
                let source_ml_source2_s3 =
                    self::source_ml_source1_temporary_read_array_binding_initializer_end_loop3(
                        kinds, values, start, index, closes,
                    );
                match source_ml_source2_s3 {
                    self::MlSource1TemporaryReadArrayBindingInitializerEndLoop3Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop3Done => {
                        -1f64
                    }
                    self::MlSource1TemporaryReadArrayBindingInitializerEndLoop3Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop3Return(source_ml_source1_temporary_result3) => {
                        source_ml_source1_temporary_result3
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
                    let source_ml_source2_s4 = self::source_ml_source1_temporary_plan_java_script_array_binding_names_loop5(kinds, values, index, positions);
                    match source_ml_source2_s4 {
                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Done => {
                                vec![-1f64]
                            }
                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(source_ml_source1_temporary_result5) => {
                                source_ml_source1_temporary_result5
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_binding_default_form(
    language: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    absence: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = (*language)
            != (self::ml_string::Text::from_units(vec![
                74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
            ]));
        if ml_library_inequality_1 {
            self::ml_string::Text::from_units(vec![
                114, 101, 102, 117, 115, 101, 45, 108, 97, 110, 103, 117, 97, 103, 101,
            ])
        } else if ((*kind) == (self::ml_string::Text::from_units(vec![118, 97, 114])))
            || ((*kind)
                == (self::ml_string::Text::from_units(vec![108, 105, 116, 101, 114, 97, 108])))
        {
            self::ml_string::Text::from_units(vec![100, 101, 102, 101, 114])
        } else if ((*kind)
            == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
            && ((*absence) != (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))
        {
            self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116])
        } else {
            self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_binding_default_result_absence(
    left: &self::ml_string::Text,
    right_kind: &self::ml_string::Text,
    right: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let null_value = (((*left)
            == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))
            || ((*left) == (self::ml_string::Text::from_units(vec![98, 111, 116, 104]))))
            || (((*right_kind)
                == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
                && (((*right) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])))
                    || ((*right) == (self::ml_string::Text::from_units(vec![98, 111, 116, 104])))));
        {
            let undefined_value = ((*right_kind)
                == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))
                || (((*right_kind)
                    == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
                    && (((*right)
                        == (self::ml_string::Text::from_units(vec![
                            117, 110, 100, 101, 102, 105, 110, 101, 100,
                        ])))
                        || ((*right)
                            == (self::ml_string::Text::from_units(vec![98, 111, 116, 104])))));
            if null_value {
                if undefined_value {
                    self::ml_string::Text::from_units(vec![98, 111, 116, 104])
                } else {
                    self::ml_string::Text::from_units(vec![110, 117, 108, 108])
                }
            } else if undefined_value {
                self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_binding_default_absence_code(absence: &self::ml_string::Text) -> f64 {
    if (*absence) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])) {
        1f64
    } else if (*absence)
        == (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))
    {
        2f64
    } else if (*absence) == (self::ml_string::Text::from_units(vec![98, 111, 116, 104])) {
        3f64
    } else {
        0f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_binding_default_absence_name(code: f64) -> self::ml_string::Text {
    if code == 1f64 {
        self::ml_string::Text::from_units(vec![110, 117, 108, 108])
    } else if code == 2f64 {
        self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
    } else if code == 3f64 {
        self::ml_string::Text::from_units(vec![98, 111, 116, 104])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn plan_java_script_binding_default_inference(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    value: f64,
    fallback: f64,
) -> Vec<f64> {
    {
        let kind = self::read_array_binding_fact_field(
            starts,
            counts,
            labels,
            targets,
            value,
            &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
        );
        if kind < 0f64 {
            vec![-1f64]
        } else if (self::read_required_text_fact(texts, kind))
            == (self::ml_string::Text::from_units(vec![118, 97, 114]))
        {
            Vec::<f64>::new()
        } else {
            {
                let left = self::read_java_script_binding_default_term(
                    texts, starts, counts, labels, targets, value,
                );
                {
                    let right = self::read_java_script_binding_default_term(
                        texts, starts, counts, labels, targets, fallback,
                    );
                    {
                        let element = self::read_required_number_fact(&(left), 0f64);
                        {
                            let other = self::read_required_number_fact(&(right), 0f64);
                            if (element < 0f64) || (other < 0f64) {
                                vec![-1f64]
                            } else {
                                {
                                    let code = self::read_required_number_fact(&(left), 1f64);
                                    {
                                        let right_code =
                                            self::read_required_number_fact(&(right), 1f64);
                                        if code == 0f64 {
                                            vec![0f64, value]
                                        } else if code == 1f64 {
                                            vec![1f64, element]
                                        } else {
                                            {
                                                let first_kind =
                                                    self::read_array_binding_fact_field(
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        element,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            107, 105, 110, 100,
                                                        ])),
                                                    );
                                                {
                                                    let other_kind =
                                                        self::read_array_binding_fact_field(
                                                            starts,
                                                            counts,
                                                            labels,
                                                            targets,
                                                            other,
                                                            &(self::ml_string::Text::from_units(
                                                                vec![107, 105, 110, 100],
                                                            )),
                                                        );
                                                    if (first_kind < 0f64) || (other_kind < 0f64) {
                                                        vec![-1f64]
                                                    } else {
                                                        {
                                                            let first_unit = (self::read_required_text_fact(texts, first_kind)) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]));
                                                            {
                                                                let second_unit = (self::read_required_text_fact(texts, other_kind)) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]));
                                                                {
                                                                    let output = self::read_java_script_binding_default_absence_code(&(self::read_java_script_binding_default_result_absence(&(self::read_java_script_binding_default_absence_name(code)), &(if right_code == 0f64 {
                                                                                self::read_required_text_fact(texts, other_kind)
                                                                            } else {
                                                                                self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])
                                                                            }), &(self::read_java_script_binding_default_absence_name(right_code)))));
                                                                    {
                                                                        let base = if first_unit {
                                                                            other
                                                                        } else {
                                                                            element
                                                                        };
                                                                        if first_unit || second_unit
                                                                        {
                                                                            vec![output, base]
                                                                        } else {
                                                                            vec![
                                                                                output, base,
                                                                                element, other,
                                                                            ]
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
pub fn read_java_script_binding_default_term(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    value: f64,
) -> Vec<f64> {
    {
        let node = value;
        {
            let absence = self::ml_string::Text::from_units(vec![]);
            {
                let kind = self::read_array_binding_fact_field(
                    starts,
                    counts,
                    labels,
                    targets,
                    node,
                    &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
                );
                {
                    let source_ml_source2_s5 = self::source_ml_source1_temporary_read_java_script_binding_default_term_loop9(texts, starts, counts, labels, targets, node, absence, kind);
                    match source_ml_source2_s5 {
                        self::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Result::MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Done(node_2, absence_2, kind_2) => {
                            if kind_2 < 0f64 {
                                vec![-1f64, 0f64]
                            } else {
                                vec![node_2, self::read_java_script_binding_default_absence_code(&(absence_2))]
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
pub fn read_java_script_binding_default_lift_form(
    kind: &self::ml_string::Text,
    element: &self::ml_string::Text,
    absence: &self::ml_string::Text,
    drop_undefined: bool,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])) {
        self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100])
    } else {
        let ml_library_inequality_2 =
            (*kind) != (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110]));
        if ml_library_inequality_2 {
            self::ml_string::Text::from_units(vec![112, 114, 101, 115, 101, 110, 116])
        } else {
            let ml_library_inequality_1 =
                (*element) != (self::ml_string::Text::from_units(vec![117, 110, 105, 116]));
            if ml_library_inequality_1 {
                self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
            } else if drop_undefined {
                self::ml_string::Text::from_units(vec![110, 117, 108, 108])
            } else if (*absence) == (self::ml_string::Text::from_units(vec![98, 111, 116, 104])) {
                self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 99, 101, 115])
            } else {
                absence.clone()
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_binding_default_lift_absences(
    absence: &self::ml_string::Text,
    drop_undefined: bool,
) -> Vec<self::ml_string::Text> {
    if (*absence) == (self::ml_string::Text::from_units(vec![110, 117, 108, 108])) {
        vec![self::ml_string::Text::from_units(vec![110, 117, 108, 108])]
    } else if (*absence)
        == (self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]))
    {
        if drop_undefined {
            Vec::<self::ml_string::Text>::new()
        } else {
            vec![self::ml_string::Text::from_units(vec![
                117, 110, 100, 101, 102, 105, 110, 101, 100,
            ])]
        }
    } else if drop_undefined {
        vec![self::ml_string::Text::from_units(vec![110, 117, 108, 108])]
    } else {
        vec![
            self::ml_string::Text::from_units(vec![110, 117, 108, 108]),
            self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]),
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop1Result {
    MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop1Done,
    MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop1Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_java_script_array_binding_name_loop1(
    name: &self::ml_string::Text,
    reserved: &[self::ml_string::Text],
    mut index: f64,
) -> self::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop1Result {
    loop {
        return if index < (reserved.len() as f64) {
            if (self::read_required_text_fact(reserved, index)) == (*name) {
                self::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop1Result::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop1Return(false)
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
            self::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop1Result::MlSource1TemporaryAcceptJavaScriptArrayBindingNameLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadArrayBindingFactFieldLoop2Result {
    MlSource1TemporaryReadArrayBindingFactFieldLoop2Done,
    MlSource1TemporaryReadArrayBindingFactFieldLoop2Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_array_binding_fact_field_loop2(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    field: &self::ml_string::Text,
    mut at: f64,
    mut end: f64,
) -> self::MlSource1TemporaryReadArrayBindingFactFieldLoop2Result {
    loop {
        return if at < end {
            if (self::read_required_text_fact(labels, at)) == (*field) {
                self::MlSource1TemporaryReadArrayBindingFactFieldLoop2Result::MlSource1TemporaryReadArrayBindingFactFieldLoop2Return(self::read_required_number_fact(targets, at))
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
            self::MlSource1TemporaryReadArrayBindingFactFieldLoop2Result::MlSource1TemporaryReadArrayBindingFactFieldLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadArrayBindingInitializerEndLoop3Result {
    MlSource1TemporaryReadArrayBindingInitializerEndLoop3Done,
    MlSource1TemporaryReadArrayBindingInitializerEndLoop3Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadArrayBindingInitializerEndJoin4 {
    MlSource1TemporaryReadArrayBindingInitializerEndJoin4Next(Vec<self::ml_string::Text>),
    MlSource1TemporaryReadArrayBindingInitializerEndJoin4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_array_binding_initializer_end_loop3(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut start: f64,
    mut index: f64,
    mut closes: Vec<self::ml_string::Text>,
) -> self::MlSource1TemporaryReadArrayBindingInitializerEndLoop3Result {
    loop {
        return if (index < (values.len() as f64)) && (index < (kinds.len() as f64)) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if kind == (self::ml_string::Text::from_units(vec![101, 111, 102])) {
                        self::MlSource1TemporaryReadArrayBindingInitializerEndLoop3Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop3Return(-1f64)
                    } else {
                        {
                            let source_ml_source2_s8 = if kind
                                == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))
                            {
                                if ((closes.len() as f64) == 0f64)
                                    && ((value == (self::ml_string::Text::from_units(vec![44])))
                                        || (value == (self::ml_string::Text::from_units(vec![93]))))
                                {
                                    self::MlSource1TemporaryReadArrayBindingInitializerEndJoin4::MlSource1TemporaryReadArrayBindingInitializerEndJoin4Return(if index > start {
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
                                            let source_ml_source2_concat_value9 = closes;
                                            {
                                                let source_ml_source2_concat_value10 =
                                                    vec![if value
                                                        == (self::ml_string::Text::from_units(
                                                            vec![40],
                                                        )) {
                                                        self::ml_string::Text::from_units(vec![41])
                                                    } else if value
                                                        == (self::ml_string::Text::from_units(
                                                            vec![91],
                                                        ))
                                                    {
                                                        self::ml_string::Text::from_units(vec![93])
                                                    } else {
                                                        self::ml_string::Text::from_units(vec![125])
                                                    }];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<self::ml_string::Text>::new(),
                                                        source_ml_source2_concat_value9,
                                                    ),
                                                    source_ml_source2_concat_value10,
                                                )
                                            }
                                        };
                                        self::MlSource1TemporaryReadArrayBindingInitializerEndJoin4::MlSource1TemporaryReadArrayBindingInitializerEndJoin4Next(closes_2)
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
                                        self::MlSource1TemporaryReadArrayBindingInitializerEndJoin4::MlSource1TemporaryReadArrayBindingInitializerEndJoin4Return(-1f64)
                                    } else {
                                        {
                                            let closes_3 = self::ml_array::slice(
                                                &closes,
                                                0f64,
                                                ((closes.len() as f64) - 1f64),
                                            );
                                            self::MlSource1TemporaryReadArrayBindingInitializerEndJoin4::MlSource1TemporaryReadArrayBindingInitializerEndJoin4Next(closes_3)
                                        }
                                    }
                                } else {
                                    self::MlSource1TemporaryReadArrayBindingInitializerEndJoin4::MlSource1TemporaryReadArrayBindingInitializerEndJoin4Next(closes)
                                }
                            } else {
                                self::MlSource1TemporaryReadArrayBindingInitializerEndJoin4::MlSource1TemporaryReadArrayBindingInitializerEndJoin4Next(closes)
                            };
                            match source_ml_source2_s8 {
                                self::MlSource1TemporaryReadArrayBindingInitializerEndJoin4::MlSource1TemporaryReadArrayBindingInitializerEndJoin4Next(closes_4) => {
                                    {
                                        let index_2 = index + 1f64;
                                        {
                                            (index, closes) = (index_2, closes_4);
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadArrayBindingInitializerEndJoin4::MlSource1TemporaryReadArrayBindingInitializerEndJoin4Return(source_ml_source1_temporary_result4) => {
                                    self::MlSource1TemporaryReadArrayBindingInitializerEndLoop3Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop3Return(source_ml_source1_temporary_result4)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadArrayBindingInitializerEndLoop3Result::MlSource1TemporaryReadArrayBindingInitializerEndLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Done,
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8 {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Next(f64),
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7 {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Next(f64, Vec<f64>),
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Return(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6 {
    MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6Next(f64, bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_java_script_array_binding_names_loop5(
    kinds: &[self::ml_string::Text],
    values: &[self::ml_string::Text],
    mut index: f64,
    mut positions: Vec<f64>,
) -> self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result {
    loop {
        return if (index < (values.len() as f64)) && (index < (kinds.len() as f64)) {
            {
                let kind = self::read_required_text_fact(kinds, index);
                {
                    let value = self::read_required_text_fact(values, index);
                    if (kind == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])))
                        && (value == (self::ml_string::Text::from_units(vec![93])))
                    {
                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return({
                            let source_ml_source2_concat_value11 = vec![index];
                            {
                                let source_ml_source2_concat_value12 = positions;
                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value11), source_ml_source2_concat_value12)
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
                                    let source_ml_source2_s13 = if (kind
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
                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6Next(index_3, rest_2)
                                            }
                                        }
                                    } else {
                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6Next(index, rest)
                                    };
                                    match source_ml_source2_s13 {
                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin6Next(index_4, rest_3) => {
                                            if (index_4 >= (values.len() as f64)) || (index_4 >= (kinds.len() as f64)) {
                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                            } else {
                                                {
                                                    let source_ml_source2_s18 = if ((self::read_required_text_fact(kinds, index_4)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_4)) == (self::ml_string::Text::from_units(vec![91]))) {
                                                        if rest_3 {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let nested = self::plan_java_script_array_binding_names(kinds, values, index_4);
                                                                if (self::read_required_number_fact(&(nested), 0f64)) < 0f64 {
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                                } else {
                                                                    {
                                                                        let positions_2 = {
                                                                            let source_ml_source2_concat_value19 = positions;
                                                                            {
                                                                                let source_ml_source2_concat_value20 = self::ml_array::slice(&nested, 1f64, f64::INFINITY);
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value19), source_ml_source2_concat_value20)
                                                                            }
                                                                        };
                                                                        {
                                                                            let index_5 = self::read_required_number_fact(&(nested), 0f64) + 1f64;
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Next(index_5, positions_2)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    } else {
let ml_library_inequality_3 = (self::read_required_text_fact(kinds, index_4)) != (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 102, 105, 101, 114])) ;
if ml_library_inequality_3 {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Return(vec![-1f64])
                                                        } else {
                                                            {
                                                                let positions_3 = {
                                                                    let source_ml_source2_concat_value21 = positions;
                                                                    {
                                                                        let source_ml_source2_concat_value22 = vec![index_4];
                                                                        self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value21), source_ml_source2_concat_value22)
                                                                    }
                                                                };
                                                                {
                                                                    let index_6 = index_4 + 1f64;
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Next(index_6, positions_3)
                                                                }
                                                            }
                                                        }
};
                                                    match source_ml_source2_s18 {
                                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Next(index_7, positions_4) => {
                                                            if (index_7 >= (values.len() as f64)) || (index_7 >= (kinds.len() as f64)) {
                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                                            } else if rest_3 {
                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![93]))) {
                                                                        {
                                                                            let source_ml_source2_concat_value23 = vec![index_7];
                                                                            {
                                                                                let source_ml_source2_concat_value24 = positions_4;
                                                                                self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value23), source_ml_source2_concat_value24)
                                                                            }
                                                                        }
                                                                    } else {
                                                                        vec![-1f64]
                                                                    })
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_s25 = if ((self::read_required_text_fact(kinds, index_7)) == (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116]))) && ((self::read_required_text_fact(values, index_7)) == (self::ml_string::Text::from_units(vec![61]))) {
                                                                            {
                                                                                let index_8 = self::read_array_binding_initializer_end(kinds, values, (index_7 + 1f64));
                                                                                if index_8 < 0f64 {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Return(vec![-1f64])
                                                                                } else {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Next(index_8)
                                                                                }
                                                                            }
                                                                        } else {
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Next(index_7)
                                                                        };
                                                                        match source_ml_source2_s25 {
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Next(index_9) => {
                                                                                {
let ml_library_inequality_2 = (self::read_required_text_fact(kinds, index_9)) != (self::ml_string::Text::from_units(vec![112, 117, 110, 99, 116])) ;
if ml_library_inequality_2 {
                                                                                    self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
                                                                                } else {
                                                                                    {
                                                                                        let next = self::read_required_text_fact(values, index_9);
                                                                                        if next == (self::ml_string::Text::from_units(vec![93])) {
                                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return({
                                                                                                let source_ml_source2_concat_value26 = vec![index_9];
                                                                                                {
                                                                                                    let source_ml_source2_concat_value27 = positions_4;
                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value26), source_ml_source2_concat_value27)
                                                                                                }
                                                                                            })
                                                                                        } else {
let ml_library_inequality_1 = next != (self::ml_string::Text::from_units(vec![44])) ;
if ml_library_inequality_1 {
                                                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(vec![-1f64])
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
                                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin8Return(source_ml_source1_temporary_result8) => {
                                                                                self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(source_ml_source1_temporary_result8)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                        }
                                                        self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7::MlSource1TemporaryPlanJavaScriptArrayBindingNamesJoin7Return(source_ml_source1_temporary_result8_2) => {
                                                            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Return(source_ml_source1_temporary_result8_2)
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
            self::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Result::MlSource1TemporaryPlanJavaScriptArrayBindingNamesLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Result {
    MlSource1TemporaryReadJavaScriptBindingDefaultTermLoop9Done(f64, self::ml_string::Text, f64),
}
