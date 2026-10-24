// Generated from complete js/src/translation/javascript-record-order-rules.js. Do not edit by hand.
use super::MlTemporaryReadJavaScriptRecordOrderRebaseErrorLoop34Result;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_record_order_name;
use super::read_java_script_record_order_position;
use super::read_record_field_position;

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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_property_key_texts_loop38(
    units: &[f64],
    mut key: self::ml_string::Text,
    mut at: f64,
) -> self::ml_string::Text {
    loop {
        if at < (units.len() as f64) {
            {
                let key_2 = key.concat(&{
                    let source_ml_source1_character_argument38 =
                        self::read_java_script_record_order_position(units, at);
                    self::ml_string::Text::from_units(
                        self::ml_utf16_text::character_units_utf16_text(&vec![
                            source_ml_source1_character_argument38,
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
pub fn source_ml_temporary_read_java_script_record_property_key_texts_loop37(
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
                            let key_2 = self::source_ml_temporary_read_java_script_record_property_key_texts_loop38(&(units), key, at);
                            {
                                let result_2 = {
                                    let source_ml_source1_concat_value39 = result;
                                    {
                                        let source_ml_source1_concat_value40 = vec![key_2];
                                        self::ml_array::append(
                                            self::ml_array::append(
                                                Vec::<self::ml_string::Text>::new(),
                                                source_ml_source1_concat_value39,
                                            ),
                                            source_ml_source1_concat_value40,
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
pub fn source_ml_temporary_create_java_script_dense_property_key_texts_loop39(
    mut length: f64,
    mut result: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < length {
            {
                let result_2 = {
                    let source_ml_source1_concat_value42 = result;
                    {
                        let source_ml_source1_concat_value43 = vec![{
                            let source_ml_source1_string_argument41 = index;
                            self::ml_string::Text::from_text(&self::ml_number::js_number(
                                source_ml_source1_string_argument41,
                            ))
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source1_concat_value42,
                            ),
                            source_ml_source1_concat_value43,
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
pub fn source_ml_temporary_create_java_script_string_own_property_values_loop40(
    text: &self::ml_string::Text,
    mut result: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (text.length()) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value51 = result;
                    {
                        let source_ml_source1_concat_value52 = vec![{
                            let source_ml_source1_string_unit_operand44 = text.clone();
                            {
                                let source_ml_source1_string_unit_operand45 = index;
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
                                            let source_ml_source1_string_range_operand46 = text_2.clone();
                                            {
                                                let source_ml_source1_string_range_operand47 = index_2;
                                                {
                                                    let source_ml_source1_string_range_operand48 = index_2 + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position49 = source_ml_source1_string_range_operand47;
                                                        {
                                                            let source_ml_source1_string_range_position50 = source_ml_source1_string_range_operand48;
                                                            source_ml_source1_string_range_operand46.slice(source_ml_source1_string_range_position49, source_ml_source1_string_range_position50)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand44, source_ml_source1_string_unit_operand45)
                            }
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source1_concat_value51,
                            ),
                            source_ml_source1_concat_value52,
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
