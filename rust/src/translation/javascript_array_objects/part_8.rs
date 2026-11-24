// Generated from complete js/src/translation/javascript-array-objects.js. Do not edit by hand.
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_array_index;
use super::read_java_script_semantic_text_unit;
use super::test_java_script_semantic_same_value;

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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f530) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f531) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f532) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f533) => true,
                    self::JavaScriptSemanticValue::SourceText(
                        source_ml_source1_f534,
                        source_ml_source1_f535,
                    ) => false,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f536) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f537,
                        source_ml_source1_f538,
                    ) => false,
                } {
                    ({
                        let source_ml_source1_nullish_field_receiver540 = (
                            match left.clone() {
                                self::JavaScriptSemanticValue::Undefined => {
                                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                }
                                self::JavaScriptSemanticValue::Null => {
                                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                }
                                self::JavaScriptSemanticValue::Boolean(_) => {
                                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                }
                                self::JavaScriptSemanticValue::Number(_) => {
                                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                }
                                self::JavaScriptSemanticValue::Integer(_) => {
                                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                }
                                self::JavaScriptSemanticValue::Text(
                                    source_ml_source1_data_property_value539,
                                ) => Some(source_ml_source1_data_property_value539.clone()),
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                }
                            },
                            at,
                        );
                        {
                            let source_ml_source1_computed_property_key541 =
                                (source_ml_source1_nullish_field_receiver540.clone()).1;
                            if match (source_ml_source1_nullish_field_receiver540.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present542 =
                                        ((source_ml_source1_nullish_field_receiver540.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    {
                                        let source_ml_source1_array_read_value543 =
                                            source_ml_source1_nullish_field_present542;
                                        {
                                            let source_ml_source1_array_read_index544 =
                                                source_ml_source1_computed_property_key541;
                                            {
                                                let ml_reference_array164 =
                                                    source_ml_source1_array_read_value543;
                                                let ml_reference_position164 =
                                                    source_ml_source1_array_read_index544;
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array164.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position164,
                                                    ),
                                                )
                                            }
                                        }
                                    }
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
                                            (source_ml_source1_nullish_field_receiver540.clone()).1,
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f545) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f546) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f547) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f548) => false,
                    self::JavaScriptSemanticValue::SourceText(
                        source_ml_source1_f549,
                        source_ml_source1_f550,
                    ) => true,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f551) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f552,
                        source_ml_source1_f553,
                    ) => false,
                } {
                    {
                        let source_ml_source1_nullish_field_receiver555 = (match left.clone() {
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
                                source_ml_source1_data_property_value554,
                            ) => Some(source_ml_source1_data_property_value554.clone()),
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        },);
                        if (match (source_ml_source1_nullish_field_receiver555.clone()).0 {
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
                                let source_ml_source1_nullish_field_present556 =
                                    ((source_ml_source1_nullish_field_receiver555.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let source_ml_source1_string_unit_operand557 =
                                        source_ml_source1_nullish_field_present556;
                                    {
                                        let source_ml_source1_string_unit_operand558 = at;
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
                                                        let source_ml_source1_string_unit_operand564 = {
                                                            let source_ml_source1_string_range_operand559 = text.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand560 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand561 = index + 1f64;
                                                                    {
                                                                        let source_ml_source1_string_range_position562 = source_ml_source1_string_range_operand560;
                                                                        {
                                                                            let source_ml_source1_string_range_position563 = source_ml_source1_string_range_operand561;
                                                                            source_ml_source1_string_range_operand559.slice(source_ml_source1_string_range_position562, source_ml_source1_string_range_position563)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand565 = 0f64;
                                                            source_ml_source1_string_unit_operand564.code_point_at(source_ml_source1_string_unit_operand565)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand557, source_ml_source1_string_unit_operand558)
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f566) => false,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f567) => false,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f568) => false,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f569) => true,
                        self::JavaScriptSemanticValue::SourceText(
                            source_ml_source1_f570,
                            source_ml_source1_f571,
                        ) => false,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f572) => false,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f573,
                            source_ml_source1_f574,
                        ) => false,
                    } {
                        ({
                            let source_ml_source1_nullish_field_receiver576 = (
                                match right.clone() {
                                    self::JavaScriptSemanticValue::Undefined => {
                                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                    }
                                    self::JavaScriptSemanticValue::Null => {
                                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                    }
                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                    }
                                    self::JavaScriptSemanticValue::Number(_) => {
                                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                    }
                                    self::JavaScriptSemanticValue::Integer(_) => {
                                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                    }
                                    self::JavaScriptSemanticValue::Text(
                                        source_ml_source1_data_property_value575,
                                    ) => Some(source_ml_source1_data_property_value575.clone()),
                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        None::<std::rc::Rc<std::cell::RefCell<Vec<f64>>>>
                                    }
                                },
                                at,
                            );
                            {
                                let source_ml_source1_computed_property_key577 =
                                    (source_ml_source1_nullish_field_receiver576.clone()).1;
                                if match (source_ml_source1_nullish_field_receiver576.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source1_nullish_field_present578 =
                                            ((source_ml_source1_nullish_field_receiver576.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        {
                                            let source_ml_source1_array_read_value579 =
                                                source_ml_source1_nullish_field_present578;
                                            {
                                                let source_ml_source1_array_read_index580 =
                                                    source_ml_source1_computed_property_key577;
                                                {
                                                    let ml_reference_array165 =
                                                        source_ml_source1_array_read_value579;
                                                    let ml_reference_position165 =
                                                        source_ml_source1_array_read_index580;
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array165.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position165,
                                                        ),
                                                    )
                                                }
                                            }
                                        }
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
                                                (source_ml_source1_nullish_field_receiver576
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f581) => false,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f582) => false,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f583) => false,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f584) => false,
                        self::JavaScriptSemanticValue::SourceText(
                            source_ml_source1_f585,
                            source_ml_source1_f586,
                        ) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f587) => false,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f588,
                            source_ml_source1_f589,
                        ) => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_receiver591 =
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
                                        source_ml_source1_data_property_value590,
                                    ) => Some(source_ml_source1_data_property_value590.clone()),
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        None::<self::ml_string::Text>
                                    }
                                },);
                            if (match (source_ml_source1_nullish_field_receiver591.clone()).0 {
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
                                    let source_ml_source1_nullish_field_present592 =
                                        ((source_ml_source1_nullish_field_receiver591.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    {
                                        let source_ml_source1_string_unit_operand593 =
                                            source_ml_source1_nullish_field_present592;
                                        {
                                            let source_ml_source1_string_unit_operand594 = at;
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
                                                            let source_ml_source1_string_unit_operand600 = {
                                                                let source_ml_source1_string_range_operand595 = text.clone();
                                                                {
                                                                    let source_ml_source1_string_range_operand596 = index;
                                                                    {
                                                                        let source_ml_source1_string_range_operand597 = index + 1f64;
                                                                        {
                                                                            let source_ml_source1_string_range_position598 = source_ml_source1_string_range_operand596;
                                                                            {
                                                                                let source_ml_source1_string_range_position599 = source_ml_source1_string_range_operand597;
                                                                                source_ml_source1_string_range_operand595.slice(source_ml_source1_string_range_position598, source_ml_source1_string_range_position599)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_string_unit_operand601 = 0f64;
                                                                source_ml_source1_string_unit_operand600.code_point_at(source_ml_source1_string_unit_operand601)
                                                            }
                                                        }).clone().unwrap_or_else(|| f64::NAN)
                                                    }
                                                } }))(source_ml_source1_string_unit_operand593, source_ml_source1_string_unit_operand594)
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
    mut source_ml_temporary_values3: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_temporary_index3: self::ml::Big,
) -> self::ml_string::Text {
    loop {
        if source_ml_temporary_index3
            < ({
                let ml_reference_array166 = source_ml_temporary_values3.clone();
                self::ml::Big::from_u128(ml_reference_array166.borrow().len() as u128)
            })
        {
            {
                let unit = {
                    let source_ml_source1_array_read_value602 = source_ml_temporary_values3.clone();
                    {
                        let source_ml_source1_array_read_index603 =
                            source_ml_temporary_index3.clone();
                        {
                            let ml_reference_array167 = source_ml_source1_array_read_value602;
                            let ml_reference_position167 = source_ml_source1_array_read_index603;
                            self::ml_array::at(
                                &*ml_reference_array167.borrow(),
                                ml_reference_position167.to_index(),
                            )
                        }
                    }
                };
                {
                    let text_2 = text.concat(&{
                        let source_ml_source1_character_argument604 = unit;
                        self::ml_string::Text::from_units(
                            self::ml_utf16_text::character_units_utf16_text(&vec![
                                source_ml_source1_character_argument604,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result {
    MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_construct_java_script_semantic_text_units_loop5(
    mut value: self::JavaScriptSemanticValue,
    mut length: f64,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result {
    loop {
        if at < length {
            {
                let source_ml_temporary_construct_java_script_semantic_text_units_ignored6 = {
                    let ml_method_owner168 = units.clone();
                    let ml_method_argument168_0 =
                        self::read_java_script_semantic_text_unit(value.clone(), at);
                    let ml_inserted = vec![ml_method_argument168_0];
                    let mut ml_method_values168 = ml_method_owner168.borrow_mut();
                    let ml_length = ml_method_values168
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values168.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        }
        return {
            self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop5Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin7 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin7Return(
        std::rc::Rc<
            std::cell::RefCell<(
                std::rc::Rc<
                    std::cell::RefCell<(
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
                    )>,
                >,
                self::ml_string::Text,
                self::ml_string::Text,
                std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
            )>,
        >,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop8(
    mut key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut value: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Result {
    loop {
        return if at
            < ({
                let ml_reference_array169 = key.clone();
                ml_reference_array169.borrow().len() as f64
            }) {
            {
                let digit = match {
                    let source_ml_source1_array_read_value605 = key.clone();
                    {
                        let source_ml_source1_array_read_index606 = at;
                        {
                            let ml_reference_array170 = source_ml_source1_array_read_value605;
                            let ml_reference_position170 = source_ml_source1_array_read_index606;
                            self::ml_array::optional_at(
                                &*ml_reference_array170.borrow(),
                                self::ml_array::number_index(ml_reference_position170),
                            )
                        }
                    }
                } {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                } - 48f64;
                if (!(self::ml_math::is_integer(digit)) || (digit < 0f64)) || (digit > 9f64) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Return(-1f64)
                } else {
                    {
                        let value_2 = (value * 10f64) + digit;
                        {
                            let at_2 = at + 1f64;
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptArrayPropertyLoop9Result {
    MlTemporaryFindJavaScriptArrayPropertyLoop9Done,
    MlTemporaryFindJavaScriptArrayPropertyLoop9Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_array_property_loop9(
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut key: self::JavaScriptSemanticValue,
    mut at: f64,
) -> self::MlTemporaryFindJavaScriptArrayPropertyLoop9Result {
    loop {
        return if at
            < ({
                let ml_reference_array171 = properties.clone();
                ml_reference_array171.borrow().len() as f64
            }) {
            {
                let property = {
                    let source_ml_source1_array_read_value607 = properties.clone();
                    {
                        let source_ml_source1_array_read_index608 = at;
                        {
                            let ml_reference_array172 = source_ml_source1_array_read_value607;
                            let ml_reference_position172 = source_ml_source1_array_read_index608;
                            self::ml_array::optional_at(
                                &*ml_reference_array172.borrow(),
                                self::ml_array::number_index(ml_reference_position172),
                            )
                        }
                    }
                };
                if !(match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    {
                        let source_ml_source1_nullish_field_receiver609 = (property.clone(),);
                        if match (source_ml_source1_nullish_field_receiver609.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present610 =
                                    ((source_ml_source1_nullish_field_receiver609.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present610 {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        source_ml_source1_data_property_value611,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value611 =
                                            *source_ml_source1_data_property_value611;
                                        source_ml_source1_data_property_value611.clone()
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
                                    97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    },
                    key.clone(),
                ) {
                    self::MlTemporaryFindJavaScriptArrayPropertyLoop9Result::MlTemporaryFindJavaScriptArrayPropertyLoop9Return(at)
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
        } else {
            self::MlTemporaryFindJavaScriptArrayPropertyLoop9Result::MlTemporaryFindJavaScriptArrayPropertyLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptArrayObjectErrorLoop11Result {
    MlTemporaryReadJavaScriptArrayObjectErrorLoop11Done,
    MlTemporaryReadJavaScriptArrayObjectErrorLoop11Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_array_object_error_loop11(
    mut length: f64,
    mut source_ml_temporary_values10: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_temporary_index10: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptArrayObjectErrorLoop11Result {
    loop {
        return if source_ml_temporary_index10
            < ({
                let ml_reference_array173 = source_ml_temporary_values10.clone();
                self::ml::Big::from_u128(ml_reference_array173.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let source_ml_source1_array_read_value612 =
                        source_ml_temporary_values10.clone();
                    {
                        let source_ml_source1_array_read_index613 =
                            source_ml_temporary_index10.clone();
                        {
                            let ml_reference_array174 = source_ml_source1_array_read_value612;
                            let ml_reference_position174 = source_ml_source1_array_read_index613;
                            self::ml_array::at(
                                &*ml_reference_array174.borrow(),
                                ml_reference_position174.to_index(),
                            )
                        }
                    }
                };
                {
                    let index = self::read_java_script_array_index(match entry {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source1_data_property_value614,
                            _,
                        ) => {
                            let source_ml_source1_data_property_value614 =
                                *source_ml_source1_data_property_value614;
                            source_ml_source1_data_property_value614.clone()
                        }
                    });
                    if index >= length {
                        self::MlTemporaryReadJavaScriptArrayObjectErrorLoop11Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop11Return(self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 105, 110, 100, 101, 120, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 98, 101, 108, 111, 119, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104]))
                    } else {
                        {
                            let source_ml_temporary_index10_2 =
                                source_ml_temporary_index10.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index10 = source_ml_temporary_index10_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop11Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop11Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin17 {
    MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionJoin17Next,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop16Result {
    MlTemporaryPlanJavaScriptArrayOrdinaryDefinitionLoop16Done,
}
