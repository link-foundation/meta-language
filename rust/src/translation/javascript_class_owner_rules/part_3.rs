// Generated from complete js/src/translation/javascript-class-owner-rules.js. Do not edit by hand.
use super::MlSource1TemporaryContainsJavaScriptClosedClassBrandLoop2Result;
use super::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result;
use super::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::source_ml_source1_temporary_contains_java_script_closed_class_brand_loop2;
use super::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9;
use super::source_ml_source1_temporary_read_optional_record_field_order_followers_loop15;
use super::source_ml_source1_temporary_read_record_field_write_operator_loop13;
use super::source_ml_source1_temporary_read_record_field_write_suffix_loop14;

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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn has_java_script_closed_class_exported_brand(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    declarations: f64,
    exports: &[self::ml_string::Text],
    brands: &[self::ml_string::Text],
) -> bool {
    if (exports.len() as f64) == 0f64 {
        false
    } else if declarations < 0f64 {
        true
    } else {
        {
            let source_ml_source1_temporary_values8 = exports.to_vec();
            {
                let source_ml_source1_temporary_index8 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s3 = self::source_ml_source1_temporary_has_java_script_closed_class_exported_brand_loop9(texts, starts, counts, labels, targets, declarations, brands, &(source_ml_source1_temporary_values8), source_ml_source1_temporary_index8);
                    match source_ml_source2_s3 {
                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Done => {
                                false
                            }
                            self::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Result::MlSource1TemporaryHasJavaScriptClosedClassExportedBrandLoop9Return(source_ml_source1_temporary_result9) => {
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
            let source_ml_source1_temporary_values12 = operators;
            {
                let source_ml_source1_temporary_index12 = self::ml::Big::from_i128(0);
                {
                    let source_ml_source2_s4 =
                        self::source_ml_source1_temporary_read_record_field_write_operator_loop13(
                            operation,
                            &(source_ml_source1_temporary_values12),
                            source_ml_source1_temporary_index12,
                        );
                    match source_ml_source2_s4 {
                        self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Done => {
                            self::ml_string::Text::from_units(vec![])
                        }
                        self::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Result::MlSource1TemporaryReadRecordFieldWriteOperatorLoop13Return(source_ml_source1_temporary_result13) => {
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
pub fn read_record_field_write_name(operation: &self::ml_string::Text) -> self::ml_string::Text {
    {
        let operator = self::read_record_field_write_operator(operation);
        if operator == (self::ml_string::Text::from_units(vec![])) {
            self::ml_string::Text::from_units(vec![])
        } else {
            {
                let source_ml_source2_string_range_operand5 = operation.clone();
                {
                    let source_ml_source2_string_range_operand6 = 17f64 + operator.length();
                    {
                        let source_ml_source2_string_range_position7 =
                            source_ml_source2_string_range_operand6;
                        {
                            let source_ml_source2_string_range_position8 = f64::INFINITY;
                            source_ml_source2_string_range_operand5.slice(
                                source_ml_source2_string_range_position7,
                                source_ml_source2_string_range_position8,
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
                self::source_ml_source1_temporary_read_record_field_write_suffix_loop14(
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
pub fn read_linked_ordered_record_snapshot_operation(
    expression: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    ordered: bool,
) -> self::ml_string::Text {
    if (((*expression)
        == (self::ml_string::Text::from_units(vec![
            114, 101, 102, 101, 114, 101, 110, 99, 101, 82, 101, 97, 100,
        ])))
        && ((*kind) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100]))))
        && ordered
    {
        self::ml_string::Text::from_units(vec![
            114, 101, 97, 100, 79, 114, 100, 101, 114, 101, 100, 82, 101, 99, 111, 114, 100, 83,
            110, 97, 112, 115, 104, 111, 116,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
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
    } else {
        self::read_record_numeric_compound_binary_operation(operator)
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_numeric_compound_binary_operation(
    operator: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*operator) == (self::ml_string::Text::from_units(vec![43, 61])))
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
pub fn read_optional_record_compound_binary_operation(
    language: &self::ml_string::Text,
    operator: &self::ml_string::Text,
    kind: &self::ml_string::Text,
    absence: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*language)
        != (self::ml_string::Text::from_units(vec![74, 97, 118, 97, 83, 99, 114, 105, 112, 116])))
        || (((*absence)
            != (self::ml_string::Text::from_units(vec![
                117, 110, 100, 101, 102, 105, 110, 101, 100,
            ])))
            && ((*absence) != (self::ml_string::Text::from_units(vec![98, 111, 116, 104]))))
    {
        self::ml_string::Text::from_units(vec![])
    } else if (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
        self::read_record_numeric_compound_binary_operation(operator)
    } else if (*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])) {
        self::read_record_compound_binary_operation(language, operator, kind)
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
                                                                let source_ml_source2_string_range_operand9 = operator.clone();
                                                                {
                                                                    let source_ml_source2_string_range_operand10 = 0f64;
                                                                    {
                                                                        let source_ml_source2_string_range_operand11 = 1f64;
                                                                        {
                                                                            let source_ml_source2_string_range_position12 = source_ml_source2_string_range_operand10;
                                                                            {
                                                                                let source_ml_source2_string_range_position13 = source_ml_source2_string_range_operand11;
                                                                                source_ml_source2_string_range_operand9.slice(source_ml_source2_string_range_position12, source_ml_source2_string_range_position13)
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
pub fn read_optional_record_field_order_followers(numeric_keys: &[f64], field: f64) -> Vec<f64> {
    {
        let selected =
            (self::ml_array::optional_at(numeric_keys, self::ml_array::number_index(field)))
                .unwrap_or_else(|| -1f64);
        {
            let followers = Vec::<f64>::new();
            if selected < 0f64 {
                followers
            } else {
                {
                    let index = 0f64;
                    {
                        self::source_ml_source1_temporary_read_optional_record_field_order_followers_loop15(numeric_keys, selected, followers, index)
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn render_optional_rust_record_field_write(
    receiver: &self::ml_string::Text,
    index: &self::ml_string::Text,
    argument: &self::ml_string::Text,
    suffix: &self::ml_string::Text,
    ordered: bool,
    followers: &[self::ml_string::Text],
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
                let capture = ((((((((self::ml_string::Text::from_units(vec![
                    123, 32, 108, 101, 116, 32,
                ]))
                .concat(&owner))
                .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                .concat(receiver))
                .concat(&self::ml_string::Text::from_units(vec![
                    59, 32, 108, 101, 116, 32,
                ])))
                .concat(&value))
                .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                .concat(argument))
                .concat(&self::ml_string::Text::from_units(vec![59, 32]));
                {
                    let storage = ((((self::ml_string::Text::from_units(vec![
                        108, 101, 116, 32, 109, 117, 116, 32,
                    ]))
                    .concat(&owner))
                    .concat(&self::ml_string::Text::from_units(vec![32, 61, 32])))
                    .concat(&owner))
                    .concat(&self::ml_string::Text::from_units(vec![
                        46, 98, 111, 114, 114, 111, 119, 95, 109, 117, 116, 40, 41, 59, 32,
                    ]));
                    if ordered {
                        {
                            let field = (owner
                                .concat(&self::ml_string::Text::from_units(vec![46, 48, 46])))
                            .concat(index);
                            {
                                let order =
                                    owner.concat(&self::ml_string::Text::from_units(vec![46, 49]));
                                {
                                    let position =
                                        if (followers.len() as f64) == 0f64 {
                                            order.concat(&self::ml_string::Text::from_units(vec![
                                                46, 108, 101, 110, 40, 41,
                                            ]))
                                        } else {
                                            ((((order.concat(
                                                &self::ml_string::Text::from_units(vec![
                                                    46, 105, 116, 101, 114, 40, 41, 46, 112, 111,
                                                    115, 105, 116, 105, 111, 110, 40, 124, 109,
                                                    108, 95, 111, 114, 100, 101, 114, 101, 100, 95,
                                                    102, 105, 101, 108, 100, 124, 32, 91,
                                                ]),
                                            ))
                                            .concat(&self::ml_string::join(
                                                followers,
                                                &self::ml_string::Text::from_units(vec![44, 32]),
                                            )))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                93, 46, 99, 111, 110, 116, 97, 105, 110, 115, 40,
                                                109, 108, 95, 111, 114, 100, 101, 114, 101, 100,
                                                95, 102, 105, 101, 108, 100, 41, 41, 46, 117, 110,
                                                119, 114, 97, 112, 95, 111, 114, 40,
                                            ])))
                                            .concat(&order))
                                            .concat(&self::ml_string::Text::from_units(vec![
                                                46, 108, 101, 110, 40, 41, 41,
                                            ]))
                                        };
                                    {
                                        let next = (self::ml_string::Text::from_units(vec![
                                            109, 108, 95, 102, 105, 101, 108, 100, 95, 112, 114,
                                            101, 118, 105, 111, 117, 115,
                                        ]))
                                        .concat(suffix);
                                        (((((((((((((((((capture.concat(&storage))
                                            .concat(
                                                &self::ml_string::Text::from_units(vec![
                                                    105, 102, 32,
                                                ]),
                                            ))
                                        .concat(&field))
                                        .concat(&self::ml_string::Text::from_units(
                                            vec![
                                                46, 105, 115, 95, 110, 111, 110, 101, 40, 41, 32,
                                                123, 32, 108, 101, 116, 32,
                                            ],
                                        )))
                                        .concat(&next))
                                        .concat(&self::ml_string::Text::from_units(
                                            vec![32, 61, 32],
                                        )))
                                        .concat(&position))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            59, 32,
                                        ])))
                                        .concat(&order))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            46, 105, 110, 115, 101, 114, 116, 40,
                                        ])))
                                        .concat(&next))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            44, 32,
                                        ])))
                                        .concat(index))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            46, 48, 41, 59, 32, 125, 32,
                                        ])))
                                        .concat(&field))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 61, 32, 83, 111, 109, 101, 40, 40,
                                        ])))
                                        .concat(&value))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            44, 41, 41, 59, 32, 40, 41, 32, 125,
                                        ]))
                                    }
                                }
                            }
                        }
                    } else {
                        ((((((capture.concat(&storage)).concat(&owner))
                            .concat(&self::ml_string::Text::from_units(vec![46])))
                        .concat(index))
                        .concat(&self::ml_string::Text::from_units(vec![
                            32, 61, 32, 83, 111, 109, 101, 40, 40,
                        ])))
                        .concat(&value))
                        .concat(&self::ml_string::Text::from_units(vec![
                            44, 41, 41, 59, 32, 40, 41, 32, 125,
                        ]))
                    }
                }
            }
        }
    }
}
