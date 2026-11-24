// Generated from complete js/src/translation/javascript-record-order-rules.js. Do not edit by hand.
use super::ml_array;
use super::ml_number;
use super::ml_string;

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
                    let source_ml_source1_concat_value61 = units;
                    {
                        let source_ml_source1_concat_value62 = vec![{
                            let source_ml_source1_string_unit_operand52 = key.clone();
                            {
                                let source_ml_source1_string_unit_operand53 = index;
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
                                            let source_ml_source1_string_unit_operand59 = {
                                                let source_ml_source1_string_range_operand54 = text.clone();
                                                {
                                                    let source_ml_source1_string_range_operand55 = index_2;
                                                    {
                                                        let source_ml_source1_string_range_operand56 = index_2 + 1f64;
                                                        {
                                                            let source_ml_source1_string_range_position57 = source_ml_source1_string_range_operand55;
                                                            {
                                                                let source_ml_source1_string_range_position58 = source_ml_source1_string_range_operand56;
                                                                source_ml_source1_string_range_operand54.slice(source_ml_source1_string_range_position57, source_ml_source1_string_range_position58)
                                                            }
                                                        }
                                                    }
                                                }
                                            };
                                            {
                                                let source_ml_source1_string_unit_operand60 = 0f64;
                                                source_ml_source1_string_unit_operand59.code_point_at(source_ml_source1_string_unit_operand60)
                                            }
                                        }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source1_string_unit_operand52,
                                    source_ml_source1_string_unit_operand53,
                                )
                            }
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<f64>::new(),
                                source_ml_source1_concat_value61,
                            ),
                            source_ml_source1_concat_value62,
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
                    let source_ml_source1_concat_value64 = result;
                    {
                        let source_ml_source1_concat_value65 = vec![{
                            let source_ml_source1_string_argument63 = index;
                            self::ml_string::Text::from_text(&self::ml_number::js_number(
                                source_ml_source1_string_argument63,
                            ))
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source1_concat_value64,
                            ),
                            source_ml_source1_concat_value65,
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
                    let source_ml_source1_concat_value73 = result;
                    {
                        let source_ml_source1_concat_value74 = vec![{
                            let source_ml_source1_string_unit_operand66 = text.clone();
                            {
                                let source_ml_source1_string_unit_operand67 = index;
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
                                            let source_ml_source1_string_range_operand68 = text_2.clone();
                                            {
                                                let source_ml_source1_string_range_operand69 = index_2;
                                                {
                                                    let source_ml_source1_string_range_operand70 = index_2 + 1f64;
                                                    {
                                                        let source_ml_source1_string_range_position71 = source_ml_source1_string_range_operand69;
                                                        {
                                                            let source_ml_source1_string_range_position72 = source_ml_source1_string_range_operand70;
                                                            source_ml_source1_string_range_operand68.slice(source_ml_source1_string_range_position71, source_ml_source1_string_range_position72)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } }))(source_ml_source1_string_unit_operand66, source_ml_source1_string_unit_operand67)
                            }
                        }];
                        self::ml_array::append(
                            self::ml_array::append(
                                Vec::<self::ml_string::Text>::new(),
                                source_ml_source1_concat_value73,
                            ),
                            source_ml_source1_concat_value74,
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
