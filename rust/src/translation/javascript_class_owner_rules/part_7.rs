// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Result;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::dependency_function4;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_closed_class_observation_operation_operands;
use super::read_record_field_write_name;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop27;
use super::word;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop26(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    representations: &[self::ml_string::Text],
    mut node: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Result {
    loop {
        return if node < (starts.len() as f64) {
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
                {
                    let proof = self::word(
                        forms,
                        texts,
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(self::ml_string::Text::from_units(vec![112])),
                    );
                    {
                        let operation = self::word(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            node,
                            &(self::ml_string::Text::from_units(vec![111, 112])),
                        );
                        {
                            let reference = self::dependency_function3(
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![
                                    114, 101, 102, 101, 114, 101, 110, 99, 101,
                                ])),
                            );
                            {
                                let identity = ((kind
                                    == (self::ml_string::Text::from_units(vec![
                                        115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108,
                                    ])))
                                    || ((kind
                                        == (self::ml_string::Text::from_units(vec![
                                            98, 105, 110, 97, 114, 121,
                                        ])))
                                        && self::ml_array::includes(
                                            &vec![
                                                self::ml_string::Text::from_units(vec![101, 113]),
                                                self::ml_string::Text::from_units(vec![110, 101]),
                                            ],
                                            &operation.clone(),
                                            0f64,
                                            false,
                                            |a, b| a == b,
                                        )))
                                    || ((self::ml_array::includes(
                                        &vec![
                                            self::ml_string::Text::from_units(vec![101, 113]),
                                            self::ml_string::Text::from_units(vec![110, 101]),
                                        ],
                                        &proof.clone(),
                                        0f64,
                                        false,
                                        |a, b| a == b,
                                    ) && (reference >= 0f64))
                                        && ((self::dependency_function2(texts, reference))
                                            == (self::ml_string::Text::from_units(vec![
                                                116, 114, 117, 101,
                                            ]))));
                                {
                                    let source_ml_source1_temporary_values20 = {
                                        let source_ml_source2_string_unit_operand60 = self::read_java_script_closed_class_observation_operation_operands(&(kind), &(proof), &(operation));
                                        {
                                            let source_ml_source2_string_unit_operand61 =
                                                self::ml_string::Text::from_units(vec![32]);
                                            {
                                                let source_ml_source2_string_split_limit62 =
                                                    4_294_967_295f64;
                                                {
                                                    let source_ml_source2_string_split_separator63 =
                                                        source_ml_source2_string_unit_operand61;
                                                    (|ml_text:&self::ml_string::Text,ml_separator:Option<&self::ml_string::Text>,ml_limit:f64|{self::ml_utf16_text::split_utf16_text(&(ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()), &(ml_separator.map(|ml_value|ml_value.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()).unwrap_or_default()), ml_limit, ml_separator.is_none()).into_iter().map(|ml_units|self::ml_string::Text::from_units(ml_units.into_iter().map(|ml_unit|ml_unit as u16).collect())).collect::<Vec<_>>()})(&(source_ml_source2_string_unit_operand60), Some(&source_ml_source2_string_split_separator63), source_ml_source2_string_split_limit62)
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_temporary_index20 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s64 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop27(forms, texts, starts, counts, labels, targets, declarations, constructors, brands, representations, node, identity, &(source_ml_source1_temporary_values20), source_ml_source1_temporary_index20);
                                            match source_ml_source2_s64 {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Done => {
                                                    {
                                                        let node_2 = node + 1f64;
                                                        {
                                                            node = node_2;
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop27Return(source_ml_source1_temporary_result27) => {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Return(source_ml_source1_temporary_result27.clone())
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop26Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop35(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut pending: Vec<f64>,
    mut node: f64,
    kind: &self::ml_string::Text,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at
            < (self::dependency_function1(starts, node) + self::dependency_function1(counts, node))
        {
            if !(self::ml_array::includes(
                &vec![
                    self::ml_string::Text::from_units(vec![116, 121, 112, 101]),
                    self::ml_string::Text::from_units(vec![100, 111, 109, 97, 105, 110]),
                    self::ml_string::Text::from_units(vec![115, 112, 97, 110]),
                    self::ml_string::Text::from_units(vec![
                        109, 111, 100, 117, 108, 101, 80, 97, 116, 104,
                    ]),
                ],
                &self::dependency_function2(labels, at),
                0f64,
                false,
                |a, b| a == b,
            )) {
                {
                    let pending_2 = {
                        let source_ml_source2_concat_value65 = pending;
                        {
                            let source_ml_source2_concat_value66 = vec![
                                self::dependency_function1(targets, at),
                                if ((*kind)
                                    == (self::ml_string::Text::from_units(vec![
                                        97, 112, 112, 108, 121,
                                    ])))
                                    && ((self::dependency_function2(labels, at))
                                        == (self::ml_string::Text::from_units(vec![102, 110])))
                                {
                                    1f64
                                } else {
                                    0f64
                                },
                            ];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value65,
                                ),
                                source_ml_source2_concat_value66,
                            )
                        }
                    };
                    {
                        let at_2 = at + 1f64;
                        {
                            (pending, at) = (pending_2, at_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let at_3 = at + 1f64;
                    {
                        at = at_3;
                        continue;
                    }
                }
            }
        }
        return { pending };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34 {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34Next,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop36(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    constructors: &[self::ml_string::Text],
    representations: &[self::ml_string::Text],
    owner: &self::ml_string::Text,
    mut direct: f64,
    mut node: f64,
    kind: &self::ml_string::Text,
    mut object: f64,
    mut instance: f64,
    name: &self::ml_string::Text,
    mut methods: f64,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result {
    loop {
        return if at < (self::dependency_function1(counts, methods)) {
            {
                let method = self::dependency_function4(starts, counts, targets, methods, at);
                if (self::word(
                    forms,
                    texts,
                    starts,
                    counts,
                    labels,
                    targets,
                    method,
                    &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                )) != (*name)
                {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
                {
                    if ((*kind)
                        == (self::ml_string::Text::from_units(vec![
                            114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100,
                        ])))
                        && (((self::dependency_function2(representations, instance))
                            != (self::ml_string::Text::from_units(vec![
                                99, 108, 111, 115, 117, 114, 101, 32, 115, 108, 111, 116, 115,
                            ])))
                            || (direct != 1f64))
                    {
                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 114, 101, 99, 101, 105, 118, 101, 114, 45, 114, 101, 116, 97, 105, 110, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 100, 101, 116, 97, 99, 104, 101, 100, 32, 115, 108, 111, 116, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 115]))
                    } else if (*kind)
                        == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]))
                    {
                        {
                            let args = self::dependency_function3(
                                starts,
                                counts,
                                labels,
                                targets,
                                node,
                                &(self::ml_string::Text::from_units(vec![97, 114, 103, 115])),
                            );
                            {
                                let value = if args < 0f64 {
                                    -1f64
                                } else {
                                    self::dependency_function4(starts, counts, targets, args, 0f64)
                                };
                                {
                                    let helper = self::word(
                                        forms,
                                        texts,
                                        starts,
                                        counts,
                                        labels,
                                        targets,
                                        method,
                                        &(self::ml_string::Text::from_units(vec![
                                            104, 101, 108, 112, 101, 114,
                                        ])),
                                    );
                                    {
                                        let call_args = self::dependency_function3(
                                            starts,
                                            counts,
                                            labels,
                                            targets,
                                            value,
                                            &(self::ml_string::Text::from_units(vec![
                                                97, 114, 103, 115,
                                            ])),
                                        );
                                        {
                                            let receiver = if call_args < 0f64 {
                                                -1f64
                                            } else {
                                                self::dependency_function4(
                                                    starts, counts, targets, call_args, 0f64,
                                                )
                                            };
                                            if (((((((*owner)
                                                != (self::dependency_function2(
                                                    constructors,
                                                    instance,
                                                )))
                                                || ((self::dependency_function2(
                                                    representations,
                                                    instance,
                                                )) != (self::ml_string::Text::from_units(
                                                    vec![
                                                        99, 108, 111, 115, 117, 114, 101, 32, 115,
                                                        108, 111, 116, 115,
                                                    ],
                                                ))))
                                                || ((self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    value,
                                                    &(self::ml_string::Text::from_units(vec![107])),
                                                )) != (self::ml_string::Text::from_units(
                                                    vec![99, 97, 108, 108],
                                                ))))
                                                || ((self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    value,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        102, 110,
                                                    ])),
                                                )) != helper))
                                                || ((self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    object,
                                                    &(self::ml_string::Text::from_units(vec![107])),
                                                )) != (self::ml_string::Text::from_units(
                                                    vec![118, 97, 114],
                                                ))))
                                                || ((self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    receiver,
                                                    &(self::ml_string::Text::from_units(vec![107])),
                                                )) != (self::ml_string::Text::from_units(
                                                    vec![118, 97, 114],
                                                ))))
                                                || ((self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    object,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        110, 97, 109, 101,
                                                    ])),
                                                )) != (self::word(
                                                    forms,
                                                    texts,
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    receiver,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        110, 97, 109, 101,
                                                    ])),
                                                )))
                                            {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 32, 115, 108, 111, 116, 115, 32, 97, 114, 101, 32, 105, 110, 115, 116, 97, 108, 108, 101, 100, 32, 111, 110, 108, 121, 32, 98, 121, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 102, 97, 99, 116, 111, 114, 121, 32, 105, 110, 115, 105, 100, 101, 32, 116, 104, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                            } else {
                                                {
                                                    let at_3 = at + 1f64;
                                                    {
                                                        at = at_3;
                                                        continue;
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
                            let at_4 = at + 1f64;
                            {
                                at = at_4;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop33(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    representations: &[self::ml_string::Text],
    owner_nodes: &[f64],
    owner: &self::ml_string::Text,
    mut pending: Vec<f64>,
    mut seen: Vec<f64>,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Result {
    loop {
        return {
            let ml_library_condition_1 = {
                let ml_value = pending.len() as f64;
                ml_value != 0f64 && !ml_value.is_nan()
            };
            if ml_library_condition_1 {
                {
                    let direct =
                        self::dependency_function1(&(pending), ((pending.len() as f64) - 1f64));
                    {
                        let node =
                            self::dependency_function1(&(pending), ((pending.len() as f64) - 2f64));
                        {
                            let pending_2 = self::ml_array::slice(&pending, 0f64, -2f64);
                            if (node < 0f64)
                                || self::ml_array::includes(
                                    &seen,
                                    &((node * 2f64) + direct),
                                    0f64,
                                    false,
                                    |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                                )
                            {
                                {
                                    pending = pending_2;
                                    continue;
                                }
                            }
                            {
                                if (seen.len() as f64) > 100_000f64 {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Return(self::ml_string::Text::from_units(vec![111, 119, 110, 101, 100, 32, 99, 108, 97, 115, 115, 32, 98, 111, 100, 121, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 98, 117, 100, 103, 101, 116]))
                                } else {
                                    {
                                        let seen_2 = {
                                            let source_ml_source2_concat_value67 = seen;
                                            {
                                                let source_ml_source2_concat_value68 =
                                                    vec![((node * 2f64) + direct)];
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source2_concat_value67,
                                                    ),
                                                    source_ml_source2_concat_value68,
                                                )
                                            }
                                        };
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
                                            {
                                                let type_ = self::dependency_function3(
                                                    starts,
                                                    counts,
                                                    labels,
                                                    targets,
                                                    node,
                                                    &(self::ml_string::Text::from_units(vec![
                                                        116, 121, 112, 101,
                                                    ])),
                                                );
                                                {
                                                    let class_owner = self::word(
                                                        forms,
                                                        texts,
                                                        starts,
                                                        counts,
                                                        labels,
                                                        targets,
                                                        type_,
                                                        &(self::ml_string::Text::from_units(vec![
                                                            115, 111, 117, 114, 99, 101, 67, 108,
                                                            97, 115, 115, 79, 119, 110, 101, 114,
                                                        ])),
                                                    );
                                                    if ((kind
                                                        == (self::ml_string::Text::from_units(
                                                            vec![114, 101, 99, 111, 114, 100],
                                                        )))
                                                        && (class_owner
                                                            != (self::ml_string::Text::from_units(
                                                                vec![],
                                                            ))))
                                                        && (class_owner != (*owner))
                                                    {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 100, 97, 116, 97, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 115, 32, 98, 101, 108, 111, 110, 103, 32, 111, 110, 108, 121, 32, 116, 111, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
                                                    } else {
                                                        {
                                                            let object = self::dependency_function3(starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116])));
                                                            {
                                                                let object_type = self::dependency_function3(starts, counts, labels, targets, object, &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])));
                                                                {
                                                                    let instance = self::ml_array::index_of(constructors, &self::word(forms, texts, starts, counts, labels, targets, object_type, &(self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 67, 108, 97, 115, 115, 79, 119, 110, 101, 114]))), 0f64, false, |a, b| a == b);
                                                                    {
                                                                        let source_ml_source2_s70 =
                                                                            if instance >= 0f64 {
                                                                                {
                                                                                    let definition = self::dependency_function1(owner_nodes, instance);
                                                                                    {
                                                                                        let brand = self::dependency_function2(brands, instance);
                                                                                        {
                                                                                            let name = if kind == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100])) {
                                                                                        self::word(forms, texts, starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])))
                                                                                    } else if kind == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])) {
                                                                                            self::read_record_field_write_name(&(self::word(forms, texts, starts, counts, labels, targets, node, &(self::ml_string::Text::from_units(vec![111, 112])))))
                                                                                        } else {
                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                        };
                                                                                            if name == brand {
                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34Return(self::ml_string::Text::from_units(vec![97, 32, 115, 121, 110, 116, 104, 101, 116, 105, 99, 32, 99, 108, 97, 115, 115, 32, 98, 114, 97, 110, 100, 32, 105, 115, 32, 110, 111, 116, 32, 97, 110, 32, 111, 98, 115, 101, 114, 118, 97, 98, 108, 101, 32, 112, 117, 98, 108, 105, 99, 32, 100, 97, 116, 97, 32, 102, 105, 101, 108, 100]))
                                                                                    } else {
                                                                                        {
                                                                                            let methods = self::dependency_function3(starts, counts, labels, targets, definition, &(self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 115])));
                                                                                            {
                                                                                                let at = 0f64;
                                                                                                {
                                                                                                    let source_ml_source2_s71 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop36(forms, texts, starts, counts, labels, targets, constructors, representations, owner, direct, node, &(kind), object, instance, &(name), methods, at);
                                                                                                    match source_ml_source2_s71 {
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34Next
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(source_ml_source1_temporary_result36) => {
                                                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34Return(source_ml_source1_temporary_result36.clone())
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
                                                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34Next
                                                                            };
                                                                        match source_ml_source2_s70 {
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34Next => {
                                                                            {
                                                                                let at_2 = self::dependency_function1(starts, node);
                                                                                {
                                                                                    let pending_3 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop35(starts, counts, labels, targets, pending_2, node, &(kind), at_2);
                                                                                    {
                                                                                        (pending, seen) = (pending_3, seen_2);
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34::MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin34Return(source_ml_source1_temporary_result35) => {
                                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Return(source_ml_source1_temporary_result35.clone())
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
            } else {
                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Done
            }
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop25(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    constructors: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
    representations: &[self::ml_string::Text],
    owner_nodes: &[f64],
    roots: &[f64],
    scopes: &[self::ml_string::Text],
    mut scope: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Result {
    loop {
        return if scope < (roots.len() as f64) {
            {
                let owner = self::dependency_function2(scopes, scope);
                {
                    let pending = vec![self::dependency_function1(roots, scope), 0f64];
                    {
                        let seen = Vec::<f64>::new();
                        {
                            let source_ml_source2_s72 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop33(forms, texts, starts, counts, labels, targets, constructors, brands, representations, owner_nodes, &(owner), pending, seen);
                            match source_ml_source2_s72 {
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Done => {
                                    {
                                        let scope_2 = scope + 1f64;
                                        {
                                            scope = scope_2;
                                            continue;
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop33Return(source_ml_source1_temporary_result33) => {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Return(source_ml_source1_temporary_result33)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop25Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop24Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop24Done(Vec<f64>, Vec<self::ml_string::Text>),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop24(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut declarations: f64,
    mut roots: Vec<f64>,
    mut scopes: Vec<self::ml_string::Text>,
    mut declaration: f64,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop24Result {
    loop {
        if (declarations >= 0f64)
            && (declaration < (self::dependency_function1(counts, declarations)))
        {
            {
                let row =
                    self::dependency_function4(starts, counts, targets, declarations, declaration);
                {
                    let roots_2 = {
                        let source_ml_source2_concat_value73 = roots;
                        {
                            let source_ml_source2_concat_value74 = vec![row];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source2_concat_value73,
                                ),
                                source_ml_source2_concat_value74,
                            )
                        }
                    };
                    {
                        let scopes_2 = {
                            let source_ml_source2_concat_value75 = scopes;
                            {
                                let source_ml_source2_concat_value76 = vec![self::word(
                                    forms,
                                    texts,
                                    starts,
                                    counts,
                                    labels,
                                    targets,
                                    row,
                                    &(self::ml_string::Text::from_units(vec![
                                        102, 117, 108, 108, 78, 97, 109, 101,
                                    ])),
                                )];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<self::ml_string::Text>::new(),
                                        source_ml_source2_concat_value75,
                                    ),
                                    source_ml_source2_concat_value76,
                                )
                            }
                        };
                        {
                            let declaration_2 = declaration + 1f64;
                            {
                                (roots, scopes, declaration) = (roots_2, scopes_2, declaration_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop24Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop24Done(roots, scopes)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop23Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop37(
    forms: &[self::ml_string::Text],
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    mut methods: f64,
    mut complete: Vec<self::ml_string::Text>,
    mut at: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if at < (self::dependency_function1(counts, methods)) {
            {
                let complete_2 = {
                    let source_ml_source2_concat_value77 = complete;
                    {
                        let source_ml_source2_concat_value78 = vec![self::word(
                            forms,
                            texts,
                            starts,
                            counts,
                            labels,
                            targets,
                            self::dependency_function4(starts, counts, targets, methods, at),
                            &(self::ml_string::Text::from_units(vec![110, 97, 109, 101])),
                        )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source2_concat_value77,
                            ),
                            source_ml_source2_concat_value78,
                        )
                    }
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (complete, at) = (complete_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { complete };
    }
}
