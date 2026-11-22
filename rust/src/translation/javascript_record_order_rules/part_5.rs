// Generated from complete js/src/translation/javascript-record-order-rules.js. Do not edit by hand.
use super::accept_java_script_record_order_positions;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_literal_property_index;
use super::read_java_script_record_order_name;
use super::read_java_script_record_order_position;
use super::read_record_field_position;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_sort_java_script_record_property_positions_loop28(
    names: &[Vec<f64>],
    insertion: &[f64],
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (insertion.len() as f64) {
            {
                let field = self::read_java_script_record_order_position(insertion, index);
                if (self::read_java_script_literal_property_index(
                    &(self::read_java_script_record_order_name(names, field)),
                )) < 0f64
                {
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value32 = result;
                            {
                                let source_ml_source1_concat_value33 = vec![field];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value32,
                                    ),
                                    source_ml_source1_concat_value33,
                                )
                            }
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (result, index) = (result_2, index_2);
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
        }
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Result {
    MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_present_record_property_order_loop29(
    insertion: &[f64],
    present: &[bool],
    mut index: f64,
) -> self::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Result {
    loop {
        return if index < (present.len() as f64) {
            if {
                let source_ml_source1_comparison_operand34 =
                    self::ml_array::optional_at(present, self::ml_array::number_index(index));
                {
                    let source_ml_source1_comparison_operand35 = self::ml_array::includes(
                        insertion,
                        &index,
                        0f64,
                        false,
                        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                    );
                    !(match (
                        source_ml_source1_comparison_operand34,
                        source_ml_source1_comparison_operand35,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }
            } {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100,
                        32, 114, 101, 99, 111, 114, 100, 32, 102, 105, 101, 108, 100, 32, 115, 101,
                        113, 117, 101, 110, 99, 101,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
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
            self::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Result::MlTemporaryCreateJavaScriptPresentRecordPropertyOrderLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result {
    MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Done,
    MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_present_record_property_order_loop30(
    order: &[f64],
    expected: &[f64],
    mut index: f64,
) -> self::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result {
    loop {
        return if index < (order.len() as f64) {
            if (self::read_java_script_record_order_position(order, index))
                == (self::read_java_script_record_order_position(expected, index))
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
                self::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Return(false)
            }
        } else {
            self::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Result::MlTemporaryAcceptJavaScriptPresentRecordPropertyOrderLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result {
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Done,
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_property_order_loop31(
    order: &[f64],
    expected: &[f64],
    mut index: f64,
) -> self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result {
    loop {
        return if index < (order.len() as f64) {
            if (self::read_java_script_record_order_position(order, index))
                == (self::read_java_script_record_order_position(expected, index))
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
                self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Return(false)
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop31Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop33(
    names: &[Vec<f64>],
    mut insertion: Vec<f64>,
    values: &[f64],
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result {
    loop {
        return if index < (values.len() as f64) {
            {
                let field = self::read_java_script_record_order_position(values, index);
                if ((!(self::ml_math::is_integer(field)) || (field < 0f64))
                    || (field >= (names.len() as f64)))
                    || ((field == 0f64) && ((1f64 / field) < 0f64))
                {
                    self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Return(self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100]))
                } else {
                    if !(self::ml_array::includes(
                        &insertion,
                        &field,
                        0f64,
                        false,
                        |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                    )) {
                        {
                            let insertion_2 = {
                                let source_ml_source1_concat_value36 = insertion;
                                {
                                    let source_ml_source1_concat_value37 = vec![field];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value36,
                                        ),
                                        source_ml_source1_concat_value37,
                                    )
                                }
                            };
                            {
                                let index_2 = index + 1f64;
                                {
                                    (insertion, index) = (insertion_2, index_2);
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
            }
        } else {
            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Done(insertion)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop32(
    names: &[Vec<f64>],
    chunks: &[Vec<f64>],
    mut insertion: Vec<f64>,
    mut chunk: f64,
) -> self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result {
    loop {
        return if chunk < (chunks.len() as f64) {
            {
                let values = self::read_java_script_record_order_name(chunks, chunk);
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s38 = self::source_ml_temporary_read_java_script_record_order_merge_error_loop33(names, insertion, &(values), index);
                        match source_ml_source1_s38 {
                            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Done(insertion_2) => {
                                {
                                    let chunk_2 = chunk + 1f64;
                                    {
                                        (insertion, chunk) = (insertion_2, chunk_2);
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop33Return(source_ml_temporary_result33) => {
                                self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Return(source_ml_temporary_result33)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop32Done(insertion)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_java_script_record_property_order_loop35(
    mut insertion: Vec<f64>,
    values: &[f64],
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (values.len() as f64) {
            {
                let field = self::read_java_script_record_order_position(values, index);
                if !(self::ml_array::includes(
                    &insertion,
                    &field,
                    0f64,
                    false,
                    |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                )) {
                    {
                        let insertion_2 = {
                            let source_ml_source1_concat_value39 = insertion;
                            {
                                let source_ml_source1_concat_value40 = vec![field];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value39,
                                    ),
                                    source_ml_source1_concat_value40,
                                )
                            }
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (insertion, index) = (insertion_2, index_2);
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
        }
        return { insertion };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_java_script_record_property_order_loop34(
    chunks: &[Vec<f64>],
    mut insertion: Vec<f64>,
    mut chunk: f64,
) -> Vec<f64> {
    loop {
        if chunk < (chunks.len() as f64) {
            {
                let values = self::read_java_script_record_order_name(chunks, chunk);
                {
                    let index = 0f64;
                    {
                        let insertion_2 = self::source_ml_temporary_merge_java_script_record_property_order_loop35(insertion, &(values), index);
                        {
                            let chunk_2 = chunk + 1f64;
                            {
                                (insertion, chunk) = (insertion_2, chunk_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { insertion };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_java_script_present_record_property_order_loop37(
    mut insertion: Vec<f64>,
    positions: &[f64],
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (positions.len() as f64) {
            {
                let field = self::read_java_script_record_order_position(positions, index);
                if !(self::ml_array::includes(
                    &insertion,
                    &field,
                    0f64,
                    false,
                    |a: &f64, b: &f64| a == b || (a.is_nan() && b.is_nan()),
                )) {
                    {
                        let insertion_2 = {
                            let source_ml_source1_concat_value41 = insertion;
                            {
                                let source_ml_source1_concat_value42 = vec![field];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value41,
                                    ),
                                    source_ml_source1_concat_value42,
                                )
                            }
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (insertion, index) = (insertion_2, index_2);
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
        }
        return { insertion };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_java_script_present_record_property_order_loop36(
    names: &[Vec<f64>],
    chunks: &[Vec<f64>],
    mut insertion: Vec<f64>,
    mut chunk: f64,
) -> Vec<f64> {
    loop {
        return if chunk < (chunks.len() as f64) {
            {
                let positions = self::read_java_script_record_order_name(chunks, chunk);
                if (self::accept_java_script_record_order_positions(
                    (names.len() as f64),
                    &(positions),
                )) {
                    {
                        let index = 0f64;
                        {
                            let insertion_2 = self::source_ml_temporary_merge_java_script_present_record_property_order_loop37(insertion, &(positions), index);
                            {
                                let chunk_2 = chunk + 1f64;
                                {
                                    (insertion, chunk) = (insertion_2, chunk_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
                {
                    (self::ml_source_exception::SourceException {
                        class_name: String::from("TypeError"),
                        message_units: (self::ml_string::Text::from_units(vec![
                            105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101,
                            100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 111, 117, 114, 99, 101,
                            32, 102, 105, 101, 108, 100,
                        ]))
                        .0,
                        message_present: true,
                    })
                    .raise()
                }
            }
        } else {
            insertion
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result {
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Done,
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_rebase_error_loop38(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result {
    loop {
        return if index < (source.len() as f64) {
            if (self::read_record_field_position(
                target,
                &(self::read_java_script_record_order_name(source, index)),
            )) < 0f64
            {
                self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 112, 114, 101, 97, 100, 32, 102, 105, 101, 108, 100, 32, 105, 115, 32, 109, 105, 115, 115, 105, 110, 103, 32, 102, 114, 111, 109, 32, 105, 116, 115, 32, 116, 97, 114, 103, 101, 116]))
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
            self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop38Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Result {
    MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_rebase_java_script_present_record_property_order_loop39(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Result {
    loop {
        return if index < (source.len() as f64) {
            if (self::read_record_field_position(
                target,
                &(self::read_java_script_record_order_name(source, index)),
            )) < 0f64
            {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114,
                        100, 32, 115, 112, 114, 101, 97, 100, 32, 102, 105, 101, 108, 100, 32, 105,
                        115, 32, 109, 105, 115, 115, 105, 110, 103, 32, 102, 114, 111, 109, 32,
                        105, 116, 115, 32, 116, 97, 114, 103, 101, 116,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
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
            self::MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Result::MlTemporaryRebaseJavaScriptPresentRecordPropertyOrderLoop39Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Result {
    MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_copy_java_script_record_property_order_loop40(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Result {
    loop {
        return if index < (target.len() as f64) {
            if (self::read_record_field_position(
                source,
                &(self::read_java_script_record_order_name(target, index)),
            )) < 0f64
            {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 115, 116, 32, 116,
                        97, 114, 103, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 32,
                        99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32,
                        115, 117, 98, 115, 101, 116,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
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
            self::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Result::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop40Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Result {
    MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_copy_java_script_present_record_property_order_loop41(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Result {
    loop {
        return if index < (target.len() as f64) {
            if (self::read_record_field_position(
                source,
                &(self::read_java_script_record_order_name(target, index)),
            )) < 0f64
            {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 115, 116, 32, 116,
                        97, 114, 103, 101, 116, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 32,
                        99, 111, 109, 112, 108, 101, 116, 101, 32, 115, 111, 117, 114, 99, 101, 32,
                        115, 117, 98, 115, 101, 116,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
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
            self::MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Result::MlTemporaryCopyJavaScriptPresentRecordPropertyOrderLoop41Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_project_java_script_record_property_order_loop42(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    order: &[f64],
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (order.len() as f64) {
            {
                let field = self::read_record_field_position(
                    target,
                    &(self::read_java_script_record_order_name(
                        source,
                        self::read_java_script_record_order_position(order, index),
                    )),
                );
                if field >= 0f64 {
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value43 = result;
                            {
                                let source_ml_source1_concat_value44 = vec![field];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value43,
                                    ),
                                    source_ml_source1_concat_value44,
                                )
                            }
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (result, index) = (result_2, index_2);
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
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_property_keys_loop43(
    names: &[Vec<f64>],
    order: &[f64],
    mut result: Vec<Vec<f64>>,
    mut index: f64,
) -> Vec<Vec<f64>> {
    loop {
        if index < (order.len() as f64) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value45 = result;
                    {
                        let source_ml_source1_concat_value46 =
                            vec![self::read_java_script_record_order_name(
                                names,
                                self::read_java_script_record_order_position(order, index),
                            )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<Vec<f64>>::new(),
                                source_ml_source1_concat_value45,
                            ),
                            source_ml_source1_concat_value46,
                        )
                    }
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_present_record_property_key_texts_loop44(
    names: &[Vec<f64>],
    order: &[f64],
    mut rows: Vec<Vec<f64>>,
    mut index: f64,
) -> Vec<Vec<f64>> {
    loop {
        if index < (order.len() as f64) {
            {
                let rows_2 = {
                    let source_ml_source1_concat_value47 = rows;
                    {
                        let source_ml_source1_concat_value48 =
                            vec![self::read_java_script_record_order_name(
                                names,
                                self::read_java_script_record_order_position(order, index),
                            )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<Vec<f64>>::new(),
                                source_ml_source1_concat_value47,
                            ),
                            source_ml_source1_concat_value48,
                        )
                    }
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (rows, index) = (rows_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { rows };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_property_name_texts_loop46(
    units: &[f64],
    mut key: self::ml_string::Text,
    mut at: f64,
) -> self::ml_string::Text {
    loop {
        if at < (units.len() as f64) {
            {
                let key_2 = key.concat(&{
                    let source_ml_source1_character_argument49 =
                        self::read_java_script_record_order_position(units, at);
                    self::ml_string::Text::from_units(
                        self::ml_utf16_text::character_units_utf16_text(&vec![
                            source_ml_source1_character_argument49,
                        ])
                        .into_iter()
                        .map(|ml_unit| ml_unit as u16)
                        .collect(),
                    )
                });
                {
                    let at_2 = at + 1f64;
                    {
                        (key, at) = (key_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { key.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_property_name_texts_loop45(
    rows: &[Vec<f64>],
    mut result: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (rows.len() as f64) {
            {
                let units = self::read_java_script_record_order_name(rows, index);
                {
                    let key = self::ml_string::Text::from_units(vec![]);
                    {
                        let at = 0f64;
                        {
                            let key_2 = self::source_ml_temporary_read_java_script_record_property_name_texts_loop46(&(units), key, at);
                            {
                                let result_2 = {
                                    let source_ml_source1_concat_value50 = result;
                                    {
                                        let source_ml_source1_concat_value51 = vec![key_2];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                source_ml_source1_concat_value50,
                                            ),
                                            source_ml_source1_concat_value51,
                                        )
                                    }
                                };
                                {
                                    let index_2 = index + 1f64;
                                    {
                                        (result, index) = (result_2, index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { result };
    }
}
