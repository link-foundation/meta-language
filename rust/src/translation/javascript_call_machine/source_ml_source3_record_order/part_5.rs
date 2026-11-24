// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
pub fn source_ml_temporary_project_java_script_record_property_order_loop42(
    mut target: Vec<Vec<f64>>,
    mut source: Vec<Vec<f64>>,
    mut order: Vec<f64>,
    mut result: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if (index < (order.len() as f64)) {
            {
                let field = self::source_ml_source3_record_order::read_record_field_position(target.clone(), self::source_ml_source3_record_order::read_java_script_record_order_name(source.clone(), self::source_ml_source3_record_order::read_java_script_record_order_position(order.clone(), index)));
                if (field >= (0f64)) {
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
                            let index_2 = (index + 1f64);
                            {
                                (result, index) = (result_2, index_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_temporary_read_java_script_record_property_keys_loop43(
    mut names: Vec<Vec<f64>>,
    mut order: Vec<f64>,
    mut result: Vec<Vec<f64>>,
    mut index: f64,
) -> Vec<Vec<f64>> {
    loop {
        return if (index < (order.len() as f64)) {
            {
                let result_2 = {
                    let source_ml_source1_concat_value45 = result;
                    {
                        let source_ml_source1_concat_value46 = vec![self::source_ml_source3_record_order::read_java_script_record_order_name(names.clone(), self::source_ml_source3_record_order::read_java_script_record_order_position(order.clone(), index))];
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
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_temporary_read_java_script_present_record_property_key_texts_loop44(
    mut names: Vec<Vec<f64>>,
    mut order: Vec<f64>,
    mut rows: Vec<Vec<f64>>,
    mut index: f64,
) -> Vec<Vec<f64>> {
    loop {
        return if (index < (order.len() as f64)) {
            {
                let rows_2 = {
                    let source_ml_source1_concat_value47 = rows;
                    {
                        let source_ml_source1_concat_value48 = vec![self::source_ml_source3_record_order::read_java_script_record_order_name(names.clone(), self::source_ml_source3_record_order::read_java_script_record_order_position(order.clone(), index))];
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
                    let index_2 = (index + 1f64);
                    {
                        (rows, index) = (rows_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            rows
        };
    }
}

pub fn source_ml_temporary_read_java_script_record_property_name_texts_loop46(
    mut units: Vec<f64>,
    mut key: self::ml_string::Text,
    mut at: f64,
) -> self::ml_string::Text {
    loop {
        return if (at < (units.len() as f64)) {
            {
                let key_2 = key.concat(&{
                        let source_ml_source1_character_argument49 = self::source_ml_source3_record_order::read_java_script_record_order_position(units.clone(), at);
                        self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![source_ml_source1_character_argument49]).into_iter().map(|ml_unit| ml_unit as u16).collect())
                    });
                {
                    let at_2 = (at + 1f64);
                    {
                        (key, at) = (key_2.clone(), at_2);
                        continue;
                    }
                }
            }
        } else {
            key.clone()
        };
    }
}

pub fn source_ml_temporary_read_java_script_record_property_name_texts_loop45(
    mut rows: Vec<Vec<f64>>,
    mut result: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        return if (index < (rows.len() as f64)) {
            {
                let units =
                    self::source_ml_source3_record_order::read_java_script_record_order_name(
                        rows.clone(),
                        index,
                    );
                {
                    let key = self::ml_string::Text::from_units(vec![]);
                    {
                        let at = 0f64;
                        {
                            let key_2 = self::source_ml_source3_record_order::source_ml_temporary_read_java_script_record_property_name_texts_loop46(units, key.clone(), at);
                            {
                                let result_2 = {
                                    let source_ml_source1_concat_value50 = result;
                                    {
                                        let source_ml_source1_concat_value51 = vec![key_2.clone()];
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
                                    let index_2 = (index + 1f64);
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
        } else {
            result
        };
    }
}

pub fn source_ml_temporary_read_java_script_record_property_index_loop47(
    mut key: self::ml_string::Text,
    mut units: Vec<f64>,
    mut index: f64,
) -> Vec<f64> {
    loop {
        return if (index < (key.length())) {
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
                                            if ((index_2 < (0f64)) || (index_2 >= (text.length())))
                                            {
                                                f64::NAN
                                            } else {
                                                ({
                                                let source_ml_source1_string_unit_operand59 = {
                                                    let source_ml_source1_string_range_operand54 = text.clone();
                                                    {
                                                        let source_ml_source1_string_range_operand55 = index_2;
                                                        {
                                                            let source_ml_source1_string_range_operand56 = (index_2 + 1f64);
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
                                    source_ml_source1_string_unit_operand52.clone(),
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
                    let index_2 = (index + 1f64);
                    {
                        (units, index) = (units_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            units
        };
    }
}

pub fn source_ml_temporary_create_java_script_dense_property_key_texts_loop48(
    mut length: f64,
    mut result: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        return if (index < length) {
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
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

pub fn source_ml_temporary_create_java_script_string_own_property_values_loop49(
    mut text: self::ml_string::Text,
    mut result: Vec<self::ml_string::Text>,
    mut index: f64,
) -> Vec<self::ml_string::Text> {
    loop {
        return if (index < (text.length())) {
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
                                        if ((index_2 < (0f64)) || (index_2 >= (text_2.length()))) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            {
                                                let source_ml_source1_string_range_operand68 = text_2.clone();
                                                {
                                                    let source_ml_source1_string_range_operand69 = index_2;
                                                    {
                                                        let source_ml_source1_string_range_operand70 = (index_2 + 1f64);
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
                                    } }))(source_ml_source1_string_unit_operand66.clone(), source_ml_source1_string_unit_operand67)
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
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}
