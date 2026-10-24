// Generated from complete js/src/translation/javascript-record-order-rules.js. Do not edit by hand.
use super::ml_array;
use super::ml_math;
use super::read_java_script_literal_property_index;
use super::read_java_script_record_order_name;
use super::read_java_script_record_order_position;
use super::read_record_field_position;
use super::source_ml_temporary_create_java_script_record_property_order_loop27;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_record_property_order_loop26(
    names: &[Vec<f64>],
    mut numeric: Vec<f64>,
    mut field: f64,
) -> Vec<f64> {
    loop {
        if field < (names.len() as f64) {
            {
                let key = self::read_java_script_literal_property_index(
                    &(self::read_java_script_record_order_name(names, field)),
                );
                if key < 0f64 {
                    {
                        let field_2 = field + 1f64;
                        {
                            field = field_2;
                            continue;
                        }
                    }
                }
                {
                    {
                        let position = 0f64;
                        {
                            let position_2 = self::source_ml_temporary_create_java_script_record_property_order_loop27(names, &(numeric), key, position);
                            {
                                let numeric_2 = {
                                    let source_ml_source1_concat_value24 =
                                        self::ml_array::slice(&numeric, 0f64, position_2);
                                    {
                                        let source_ml_source1_concat_value25 = vec![field];
                                        {
                                            let source_ml_source1_concat_value26 =
                                                self::ml_array::slice(
                                                    &numeric,
                                                    position_2,
                                                    f64::INFINITY,
                                                );
                                            self::ml_array::append(
                                                self::ml_array::append(
                                                    self::ml_array::append(
                                                        Vec::<f64>::new(),
                                                        source_ml_source1_concat_value24,
                                                    ),
                                                    source_ml_source1_concat_value25,
                                                ),
                                                source_ml_source1_concat_value26,
                                            )
                                        }
                                    }
                                };
                                {
                                    let field_3 = field + 1f64;
                                    {
                                        (numeric, field) = (numeric_2, field_3);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        return { numeric };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_record_property_order_loop28(
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
                            let source_ml_source1_concat_value27 = result;
                            {
                                let source_ml_source1_concat_value28 = vec![field];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value27,
                                    ),
                                    source_ml_source1_concat_value28,
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
pub enum MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result {
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Done,
    MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_accept_java_script_record_property_order_loop29(
    order: &[f64],
    expected: &[f64],
    mut index: f64,
) -> self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result {
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
                self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Return(false)
            }
        } else {
            self::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Result::MlTemporaryAcceptJavaScriptRecordPropertyOrderLoop29Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Return(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result {
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Done(Vec<f64>),
    MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop31(
    names: &[Vec<f64>],
    mut insertion: Vec<f64>,
    values: &[f64],
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result {
    loop {
        return if index < (values.len() as f64) {
            {
                let field = self::read_java_script_record_order_position(values, index);
                if ((!(self::ml_math::is_integer(field)) || (field < 0f64))
                    || (field >= (names.len() as f64)))
                    || ((field == 0f64) && ((1f64 / field) < 0f64))
                {
                    self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(String::from("invalid ordered record source field"))
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
                                let source_ml_source1_concat_value29 = insertion;
                                {
                                    let source_ml_source1_concat_value30 = vec![field];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value29,
                                        ),
                                        source_ml_source1_concat_value30,
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
            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Done(insertion)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_merge_error_loop30(
    names: &[Vec<f64>],
    chunks: &[Vec<f64>],
    mut insertion: Vec<f64>,
    mut chunk: f64,
) -> self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result {
    loop {
        return if chunk < (chunks.len() as f64) {
            {
                let values = self::read_java_script_record_order_name(chunks, chunk);
                {
                    let index = 0f64;
                    {
                        let source_ml_source1_s31 = self::source_ml_temporary_read_java_script_record_order_merge_error_loop31(names, insertion, &(values), index);
                        match source_ml_source1_s31 {
                            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Done(insertion_2) => {
                                {
                                    let chunk_2 = chunk + 1f64;
                                    {
                                        (insertion, chunk) = (insertion_2, chunk_2);
                                        continue;
                                    }
                                }
                            }
                            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(source_ml_temporary_result31) => {
                                self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Return(source_ml_temporary_result31)
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Done(insertion)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_merge_java_script_record_property_order_loop33(
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
                            let source_ml_source1_concat_value32 = insertion;
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
pub fn source_ml_temporary_merge_java_script_record_property_order_loop32(
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
                        let insertion_2 = self::source_ml_temporary_merge_java_script_record_property_order_loop33(insertion, &(values), index);
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result {
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Done,
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_order_rebase_error_loop34(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result {
    loop {
        return if index < (source.len() as f64) {
            if (self::read_record_field_position(
                target,
                &(self::read_java_script_record_order_name(source, index)),
            )) < 0f64
            {
                self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Return(String::from("an ordered record spread field is missing from its target"))
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
            self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_rebase_java_script_record_property_order_loop35(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    order: &[f64],
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (order.len() as f64) {
            {
                let name = self::read_java_script_record_order_name(
                    source,
                    self::read_java_script_record_order_position(order, index),
                );
                {
                    let field = self::read_record_field_position(target, &(name));
                    {
                        let result_2 = {
                            let source_ml_source1_concat_value34 = result;
                            {
                                let source_ml_source1_concat_value35 = vec![field];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value34,
                                    ),
                                    source_ml_source1_concat_value35,
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
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_property_keys_loop36(
    names: &[Vec<f64>],
    order: &[f64],
    mut result: Vec<Vec<f64>>,
    mut index: f64,
) -> Vec<Vec<f64>> {
    loop {
        if index < (order.len() as f64) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value36 = result;
                    {
                        let source_ml_source1_concat_value37 =
                            vec![self::read_java_script_record_order_name(
                                names,
                                self::read_java_script_record_order_position(order, index),
                            )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<Vec<f64>>::new(),
                                source_ml_source1_concat_value36,
                            ),
                            source_ml_source1_concat_value37,
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
