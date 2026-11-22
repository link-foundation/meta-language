// Generated from complete js/src/translation/object-rest-rules.js. Do not edit by hand.
use super::accept_record_field_name;
use super::ml_array;
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
                    let source_ml_source1_concat_value46 = result;
                    {
                        let source_ml_source1_concat_value47 =
                            vec![self::read_java_script_record_order_name(
                                names,
                                self::read_java_script_record_order_position(order, index),
                            )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<Vec<f64>>::new(),
                                source_ml_source1_concat_value46,
                            ),
                            source_ml_source1_concat_value47,
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
                    let source_ml_source1_concat_value48 = rows;
                    {
                        let source_ml_source1_concat_value49 =
                            vec![self::read_java_script_record_order_name(
                                names,
                                self::read_java_script_record_order_position(order, index),
                            )];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<Vec<f64>>::new(),
                                source_ml_source1_concat_value48,
                            ),
                            source_ml_source1_concat_value49,
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
                    let source_ml_source1_character_argument50 =
                        self::read_java_script_record_order_position(units, at);
                    self::ml_string::Text::from_units(
                        self::ml_utf16_text::character_units_utf16_text(&vec![
                            source_ml_source1_character_argument50,
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
                                    let source_ml_source1_concat_value51 = result;
                                    {
                                        let source_ml_source1_concat_value52 = vec![key_2];
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
                    }
                }
            }
        }
        return { result };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_record_property_index_loop47(
    key: &self::ml_string::Text,
    mut units: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        if index < (key.length()) {
            {
                let units_2 = {
                    let source_ml_source1_concat_value62 = units;
                    {
                        let source_ml_source1_concat_value63 = vec![{
                            let source_ml_source1_string_unit_operand53 = key.clone();
                            {
                                let source_ml_source1_string_unit_operand54 = index;
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
                                            let source_ml_source1_string_unit_operand60 = {
                                                let source_ml_source1_string_range_operand55 = text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand56 = index_2;
                                                    {
                                                        let source_ml_source1_string_range_operand57 = index_2 + 1f64;
                                                        {
                                                            let source_ml_source1_string_range_position58 = source_ml_source1_string_range_operand56;
                                                            {
                                                                let source_ml_source1_string_range_position59 = source_ml_source1_string_range_operand57;
                                                                source_ml_source1_string_range_operand55.slice(source_ml_source1_string_range_position58, source_ml_source1_string_range_position59)
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            {
                                                let source_ml_source1_string_unit_operand61 = 0f64;
                                                source_ml_source1_string_unit_operand60.code_point_at(source_ml_source1_string_unit_operand61)
                                            }
                                        }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source1_string_unit_operand53,
                                    source_ml_source1_string_unit_operand54,
                                )
                            }
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source1_concat_value62,
                            ),
                            source_ml_source1_concat_value63,
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
pub fn source_ml_temporary_create_java_script_dense_property_key_texts_loop48(
    mut length: f64,
    mut result: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < length {
            {
                let result_2 = {
                    let source_ml_source1_concat_value65 = result;
                    {
                        let source_ml_source1_concat_value66 = vec![{
                            let source_ml_source1_string_argument64 = index;
                            self::ml_string::Text::from_text(&self::ml_number::js_number(
                                source_ml_source1_string_argument64,
                            ))
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source1_concat_value65,
                            ),
                            source_ml_source1_concat_value66,
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
pub fn source_ml_temporary_create_java_script_string_own_property_values_loop49(
    text: &self::ml_string::Text,
    mut result: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        if index < (text.length()) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value74 = result;
                    {
                        let source_ml_source1_concat_value75 = vec![{
                            let source_ml_source1_string_unit_operand67 = text.clone();
                            {
                                let source_ml_source1_string_unit_operand68 = index;
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
                                            let source_ml_source1_string_range_operand69 = text_2.clone();
                                            {
                                                let source_ml_source1_string_range_operand70 = index_2;
                                                {
                                                    let source_ml_source1_string_range_operand71 = index_2 + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position72 = source_ml_source1_string_range_operand70;
                                                        {
                                                            let source_ml_source1_string_range_position73 = source_ml_source1_string_range_operand71;
                                                            source_ml_source1_string_range_operand69.slice(source_ml_source1_string_range_position72, source_ml_source1_string_range_position73)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand67, source_ml_source1_string_unit_operand68)
                            }
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source1_concat_value74,
                            ),
                            source_ml_source1_concat_value75,
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
pub fn source_ml_temporary_select_java_script_object_rest_fields_loop51(
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
                        let source_ml_source1_concat_value76 = result;
                        {
                            let source_ml_source1_concat_value77 = vec![index];
                            self::ml_array::append(
                                self::ml_array::append(
                                    Vec::<f64>::new(),
                                    source_ml_source1_concat_value76,
                                ),
                                source_ml_source1_concat_value77,
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
pub enum MlTemporarySelectJavaScriptObjectRestFieldsLoop50Result {
    MlTemporarySelectJavaScriptObjectRestFieldsLoop50Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_select_java_script_object_rest_fields_loop50(
    excluded: &[Vec<f64>],
    mut index: f64,
) -> self::MlTemporarySelectJavaScriptObjectRestFieldsLoop50Result {
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
            self::MlTemporarySelectJavaScriptObjectRestFieldsLoop50Result::MlTemporarySelectJavaScriptObjectRestFieldsLoop50Done
        };
    }
}
