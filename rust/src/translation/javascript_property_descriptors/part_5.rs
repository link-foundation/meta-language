// Generated from complete js/src/translation/javascript-property-descriptors.js. Do not edit by hand.
use super::JavaScriptSemanticValue;
use super::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_semantic_text_unit;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop2(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    loop {
        return if at < length {
            {
                let a = if match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => false,
                    self::JavaScriptSemanticValue::Null => false,
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f343) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f344) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f345) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f346) => true,
                    self::JavaScriptSemanticValue::SourceText(
                        source_ml_source1_f347,
                        source_ml_source1_f348,
                    ) => false,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f349) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f350,
                        source_ml_source1_f351,
                    ) => false,
                } {
                    ({
                        let source_ml_source1_nullish_field_receiver353 = (
                            match left.clone() {
                                self::JavaScriptSemanticValue::Undefined => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Null => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Boolean(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Number(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Integer(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Text(
                                    source_ml_source1_data_property_value352,
                                ) => Some(source_ml_source1_data_property_value352),
                                self::JavaScriptSemanticValue::SourceText(_, _) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Symbol(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Object(_, _) => None::<Vec<f64>>,
                            },
                            at,
                        );
                        {
                            let source_ml_source1_computed_property_key354 =
                                (source_ml_source1_nullish_field_receiver353.clone()).1;
                            if match (source_ml_source1_nullish_field_receiver353.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present355 =
                                        ((source_ml_source1_nullish_field_receiver353.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    self::ml_array::optional_at(
                                        &source_ml_source1_nullish_field_present355,
                                        self::ml_array::number_index(
                                            source_ml_source1_computed_property_key354,
                                        ),
                                    )
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (((self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39,
                                    ]))
                                    .concat(&self::ml_string::Text::from_text(
                                        &self::ml_number::js_number(
                                            (source_ml_source1_nullish_field_receiver353.clone()).1,
                                        ),
                                    )))
                                    .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| -1f64)
                } else if match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => false,
                    self::JavaScriptSemanticValue::Null => false,
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f356) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f357) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f358) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f359) => false,
                    self::JavaScriptSemanticValue::SourceText(
                        source_ml_source1_f360,
                        source_ml_source1_f361,
                    ) => true,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f362) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f363,
                        source_ml_source1_f364,
                    ) => false,
                } {
                    {
                        let source_ml_source1_nullish_field_receiver366 = (match left.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                            self::JavaScriptSemanticValue::SourceText(
                                _,
                                source_ml_source1_data_property_value365,
                            ) => Some(source_ml_source1_data_property_value365.clone()),
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        },);
                        if (match (source_ml_source1_nullish_field_receiver366.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present;
                                self::ml_string::Text::from_units(vec![
                                    115, 116, 114, 105, 110, 103,
                                ])
                            }
                            None => self::ml_string::Text::from_units(vec![
                                117, 110, 100, 101, 102, 105, 110, 101, 100,
                            ]),
                        }) == (self::ml_string::Text::from_units(vec![
                            115, 116, 114, 105, 110, 103,
                        ])) {
                            {
                                let source_ml_source1_nullish_field_present367 =
                                    ((source_ml_source1_nullish_field_receiver366.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let source_ml_source1_string_unit_operand368 =
                                        source_ml_source1_nullish_field_present367;
                                    {
                                        let source_ml_source1_string_unit_operand369 = at;
                                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if (index < 0f64) || (index >= (text.length())) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                        let source_ml_source1_string_unit_operand375 = {
                                                            let source_ml_source1_string_range_operand370 = text.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand371 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand372 = index + 1f64;
                                                                    {
                                                                        let source_ml_source1_string_range_position373 = source_ml_source1_string_range_operand371;
                                                                        {
                                                                            let source_ml_source1_string_range_position374 = source_ml_source1_string_range_operand372;
                                                                            source_ml_source1_string_range_operand370.slice(source_ml_source1_string_range_position373, source_ml_source1_string_range_position374)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand376 = 0f64;
                                                            source_ml_source1_string_unit_operand375.code_point_at(source_ml_source1_string_unit_operand376)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand368, source_ml_source1_string_unit_operand369)
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 99, 104, 97, 114, 67, 111, 100,
                                    101, 65, 116, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    }
                } else {
                    -1f64
                };
                {
                    let b = if match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => false,
                        self::JavaScriptSemanticValue::Null => false,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f377) => false,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f378) => false,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f379) => false,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f380) => true,
                        self::JavaScriptSemanticValue::SourceText(
                            source_ml_source1_f381,
                            source_ml_source1_f382,
                        ) => false,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f383) => false,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f384,
                            source_ml_source1_f385,
                        ) => false,
                    } {
                        ({
                            let source_ml_source1_nullish_field_receiver387 = (
                                match right.clone() {
                                    self::JavaScriptSemanticValue::Undefined => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Null => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Boolean(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Number(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Integer(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Text(
                                        source_ml_source1_data_property_value386,
                                    ) => Some(source_ml_source1_data_property_value386),
                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                        None::<Vec<f64>>
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Object(_, _) => None::<Vec<f64>>,
                                },
                                at,
                            );
                            {
                                let source_ml_source1_computed_property_key388 =
                                    (source_ml_source1_nullish_field_receiver387.clone()).1;
                                if match (source_ml_source1_nullish_field_receiver387.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source1_nullish_field_present389 =
                                            ((source_ml_source1_nullish_field_receiver387.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        self::ml_array::optional_at(
                                            &source_ml_source1_nullish_field_present389,
                                            self::ml_array::number_index(
                                                source_ml_source1_computed_property_key388,
                                            ),
                                        )
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException {
                                        class_name: String::from("TypeError"),
                                        message_units: (((self::ml_string::Text::from_units(
                                            vec![
                                                67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100,
                                                32, 112, 114, 111, 112, 101, 114, 116, 105, 101,
                                                115, 32, 111, 102, 32, 117, 110, 100, 101, 102,
                                                105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105,
                                                110, 103, 32, 39,
                                            ],
                                        ))
                                        .concat(&self::ml_string::Text::from_text(
                                            &self::ml_number::js_number(
                                                (source_ml_source1_nullish_field_receiver387
                                                    .clone())
                                                .1,
                                            ),
                                        )))
                                        .concat(&self::ml_string::Text::from_units(vec![39, 41])))
                                        .0,
                                        message_present: true,
                                    })
                                    .raise()
                                }
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| -1f64)
                    } else if match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => false,
                        self::JavaScriptSemanticValue::Null => false,
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f390) => false,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f391) => false,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f392) => false,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f393) => false,
                        self::JavaScriptSemanticValue::SourceText(
                            source_ml_source1_f394,
                            source_ml_source1_f395,
                        ) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f396) => false,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f397,
                            source_ml_source1_f398,
                        ) => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_receiver400 =
                                (match right.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Text(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::SourceText(
                                        _,
                                        source_ml_source1_data_property_value399,
                                    ) => Some(source_ml_source1_data_property_value399.clone()),
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        None::<self::ml_string::Text>
                                    }
                                },);
                            if (match (source_ml_source1_nullish_field_receiver400.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present;
                                    self::ml_string::Text::from_units(vec![
                                        115, 116, 114, 105, 110, 103,
                                    ])
                                }
                                None => self::ml_string::Text::from_units(vec![
                                    117, 110, 100, 101, 102, 105, 110, 101, 100,
                                ]),
                            }) == (self::ml_string::Text::from_units(vec![
                                115, 116, 114, 105, 110, 103,
                            ])) {
                                {
                                    let source_ml_source1_nullish_field_present401 =
                                        ((source_ml_source1_nullish_field_receiver400.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    {
                                        let source_ml_source1_string_unit_operand402 =
                                            source_ml_source1_nullish_field_present401;
                                        {
                                            let source_ml_source1_string_unit_operand403 = at;
                                            (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                    let index = if position.is_nan() {
                                                        0f64
                                                    } else {
                                                        position.trunc()
                                                    };
                                                    if (index < 0f64) || (index >= (text.length())) {
                                                        f64::NAN
                                                    } else {
                                                        ({
                                                            let source_ml_source1_string_unit_operand409 = {
                                                                let source_ml_source1_string_range_operand404 = text.clone();
                                                                {
                                                                    let source_ml_source1_string_range_operand405 = index;
                                                                    {
                                                                        let source_ml_source1_string_range_operand406 = index + 1f64;
                                                                        {
                                                                            let source_ml_source1_string_range_position407 = source_ml_source1_string_range_operand405;
                                                                            {
                                                                                let source_ml_source1_string_range_position408 = source_ml_source1_string_range_operand406;
                                                                                source_ml_source1_string_range_operand404.slice(source_ml_source1_string_range_position407, source_ml_source1_string_range_position408)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_string_unit_operand410 = 0f64;
                                                                source_ml_source1_string_unit_operand409.code_point_at(source_ml_source1_string_unit_operand410)
                                                            }
                                                        }).clone().unwrap_or_else(|| f64::NAN)
                                                    }
                                                } }))(source_ml_source1_string_unit_operand402, source_ml_source1_string_unit_operand403)
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException {
                                    class_name: String::from("TypeError"),
                                    message_units: (self::ml_string::Text::from_units(vec![
                                        67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                        114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102,
                                        32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40,
                                        114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 104, 97, 114,
                                        67, 111, 100, 101, 65, 116, 39, 41,
                                    ]))
                                    .0,
                                    message_present: true,
                                })
                                .raise()
                            }
                        }
                    } else {
                        -1f64
                    };
                    {
                        let ml_library_inequality_1 = a != b;
                        if ml_library_inequality_1 {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
                        } else {
                            {
                                let at_2 = at + 1f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_text_string_loop4(
    mut text: self::ml_string::Text,
    source_ml_temporary_values3: &[f64],
    mut source_ml_temporary_index3: self::ml::Big,
) -> self::ml_string::Text {
    loop {
        if source_ml_temporary_index3
            < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))
        {
            {
                let unit = self::ml_array::at(
                    source_ml_temporary_values3,
                    source_ml_temporary_index3.to_index(),
                );
                {
                    let text_2 = text.concat(&{
                        let source_ml_source1_character_argument411 = unit;
                        self::ml_string::Text::from_units(
                            self::ml_utf16_text::character_units_utf16_text(&vec![
                                source_ml_source1_character_argument411,
                            ])
                            .into_iter()
                            .map(|ml_unit| ml_unit as u16)
                            .collect(),
                        )
                    });
                    {
                        let source_ml_temporary_index3_2 =
                            source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_temporary_index3) =
                                (text_2, source_ml_temporary_index3_2);
                            continue;
                        }
                    }
                }
            }
        }
        return { text.clone() };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop5(
    mut value: self::JavaScriptSemanticValue,
    mut length: f64,
    mut units: Vec<f64>,
    mut at: f64,
) -> Vec<f64> {
    loop {
        if at < length {
            {
                let units_2 = {
                    let (ml_append_right, ml_append_left) = (
                        vec![self::read_java_script_semantic_text_unit(value.clone(), at)],
                        units,
                    );
                    self::ml_array::append(ml_append_left, ml_append_right)
                };
                {
                    let at_2 = at + 1f64;
                    {
                        (units, at) = (units_2, at_2);
                        continue;
                    }
                }
            }
        }
        return { units };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin6 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin6Return(
        (
            (
                bool,
                bool,
                bool,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                bool,
                bool,
            ),
            self::ml_string::Text,
            self::ml_string::Text,
            Vec<self::ml_string::Text>,
        ),
    ),
}
