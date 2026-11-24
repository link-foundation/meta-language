// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryFreshLoop22Result;
use super::MlSource1TemporaryFreshLoop23Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result;
use super::accept_linked_java_script_source_exception;
use super::carries;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::exact;
use super::ml_array;
use super::ml_string;
use super::read_java_script_class_owner_model;
use super::source_ml_source1_temporary_fresh_loop22;
use super::source_ml_source1_temporary_fresh_loop23;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop28;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop29;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop30;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop31;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop46;
use super::word;

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
            if (kind == (self::ml_string::Text::from_units(vec![97, 98, 111, 114, 116])))
                || self::accept_linked_java_script_source_exception(
                    &(kind),
                    &(self::word(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(self::ml_string::Text::from_units(vec![111, 112])),
                    )),
                )
            {
                -3f64
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
                        if kind
                            == (self::ml_string::Text::from_units(vec![
                                114, 101, 102, 101, 114, 101, 110, 99, 101, 65, 108, 108, 111, 99,
                                97, 116, 101,
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
                                        let source_ml_source2_s34 =
                                            self::source_ml_source1_temporary_fresh_loop22(
                                                bindings,
                                                values,
                                                &(name),
                                                at,
                                            );
                                        match source_ml_source2_s34 {
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
                                    &(self::ml_string::Text::from_units(vec![
                                        118, 97, 108, 117, 101,
                                    ])),
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
                                    if origin == -3f64 {
                                        -3f64
                                    } else {
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
                                                let source_ml_source2_concat_value35 =
                                                    bindings.to_vec();
                                                {
                                                    let source_ml_source2_concat_value36 =
                                                        vec![self::word(
                                                            forms,
                                                            texts,
                                                            starts,
                                                            counts,
                                                            labels,
                                                            targets,
                                                            node,
                                                            &(self::ml_string::Text::from_units(
                                                                vec![110, 97, 109, 101],
                                                            )),
                                                        )];
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<self::ml_string::Text>::new(),
                                                            source_ml_source2_concat_value35,
                                                        ),
                                                        source_ml_source2_concat_value36,
                                                    )
                                                }
                                            }),
                                            &({
                                                let source_ml_source2_concat_value37 =
                                                    values.to_vec();
                                                {
                                                    let source_ml_source2_concat_value38 =
                                                        vec![origin];
                                                    self::ml_array::append(
                                                        self::ml_array::append(
                                                            Vec::<f64>::new(),
                                                            source_ml_source2_concat_value37,
                                                        ),
                                                        source_ml_source2_concat_value38,
                                                    )
                                                }
                                            }),
                                            (depth + 1f64),
                                        )
                                    }
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
                                        &(self::ml_string::Text::from_units(vec![
                                            116, 104, 101, 110,
                                        ])),
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
                                    if left == -3f64 {
                                        right
                                    } else if (right == -3f64)
                                        || ((left >= 0f64) && (left == right))
                                    {
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
                                                let source_ml_source2_s39 =
                                                    self::source_ml_source1_temporary_fresh_loop23(
                                                        forms, texts, starts, counts, labels,
                                                        targets, owner, bindings, values, depth,
                                                        origin_2, items, at_2,
                                                    );
                                                match source_ml_source2_s39 {
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
                    } else {
                        -2f64
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
                                        let source_ml_source2_s41 = if witness >= 0f64 {
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
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(self::ml_string::Text::from_units(vec![117, 110, 114, 101, 99, 111, 103, 110, 105, 122, 101, 100, 32, 111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 119, 105, 116, 110, 101, 115, 115, 32, 104, 101, 97, 100, 101, 114]))
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
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(self::ml_string::Text::from_units(vec![99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 111, 119, 110, 101, 114, 115, 32, 97, 110, 100, 32, 100, 101, 99, 108, 97, 114, 97, 116, 105, 111, 110, 115, 32, 97, 114, 101, 32, 114, 101, 113, 117, 105, 114, 101, 100]))
                                                    } else {
                                                        {
                                                            let at = 0f64;
                                                            {
                                                                let source_ml_source2_s42 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop46(forms, texts, starts, counts, labels, targets, declarations, constructors, brands, representations, owner_nodes, constructor_nodes, owners, at);
                                                                match source_ml_source2_s42 {
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Done(constructors_2, brands_2, representations_2, owner_nodes_2, constructor_nodes_2) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors_2, brands_2, representations_2, owner_nodes_2, constructor_nodes_2)
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop46Return(source_ml_source1_temporary_result46) => {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(source_ml_source1_temporary_result46)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors, brands, representations, owner_nodes, constructor_nodes)
                                        };
                                        match source_ml_source2_s41 {
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Next(constructors_3, brands_3, representations_3, owner_nodes_3, constructor_nodes_3) => {
                                                {
                                                    let node = 0f64;
                                                    {
                                                        let source_ml_source2_s43 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop28(forms, texts, starts, counts, labels, targets, type_keys, &(constructors_3), &(brands_3), &(representations_3), &(owner_nodes_3), &(constructor_nodes_3), node);
                                                        match source_ml_source2_s43 {
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Done => {
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
                                                                                    let source_ml_source2_s44 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop29(forms, texts, starts, counts, labels, targets, declarations, roots, scopes, declaration);
                                                                                    match source_ml_source2_s44 {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop29Done(roots_2, scopes_2) => {
                                                                                            {
                                                                                                let scope = 0f64;
                                                                                                {
                                                                                                    let source_ml_source2_s45 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop30(forms, texts, starts, counts, labels, targets, &(constructors_3), &(brands_3), &(representations_3), &(owner_nodes_3), &(roots_2), &(scopes_2), scope);
                                                                                                    match source_ml_source2_s45 {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done => {
                                                                                                            {
                                                                                                                let node_2 = 0f64;
                                                                                                                {
                                                                                                                    let source_ml_source2_s46 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop31(forms, texts, starts, counts, labels, targets, declarations, &(constructors_3), &(brands_3), &(representations_3), node_2);
                                                                                                                    match source_ml_source2_s46 {
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done => {
                                                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                                                        }
                                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(source_ml_source1_temporary_result31) => {
                                                                                                                            source_ml_source1_temporary_result31
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(source_ml_source1_temporary_result37) => {
                                                                                                            source_ml_source1_temporary_result37
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
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop28Return(source_ml_source1_temporary_result41) => {
                                                                source_ml_source1_temporary_result41
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin27Return(source_ml_source1_temporary_result45) => {
                                                source_ml_source1_temporary_result45
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_dependency_function3_loop1(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    field: &self::ml_string::Text,
    mut start: f64,
    mut count: f64,
    mut at: f64,
) -> self::MlSource1TemporaryDependencyFunction3Loop1Result {
    loop {
        return if at < (start + count) {
            if (self::dependency_function2(labels, at)) == (*field) {
                self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Return(self::dependency_function1(targets, at))
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
            self::MlSource1TemporaryDependencyFunction3Loop1Result::MlSource1TemporaryDependencyFunction3Loop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done,
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Result {
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Done(Vec<f64>),
    MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop3Return(bool),
}
