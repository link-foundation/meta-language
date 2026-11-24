// Generated from complete js/src/translation/javascript-collection-iterator-call-source.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptCallFunction;
use super::JavaScriptCallProgram;
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f625) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f626) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f627) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f628) => true,
                    self::JavaScriptSemanticValue::SourceText(
                        source_ml_source1_f629,
                        source_ml_source1_f630,
                    ) => false,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f631) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f632,
                        source_ml_source1_f633,
                    ) => false,
                } {
                    ({
                        let source_ml_source1_nullish_field_receiver635 = (
                            match left.clone() {
                                self::JavaScriptSemanticValue::Undefined => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Null => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Boolean(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Number(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Integer(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Text(
                                    source_ml_source1_data_property_value634,
                                ) => Some(source_ml_source1_data_property_value634),
                                self::JavaScriptSemanticValue::SourceText(_, _) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Symbol(_) => None::<Vec<f64>>,
                                self::JavaScriptSemanticValue::Object(_, _) => None::<Vec<f64>>,
                            },
                            at,
                        );
                        {
                            let source_ml_source1_computed_property_key636 =
                                (source_ml_source1_nullish_field_receiver635.clone()).1;
                            if match (source_ml_source1_nullish_field_receiver635.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                {
                                    let source_ml_source1_nullish_field_present637 =
                                        ((source_ml_source1_nullish_field_receiver635.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    self::ml_array::optional_at(
                                        &source_ml_source1_nullish_field_present637,
                                        self::ml_array::number_index(
                                            source_ml_source1_computed_property_key636,
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
                                            (source_ml_source1_nullish_field_receiver635.clone()).1,
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
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f638) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f639) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f640) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f641) => false,
                    self::JavaScriptSemanticValue::SourceText(
                        source_ml_source1_f642,
                        source_ml_source1_f643,
                    ) => true,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f644) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f645,
                        source_ml_source1_f646,
                    ) => false,
                } {
                    {
                        let source_ml_source1_nullish_field_receiver648 = (match left.clone() {
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
                                source_ml_source1_data_property_value647,
                            ) => Some(source_ml_source1_data_property_value647.clone()),
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        },);
                        if (match (source_ml_source1_nullish_field_receiver648.clone()).0 {
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
                                let source_ml_source1_nullish_field_present649 =
                                    ((source_ml_source1_nullish_field_receiver648.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                {
                                    let source_ml_source1_string_unit_operand650 =
                                        source_ml_source1_nullish_field_present649;
                                    {
                                        let source_ml_source1_string_unit_operand651 = at;
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
                                                        let source_ml_source1_string_unit_operand657 = {
                                                            let source_ml_source1_string_range_operand652 = text.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand653 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand654 = index + 1f64;
                                                                    {
                                                                        let source_ml_source1_string_range_position655 = source_ml_source1_string_range_operand653;
                                                                        {
                                                                            let source_ml_source1_string_range_position656 = source_ml_source1_string_range_operand654;
                                                                            source_ml_source1_string_range_operand652.slice(source_ml_source1_string_range_position655, source_ml_source1_string_range_position656)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand658 = 0f64;
                                                            source_ml_source1_string_unit_operand657.code_point_at(source_ml_source1_string_unit_operand658)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand650, source_ml_source1_string_unit_operand651)
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f659) => false,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f660) => false,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f661) => false,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f662) => true,
                        self::JavaScriptSemanticValue::SourceText(
                            source_ml_source1_f663,
                            source_ml_source1_f664,
                        ) => false,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f665) => false,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f666,
                            source_ml_source1_f667,
                        ) => false,
                    } {
                        ({
                            let source_ml_source1_nullish_field_receiver669 = (
                                match right.clone() {
                                    self::JavaScriptSemanticValue::Undefined => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Null => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Boolean(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Number(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Integer(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Text(
                                        source_ml_source1_data_property_value668,
                                    ) => Some(source_ml_source1_data_property_value668),
                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                        None::<Vec<f64>>
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => None::<Vec<f64>>,
                                    self::JavaScriptSemanticValue::Object(_, _) => None::<Vec<f64>>,
                                },
                                at,
                            );
                            {
                                let source_ml_source1_computed_property_key670 =
                                    (source_ml_source1_nullish_field_receiver669.clone()).1;
                                if match (source_ml_source1_nullish_field_receiver669.clone()).0 {
                                    Some(ml_present) => {
                                        let _ = ml_present.clone();
                                        true
                                    }
                                    None => false,
                                } {
                                    {
                                        let source_ml_source1_nullish_field_present671 =
                                            ((source_ml_source1_nullish_field_receiver669.clone())
                                                .0)
                                                .clone()
                                                .expect("a proven present value");
                                        self::ml_array::optional_at(
                                            &source_ml_source1_nullish_field_present671,
                                            self::ml_array::number_index(
                                                source_ml_source1_computed_property_key670,
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
                                                (source_ml_source1_nullish_field_receiver669
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
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f672) => false,
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f673) => false,
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f674) => false,
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f675) => false,
                        self::JavaScriptSemanticValue::SourceText(
                            source_ml_source1_f676,
                            source_ml_source1_f677,
                        ) => true,
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f678) => false,
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f679,
                            source_ml_source1_f680,
                        ) => false,
                    } {
                        {
                            let source_ml_source1_nullish_field_receiver682 =
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
                                        source_ml_source1_data_property_value681,
                                    ) => Some(source_ml_source1_data_property_value681.clone()),
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        None::<self::ml_string::Text>
                                    }
                                },);
                            if (match (source_ml_source1_nullish_field_receiver682.clone()).0 {
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
                                    let source_ml_source1_nullish_field_present683 =
                                        ((source_ml_source1_nullish_field_receiver682.clone()).0)
                                            .clone()
                                            .expect("a proven present value");
                                    {
                                        let source_ml_source1_string_unit_operand684 =
                                            source_ml_source1_nullish_field_present683;
                                        {
                                            let source_ml_source1_string_unit_operand685 = at;
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
                                                            let source_ml_source1_string_unit_operand691 = {
                                                                let source_ml_source1_string_range_operand686 = text.clone();
                                                                {
                                                                    let source_ml_source1_string_range_operand687 = index;
                                                                    {
                                                                        let source_ml_source1_string_range_operand688 = index + 1f64;
                                                                        {
                                                                            let source_ml_source1_string_range_position689 = source_ml_source1_string_range_operand687;
                                                                            {
                                                                                let source_ml_source1_string_range_position690 = source_ml_source1_string_range_operand688;
                                                                                source_ml_source1_string_range_operand686.slice(source_ml_source1_string_range_position689, source_ml_source1_string_range_position690)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_string_unit_operand692 = 0f64;
                                                                source_ml_source1_string_unit_operand691.code_point_at(source_ml_source1_string_unit_operand692)
                                                            }
                                                        }).clone().unwrap_or_else(|| f64::NAN)
                                                    }
                                                } }))(source_ml_source1_string_unit_operand684, source_ml_source1_string_unit_operand685)
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
                        let source_ml_source1_character_argument693 = unit;
                        self::ml_string::Text::from_units(
                            self::ml_utf16_text::character_units_utf16_text(&vec![
                                source_ml_source1_character_argument693,
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
pub enum MlTemporaryFindJavaScriptCallConstructorFactLoop6Result {
    MlTemporaryFindJavaScriptCallConstructorFactLoop6Done,
    MlTemporaryFindJavaScriptCallConstructorFactLoop6Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_call_constructor_fact_loop6(
    mut facts: Vec<self::JavaScriptArrayConstructorFact>,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryFindJavaScriptCallConstructorFactLoop6Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let fact =
                        self::ml_array::optional_at(&facts, self::ml_array::number_index(middle));
                    if match (fact.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryFindJavaScriptCallConstructorFactLoop6Result::MlTemporaryFindJavaScriptCallConstructorFactLoop6Return(-1f64)
                    } else if (match fact.clone().expect("a proven present value") {
                        self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                            source_ml_source1_data_property_value695,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value695.clone(),
                    }) == identity
                    {
                        self::MlTemporaryFindJavaScriptCallConstructorFactLoop6Result::MlTemporaryFindJavaScriptCallConstructorFactLoop6Return(middle)
                    } else {
                        if (match fact.clone().expect("a proven present value") {
                            self::JavaScriptArrayConstructorFact::ArrayConstructorFact(
                                source_ml_source1_data_property_value697,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value697.clone(),
                        }) < identity
                        {
                            {
                                let low_2 = middle + 1f64;
                                {
                                    low = low_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let high_2 = middle;
                                {
                                    high = high_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptCallConstructorFactLoop6Result::MlTemporaryFindJavaScriptCallConstructorFactLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptCallCodeFunctionLoop7Result {
    MlTemporaryFindJavaScriptCallCodeFunctionLoop7Done,
    MlTemporaryFindJavaScriptCallCodeFunctionLoop7Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_call_code_function_loop7(
    mut program: self::JavaScriptCallProgram,
    mut identity: self::ml::Big,
    mut low: f64,
    mut high: f64,
) -> self::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Result {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let entry = self::ml_array::optional_at(
                        (match &program {
                            self::JavaScriptCallProgram::CallProgram(
                                source_ml_source1_data_property_value698,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value698.as_slice(),
                        }),
                        self::ml_array::number_index(middle),
                    );
                    if match (entry.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Return(-1f64)
                    } else if (match entry.clone().expect("a proven present value") {
                        self::JavaScriptCallFunction::CallFunction(
                            source_ml_source1_data_property_value700,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source1_data_property_value700.clone(),
                    }) == identity
                    {
                        self::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Return(middle)
                    } else {
                        if (match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(
                                source_ml_source1_data_property_value702,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source1_data_property_value702.clone(),
                        }) < identity
                        {
                            {
                                let low_2 = middle + 1f64;
                                {
                                    low = low_2;
                                    continue;
                                }
                            }
                        }
                        {
                            {
                                let high_2 = middle;
                                {
                                    high = high_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Result {
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Done(Vec<self::JavaScriptCallFunction>),
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop8Return(Vec<self::JavaScriptCallFunction>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Result {
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Done(Vec<self::JavaScriptCallFunction>),
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop9Return(Vec<self::JavaScriptCallFunction>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Result {
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Done(Vec<self::JavaScriptCallFunction>),
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Return(Vec<self::JavaScriptCallFunction>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_call_code_functions_loop12(
    mut functions: Vec<self::JavaScriptCallFunction>,
    mut ordered: Vec<self::JavaScriptCallFunction>,
    mut merged: Vec<self::JavaScriptCallFunction>,
    mut end: f64,
    mut index: f64,
) -> self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Result {
    loop {
        return if index < end {
            {
                let value =
                    self::ml_array::optional_at(&ordered, self::ml_array::number_index(index));
                if match (value.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Return(functions)
                } else {
                    {
                        let merged_2 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![value.clone().expect("a proven present value")], merged);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (merged, index) = (merged_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop12Done(merged)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Result {
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Done(Vec<self::JavaScriptCallFunction>),
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Return(Vec<self::JavaScriptCallFunction>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_order_java_script_call_code_functions_loop11(
    mut functions: Vec<self::JavaScriptCallFunction>,
    mut ordered: Vec<self::JavaScriptCallFunction>,
    mut merged: Vec<self::JavaScriptCallFunction>,
    mut middle: f64,
    mut index: f64,
) -> self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Result {
    loop {
        return if index < middle {
            {
                let value =
                    self::ml_array::optional_at(&ordered, self::ml_array::number_index(index));
                if match (value.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Return(functions)
                } else {
                    {
                        let merged_2 = {
                            let (ml_append_right, ml_append_left) =
                                (vec![value.clone().expect("a proven present value")], merged);
                            self::ml_array::append(ml_append_left, ml_append_right)
                        };
                        {
                            let index_2 = index + 1f64;
                            {
                                (merged, index) = (merged_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Result::MlTemporaryOrderJavaScriptCallCodeFunctionsLoop11Done(merged)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Result {
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Done(
        Vec<self::JavaScriptCallFunction>,
        f64,
        f64,
    ),
    MlTemporaryOrderJavaScriptCallCodeFunctionsLoop10Return(Vec<self::JavaScriptCallFunction>),
}
