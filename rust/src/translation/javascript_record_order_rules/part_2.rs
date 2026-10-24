// Generated from complete js/src/translation/javascript-record-order-rules.js. Do not edit by hand.
use super::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result;
use super::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result;
use super::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result;
use super::MlTemporaryAcceptRecordFieldNameLoop3Result;
use super::MlTemporaryCompareRecordFieldNamesLoop4Result;
use super::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result;
use super::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result;
use super::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result;
use super::MlTemporaryReadRecordFieldInsertionOrderLoop12Result;
use super::MlTemporaryReadRecordFieldPositionLoop5Result;
use super::MlTemporaryReadRecordFieldPositionLoop6Result;
use super::MlTemporaryReadRecordSortedFieldPositionsLoop7Result;
use super::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result;
use super::MlTemporaryRecordTypeFactFieldLoop15Result;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::read_record_value_order;
use super::read_required_number_fact;
use super::read_required_record_name;
use super::read_required_text_fact;
use super::source_ml_temporary_accept_java_script_record_order_names_loop23;
use super::source_ml_temporary_accept_java_script_record_order_permutation_loop24;
use super::source_ml_temporary_accept_java_script_record_property_order_loop29;
use super::source_ml_temporary_accept_record_field_name_loop3;
use super::source_ml_temporary_compare_record_field_names_loop4;
use super::source_ml_temporary_create_java_script_record_property_order_loop26;
use super::source_ml_temporary_create_java_script_record_property_order_loop28;
use super::source_ml_temporary_merge_java_script_record_property_order_loop32;
use super::source_ml_temporary_read_java_script_literal_property_index_loop22;
use super::source_ml_temporary_read_java_script_record_order_merge_error_loop30;
use super::source_ml_temporary_read_java_script_record_order_rebase_error_loop34;
use super::source_ml_temporary_read_record_field_insertion_order_loop12;
use super::source_ml_temporary_read_record_field_insertion_order_loop13;
use super::source_ml_temporary_read_record_field_order_loop11;
use super::source_ml_temporary_read_record_field_position_loop5;
use super::source_ml_temporary_read_record_field_position_loop6;
use super::source_ml_temporary_read_record_sorted_field_positions_loop7;
use super::source_ml_temporary_read_record_sorted_field_positions_loop8;
use super::source_ml_temporary_read_record_type_owner_presence_loop17;
use super::source_ml_temporary_rebase_java_script_record_property_order_loop35;
use super::source_ml_temporary_record_type_fact_elements_loop16;
use super::source_ml_temporary_record_type_fact_field_loop15;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_record_field_name(name: &[f64]) -> bool {
    {
        let index = 0f64;
        {
            let source_ml_source1_s2 =
                self::source_ml_temporary_accept_record_field_name_loop3(name, index);
            match source_ml_source1_s2 {
                self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Done => {
                    true
                }
                self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Return(source_ml_temporary_result3) => {
                    source_ml_temporary_result3
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compare_record_field_names(left: &[f64], right: &[f64]) -> f64 {
    {
        let end = if (left.len() as f64) < (right.len() as f64) {
            left.len() as f64
        } else {
            right.len() as f64
        };
        {
            let index = 0f64;
            {
                let source_ml_source1_s3 =
                    self::source_ml_temporary_compare_record_field_names_loop4(
                        left, right, end, index,
                    );
                match source_ml_source1_s3 {
                    self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Done => {
                        if (left.len() as f64) < (right.len() as f64) {
                            -1f64
                        } else if (left.len() as f64) > (right.len() as f64) {
                                1f64
                            } else {
                                0f64
                            }
                    }
                    self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Return(source_ml_temporary_result4) => {
                        source_ml_temporary_result4
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_position(fields: &[Vec<f64>], name: &[f64]) -> f64 {
    if (self::accept_record_field_name(name)) {
        {
            let index = 0f64;
            {
                let source_ml_source1_s4 =
                    self::source_ml_temporary_read_record_field_position_loop5(fields, index);
                match source_ml_source1_s4 {
                    self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Done => {
                        {
                            let index_2 = 0f64;
                            {
                                let source_ml_source1_s5 = self::source_ml_temporary_read_record_field_position_loop6(fields, name, index_2);
                                match source_ml_source1_s5 {
                                    self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Done => {
                                        -1f64
                                    }
                                    self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Return(source_ml_temporary_result6) => {
                                        source_ml_temporary_result6
                                    }
                                }
                            }
                        }
                    }
                    self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Return(source_ml_temporary_result6_2) => {
                        source_ml_temporary_result6_2
                    }
                }
            }
        }
    } else {
        -1f64
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_sorted_field_positions(fields: &[Vec<f64>]) -> Vec<f64> {
    {
        let index = 0f64;
        {
            let source_ml_source1_s6 =
                self::source_ml_temporary_read_record_sorted_field_positions_loop7(fields, index);
            match source_ml_source1_s6 {
                self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Done => {
                    {
                        let sorted = self::read_record_value_order((fields.len() as f64), false);
                        {
                            let width = 1f64;
                            {
                                self::source_ml_temporary_read_record_sorted_field_positions_loop8(fields, sorted, width)
                            }
                        }
                    }
                }
                self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Return(source_ml_temporary_result10) => {
                    source_ml_temporary_result10
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_order(fields: &[Vec<f64>]) -> Vec<f64> {
    {
        let sorted = self::read_record_sorted_field_positions(fields);
        if ((sorted.len() as f64) > 0f64)
            && ((self::read_required_number_fact(&(sorted), 0f64)) < 0f64)
        {
            vec![-1f64]
        } else {
            {
                let selected = Vec::<f64>::new();
                {
                    let index = 0f64;
                    {
                        self::source_ml_temporary_read_record_field_order_loop11(
                            fields,
                            &(sorted),
                            selected,
                            index,
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_insertion_order(fields: &[Vec<f64>]) -> Vec<f64> {
    {
        let sorted = self::read_record_sorted_field_positions(fields);
        if ((sorted.len() as f64) > 0f64)
            && ((self::read_required_number_fact(&(sorted), 0f64)) < 0f64)
        {
            vec![-1f64]
        } else {
            {
                let first = Vec::<f64>::new();
                {
                    let last = Vec::<f64>::new();
                    {
                        let start = 0f64;
                        {
                            let index = 0f64;
                            {
                                let source_ml_source1_s7 = self::source_ml_temporary_read_record_field_insertion_order_loop12(fields, &(sorted), first, last, start, index);
                                match source_ml_source1_s7 {
                                    self::MlTemporaryReadRecordFieldInsertionOrderLoop12Result::MlTemporaryReadRecordFieldInsertionOrderLoop12Done(first_2, last_2) => {
                                        {
                                            let selected = Vec::<f64>::new();
                                            {
                                                let index_2 = 0f64;
                                                {
                                                    self::source_ml_temporary_read_record_field_insertion_order_loop13(fields, &(first_2), &(last_2), selected, index_2)
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
pub fn read_record_spread_role(kind: &self::ml_string::Text) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![118, 97, 114])) {
        self::ml_string::Text::from_units(vec![111, 112, 101, 110])
    } else if (*kind) == (self::ml_string::Text::from_units(vec![114, 101, 99, 111, 114, 100])) {
        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100, 115])
    } else {
        self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_type_constraint_step(
    left: &[Vec<f64>],
    right: &[Vec<f64>],
    left_open: bool,
    right_open: bool,
    phase: f64,
    index: f64,
) -> Vec<f64> {
    if phase < 1f64 {
        if index >= (left.len() as f64) {
            vec![5f64, -1f64, -1f64, 1f64, 0f64]
        } else {
            {
                let matching = self::read_record_field_position(
                    right,
                    &(self::read_required_record_name(left, index)),
                );
                if matching >= 0f64 {
                    vec![0f64, index, matching, 0f64, (index + 1f64)]
                } else {
                    vec![
                        if right_open { 1f64 } else { 3f64 },
                        index,
                        -1f64,
                        0f64,
                        (index + 1f64),
                    ]
                }
            }
        }
    } else if index >= (right.len() as f64) {
        vec![6f64, -1f64, -1f64, 1f64, index]
    } else {
        {
            let matching_2 = self::read_record_field_position(
                left,
                &(self::read_required_record_name(right, index)),
            );
            if matching_2 >= 0f64 {
                vec![5f64, matching_2, index, 1f64, (index + 1f64)]
            } else {
                vec![
                    if left_open { 2f64 } else { 4f64 },
                    -1f64,
                    index,
                    1f64,
                    (index + 1f64),
                ]
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_type_constraint_error(
    action: f64,
    name: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (action == 3f64) || (action == 4f64) {
        ((self::ml_string::Text::from_units(vec![
            114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32,
        ]))
        .concat(name))
        .concat(&self::ml_string::Text::from_units(vec![
            32, 105, 115, 32, 109, 105, 115, 115, 105, 110, 103,
        ]))
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_method_error(
    kind: &self::ml_string::Text,
    operation: &self::ml_string::Text,
    parameters: f64,
    arguments_count: f64,
) -> self::ml_string::Text {
    {
        let ml_library_inequality_1 = (*kind)
            != (self::ml_string::Text::from_units(vec![102, 117, 110, 99, 116, 105, 111, 110]));
        if ml_library_inequality_1 {
            (self::ml_string::Text::from_units(vec![
                114, 101, 99, 111, 114, 100, 32, 104, 97, 115, 32, 110, 111, 32, 109, 101, 116,
                104, 111, 100, 32,
            ]))
            .concat(operation)
        } else if arguments_count == parameters {
            self::ml_string::Text::from_units(vec![])
        } else {
            ((((self::ml_string::Text::from_units(vec![46])).concat(operation)).concat(
                &self::ml_string::Text::from_units(vec![
                    40, 41, 32, 101, 120, 112, 101, 99, 116, 115, 32,
                ]),
            ))
            .concat(&self::ml_string::Text::from_text(
                &self::ml_number::js_number(parameters),
            )))
            .concat(&self::ml_string::Text::from_units(vec![
                32, 97, 114, 103, 117, 109, 101, 110, 116, 115,
            ]))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_record_field_inference_step(fields: &[Vec<f64>], name: &[f64], open: bool) -> Vec<f64> {
    {
        let index = self::read_record_field_position(fields, name);
        if index >= 0f64 {
            vec![0f64, index]
        } else {
            vec![if open { 1f64 } else { 2f64 }, -1f64]
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn record_type_fact_field(
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    owner: f64,
    name: &self::ml_string::Text,
) -> f64 {
    if owner < 0f64 {
        -1f64
    } else {
        {
            let start = self::read_required_number_fact(starts, owner);
            {
                let end = start + self::read_required_number_fact(counts, owner);
                {
                    let edge = start;
                    {
                        let source_ml_source1_s8 =
                            self::source_ml_temporary_record_type_fact_field_loop15(
                                labels, targets, name, end, edge,
                            );
                        match source_ml_source1_s8 {
                            self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Done => {
                                -1f64
                            }
                            self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Return(source_ml_temporary_result15) => {
                                source_ml_temporary_result15
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
pub fn record_type_fact_kind(
    texts: &[self::ml_string::Text],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    owner: f64,
) -> self::ml_string::Text {
    {
        let kind = self::record_type_fact_field(
            starts,
            counts,
            labels,
            targets,
            owner,
            &(self::ml_string::Text::from_units(vec![107, 105, 110, 100])),
        );
        if kind < 0f64 {
            self::ml_string::Text::from_units(vec![])
        } else {
            self::read_required_text_fact(texts, kind)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn record_type_fact_elements(
    starts: &[f64],
    counts: &[f64],
    targets: &[f64],
    owner: f64,
) -> Vec<f64> {
    {
        let values = Vec::<f64>::new();
        if owner < 0f64 {
            values
        } else {
            {
                let start = self::read_required_number_fact(starts, owner);
                {
                    let end = start + self::read_required_number_fact(counts, owner);
                    {
                        let edge = start;
                        {
                            self::source_ml_temporary_record_type_fact_elements_loop16(
                                targets, values, end, edge,
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
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_record_type_owner_presence(
    texts: &[self::ml_string::Text],
    numbers: &[f64],
    starts: &[f64],
    counts: &[f64],
    labels: &[self::ml_string::Text],
    targets: &[f64],
    bindings: &[f64],
    root: f64,
    owner: f64,
) -> bool {
    {
        let pending = vec![root];
        {
            let seen = Vec::<f64>::new();
            {
                let position = 0f64;
                {
                    let source_ml_source1_s9 =
                        self::source_ml_temporary_read_record_type_owner_presence_loop17(
                            texts, numbers, starts, counts, labels, targets, bindings, owner,
                            pending, seen, position,
                        );
                    match source_ml_source1_s9 {
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Done => {
                            false
                        }
                        self::MlTemporaryReadRecordTypeOwnerPresenceLoop17Result::MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(source_ml_temporary_result17) => {
                            source_ml_temporary_result17
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_literal_property_index(key: &[f64]) -> f64 {
    if ((key.len() as f64) == 0f64) || ((key.len() as f64) > 10f64) {
        -1f64
    } else {
        if ((key.len() as f64) > 1f64) && {
            let source_ml_source1_comparison_operand10 =
                self::ml_array::optional_at(key, self::ml_array::number_index(0f64));
            {
                let source_ml_source1_comparison_operand11 = 48f64;
                match (
                    source_ml_source1_comparison_operand10,
                    source_ml_source1_comparison_operand11,
                ) {
                    (Some(ml_left), ml_right) => ml_left == ml_right,
                    _ => false,
                }
            }
        } {
            -1f64
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s12 = self::source_ml_temporary_read_java_script_literal_property_index_loop22(key, value, at);
                        match source_ml_source1_s12 {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Done(value_2) => {
                                if value_2 <= 4_294_967_294f64 {
                                    value_2
                                } else {
                                    -1f64
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop22Return(source_ml_temporary_result22) => {
                                source_ml_temporary_result22
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
pub fn read_java_script_literal_property_form(
    kind: &self::ml_string::Text,
    index: f64,
) -> self::ml_string::Text {
    if ((((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
        && (index >= 0f64)
    {
        self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
    } else {
        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_index_number_form(
    kind: &self::ml_string::Text,
    element: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![105, 110, 116])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 116])))
    {
        self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))
        || (((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
            && ((*element) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))
    {
        self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
        && ((((*element) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
            || ((*element) == (self::ml_string::Text::from_units(vec![105, 110, 116]))))
            || ((*element) == (self::ml_string::Text::from_units(vec![110, 97, 116]))))
    {
        self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
    } else {
        self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_record_order_name() -> Vec<f64> {
    (self::ml_source_exception::SourceException {
        class_name: String::from("TypeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32,
            110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108,
            32, 111, 119, 110, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn refuse_java_script_record_order_position() -> f64 {
    (self::ml_source_exception::SourceException {
        class_name: String::from("TypeError"),
        message_units: (self::ml_string::Text::from_units(vec![
            97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32,
            110, 101, 101, 100, 115, 32, 101, 118, 101, 114, 121, 32, 97, 99, 116, 117, 97, 108,
            32, 102, 105, 101, 108, 100, 32, 112, 111, 115, 105, 116, 105, 111, 110,
        ]))
        .0,
        message_present: true,
    })
    .raise()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_name(names: &[Vec<f64>], index: f64) -> Vec<f64> {
    (self::ml_array::optional_at(names, self::ml_array::number_index(index)))
        .unwrap_or_else(self::refuse_java_script_record_order_name)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_position(positions: &[f64], index: f64) -> f64 {
    (self::ml_array::optional_at(positions, self::ml_array::number_index(index)))
        .unwrap_or_else(self::refuse_java_script_record_order_position)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_order_names(names: &[Vec<f64>]) -> bool {
    {
        let index = 0f64;
        {
            let source_ml_source1_s13 =
                self::source_ml_temporary_accept_java_script_record_order_names_loop23(
                    names, index,
                );
            match source_ml_source1_s13 {
                self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Done => {
                    true
                }
                self::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Result::MlTemporaryAcceptJavaScriptRecordOrderNamesLoop23Return(source_ml_temporary_result23) => {
                    source_ml_temporary_result23
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_order_permutation(count: f64, order: &[f64]) -> bool {
    if ((!(self::ml_math::is_safe_integer(count)) || (count < 0f64))
        || ((count == 0f64) && ((1f64 / count) < 0f64)))
        || ((order.len() as f64) != count)
    {
        false
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s14 =
                    self::source_ml_temporary_accept_java_script_record_order_permutation_loop24(
                        count, order, index,
                    );
                match source_ml_source1_s14 {
                    self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Done => {
                        true
                    }
                    self::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Result::MlTemporaryAcceptJavaScriptRecordOrderPermutationLoop24Return(source_ml_temporary_result24) => {
                        source_ml_temporary_result24
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_creation_error(
    names: &[Vec<f64>],
    insertion: &[f64],
) -> self::ml_string::Text {
    if !(self::accept_java_script_record_order_names(names))
        || !(self::accept_java_script_record_order_permutation((names.len() as f64), insertion))
    {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101,
            99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 115, 101, 113, 117, 101, 110, 99,
            101,
        ])
    } else {
        self::ml_string::Text::from_units(vec![])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_record_property_order(names: &[Vec<f64>], insertion: &[f64]) -> Vec<f64> {
    {
        let error = self::read_java_script_record_order_creation_error(names, insertion);
        if error.0.is_empty() {
            {
                let numeric = Vec::<f64>::new();
                {
                    let field = 0f64;
                    {
                        let numeric_2 = self::source_ml_temporary_create_java_script_record_property_order_loop26(names, numeric, field);
                        {
                            let result = numeric_2;
                            {
                                let index = 0f64;
                                {
                                    self::source_ml_temporary_create_java_script_record_property_order_loop28(names, insertion, result, index)
                                }
                            }
                        }
                    }
                }
            }
        } else {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (error.clone()).0,
                message_present: true,
            })
            .raise()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_record_property_order(names: &[Vec<f64>], order: &[f64]) -> bool {
    if !(self::accept_java_script_record_order_names(names))
        || !(self::accept_java_script_record_order_permutation((names.len() as f64), order))
    {
        false
    } else {
        {
            let expected = self::create_java_script_record_property_order(names, order);
            {
                let index = 0f64;
                {
                    let source_ml_source1_s15 =
                        self::source_ml_temporary_accept_java_script_record_property_order_loop29(
                            order,
                            &(expected),
                            index,
                        );
                    match source_ml_source1_s15 {
                        self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Done => {
                            true
                        }
                        self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Return(source_ml_temporary_result29) => {
                            source_ml_temporary_result29
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_merge_error(
    names: &[Vec<f64>],
    chunks: &[Vec<f64>],
) -> self::ml_string::Text {
    if (self::accept_java_script_record_order_names(names)) {
        {
            let insertion = Vec::<f64>::new();
            {
                let chunk = 0f64;
                {
                    let source_ml_source1_s16 =
                        self::source_ml_temporary_read_java_script_record_order_merge_error_loop30(
                            names, chunks, insertion, chunk,
                        );
                    match source_ml_source1_s16 {
                        self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Done(insertion_2) => {
                            self::read_java_script_record_order_creation_error(names, &(insertion_2))
                        }
                        self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Return(source_ml_temporary_result30) => {
                            source_ml_temporary_result30
                        }
                    }
                }
            }
        }
    } else {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101,
            99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101, 115,
        ])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn merge_java_script_record_property_order(
    names: &[Vec<f64>],
    chunks: &[Vec<f64>],
) -> Vec<f64> {
    {
        let error = self::read_java_script_record_order_merge_error(names, chunks);
        if error.0.is_empty() {
            {
                let insertion = Vec::<f64>::new();
                {
                    let chunk = 0f64;
                    {
                        let insertion_2 = self::source_ml_temporary_merge_java_script_record_property_order_loop32(chunks, insertion, chunk);
                        self::create_java_script_record_property_order(names, &(insertion_2))
                    }
                }
            }
        } else {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (error.clone()).0,
                message_present: true,
            })
            .raise()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_rebase_error(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    order: &[f64],
) -> self::ml_string::Text {
    if !(self::accept_java_script_record_order_names(target))
        || !(self::accept_java_script_record_property_order(source, order))
    {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101,
            99, 111, 114, 100, 32, 115, 112, 114, 101, 97, 100, 32, 115, 111, 117, 114, 99, 101,
        ])
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s17 =
                    self::source_ml_temporary_read_java_script_record_order_rebase_error_loop34(
                        target, source, index,
                    );
                match source_ml_source1_s17 {
                    self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Done => {
                        self::ml_string::Text::from_units(vec![])
                    }
                    self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Return(source_ml_temporary_result34) => {
                        source_ml_temporary_result34
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn rebase_java_script_record_property_order(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    order: &[f64],
) -> Vec<f64> {
    {
        let error = self::read_java_script_record_order_rebase_error(target, source, order);
        if error.0.is_empty() {
            {
                let result = Vec::<f64>::new();
                {
                    let index = 0f64;
                    {
                        self::source_ml_temporary_rebase_java_script_record_property_order_loop35(
                            target, source, order, result, index,
                        )
                    }
                }
            }
        } else {
            (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (error.clone()).0,
                message_present: true,
            })
            .raise()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_order_key_error(
    names: &[Vec<f64>],
    order: &[f64],
) -> self::ml_string::Text {
    if self::accept_java_script_record_property_order(names, order) {
        self::ml_string::Text::from_units(vec![])
    } else {
        self::ml_string::Text::from_units(vec![
            105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101,
            99, 111, 114, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114, 100, 101,
            114,
        ])
    }
}
