// Generated from complete js/src/translation/object-rest-rules.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop30Result;
use super::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result;
use super::accept_record_field_name;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_object_rest_name;
use super::read_java_script_record_order_name;
use super::read_java_script_record_order_position;
use super::read_record_field_position;

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
                    self::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Result::MlTemporaryReadJavaScriptRecordOrderMergeErrorLoop31Return(self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 111, 117, 114, 99, 101, 32, 102, 105, 101, 108, 100]))
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
                                let source_ml_source1_concat_value31 = insertion;
                                {
                                    let source_ml_source1_concat_value32 = vec![field];
                                    self::ml_array::append(
                                        self::ml_array::append(
                                            Vec::<f64>::new(),
                                            source_ml_source1_concat_value31,
                                        ),
                                        source_ml_source1_concat_value32,
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
                        let source_ml_source1_s33 = self::source_ml_temporary_read_java_script_record_order_merge_error_loop31(names, insertion, &(values), index);
                        match source_ml_source1_s33 {
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
                            let source_ml_source1_concat_value34 = insertion;
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
    MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Return(self::ml_string::Text),
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
                self::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 101, 114, 101, 100, 32, 114, 101, 99, 111, 114, 100, 32, 115, 112, 114, 101, 97, 100, 32, 102, 105, 101, 108, 100, 32, 105, 115, 32, 109, 105, 115, 115, 105, 110, 103, 32, 102, 114, 111, 109, 32, 105, 116, 115, 32, 116, 97, 114, 103, 101, 116]))
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
                            let source_ml_source1_concat_value36 = result;
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
pub fn source_ml_temporary_copy_java_script_record_property_order_loop37(
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
                            let source_ml_source1_concat_value38 = result;
                            {
                                let source_ml_source1_concat_value39 = vec![field];
                                self::ml_array::append(
                                    self::ml_array::append(
                                        Vec::<f64>::new(),
                                        source_ml_source1_concat_value38,
                                    ),
                                    source_ml_source1_concat_value39,
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
pub enum MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Result {
    MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_copy_java_script_record_property_order_loop36(
    target: &[Vec<f64>],
    source: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Result {
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
            self::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Result::MlTemporaryCopyJavaScriptRecordPropertyOrderLoop36Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_property_keys_loop38(
    names: &[Vec<f64>],
    order: &[f64],
    mut result: Vec<Vec<f64>>,
    mut index: f64,
) -> Vec<Vec<f64>> {
    loop {
        if index < (order.len() as f64) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value40 = result;
                    {
                        let source_ml_source1_concat_value41 =
                            vec![self::read_java_script_record_order_name(
                                names,
                                self::read_java_script_record_order_position(order, index),
                            )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<Vec<f64>>::new(),
                                source_ml_source1_concat_value40,
                            ),
                            source_ml_source1_concat_value41,
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
pub fn source_ml_temporary_read_java_script_record_property_key_texts_loop40(
    units: &[f64],
    mut key: self::ml_string::Text,
    mut at: f64,
) -> self::ml_string::Text {
    loop {
        if at < (units.len() as f64) {
            {
                let key_2 = key.concat(&{
                    let source_ml_source1_character_argument42 =
                        self::read_java_script_record_order_position(units, at);
                    self::ml_string::Text::from_units(
                        self::ml_utf16_text::character_units_utf16_text(&vec![
                            source_ml_source1_character_argument42,
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
pub fn source_ml_temporary_read_java_script_record_property_key_texts_loop39(
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
                            let key_2 = self::source_ml_temporary_read_java_script_record_property_key_texts_loop40(&(units), key, at);
                            {
                                let result_2 = {
                                    let source_ml_source1_concat_value43 = result;
                                    {
                                        let source_ml_source1_concat_value44 = vec![key_2];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
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
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_property_index_loop41(
    key: &self::ml_string::Text,
    mut units: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (key.length()) {
            {
                let units_2 = {
                    let source_ml_source1_concat_value54 = units;
                    {
                        let source_ml_source1_concat_value55 = vec![{
                            let source_ml_source1_string_unit_operand45 = key.clone();
                            {
                                let source_ml_source1_string_unit_operand46 = index;
                                (std::rc::Rc::new(
                                    move |text: self::ml_string::Text, position: f64| -> f64 {
                                        {
                                            let index_2 = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index_2 < 0f64) || (index_2 >= (text.length())) {
                                                f64::NAN
                                            } else {
                                                ({
                                            let source_ml_source1_string_unit_operand52 = {
                                                let source_ml_source1_string_range_operand47 = text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand48 = index_2;
                                                    {
                                                        let source_ml_source1_string_range_operand49 = index_2 + 1f64;
                                                        {
                                                            let source_ml_source1_string_range_position50 = source_ml_source1_string_range_operand48;
                                                            {
                                                                let source_ml_source1_string_range_position51 = source_ml_source1_string_range_operand49;
                                                                source_ml_source1_string_range_operand47.slice(source_ml_source1_string_range_position50, source_ml_source1_string_range_position51)
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            {
                                                let source_ml_source1_string_unit_operand53 = 0f64;
                                                source_ml_source1_string_unit_operand52.code_point_at(source_ml_source1_string_unit_operand53)
                                            }
                                        }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source1_string_unit_operand45,
                                    source_ml_source1_string_unit_operand46,
                                )
                            }
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source1_concat_value54,
                            ),
                            source_ml_source1_concat_value55,
                        )
                    }
                };
                {
                    let index_2 = index + 1f64;
                    {
                        (units, index) = (units_2, index_2);
                        continue;
                    }
                }
            }
        }
        return { units };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_dense_property_key_texts_loop42(
    mut length: f64,
    mut result: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < length {
            {
                let result_2 = {
                    let source_ml_source1_concat_value57 = result;
                    {
                        let source_ml_source1_concat_value58 = vec![{
                            let source_ml_source1_string_argument56 = index;
                            self::ml_string::Text::from_text(&self::ml_number::js_number(
                                source_ml_source1_string_argument56,
                            ))
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source1_concat_value57,
                            ),
                            source_ml_source1_concat_value58,
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
pub fn source_ml_temporary_create_java_script_string_own_property_values_loop43(
    text: &self::ml_string::Text,
    mut result: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (text.length()) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value66 = result;
                    {
                        let source_ml_source1_concat_value67 = vec![{
                            let source_ml_source1_string_unit_operand59 = text.clone();
                            {
                                let source_ml_source1_string_unit_operand60 = index;
                                (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> self::ml_string::Text { {
                                    let index_2 = if position.is_nan() {
                                        0f64
                                    } else {
                                        position.trunc()
                                    };
                                    if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                        self::ml_string::Text::from_units(vec![])
                                    } else {
                                        {
                                            let source_ml_source1_string_range_operand61 = text_2.clone();
                                            {
                                                let source_ml_source1_string_range_operand62 = index_2;
                                                {
                                                    let source_ml_source1_string_range_operand63 = index_2 + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position64 = source_ml_source1_string_range_operand62;
                                                        {
                                                            let source_ml_source1_string_range_position65 = source_ml_source1_string_range_operand63;
                                                            source_ml_source1_string_range_operand61.slice(source_ml_source1_string_range_position64, source_ml_source1_string_range_position65)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand59, source_ml_source1_string_unit_operand60)
                            }
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source1_concat_value66,
                            ),
                            source_ml_source1_concat_value67,
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
pub fn source_ml_temporary_select_java_script_object_rest_fields_loop45(
    names: &[Vec<f64>],
    excluded: &[Vec<f64>],
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (names.len() as f64) {
            if (self::read_record_field_position(
                excluded,
                &(self::read_java_script_object_rest_name(names, index)),
            )) < 0f64
            {
                {
                    let result_2 = {
                        let source_ml_source1_concat_value68 = result;
                        {
                            let source_ml_source1_concat_value69 = vec![index];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source1_concat_value68,
                                ),
                                source_ml_source1_concat_value69,
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
        return { result };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporarySelectJavaScriptObjectRestFieldsLoop44Result {
    MlTemporarySelectJavaScriptObjectRestFieldsLoop44Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_select_java_script_object_rest_fields_loop44(
    excluded: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporarySelectJavaScriptObjectRestFieldsLoop44Result {
    loop {
        return if index < (excluded.len() as f64) {
            if (self::accept_record_field_name(
                &(self::read_java_script_object_rest_name(excluded, index)),
            )) {
                {
                    let index_2 = index + 1f64;
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
            {
                (self::ml_source_exception::SourceException {
                    class_name: String::from("TypeError"),
                    message_units: (self::ml_string::Text::from_units(vec![
                        111, 98, 106, 101, 99, 116, 32, 114, 101, 115, 116, 32, 101, 120, 99, 108,
                        117, 115, 105, 111, 110, 115, 32, 110, 101, 101, 100, 32, 101, 120, 97, 99,
                        116, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 112, 114, 111, 112,
                        101, 114, 116, 121, 32, 110, 97, 109, 101, 115,
                    ]))
                    .0,
                    message_present: true,
                })
                .raise()
            }
        } else {
            self::MlTemporarySelectJavaScriptObjectRestFieldsLoop44Result::MlTemporarySelectJavaScriptObjectRestFieldsLoop44Done
        };
    }
}
