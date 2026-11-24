// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result;
use super::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::dependency_function4;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_closed_class_observation_operation_operands;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop33;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop35;
use super::source_ml_source1_temporary_read_java_script_class_owner_error_loop36;
use super::word;

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop34(
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
    mut identity: bool,
    source_ml_source1_temporary_values26: &[f64],
    mut source_ml_source1_temporary_index26: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result {
    loop {
        return if source_ml_source1_temporary_index26
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values26.len() as u128))
        {
            {
                let value = self::ml_array::at(
                    source_ml_source1_temporary_values26,
                    source_ml_source1_temporary_index26.to_index(),
                );
                {
                    let type_ = self::dependency_function3(
                        starts,
                        counts,
                        labels,
                        targets,
                        value,
                        &(self::ml_string::Text::from_units(vec![116, 121, 112, 101])),
                    );
                    if type_ < 0f64 {
                        {
                            let source_ml_source1_temporary_index26_2 =
                                source_ml_source1_temporary_index26
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index26 =
                                    source_ml_source1_temporary_index26_2;
                                continue;
                            }
                        }
                    }
                    {
                        {
                            let owner = 0f64;
                            {
                                let source_ml_source2_s76 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop35(texts, starts, counts, labels, targets, declarations, constructors, brands, representations, identity, type_, owner);
                                match source_ml_source2_s76 {
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Done => {
                                        {
                                            let pending = vec![type_];
                                            {
                                                let seen = Vec::<f64>::new();
                                                {
                                                    let source_ml_source2_s77 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop36(forms, texts, starts, counts, labels, targets, constructors, representations, identity, pending, seen);
                                                    match source_ml_source2_s77 {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Done => {
                                                            {
                                                                let source_ml_source1_temporary_index26_3 = source_ml_source1_temporary_index26.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index26 = source_ml_source1_temporary_index26_3;
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop36Return(source_ml_source1_temporary_result36) => {
                                                            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(source_ml_source1_temporary_result36)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop35Return(source_ml_source1_temporary_result37) => {
                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(source_ml_source1_temporary_result37)
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Done
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop32(
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
    mut identity: bool,
    source_ml_source1_temporary_values25: &[self::ml_string::Text],
    mut source_ml_source1_temporary_index25: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result {
    loop {
        return if source_ml_source1_temporary_index25
            < (self::ml::Big::from_u128(source_ml_source1_temporary_values25.len() as u128))
        {
            {
                let key = self::ml_array::at(
                    source_ml_source1_temporary_values25,
                    source_ml_source1_temporary_index25.to_index(),
                );
                if key.0.is_empty() {
                    {
                        let source_ml_source1_temporary_index25_4 =
                            source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index25 =
                                source_ml_source1_temporary_index25_4;
                            continue;
                        }
                    }
                }
                {
                    {
                        let operand = self::dependency_function3(
                            starts,
                            counts,
                            labels,
                            targets,
                            node,
                            &(key),
                        );
                        if operand < 0f64 {
                            {
                                let source_ml_source1_temporary_index25_2 =
                                    source_ml_source1_temporary_index25
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index25 =
                                        source_ml_source1_temporary_index25_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let values = if (self::dependency_function2(forms, operand))
                                    == (self::ml_string::Text::from_units(vec![
                                        97, 114, 114, 97, 121,
                                    ])) {
                                    Vec::<f64>::new()
                                } else {
                                    vec![operand]
                                };
                                {
                                    let values_3 = if (self::dependency_function2(forms, operand))
                                        == (self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121,
                                        ])) {
                                        {
                                            let at = 0f64;
                                            {
                                                self::source_ml_source1_temporary_read_java_script_class_owner_error_loop33(starts, counts, targets, operand, values, at)
                                            }
                                        }
                                    } else {
                                        values
                                    };
                                    {
                                        let source_ml_source1_temporary_values26 = values_3;
                                        {
                                            let source_ml_source1_temporary_index26 =
                                                self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source2_s78 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop34(forms, texts, starts, counts, labels, targets, declarations, constructors, brands, representations, identity, &(source_ml_source1_temporary_values26), source_ml_source1_temporary_index26);
                                                match source_ml_source2_s78 {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Done => {
                                                        {
                                                            let source_ml_source1_temporary_index25_3 = source_ml_source1_temporary_index25.add(&self::ml::Big::from_i128(1));
                                                            {
                                                                source_ml_source1_temporary_index25 = source_ml_source1_temporary_index25_3;
                                                                continue;
                                                            }
                                                        }
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop34Return(source_ml_source1_temporary_result34) => {
                                                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Return(source_ml_source1_temporary_result34)
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Done
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
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop31(
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
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result {
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
                                    let source_ml_source1_temporary_values25 = {
                                        let source_ml_source2_string_unit_operand79 = self::read_java_script_closed_class_observation_operation_operands(&(kind), &(proof), &(operation));
                                        {
                                            let source_ml_source2_string_unit_operand80 =
                                                self::ml_string::Text::from_units(vec![32]);
                                            {
                                                let source_ml_source2_string_split_limit81 =
                                                    4_294_967_295f64;
                                                {
                                                    let source_ml_source2_string_split_separator82 =
                                                        source_ml_source2_string_unit_operand80;
                                                    (|ml_text:&self::ml_string::Text,ml_separator:Option<&self::ml_string::Text>,ml_limit:f64|{self::ml_utf16_text::split_utf16_text(&(ml_text.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()), &(ml_separator.map(|ml_value|ml_value.0.iter().map(|ml_unit|f64::from(*ml_unit)).collect::<Vec<f64>>()).unwrap_or_default()), ml_limit, ml_separator.is_none()).into_iter().map(|ml_units|self::ml_string::Text::from_units(ml_units.into_iter().map(|ml_unit|ml_unit as u16).collect())).collect::<Vec<_>>()})(&(source_ml_source2_string_unit_operand79), Some(&source_ml_source2_string_split_separator82), source_ml_source2_string_split_limit81)
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_temporary_index25 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s83 = self::source_ml_source1_temporary_read_java_script_class_owner_error_loop32(forms, texts, starts, counts, labels, targets, declarations, constructors, brands, representations, node, identity, &(source_ml_source1_temporary_values25), source_ml_source1_temporary_index25);
                                            match source_ml_source2_s83 {
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Done => {
                                                    {
                                                        let node_2 = node + 1f64;
                                                        {
                                                            node = node_2;
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop32Return(source_ml_source1_temporary_result32) => {
                                                    self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Return(source_ml_source1_temporary_result32.clone())
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop31Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop30Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop38Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop40(
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
                        let source_ml_source2_concat_value84 = pending;
                        {
                            let source_ml_source2_concat_value85 = vec![
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
                                    source_ml_source2_concat_value84,
                                ),
                                source_ml_source2_concat_value85,
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
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39 {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Next,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorJoin39Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result {
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done,
    MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn source_ml_source1_temporary_read_java_script_class_owner_error_loop41(
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
) -> self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result {
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
                        self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 114, 101, 99, 101, 105, 118, 101, 114, 45, 114, 101, 116, 97, 105, 110, 105, 110, 103, 32, 99, 97, 108, 108, 115, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 100, 101, 116, 97, 99, 104, 101, 100, 32, 115, 108, 111, 116, 32, 111, 98, 115, 101, 114, 118, 97, 116, 105, 111, 110, 115]))
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
                                                self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Return(self::ml_string::Text::from_units(vec![99, 108, 97, 115, 115, 32, 109, 101, 116, 104, 111, 100, 32, 115, 108, 111, 116, 115, 32, 97, 114, 101, 32, 105, 110, 115, 116, 97, 108, 108, 101, 100, 32, 111, 110, 108, 121, 32, 98, 121, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 102, 97, 99, 116, 111, 114, 121, 32, 105, 110, 115, 105, 100, 101, 32, 116, 104, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114]))
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
            self::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Result::MlSource1TemporaryReadJavaScriptClassOwnerErrorLoop41Done
        };
    }
}
