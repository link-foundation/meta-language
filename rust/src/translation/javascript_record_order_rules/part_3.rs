// Generated from complete js/src/translation/javascript-record-order-rules.js. Do not edit by hand.
use super::accept_record_field_name;
use super::compare_record_field_names;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_record_order_key_error;
use super::read_required_number_fact;
use super::read_required_record_name;
use super::read_required_text_fact;
use super::source_ml_temporary_create_java_script_dense_property_key_texts_loop39;
use super::source_ml_temporary_read_java_script_record_property_key_texts_loop37;
use super::source_ml_temporary_read_java_script_record_property_keys_loop36;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_record_property_keys(names: &[Vec<f64>], order: &[f64]) -> Vec<Vec<f64>> {
    {
        let error = self::read_java_script_record_order_key_error(names, order);
        if error.0.is_empty() {
            {
                let result = Vec::<Vec<f64>>::new();
                {
                    let index = 0f64;
                    {
                        self::source_ml_temporary_read_java_script_record_property_keys_loop36(
                            names, order, result, index,
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
pub fn read_java_script_record_property_key_texts(
    names: &[Vec<f64>],
    order: &[f64],
) -> Vec<self::ml_string::Text> {
    {
        let rows = self::read_java_script_record_property_keys(names, order);
        {
            let result = Vec::<self::ml_string::Text>::new();
            {
                let index = 0f64;
                {
                    self::source_ml_temporary_read_java_script_record_property_key_texts_loop37(
                        &(rows),
                        result,
                        index,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn create_java_script_dense_property_key_texts(length: f64) -> Vec<self::ml_string::Text> {
    if (!(self::ml_math::is_safe_integer(length)) || (length < 0f64))
        || ((length == 0f64) && ((1f64 / length) < 0f64))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                105, 110, 118, 97, 108, 105, 100, 32, 100, 101, 110, 115, 101, 32, 111, 119, 110,
                45, 102, 105, 101, 108, 100, 32, 108, 101, 110, 103, 116, 104,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        {
            let result = Vec::<self::ml_string::Text>::new();
            {
                let index = 0f64;
                {
                    self::source_ml_temporary_create_java_script_dense_property_key_texts_loop39(
                        length, result, index,
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_value_order_loop1(
    mut count: f64,
    mut reverse: bool,
    mut order: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < count {
            {
                let order_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![if reverse {
                            (count - index) - 1f64
                        } else {
                            index
                        }],
                        order,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (order, index) = (order_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { order };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRecordProjectionErrorLoop2Result {
    MlTemporaryReadRecordProjectionErrorLoop2Done,
    MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_projection_error_loop2(
    names: &[Vec<f64>],
    entries: &[bool],
    mut index: f64,
) -> self::MlTemporaryReadRecordProjectionErrorLoop2Result {
    loop {
        return if index < (names.len() as f64) {
            if ((self::ml_array::optional_at(entries, self::ml_array::number_index(index)))
                .unwrap_or_default())
            {
                if (self::accept_record_field_name(
                    &(self::read_required_record_name(names, index)),
                )) {
                    if (index > 0f64)
                        && ((self::compare_record_field_names(
                            &(self::read_required_record_name(names, (index - 1f64))),
                            &(self::read_required_record_name(names, index)),
                        )) >= 0f64)
                    {
                        self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 117, 110, 105, 113, 117, 101, 32, 102, 105, 101, 108, 100, 115, 32, 105, 110, 32, 85, 84, 70, 49, 54, 32, 111, 114, 100, 101, 114]))
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
                    self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 97, 110, 32, 105, 110, 118, 97, 108, 105, 100, 32, 85, 84, 70, 49, 54, 32, 102, 105, 101, 108, 100, 32, 110, 97, 109, 101]))
                }
            } else {
                self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Return(self::ml_string::Text::from_units(vec![99, 104, 101, 99, 107, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 112, 114, 111, 106, 101, 99, 116, 105, 111, 110, 32, 104, 97, 115, 32, 97, 32, 109, 97, 108, 102, 111, 114, 109, 101, 100, 32, 102, 105, 101, 108, 100, 32, 101, 110, 116, 114, 121]))
            }
        } else {
            self::MlTemporaryReadRecordProjectionErrorLoop2Result::MlTemporaryReadRecordProjectionErrorLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryAcceptRecordFieldNameLoop3Result {
    MlTemporaryAcceptRecordFieldNameLoop3Done,
    MlTemporaryAcceptRecordFieldNameLoop3Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_record_field_name_loop3(
    name: &[f64],
    mut index: f64,
) -> self::MlTemporaryAcceptRecordFieldNameLoop3Result {
    loop {
        return if index < (name.len() as f64) {
            {
                let unit = self::ml_array::optional_at(name, self::ml_array::number_index(index));
                if (!((unit).is_some_and(self::ml_math::is_integer)) || {
                    let source_ml_source1_comparison_operand18 = unit;
                    {
                        let source_ml_source1_comparison_operand19 = 0f64;
                        if match (source_ml_source1_comparison_operand18.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            (f64::NAN) < source_ml_source1_comparison_operand19
                        } else {
                            (source_ml_source1_comparison_operand18
                                .clone()
                                .expect("a proven present value"))
                                < source_ml_source1_comparison_operand19
                        }
                    }
                }) || {
                    let source_ml_source1_comparison_operand20 = unit;
                    {
                        let source_ml_source1_comparison_operand21 = 65_535f64;
                        if match (source_ml_source1_comparison_operand20.clone(), None::<f64>) {
                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                            (None, None) => true,
                            _ => false,
                        } {
                            (f64::NAN) > source_ml_source1_comparison_operand21
                        } else {
                            (source_ml_source1_comparison_operand20
                                .clone()
                                .expect("a proven present value"))
                                > source_ml_source1_comparison_operand21
                        }
                    }
                } {
                    self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Return(false)
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryAcceptRecordFieldNameLoop3Result::MlTemporaryAcceptRecordFieldNameLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompareRecordFieldNamesLoop4Result {
    MlTemporaryCompareRecordFieldNamesLoop4Done,
    MlTemporaryCompareRecordFieldNamesLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compare_record_field_names_loop4(
    left: &[f64],
    right: &[f64],
    mut end: f64,
    mut index: f64,
) -> self::MlTemporaryCompareRecordFieldNamesLoop4Result {
    loop {
        return if index < end {
            if (self::read_required_number_fact(left, index))
                < (self::read_required_number_fact(right, index))
            {
                self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Return(-1f64)
            } else if (self::read_required_number_fact(left, index))
                > (self::read_required_number_fact(right, index))
            {
                self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Return(1f64)
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
            self::MlTemporaryCompareRecordFieldNamesLoop4Result::MlTemporaryCompareRecordFieldNamesLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordFieldPositionLoop6Result {
    MlTemporaryReadRecordFieldPositionLoop6Done,
    MlTemporaryReadRecordFieldPositionLoop6Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_position_loop6(
    fields: &[Vec<f64>],
    name: &[f64],
    mut index: f64,
) -> self::MlTemporaryReadRecordFieldPositionLoop6Result {
    loop {
        return if index < (fields.len() as f64) {
            {
                let comparison = self::compare_record_field_names(
                    &(self::read_required_record_name(fields, index)),
                    name,
                );
                if ((comparison).partial_cmp(&0f64) != Some(std::cmp::Ordering::Less))
                    && ((comparison).partial_cmp(&0f64) != Some(std::cmp::Ordering::Greater))
                {
                    self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Return(index)
                } else {
                    {
                        let index_2 = index + 1f64;
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadRecordFieldPositionLoop6Result::MlTemporaryReadRecordFieldPositionLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordFieldPositionLoop5Result {
    MlTemporaryReadRecordFieldPositionLoop5Done,
    MlTemporaryReadRecordFieldPositionLoop5Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_position_loop5(
    fields: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporaryReadRecordFieldPositionLoop5Result {
    loop {
        return if index < (fields.len() as f64) {
            if (self::accept_record_field_name(&(self::read_required_record_name(fields, index)))) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Return(-1f64)
            }
        } else {
            self::MlTemporaryReadRecordFieldPositionLoop5Result::MlTemporaryReadRecordFieldPositionLoop5Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_sorted_field_positions_loop10(
    fields: &[Vec<f64>],
    sorted: &[f64],
    mut merged: Vec<f64>,
    mut middle: f64,
    mut end: f64,
    mut left: f64,
    mut right: f64,
) -> Vec<f64> {
    loop {
        if (left < middle) || (right < end) {
            if (right >= end)
                || ((left < middle)
                    && ((self::compare_record_field_names(
                        &(self::read_required_record_name(
                            fields,
                            self::read_required_number_fact(sorted, left),
                        )),
                        &(self::read_required_record_name(
                            fields,
                            self::read_required_number_fact(sorted, right),
                        )),
                    )) <= 0f64))
            {
                {
                    let merged_2 = {
                        let (ml_append_right, ml_append_left) =
                            (vec![self::read_required_number_fact(sorted, left)], merged);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let left_2 = left + 1f64;
                        {
                            (merged, left) = (merged_2, left_2);
                            continue;
                        }
                    }
                }
            }
            {
                {
                    let merged_3 = {
                        let (ml_append_right, ml_append_left) =
                            (vec![self::read_required_number_fact(sorted, right)], merged);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let right_2 = right + 1f64;
                        {
                            (merged, right) = (merged_3, right_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { merged };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_sorted_field_positions_loop9(
    fields: &[Vec<f64>],
    sorted: &[f64],
    mut width: f64,
    mut merged: Vec<f64>,
    mut start: f64,
) -> Vec<f64> {
    loop {
        if start < (sorted.len() as f64) {
            {
                let middle = if (start + width) < (sorted.len() as f64) {
                    start + width
                } else {
                    sorted.len() as f64
                };
                {
                    let end = if (middle + width) < (sorted.len() as f64) {
                        middle + width
                    } else {
                        sorted.len() as f64
                    };
                    {
                        let left = start;
                        {
                            let right = middle;
                            {
                                let merged_2 = self::source_ml_temporary_read_record_sorted_field_positions_loop10(fields, sorted, merged, middle, end, left, right);
                                {
                                    let start_2 = start + (width * 2f64);
                                    {
                                        (merged, start) = (merged_2, start_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { merged };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_sorted_field_positions_loop8(
    fields: &[Vec<f64>],
    mut sorted: Vec<f64>,
    mut width: f64,
) -> Vec<f64> {
    loop {
        if width < (sorted.len() as f64) {
            {
                let merged = Vec::<f64>::new();
                {
                    let start = 0f64;
                    {
                        let merged_2 =
                            self::source_ml_temporary_read_record_sorted_field_positions_loop9(
                                fields,
                                &(sorted),
                                width,
                                merged,
                                start,
                            );
                        {
                            let sorted_2 = merged_2;
                            {
                                let width_2 = width * 2f64;
                                {
                                    (sorted, width) = (sorted_2, width_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        }
        return { sorted };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordSortedFieldPositionsLoop7Result {
    MlTemporaryReadRecordSortedFieldPositionsLoop7Done,
    MlTemporaryReadRecordSortedFieldPositionsLoop7Return(Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_sorted_field_positions_loop7(
    fields: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result {
    loop {
        return if index < (fields.len() as f64) {
            if (self::accept_record_field_name(&(self::read_required_record_name(fields, index)))) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Return(vec![-1f64])
            }
        } else {
            self::MlTemporaryReadRecordSortedFieldPositionsLoop7Result::MlTemporaryReadRecordSortedFieldPositionsLoop7Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_order_loop11(
    fields: &[Vec<f64>],
    sorted: &[f64],
    mut selected: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (sorted.len() as f64) {
            if ((index + 1f64) >= (sorted.len() as f64))
                || ((self::compare_record_field_names(
                    &(self::read_required_record_name(
                        fields,
                        self::read_required_number_fact(sorted, index),
                    )),
                    &(self::read_required_record_name(
                        fields,
                        self::read_required_number_fact(sorted, (index + 1f64)),
                    )),
                )) < 0f64)
            {
                {
                    let selected_2 = {
                        let (ml_append_right, ml_append_left) = (
                            vec![self::read_required_number_fact(sorted, index)],
                            selected,
                        );
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let index_2 = index + 1f64;
                        {
                            (selected, index) = (selected_2, index_2);
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
        return { selected };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_insertion_order_loop14(
    fields: &[Vec<f64>],
    first: &[f64],
    last: &[f64],
    mut selected: Vec<f64>,
    mut index: f64,
    mut lower: f64,
    mut upper: f64,
) -> Vec<f64> {
    loop {
        if lower < upper {
            {
                let middle = ((lower + upper) / 2f64).floor();
                {
                    let comparison = self::compare_record_field_names(
                        &(self::read_required_record_name(fields, index)),
                        &(self::read_required_record_name(
                            fields,
                            self::read_required_number_fact(last, middle),
                        )),
                    );
                    if comparison < 0f64 {
                        {
                            let upper_2 = middle;
                            {
                                upper = upper_2;
                                continue;
                            }
                        }
                    }
                    {
                        if comparison > 0f64 {
                            {
                                let lower_2 = middle + 1f64;
                                {
                                    lower = lower_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let selected_3 =
                                    if ((self::read_required_number_fact(first, middle))
                                        .partial_cmp(&(index))
                                        != Some(std::cmp::Ordering::Less))
                                        && ((self::read_required_number_fact(first, middle))
                                            .partial_cmp(&(index))
                                            != Some(std::cmp::Ordering::Greater))
                                    {
                                        {
                                            {
                                                let (ml_append_right, ml_append_left) = (
                                                    vec![self::read_required_number_fact(
                                                        last, middle,
                                                    )],
                                                    selected,
                                                );
                                                self::ml_array::append(
                                                    ml_append_left,
                                                    ml_append_right,
                                                )
                                            }
                                        }
                                    } else {
                                        selected
                                    };
                                {
                                    let lower_3 = upper;
                                    {
                                        (selected, lower) = (selected_3, lower_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { selected };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_insertion_order_loop13(
    fields: &[Vec<f64>],
    first: &[f64],
    last: &[f64],
    mut selected: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (fields.len() as f64) {
            {
                let lower = 0f64;
                {
                    let upper = last.len() as f64;
                    {
                        let selected_2 =
                            self::source_ml_temporary_read_record_field_insertion_order_loop14(
                                fields, first, last, selected, index, lower, upper,
                            );
                        {
                            let index_2 = index + 1f64;
                            {
                                (selected, index) = (selected_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        }
        return { selected };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadRecordFieldInsertionOrderLoop12Result {
    MlTemporaryReadRecordFieldInsertionOrderLoop12Done(Vec<f64>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_field_insertion_order_loop12(
    fields: &[Vec<f64>],
    sorted: &[f64],
    mut first: Vec<f64>,
    mut last: Vec<f64>,
    mut start: f64,
    mut index: f64,
) -> self::MlTemporaryReadRecordFieldInsertionOrderLoop12Result {
    loop {
        if index < (sorted.len() as f64) {
            if ((index + 1f64) >= (sorted.len() as f64))
                || ((self::compare_record_field_names(
                    &(self::read_required_record_name(
                        fields,
                        self::read_required_number_fact(sorted, index),
                    )),
                    &(self::read_required_record_name(
                        fields,
                        self::read_required_number_fact(sorted, (index + 1f64)),
                    )),
                )) < 0f64)
            {
                {
                    let first_2 = {
                        let (ml_append_right, ml_append_left) =
                            (vec![self::read_required_number_fact(sorted, start)], first);
                        self::ml_array::append(ml_append_left, ml_append_right)
                    };
                    {
                        let last_2 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![self::read_required_number_fact(sorted, index)], last);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let start_2 = index + 1f64;
                            {
                                let index_2 = index + 1f64;
                                {
                                    (first, last, start, index) =
                                        (first_2, last_2, start_2, index_2);
                                    continue;
                                }
                            }
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
        return {
            self::MlTemporaryReadRecordFieldInsertionOrderLoop12Result::MlTemporaryReadRecordFieldInsertionOrderLoop12Done(first, last)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryRecordTypeFactFieldLoop15Result {
    MlTemporaryRecordTypeFactFieldLoop15Done,
    MlTemporaryRecordTypeFactFieldLoop15Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_record_type_fact_field_loop15(
    labels: &[self::ml_string::Text],
    targets: &[f64],
    name: &self::ml_string::Text,
    mut end: f64,
    mut edge: f64,
) -> self::MlTemporaryRecordTypeFactFieldLoop15Result {
    loop {
        return if edge < end {
            if (self::read_required_text_fact(labels, edge)) == (*name) {
                self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Return(self::read_required_number_fact(targets, edge))
            } else {
                {
                    let edge_2 = edge + 1f64;
                    {
                        edge = edge_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlTemporaryRecordTypeFactFieldLoop15Result::MlTemporaryRecordTypeFactFieldLoop15Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_record_type_fact_elements_loop16(
    targets: &[f64],
    mut values: Vec<f64>,
    mut end: f64,
    mut edge: f64,
) -> Vec<f64> {
    loop {
        if edge < end {
            {
                let values_2 = {
                    let (ml_append_right, ml_append_left) =
                        (vec![self::read_required_number_fact(targets, edge)], values);
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let edge_2 = edge + 1f64;
                    {
                        (values, edge) = (values_2, edge_2);
                        continue;
                    }
                }
            }
        }
        return { values };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadRecordTypeOwnerPresenceLoop17Result {
    MlTemporaryReadRecordTypeOwnerPresenceLoop17Done,
    MlTemporaryReadRecordTypeOwnerPresenceLoop17Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_record_type_owner_presence_loop19(
    seen: &[f64],
    mut node: f64,
    mut visited: bool,
    mut index: f64,
) -> bool {
    loop {
        if index < (seen.len() as f64) {
            if (self::read_required_number_fact(seen, index)) == node {
                {
                    let visited_2 = true;
                    {
                        let index_2 = index + 1f64;
                        {
                            (visited, index) = (visited_2, index_2);
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
        return { visited };
    }
}
