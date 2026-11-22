// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result;
use super::MlSource1TemporaryExactLoop13Result;
use super::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Result;
use super::MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Result;
use super::MlSource1TemporaryStringsLoop14Result;
use super::dependency_function1;
use super::dependency_function2;
use super::dependency_function3;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2;
use super::source_ml_source1_temporary_exact_loop13;
use super::source_ml_source1_temporary_names_loop15;
use super::source_ml_source1_temporary_read_java_script_class_record_owner_loop11;
use super::source_ml_source1_temporary_read_record_field_write_operator_loop9;
use super::source_ml_source1_temporary_read_record_field_write_suffix_loop10;
use super::source_ml_source1_temporary_strings_loop14;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_observation_operands(
    kind: &self::ml_string::Text,
    proof: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::ml_array::includes(
        &vec![
            self::ml_string::Text::from_units(vec![101, 113]),
            self::ml_string::Text::from_units(vec![110, 101]),
            self::ml_string::Text::from_units(vec![108, 116]),
            self::ml_string::Text::from_units(vec![108, 101]),
            self::ml_string::Text::from_units(vec![103, 116]),
            self::ml_string::Text::from_units(vec![103, 101]),
        ],
        proof,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
    } else if (*proof) == (self::ml_string::Text::from_units(vec![102, 111, 114, 97, 108, 108])) {
        self::ml_string::Text::from_units(vec![98, 105, 110, 100, 101, 114, 115])
    } else if ((*kind)
        == (self::ml_string::Text::from_units(vec![116, 111, 83, 116, 114, 105, 110, 103])))
        || ((*kind)
            == (self::ml_string::Text::from_units(vec![
                111, 112, 116, 105, 111, 110, 97, 108, 84, 101, 120, 116,
            ])))
    {
        self::ml_string::Text::from_units(vec![97, 114, 103])
    } else if (*kind)
        == (self::ml_string::Text::from_units(vec![
            115, 116, 114, 105, 99, 116, 69, 113, 117, 97, 108,
        ]))
    {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![112, 114, 105, 110, 116])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![111, 117, 116, 112, 117, 116])))
    {
        self::ml_string::Text::from_units(vec![101, 120, 112, 114])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_closed_class_observation_operation_operands(
    kind: &self::ml_string::Text,
    proof: &self::ml_string::Text,
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*kind) == (self::ml_string::Text::from_units(vec![98, 105, 110, 97, 114, 121])))
        && self::ml_array::includes(
            &vec![
                self::ml_string::Text::from_units(vec![101, 113]),
                self::ml_string::Text::from_units(vec![110, 101]),
            ],
            operation,
            0f64,
            false,
            |a, b| a == b,
        )
    {
        self::ml_string::Text::from_units(vec![108, 101, 102, 116, 32, 114, 105, 103, 104, 116])
    } else {
        self::read_java_script_closed_class_observation_operands(kind, proof)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn contains_java_script_closed_class_brand(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    root: f64,
    declarations: f64,
    brands: &[self::ml_string::Text],
) -> bool {
    if (brands.len() as f64) == 0f64 {
        false
    } else {
        {
            let pending = vec![root];
            {
                let seen = Vec::<f64>::new();
                {
                    let source_ml_source2_s2 = self::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2(texts, starts, counts, labels, targets, declarations, brands, pending, seen);
                    match source_ml_source2_s2 {
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Done => {
                            false
                        }
                        self::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Return(source_ml_source1_temporary_result2) => {
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
pub fn accept_record_field_assignment_operator(operator: &self::ml_string::Text) -> bool {
    (((((((((*operator) == (self::ml_string::Text::from_units(vec![61])))
        || ((*operator) == (self::ml_string::Text::from_units(vec![43, 61]))))
        || ((*operator) == (self::ml_string::Text::from_units(vec![45, 61]))))
        || ((*operator) == (self::ml_string::Text::from_units(vec![42, 61]))))
        || ((*operator) == (self::ml_string::Text::from_units(vec![47, 61]))))
        || ((*operator) == (self::ml_string::Text::from_units(vec![37, 61]))))
        || ((*operator) == (self::ml_string::Text::from_units(vec![42, 42, 61]))))
        || ((*operator) == (self::ml_string::Text::from_units(vec![43, 43]))))
        || ((*operator) == (self::ml_string::Text::from_units(vec![45, 45])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_write_identity(
    operator: &self::ml_string::Text,
    field: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::accept_record_field_assignment_operator(operator) {
        (((self::ml_string::Text::from_units(vec![
            119, 114, 105, 116, 101, 82, 101, 99, 111, 114, 100, 70, 105, 101, 108, 100,
        ]))
        .concat(operator))
        .concat(&self::ml_string::Text::from_units(vec![58])))
        .concat(field)
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_write_operator(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let operators = vec![
            self::ml_string::Text::from_units(vec![61]),
            self::ml_string::Text::from_units(vec![43, 61]),
            self::ml_string::Text::from_units(vec![45, 61]),
            self::ml_string::Text::from_units(vec![42, 61]),
            self::ml_string::Text::from_units(vec![47, 61]),
            self::ml_string::Text::from_units(vec![37, 61]),
            self::ml_string::Text::from_units(vec![42, 42, 61]),
            self::ml_string::Text::from_units(vec![43, 43]),
            self::ml_string::Text::from_units(vec![45, 45]),
        ];
        {
            let source_ml_source1_temporary_values8 = operators;
            {
                let source_ml_source1_temporary_index8 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s3 =
                        self::source_ml_source1_temporary_read_record_field_write_operator_loop9(
                            operation,
                            &(source_ml_source1_temporary_values8),
                            source_ml_source1_temporary_index8,
                        );
                    match source_ml_source2_s3 {
                        self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop9Return(source_ml_source1_temporary_result9) => {
                            source_ml_source1_temporary_result9
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_write_name(operation: &self::ml_string::Text) -> self::ml_string::Text {
    {
        let operator = self::read_record_field_write_operator(operation);
        if operator == (self::ml_string::Text::from_units(vec![])) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let source_ml_source2_string_range_operand4 = operation.clone();
                {
                    let source_ml_source2_string_range_operand5 = 17f64 + operator.length();
                    {
                        let source_ml_source2_string_range_position6 =
                            source_ml_source2_string_range_operand5;
                        {
                            let source_ml_source2_string_range_position7 = f64::INFINITY;
                            source_ml_source2_string_range_operand4.slice(
                                source_ml_source2_string_range_position6,
                                source_ml_source2_string_range_position7,
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_write_argument_count(operator: &self::ml_string::Text) -> f64 {
    if ((*operator) == (self::ml_string::Text::from_units(vec![43, 43])))
        || ((*operator) == (self::ml_string::Text::from_units(vec![45, 45])))
    {
        0f64
    } else {
        1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_write_suffix(
    initial: f64,
    receiver: &self::ml_string::Text,
    arguments_: &[self::ml_string::Text],
) -> f64 {
    {
        let text = (receiver.concat(&self::ml_string::Text::from_units(vec![32]))).concat(
            &self::ml_string::join(arguments_, &self::ml_string::Text::from_units(vec![32])),
        );
        {
            let suffix = initial;
            {
                self::source_ml_source1_temporary_read_record_field_write_suffix_loop10(
                    &(text),
                    suffix,
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_linked_record_field_write(
    expression: &self::ml_string::Text,
    operation: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    reference: bool,
) -> bool {
    ((((*expression) == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])))
        && ((*kind) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))))
        && reference)
        && ((self::read_record_field_write_operator(operation))
            != (self::ml_string::Text::from_units(vec![])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_write_refusal(
    operator: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    reference: bool,
    open: bool,
    field: bool,
    value_kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (self::accept_record_field_assignment_operator(operator)) {
        if (((*kind) != (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])))
            || !reference)
            || open
        {
            self::ml_string::Text::from_units(vec![
                97, 32, 102, 105, 101, 108, 100, 32, 119, 114, 105, 116, 101, 32, 110, 101, 101,
                100, 115, 32, 97, 32, 99, 108, 111, 115, 101, 100, 32, 115, 104, 97, 114, 101, 100,
                32, 114, 101, 99, 111, 114, 100, 32, 119, 105, 116, 104, 32, 101, 120, 112, 108,
                105, 99, 105, 116, 32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 115,
            ])
        } else if field {
            if ((*operator) != (self::ml_string::Text::from_units(vec![61])))
                && ((*value_kind)
                    != (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
            {
                self::ml_string::Text::from_units(vec![
                    116, 104, 105, 115, 32, 99, 111, 109, 112, 111, 117, 110, 100, 32, 102, 105,
                    101, 108, 100, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 110,
                    101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99,
                    111, 101, 114, 99, 105, 111, 110, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101,
                    32, 100, 111, 109, 97, 105, 110,
                ])
            } else {
                self::ml_string::Text::from_units(vec![])
            }
        } else {
            self::ml_string::Text::from_units(vec![
                99, 114, 101, 97, 116, 105, 110, 103, 32, 97, 32, 102, 105, 101, 108, 100, 32, 110,
                101, 101, 100, 115, 32, 97, 32, 100, 121, 110, 97, 109, 105, 99, 32, 112, 114, 111,
                112, 101, 114, 116, 121, 32, 97, 110, 100, 32, 100, 101, 115, 99, 114, 105, 112,
                116, 111, 114, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116, 97, 116, 105, 111,
                110,
            ])
        }
    } else {
        self::ml_string::Text::from_units(vec![
            116, 104, 105, 115, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 111, 112,
            101, 114, 97, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 99,
            111, 109, 112, 108, 101, 116, 101, 32, 115, 116, 97, 116, 101, 32, 111, 112, 101, 114,
            97, 116, 105, 111, 110,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_compound_binary_operation(
    language: &self::ml_string::Text,
    operator: &self::ml_string::Text,
    kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*language)
        != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116])))
        || (((*kind) != (self::ml_string::Text::from_units(vec![105, 110, 116])))
            && ((*kind) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
    {
        self::ml_string::Text::from_units(vec![])
    } else if (*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) {
        if (*operator) == (self::ml_string::Text::from_units(vec![43, 61])) {
            self::ml_string::Text::from_units(vec![112, 108, 117, 115])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    } else if ((*operator) == (self::ml_string::Text::from_units(vec![43, 61])))
        || ((*operator) == (self::ml_string::Text::from_units(vec![43, 43])))
    {
        self::ml_string::Text::from_units(vec![112, 108, 117, 115])
    } else if ((*operator) == (self::ml_string::Text::from_units(vec![45, 61])))
        || ((*operator) == (self::ml_string::Text::from_units(vec![45, 45])))
    {
        self::ml_string::Text::from_units(vec![115, 117, 98])
    } else if (*operator) == (self::ml_string::Text::from_units(vec![42, 61])) {
        self::ml_string::Text::from_units(vec![109, 117, 108])
    } else if (*operator) == (self::ml_string::Text::from_units(vec![47, 61])) {
        self::ml_string::Text::from_units(vec![100, 105, 118])
    } else if (*operator) == (self::ml_string::Text::from_units(vec![37, 61])) {
        self::ml_string::Text::from_units(vec![114, 101, 109])
    } else if (*operator) == (self::ml_string::Text::from_units(vec![42, 42, 61])) {
        self::ml_string::Text::from_units(vec![
            101, 120, 112, 111, 110, 101, 110, 116, 105, 97, 116, 105, 111, 110,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_source_write_refusal(
    language: &self::ml_string::Text,
    operator: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    reference: bool,
    open: bool,
    field: bool,
    value_kind: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let operation = self::read_record_compound_binary_operation(language, operator, value_kind);
        if operation == (self::ml_string::Text::from_units(vec![])) {
            self::read_record_field_write_refusal(
                operator, kind, reference, open, field, value_kind,
            )
        } else {
            if (((*kind) != (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])))
                || !reference)
                || open
            {
                self::ml_string::Text::from_units(vec![
                    97, 32, 102, 105, 101, 108, 100, 32, 119, 114, 105, 116, 101, 32, 110, 101,
                    101, 100, 115, 32, 97, 32, 99, 108, 111, 115, 101, 100, 32, 115, 104, 97, 114,
                    101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 119, 105, 116, 104, 32, 101,
                    120, 112, 108, 105, 99, 105, 116, 32, 111, 119, 110, 32, 102, 105, 101, 108,
                    100, 115,
                ])
            } else if field {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![
                    99, 114, 101, 97, 116, 105, 110, 103, 32, 97, 32, 102, 105, 101, 108, 100, 32,
                    110, 101, 101, 100, 115, 32, 97, 32, 100, 121, 110, 97, 109, 105, 99, 32, 112,
                    114, 111, 112, 101, 114, 116, 121, 32, 97, 110, 100, 32, 100, 101, 115, 99,
                    114, 105, 112, 116, 111, 114, 32, 114, 101, 112, 114, 101, 115, 101, 110, 116,
                    97, 116, 105, 111, 110,
                ])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_record_field_write(
    target: &self::ml_string::Text,
    operator: &self::ml_string::Text,
    receiver: &self::ml_string::Text,
    field: &self::ml_string::Text,
    index: &self::ml_string::Text,
    arguments_: &[self::ml_string::Text],
    suffix: &self::ml_string::Text,
) -> self::ml_string::Text {
    {
        let owner = (self::ml_string::Text::from_units(vec![
            109, 108, 95, 102, 105, 101, 108, 100, 95, 111, 119, 110, 101, 114,
        ]))
        .concat(suffix);
        {
            let value = (self::ml_string::Text::from_units(vec![
                109, 108, 95, 102, 105, 101, 108, 100, 95, 118, 97, 108, 117, 101,
            ]))
            .concat(suffix);
            {
                let previous = (self::ml_string::Text::from_units(vec![
                    109, 108, 95, 102, 105, 101, 108, 100, 95, 112, 114, 101, 118, 105, 111, 117,
                    115,
                ]))
                .concat(suffix);
                {
                    let unary = ((*operator) == (self::ml_string::Text::from_units(vec![43, 43])))
                        || ((*operator) == (self::ml_string::Text::from_units(vec![45, 45])));
                    if (*target)
                        == (self::ml_string::Text::from_units(vec![
                            74, 97, 118, 97, 83, 99, 114, 105, 112, 116,
                        ]))
                    {
                        (((((((self::ml_string::Text::from_units(vec![
                            40, 118, 111, 105, 100, 32, 40, 40,
                        ]))
                        .concat(receiver))
                        .concat(&self::ml_string::Text::from_units(vec![41, 91])))
                        .concat(field))
                        .concat(&self::ml_string::Text::from_units(vec![93])))
                        .concat(operator))
                        .concat(&if unary {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            ((self::ml_string::Text::from_units(vec![40])).concat(
                                &(self::ml_array::optional_at(
                                    arguments_,
                                    self::ml_array::number_index(0f64),
                                ))
                                .unwrap_or_else(|| self::ml_string::Text::from_units(vec![])),
                            ))
                            .concat(&self::ml_string::Text::from_units(vec![41]))
                        }))
                        .concat(&self::ml_string::Text::from_units(vec![41, 41]))
                    } else {
                        let ml_library_inequality_2 = (*target)
                            != (self::ml_string::Text::from_units(vec![82, 117, 115, 116]));
                        if ml_library_inequality_2 {
                            self::ml_string::Text::from_units(vec![])
                        } else {
                            {
                                let output = ((((self::ml_string::Text::from_units(vec![
                                    123, 32, 108, 101, 116, 32,
                                ]))
                                .concat(&owner))
                                .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                                .concat(receiver))
                                .concat(&self::ml_string::Text::from_units(vec![59, 32]));
                                {
                                    let output_3 = {
                                        let ml_library_inequality_1 = (*operator)
                                            != (self::ml_string::Text::from_units(vec![61]));
                                        if ml_library_inequality_1 {
                                            {
                                                output.concat(
                                                    &((((((self::ml_string::Text::from_units(
                                                        vec![108, 101, 116, 32],
                                                    ))
                                                    .concat(&previous))
                                                    .concat(
                                                        &self::ml_string::Text::from_units(vec![
                                                            32, 61, 32,
                                                        ]),
                                                    ))
                                                    .concat(&owner))
                                                    .concat(&self::ml_string::Text::from_units(
                                                        vec![
                                                            46, 98, 111, 114, 114, 111, 119, 40,
                                                            41, 46,
                                                        ],
                                                    )))
                                                    .concat(index))
                                                    .concat(&self::ml_string::Text::from_units(
                                                        vec![59, 32],
                                                    )),
                                                )
                                            }
                                        } else {
                                            output.clone()
                                        }
                                    };
                                    {
                                        let output_5 = if unary {
                                            output_3.clone()
                                        } else {
                                            {
                                                output_3.concat(
                                                    &((((self::ml_string::Text::from_units(
                                                        vec![108, 101, 116, 32],
                                                    ))
                                                    .concat(&value))
                                                    .concat(&self::ml_string::Text::from_units(
                                                        vec![32, 61, 32],
                                                    )))
                                                    .concat(
                                                        &(self::ml_array::optional_at(
                                                            arguments_,
                                                            self::ml_array::number_index(0f64),
                                                        ))
                                                        .unwrap_or_else(|| {
                                                            self::ml_string::Text::from_units(
                                                                vec![],
                                                            )
                                                        }),
                                                    ))
                                                    .concat(&self::ml_string::Text::from_units(
                                                        vec![59, 32],
                                                    )),
                                                )
                                            }
                                        };
                                        {
                                            let result = if (*operator)
                                                == (self::ml_string::Text::from_units(vec![61]))
                                            {
                                                value.clone()
                                            } else if (*operator)
                                                == (self::ml_string::Text::from_units(vec![43, 43]))
                                            {
                                                previous.concat(&self::ml_string::Text::from_units(
                                                    vec![32, 43, 32, 49, 46, 48],
                                                ))
                                            } else if (*operator)
                                                == (self::ml_string::Text::from_units(vec![45, 45]))
                                            {
                                                previous.concat(&self::ml_string::Text::from_units(
                                                    vec![32, 45, 32, 49, 46, 48],
                                                ))
                                            } else if (*operator)
                                                == (self::ml_string::Text::from_units(vec![
                                                    42, 42, 61,
                                                ]))
                                            {
                                                ((((((((((((((self::ml_string::Text::from_units(vec![105, 102, 32])).concat(&value)).concat(&self::ml_string::Text::from_units(vec![32, 61, 61, 32, 48, 46, 48, 32, 123, 32, 49, 46, 48, 32, 125, 32, 101, 108, 115, 101, 32, 105, 102, 32]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 124, 124, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 110, 97, 110, 40, 41, 32, 124, 124, 32, 40]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![46, 97, 98, 115, 40, 41, 32, 61, 61, 32, 49, 46, 48, 32, 38, 38, 32]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![46, 105, 115, 95, 105, 110, 102, 105, 110, 105, 116, 101, 40, 41, 41, 32, 123, 32, 102, 54, 52, 58, 58, 78, 65, 78, 32, 125, 32, 101, 108, 115, 101, 32, 123, 32]))).concat(&previous)).concat(&self::ml_string::Text::from_units(vec![46, 112, 111, 119, 102, 40]))).concat(&value)).concat(&self::ml_string::Text::from_units(vec![41, 32, 125]))
                                            } else {
                                                (((previous.concat(&self::ml_string::Text::from_units(vec![32]))).concat(&{
                                                                let source_ml_source2_string_range_operand8 = operator.clone();
                                                                {
                                                                    let source_ml_source2_string_range_operand9 = 0f64;
                                                                    {
                                                                        let source_ml_source2_string_range_operand10 = 1f64;
                                                                        {
                                                                            let source_ml_source2_string_range_position11 = source_ml_source2_string_range_operand9;
                                                                            {
                                                                                let source_ml_source2_string_range_position12 = source_ml_source2_string_range_operand10;
                                                                                source_ml_source2_string_range_operand8.slice(source_ml_source2_string_range_position11, source_ml_source2_string_range_position12)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            })).concat(&self::ml_string::Text::from_units(vec![32]))).concat(&value)
                                            };
                                            (((((output_5.concat(&owner)).concat(
                                                &self::ml_string::Text::from_units(vec![
                                                    46, 98, 111, 114, 114, 111, 119, 95, 109, 117,
                                                    116, 40, 41, 46,
                                                ]),
                                            ))
                                            .concat(index))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                32, 61, 32,
                                            ])))
                                            .concat(&result))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                59, 32, 40, 41, 32, 125,
                                            ]))
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
                let source_ml_source2_s13 =
                    self::source_ml_source1_temporary_read_java_script_class_record_owner_loop11(
                        constructors,
                        brands,
                        fields,
                        owner,
                        at,
                    );
                match source_ml_source2_s13 {
                    self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Done(owner_2) => {
                        owner_2
                    }
                    self::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Result::MlSource1TemporaryReadJavaScriptClassRecordOwnerLoop11Return(source_ml_source1_temporary_result11) => {
                        source_ml_source1_temporary_result11
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
            let source_ml_source1_temporary_values12 = names_2.to_vec();
            {
                let source_ml_source1_temporary_index12 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s14 = self::source_ml_source1_temporary_exact_loop13(
                        starts,
                        counts,
                        labels,
                        targets,
                        node,
                        &(source_ml_source1_temporary_values12),
                        source_ml_source1_temporary_index12,
                    );
                    match source_ml_source2_s14 {
                        self::MlSource1TemporaryExactLoop13Result::MlSource1TemporaryExactLoop13Done => {
                            true
                        }
                        self::MlSource1TemporaryExactLoop13Result::MlSource1TemporaryExactLoop13Return(source_ml_source1_temporary_result13) => {
                            source_ml_source1_temporary_result13
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
                    let source_ml_source2_s15 = self::source_ml_source1_temporary_strings_loop14(
                        forms, texts, starts, counts, targets, node, result, at,
                    );
                    match source_ml_source2_s15 {
                        self::MlSource1TemporaryStringsLoop14Result::MlSource1TemporaryStringsLoop14Done(result_2) => {
                            result_2
                        }
                        self::MlSource1TemporaryStringsLoop14Result::MlSource1TemporaryStringsLoop14Return(source_ml_source1_temporary_result14) => {
                            source_ml_source1_temporary_result14
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
                        self::source_ml_source1_temporary_names_loop15(
                            forms, texts, starts, counts, labels, targets, result, fields, at,
                        )
                    }
                }
            }
        }
    }
}
