// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryCarriesLoop16Result;
use super::MlSource1TemporaryFreshLoop17Result;
use super::MlSource1TemporaryFreshLoop18Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop24Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::exact;
use super::ml_array;
use super::ml_string;
use super::read_java_script_class_owner_model;
use super::source_ml_source1_temporary_carries_loop16;
use super::source_ml_source1_temporary_fresh_loop17;
use super::source_ml_source1_temporary_fresh_loop18;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop23;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop24;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop25;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop26;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop41;
use super::word;

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
                            let source_ml_source2_s16 =
                                self::source_ml_source1_temporary_carries_loop16(
                                    forms, texts, starts, counts, labels, targets, owner, depth,
                                    elements, at,
                                );
                            match source_ml_source2_s16 {
                                    self::MlSource1TemporaryCarriesLoop16Result::MlSource1TemporaryCarriesLoop16Done => {
                                        false
                                    }
                                    self::MlSource1TemporaryCarriesLoop16Result::MlSource1TemporaryCarriesLoop16Return(source_ml_source1_temporary_result16) => {
                                        source_ml_source1_temporary_result16
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
                                    let source_ml_source2_s17 =
                                        self::source_ml_source1_temporary_fresh_loop17(
                                            bindings,
                                            values,
                                            &(name),
                                            at,
                                        );
                                    match source_ml_source2_s17 {
                                            self::MlSource1TemporaryFreshLoop17Result::MlSource1TemporaryFreshLoop17Done => {
                                                -1f64
                                            }
                                            self::MlSource1TemporaryFreshLoop17Result::MlSource1TemporaryFreshLoop17Return(source_ml_source1_temporary_result17) => {
                                                source_ml_source1_temporary_result17
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
                                        let source_ml_source2_concat_value18 = bindings.to_vec();
                                        {
                                            let source_ml_source2_concat_value19 =
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
                                                    source_ml_source2_concat_value18,
                                                ),
                                                source_ml_source2_concat_value19,
                                            )
                                        }
                                    }),
                                    &({
                                        let source_ml_source2_concat_value20 = values.to_vec();
                                        {
                                            let source_ml_source2_concat_value21 = vec![origin];
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    Vec::<f64>::new(),
                                                    source_ml_source2_concat_value20,
                                                ),
                                                source_ml_source2_concat_value21,
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
                                            let source_ml_source2_s22 =
                                                self::source_ml_source1_temporary_fresh_loop18(
                                                    forms, texts, starts, counts, labels, targets,
                                                    owner, bindings, values, depth, origin_2,
                                                    items, at_2,
                                                );
                                            match source_ml_source2_s22 {
                                                                    self::MlSource1TemporaryFreshLoop18Result::MlSource1TemporaryFreshLoop18Done(origin_3) => {
                                                                        origin_3
                                                                    }
                                                                    self::MlSource1TemporaryFreshLoop18Result::MlSource1TemporaryFreshLoop18Return(source_ml_source1_temporary_result18) => {
                                                                        source_ml_source1_temporary_result18
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

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_class_owner_error(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    type_keys: &[self::ml_string::Text],
    root: f64,
) -> self::ml_string::Text {
    if ((((((starts.len() as f64) > 1_048_576f64)
        || ((starts.len() as f64) != (forms.len() as f64)))
        || ((starts.len() as f64) != (texts.len() as f64)))
        || ((starts.len() as f64) != (counts.len() as f64)))
        || ((starts.len() as f64) != (type_keys.len() as f64)))
        || ((labels.len() as f64) != (targets.len() as f64))
    {
        self::ml_string::Text::from_units(vec![
            99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 104, 105, 112, 32, 101, 120,
            99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101,
            32, 103, 114, 97, 112, 104, 32, 98, 117, 100, 103, 101, 116,
        ])
    } else {
        {
            let witness = self::dependency_function3(
                starts,
                counts,
                labels,
                targets,
                root,
                &(self::ml_string::Text::from_units(vec![
                    115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114, 115,
                ])),
            );
            {
                let declarations = self::dependency_function3(
                    starts,
                    counts,
                    labels,
                    targets,
                    root,
                    &(self::ml_string::Text::from_units(vec![
                        100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115,
                    ])),
                );
                {
                    let constructors = Vec::<self::ml_string::Text>::new();
                    {
                        let brands = Vec::<self::ml_string::Text>::new();
                        {
                            let representations = Vec::<self::ml_string::Text>::new();
                            {
                                let owner_nodes = Vec::<f64>::new();
                                {
                                    let constructor_nodes = Vec::<f64>::new();
                                    {
                                        let source_ml_source2_s24 = if witness >= 0f64 {
                                            if (((((((self::dependency_function2(
                                                forms, witness,
                                            )) != (self::ml_string::Text::from_units(
                                                vec![111, 98, 106, 101, 99, 116],
                                            ))) || !(self::exact(
                                                starts,
                                                counts,
                                                labels,
                                                targets,
                                                witness,
                                                &(vec![
                                                    self::ml_string::Text::from_units(vec![
                                                        115, 99, 104, 101, 109, 97, 86, 101, 114,
                                                        115, 105, 111, 110,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        108, 97, 110, 103, 117, 97, 103, 101,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        109, 111, 100, 101, 108,
                                                    ]),
                                                    self::ml_string::Text::from_units(vec![
                                                        111, 119, 110, 101, 114, 115,
                                                    ]),
                                                ]),
                                            ))) || ((self::dependency_function2(
                                                forms,
                                                self::dependency_function3(
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    witness,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        115, 99, 104, 101, 109, 97, 86, 101, 114,
                                                        115, 105, 111, 110,
                                                    ])),
                                                ),
                                            ))
                                                != (self::ml_string::Text::from_units(vec![
                                                    110, 117, 109, 98, 101, 114,
                                                ]))))
                                                || ((self::dependency_function2(
                                                    texts,
                                                    self::dependency_function3(
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        witness,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            115, 99, 104, 101, 109, 97, 86, 101,
                                                            114, 115, 105, 111, 110,
                                                        ])),
                                                    ),
                                                )) != (self::ml_string::Text::from_units(
                                                    vec![49],
                                                ))))
                                                || ((self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    witness,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        108, 97, 110, 103, 117, 97, 103, 101,
                                                    ])),
                                                )) != (self::ml_string::Text::from_units(
                                                    vec![
                                                        74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
                                                    ],
                                                ))))
                                                || ((self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    root,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        115, 111, 117, 114, 99, 101, 76, 97, 110,
                                                        103, 117, 97, 103, 101,
                                                    ])),
                                                )) != (self::ml_string::Text::from_units(
                                                    vec![
                                                        74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
                                                    ],
                                                ))))
                                                || ((self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    witness,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        109, 111, 100, 101, 108,
                                                    ])),
                                                ))
                                                    != (self::read_java_script_class_owner_model()))
                                            {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22Return(self::ml_string::Text::from_units(vec![117, 110, 114, 101, 99, 111, 103, 110, 105, 122, 101, 100, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 119, 105, 116, 110, 101, 115, 115, 32, 104, 101, 97, 100, 101, 114]))
                                            } else {
                                                {
                                                    let owners = self::dependency_function3(
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        witness,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            111, 119, 110, 101, 114, 115,
                                                        ])),
                                                    );
                                                    if ((((owners < 0f64)
                                                        || ((self::dependency_function2(
                                                            forms, owners,
                                                        ))
                                                            != (self::ml_string::Text::from_units(
                                                                vec![97, 114, 114, 97, 121],
                                                            ))))
                                                        || ((self::dependency_function1(
                                                            counts, owners,
                                                        )) < 1f64))
                                                        || ((self::dependency_function1(
                                                            counts, owners,
                                                        )) > 64f64))
                                                        || (declarations < 0f64)
                                                    {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22Return(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 32, 97, 110, 100, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 97, 114, 101, 32, 114, 101, 113, 117, 105, 114, 101, 100]))
                                                    } else {
                                                        {
                                                            let at = 0f64;
                                                            {
                                                                let source_ml_source2_s25 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop41(forms, texts, starts, counts, labels, targets, declarations, constructors, brands, representations, owner_nodes, constructor_nodes, owners, at);
                                                                match source_ml_source2_s25 {
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done(constructors_2, brands_2, representations_2, owner_nodes_2, constructor_nodes_2) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22Next(constructors_2, brands_2, representations_2, owner_nodes_2, constructor_nodes_2)
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(source_ml_source1_temporary_result41) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22Return(source_ml_source1_temporary_result41)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22Next(constructors, brands, representations, owner_nodes, constructor_nodes)
                                        };
                                        match source_ml_source2_s24 {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22Next(constructors_3, brands_3, representations_3, owner_nodes_3, constructor_nodes_3) => {
                                                {
                                                    let node = 0f64;
                                                    {
                                                        let source_ml_source2_s26 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop23(forms, texts, starts, counts, labels, targets, type_keys, &(constructors_3), &(brands_3), &(representations_3), &(owner_nodes_3), &(constructor_nodes_3), node);
                                                        match source_ml_source2_s26 {
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Done => {
                                                                if (constructors_3.len() as f64) == 0f64 {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                } else {
                                                                    {
                                                                        let roots = vec![self::dependency_function3(starts, counts, labels, targets, root, &(self::ml_string::Text::from_units(vec![109, 97, 105, 110])))];
                                                                        {
                                                                            let scopes = vec![self::ml_string::Text::from_units(vec![])];
                                                                            {
                                                                                let declaration = 0f64;
                                                                                {
                                                                                    let source_ml_source2_s27 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop24(forms, texts, starts, counts, labels, targets, declarations, roots, scopes, declaration);
                                                                                    match source_ml_source2_s27 {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop24Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop24Done(roots_2, scopes_2) => {
                                                                                            {
                                                                                                let scope = 0f64;
                                                                                                {
                                                                                                    let source_ml_source2_s28 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop25(forms, texts, starts, counts, labels, targets, &(constructors_3), &(brands_3), &(representations_3), &(owner_nodes_3), &(roots_2), &(scopes_2), scope);
                                                                                                    match source_ml_source2_s28 {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Done => {
                                                                                                            {
                                                                                                                let node_2 = 0f64;
                                                                                                                {
                                                                                                                    let source_ml_source2_s29 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop26(forms, texts, starts, counts, labels, targets, declarations, &(constructors_3), &(brands_3), &(representations_3), node_2);
                                                                                                                    match source_ml_source2_s29 {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Done => {
                                                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                                                        }
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Return(source_ml_source1_temporary_result26) => {
                                                                                                                            source_ml_source1_temporary_result26
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Return(source_ml_source1_temporary_result32) => {
                                                                                                            source_ml_source1_temporary_result32
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
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Return(source_ml_source1_temporary_result36) => {
                                                                source_ml_source1_temporary_result36
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin22Return(source_ml_source1_temporary_result40) => {
                                                source_ml_source1_temporary_result40
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryDependencyFunction3Loop1Result {
    MlSource1TemporaryDependencyFunction3Loop1Done,
    MlSource1TemporaryDependencyFunction3Loop1Return(f64),
}
