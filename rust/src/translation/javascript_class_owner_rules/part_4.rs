// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCarriesLoop21Result;
use super::MlSource1TemporaryExactLoop18Result;
use super::MlSource1TemporaryFreshLoop22Result;
use super::MlSource1TemporaryFreshLoop23Result;
use super::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result;
use super::MlSource1TemporaryStringsLoop19Result;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::source_ml_source1_temporary_carries_loop21;
use super::source_ml_source1_temporary_exact_loop18;
use super::source_ml_source1_temporary_fresh_loop22;
use super::source_ml_source1_temporary_fresh_loop23;
use super::source_ml_source1_temporary_names_loop20;
use super::source_ml_source1_temporary_read_java_script_class_record_owner_loop16;
use super::source_ml_source1_temporary_strings_loop19;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_write_statement() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        65, 110, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 32, 119, 114, 105, 116, 101, 32,
        101, 118, 97, 108, 117, 97, 116, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
        108, 32, 115, 104, 97, 114, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 111,
        110, 99, 101, 44, 32, 114, 101, 97, 100, 115, 32, 97, 32, 99, 111, 109, 112, 111, 117, 110,
        100, 32, 111, 112, 101, 114, 97, 110, 100, 32, 98, 101, 102, 111, 114, 101, 32, 116, 104,
        101, 32, 114, 105, 103, 104, 116, 45, 104, 97, 110, 100, 32, 101, 120, 112, 114, 101, 115,
        115, 105, 111, 110, 44, 32, 101, 118, 97, 108, 117, 97, 116, 101, 115, 32, 116, 104, 97,
        116, 32, 101, 120, 112, 114, 101, 115, 115, 105, 111, 110, 32, 111, 110, 99, 101, 44, 32,
        116, 104, 101, 110, 32, 119, 114, 105, 116, 101, 115, 32, 116, 104, 101, 32, 115, 97, 109,
        101, 32, 102, 105, 101, 108, 100, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32,
        97, 102, 116, 101, 114, 32, 114, 101, 108, 101, 97, 115, 105, 110, 103, 32, 101, 118, 101,
        114, 121, 32, 101, 97, 114, 108, 105, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32,
        98, 111, 114, 114, 111, 119, 59, 32, 105, 116, 115, 32, 115, 116, 97, 116, 101, 109, 101,
        110, 116, 32, 114, 101, 115, 117, 108, 116, 32, 105, 115, 32, 85, 110, 105, 116, 46,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_class_owner_model() -> self::ml_string::Text {
    self::ml_string::Text::from_units(vec![
        99, 108, 111, 115, 101, 100, 32, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 32, 99, 108,
        97, 115, 115, 32, 100, 97, 116, 97, 32, 111, 119, 110, 101, 114, 115, 32, 49,
    ])
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_class_owner_inference(
    left: &self::ml_string::Text,
    right: &self::ml_string::Text,
    left_open: bool,
    right_open: bool,
) -> self::ml_string::Text {
    if (*left) == (*right) {
        self::ml_string::Text::from_units(vec![115, 97, 109, 101])
    } else if (((*left) != (self::ml_string::Text::from_units(vec![])))
        && ((*right) == (self::ml_string::Text::from_units(vec![]))))
        && right_open
    {
        self::ml_string::Text::from_units(vec![114, 105, 103, 104, 116])
    } else if (((*right) != (self::ml_string::Text::from_units(vec![])))
        && ((*left) == (self::ml_string::Text::from_units(vec![]))))
        && left_open
    {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116])
    } else {
        self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_class_record_owner(
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    fields: &[self::ml_string::Text],
) -> self::ml_string::Text {
    {
        let owner = self::ml_string::Text::from_units(vec![]);
        {
            let at = 0f64;
            {
                let source_ml_source2_s14 =
                    self::source_ml_source1_temporary_read_java_script_class_record_owner_loop16(
                        constructors,
                        brands,
                        fields,
                        owner,
                        at,
                    );
                match source_ml_source2_s14 {
                    self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Done(owner_2) => {
                        owner_2
                    }
                    self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop16Return(source_ml_source1_temporary_result16) => {
                        source_ml_source1_temporary_result16
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_class_owner_observation(
    representation: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> bool {
    ((*representation)
        == (self::ml_string::Text::from_units(vec![
            99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115,
        ])))
        && ((*operation)
            == (self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])))
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn word(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    name: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let value = self::dependency_function3(starts, counts, labels, targets, node, name);
        if (value >= 0f64)
            && ((self::dependency_function2(forms, value))
                == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])))
        {
            self::dependency_function2(texts, value)
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn exact(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    names_2: &[self::ml_string::Text],
) -> bool {
    if (node < 0f64) || ((self::dependency_function1(counts, node)) != (names_2.len() as f64)) {
        false
    } else {
        {
            let source_ml_source1_temporary_values17 = names_2.to_vec();
            {
                let source_ml_source1_temporary_index17 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s15 = self::source_ml_source1_temporary_exact_loop18(
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(source_ml_source1_temporary_values17),
                        source_ml_source1_temporary_index17,
                    );
                    match source_ml_source2_s15 {
                        self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Done => {
                            true
                        }
                        self::MlSource1TemporaryExactLoop18Result::MlSource1TemporaryExactLoop18Return(source_ml_source1_temporary_result18) => {
                            source_ml_source1_temporary_result18
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn strings(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        if (node < 0f64)
            || ((self::dependency_function2(forms, node))
                != (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
        {
            vec![self::ml_string::Text::from_units(vec![])]
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source2_s16 = self::source_ml_source1_temporary_strings_loop19(
                        forms, texts, starts, counts, targets, node, result, at,
                    );
                    match source_ml_source2_s16 {
                        self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Done(result_2) => {
                            result_2
                        }
                        self::MlSource1TemporaryStringsLoop19Result::MlSource1TemporaryStringsLoop19Return(source_ml_source1_temporary_result19) => {
                            source_ml_source1_temporary_result19
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn names(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
) -> Vec<self::ml_string::Text> {
    {
        let result = Vec::<self::ml_string::Text>::new();
        {
            let fields = self::dependency_function3(
                starts,
                counts,
                labels,
                targets,
                node,
                &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])),
            );
            if fields < 0f64 {
                vec![self::ml_string::Text::from_units(vec![])]
            } else {
                {
                    let at = 0f64;
                    {
                        self::source_ml_source1_temporary_names_loop20(
                            forms, texts, starts, counts, labels, targets, result, fields, at,
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn carries(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    type_: f64,
    owner: &self::ml_string::Text,
    depth: f64,
) -> bool {
    if (type_ < 0f64) || (depth > 128f64) {
        false
    } else if (self::word(
        forms,
        texts,
        starts,
        counts,
        labels,
        targets,
        type_,
        &(self::ml_string::Text::from_units(vec![
            115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114,
        ])),
    )) == (*owner)
    {
        true
    } else {
        let ml_library_inequality_1 =
            (self::word(
                forms,
                texts,
                starts,
                counts,
                labels,
                targets,
                type_,
                &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
            )) != (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]));
        if ml_library_inequality_1 {
            false
        } else {
            {
                let elements = self::dependency_function3(
                    starts,
                    counts,
                    labels,
                    targets,
                    type_,
                    &(self::ml_string::Text::from_units(vec![
                        101, 108, 101, 109, 101, 110, 116, 115,
                    ])),
                );
                if elements < 0f64 {
                    false
                } else {
                    {
                        let at = 0f64;
                        {
                            let source_ml_source2_s17 =
                                self::source_ml_source1_temporary_carries_loop21(
                                    forms, texts, starts, counts, labels, targets, owner, depth,
                                    elements, at,
                                );
                            match source_ml_source2_s17 {
                                    self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Done => {
                                        false
                                    }
                                    self::MlSource1TemporaryCarriesLoop21Result::MlSource1TemporaryCarriesLoop21Return(source_ml_source1_temporary_result21) => {
                                        source_ml_source1_temporary_result21
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn payload(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    owner: &self::ml_string::Text,
    depth: f64,
) -> bool {
    if (node < 0f64) || (depth > 256f64) {
        false
    } else {
        {
            let kind = self::word(
                forms,
                texts,
                starts,
                counts,
                labels,
                targets,
                node,
                &(self::ml_string::Text::from_units(vec![107])),
            );
            if kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])) {
                (self::word(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
                    ),
                    &(self::ml_string::Text::from_units(vec![
                        115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114,
                    ])),
                )) == (*owner)
            } else if ((kind == (self::ml_string::Text::from_units(vec![108, 101, 116])))
                || (kind == (self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116]))))
                || (kind
                    == (self::ml_string::Text::from_units(vec![
                        99, 101, 108, 108, 87, 114, 105, 116, 101,
                    ])))
            {
                self::payload(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(self::ml_string::Text::from_units(vec![98, 111, 100, 121])),
                    ),
                    owner,
                    (depth + 1f64),
                )
            } else if kind == (self::ml_string::Text::from_units(vec![105, 102])) {
                self::payload(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(self::ml_string::Text::from_units(vec![116, 104, 101, 110])),
                    ),
                    owner,
                    (depth + 1f64),
                ) && self::payload(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(self::ml_string::Text::from_units(vec![101, 108, 115, 101])),
                    ),
                    owner,
                    (depth + 1f64),
                )
            } else {
                false
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn fresh(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    node: f64,
    owner: &self::ml_string::Text,
    bindings: &[self::ml_string::Text],
    values: &[f64],
    depth: f64,
) -> f64 {
    if (node < 0f64) || (depth > 256f64) {
        -1f64
    } else {
        {
            let type_ = self::dependency_function3(
                starts,
                counts,
                labels,
                targets,
                node,
                &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
            );
            if (self::carries(
                forms, texts, starts, counts, labels, targets, type_, owner, 0f64,
            )) {
                {
                    let kind = self::word(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(self::ml_string::Text::from_units(vec![107])),
                    );
                    if kind
                        == (self::ml_string::Text::from_units(vec![
                            114, 101, 102, 101, 114, 101, 110, 99, 101, 65, 108, 108, 111, 99, 97,
                            116, 101,
                        ]))
                    {
                        {
                            let reference = self::dependency_function3(
                                starts,
                                counts,
                                labels,
                                targets,
                                type_,
                                &(self::ml_string::Text::from_units(vec![
                                    114, 101, 102, 101, 114, 101, 110, 99, 101,
                                ])),
                            );
                            if (((reference >= 0f64)
                                && ((self::dependency_function2(forms, reference))
                                    == (self::ml_string::Text::from_units(vec![
                                        98, 111, 111, 108, 101, 97, 110,
                                    ]))))
                                && ((self::dependency_function2(texts, reference))
                                    == (self::ml_string::Text::from_units(vec![
                                        116, 114, 117, 101,
                                    ]))))
                                && self::payload(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::dependency_function3(
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        node,
                                        &(self::ml_string::Text::from_units(vec![
                                            118, 97, 108, 117, 101,
                                        ])),
                                    ),
                                    owner,
                                    0f64,
                                )
                            {
                                node
                            } else {
                                -1f64
                            }
                        }
                    } else if kind == (self::ml_string::Text::from_units(vec![118, 97, 114])) {
                        {
                            let name = self::word(
                                forms,
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                            );
                            {
                                let at = (bindings.len() as f64) - 1f64;
                                {
                                    let source_ml_source2_s18 =
                                        self::source_ml_source1_temporary_fresh_loop22(
                                            bindings,
                                            values,
                                            &(name),
                                            at,
                                        );
                                    match source_ml_source2_s18 {
                                            self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Done => {
                                                -1f64
                                            }
                                            self::MlSource1TemporaryFreshLoop22Result::MlSource1TemporaryFreshLoop22Return(source_ml_source1_temporary_result22) => {
                                                source_ml_source1_temporary_result22
                                            }
                                        }
                                }
                            }
                        }
                    } else if kind == (self::ml_string::Text::from_units(vec![108, 101, 116])) {
                        {
                            let value = self::dependency_function3(
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101])),
                            );
                            {
                                let origin = self::fresh(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    value,
                                    owner,
                                    bindings,
                                    values,
                                    (depth + 1f64),
                                );
                                self::fresh(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::dependency_function3(
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        node,
                                        &(self::ml_string::Text::from_units(vec![
                                            98, 111, 100, 121,
                                        ])),
                                    ),
                                    owner,
                                    &({
                                        let source_ml_source2_concat_value19 = bindings.to_vec();
                                        {
                                            let source_ml_source2_concat_value20 =
                                                vec![self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    node,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        110, 97, 109, 101,
                                                    ])),
                                                )];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<self::ml_string::Text>::new(),
                                                    source_ml_source2_concat_value19,
                                                ),
                                                source_ml_source2_concat_value20,
                                            )
                                        }
                                    }),
                                    &({
                                        let source_ml_source2_concat_value21 = values.to_vec();
                                        {
                                            let source_ml_source2_concat_value22 = vec![origin];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value21,
                                                ),
                                                source_ml_source2_concat_value22,
                                            )
                                        }
                                    }),
                                    (depth + 1f64),
                                )
                            }
                        }
                    } else if (kind
                        == (self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116])))
                        || (kind
                            == (self::ml_string::Text::from_units(vec![
                                99, 101, 108, 108, 87, 114, 105, 116, 101,
                            ])))
                    {
                        self::fresh(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::dependency_function3(
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![98, 111, 100, 121])),
                            ),
                            owner,
                            bindings,
                            values,
                            (depth + 1f64),
                        )
                    } else if kind == (self::ml_string::Text::from_units(vec![105, 102])) {
                        {
                            let left = self::fresh(
                                forms,
                                texts,
                                starts,
                                counts,
                                labels,
                                targets,
                                self::dependency_function3(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    node,
                                    &(self::ml_string::Text::from_units(vec![116, 104, 101, 110])),
                                ),
                                owner,
                                bindings,
                                values,
                                (depth + 1f64),
                            );
                            {
                                let right = self::fresh(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    self::dependency_function3(
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        node,
                                        &(self::ml_string::Text::from_units(vec![
                                            101, 108, 115, 101,
                                        ])),
                                    ),
                                    owner,
                                    bindings,
                                    values,
                                    (depth + 1f64),
                                );
                                if (left >= 0f64) && (left == right) {
                                    left
                                } else {
                                    -1f64
                                }
                            }
                        }
                    } else if kind
                        == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))
                    {
                        {
                            let origin_2 = -2f64;
                            {
                                let items = self::dependency_function3(
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    node,
                                    &(self::ml_string::Text::from_units(vec![
                                        105, 116, 101, 109, 115,
                                    ])),
                                );
                                if items < 0f64 {
                                    -1f64
                                } else {
                                    {
                                        let at_2 = 0f64;
                                        {
                                            let source_ml_source2_s23 =
                                                self::source_ml_source1_temporary_fresh_loop23(
                                                    forms, texts, starts, counts, labels, targets,
                                                    owner, bindings, values, depth, origin_2,
                                                    items, at_2,
                                                );
                                            match source_ml_source2_s23 {
                                                                    self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Done(origin_3) => {
                                                                        origin_3
                                                                    }
                                                                    self::MlSource1TemporaryFreshLoop23Result::MlSource1TemporaryFreshLoop23Return(source_ml_source1_temporary_result23) => {
                                                                        source_ml_source1_temporary_result23
                                                                    }
                                                                }
                                        }
                                    }
                                }
                            }
                        }
                    } else if kind
                        == (self::ml_string::Text::from_units(vec![
                            116, 117, 112, 108, 101, 73, 110, 100, 101, 120,
                        ]))
                    {
                        self::fresh(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::dependency_function3(
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![
                                    111, 98, 106, 101, 99, 116,
                                ])),
                            ),
                            owner,
                            bindings,
                            values,
                            (depth + 1f64),
                        )
                    } else {
                        -1f64
                    }
                }
            } else {
                -2f64
            }
        }
    }
}
