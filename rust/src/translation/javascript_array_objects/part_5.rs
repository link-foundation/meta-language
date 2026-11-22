// Generated from complete js/src/translation/javascript-array-objects.js. Do not edit by hand.
use super::JavaScriptArrayObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptArrayPropertyLoop9Result;
use super::MlTemporaryReadJavaScriptArrayObjectErrorLoop11Result;
use super::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Result;
use super::construct_java_script_semantic_text_units;
use super::create_java_script_property_descriptor;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_text_length;
use super::source_ml_temporary_find_java_script_array_property_loop9;
use super::source_ml_temporary_read_java_script_array_object_error_loop11;
use super::source_ml_temporary_read_java_script_literal_property_index_loop8;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_literal_property_index(
    key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> f64 {
    if (({
        let ml_reference_array122 = key.clone();
        ml_reference_array122.borrow().len() as f64
    }) == 0f64)
        || (({
            let ml_reference_array123 = key.clone();
            ml_reference_array123.borrow().len() as f64
        }) > 10f64)
    {
        -1f64
    } else {
        if (({
            let ml_reference_array124 = key.clone();
            ml_reference_array124.borrow().len() as f64
        }) > 1f64)
            && {
                let source_ml_source1_comparison_operand355 = {
                    let source_ml_source1_array_read_value353 = key.clone();
                    {
                        let source_ml_source1_array_read_index354 = 0f64;
                        {
                            let ml_reference_array125 = source_ml_source1_array_read_value353;
                            let ml_reference_position125 = source_ml_source1_array_read_index354;
                            self::ml_array::optional_at(
                                &*ml_reference_array125.borrow(),
                                self::ml_array::number_index(ml_reference_position125),
                            )
                        }
                    }
                };
                {
                    let source_ml_source1_comparison_operand356 = 48f64;
                    match (
                        source_ml_source1_comparison_operand355,
                        source_ml_source1_comparison_operand356,
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    }
                }
            }
        {
            -1f64
        } else {
            {
                let value = 0f64;
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s357 =
                            self::source_ml_temporary_read_java_script_literal_property_index_loop8(
                                key.clone(),
                                value,
                                at,
                            );
                        match source_ml_source1_s357 {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Done(value_2) => {
                                if value_2 <= 4_294_967_294f64 {
                                    value_2
                                } else {
                                    -1f64
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop8Return(source_ml_temporary_result8) => {
                                source_ml_temporary_result8
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_literal_property_form(
    kind: &self::ml_string::Text,
    index: f64,
) -> self::ml_string::Text {
    if ((((*kind) == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![116, 117, 112, 108, 101]))))
        || ((*kind) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))))
        && (index >= 0f64)
    {
        self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120])
    } else {
        self::ml_string::Text::from_units(vec![102, 105, 101, 108, 100])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_string_index_number_form(
    kind: &self::ml_string::Text,
    element: &self::ml_string::Text,
) -> self::ml_string::Text {
    if (*kind) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])) {
        self::ml_string::Text::from_units(vec![105, 100, 101, 110, 116, 105, 116, 121])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![105, 110, 116])))
        || ((*kind) == (self::ml_string::Text::from_units(vec![110, 97, 116])))
    {
        self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116])))
        || (((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
            && ((*element) == (self::ml_string::Text::from_units(vec![117, 110, 105, 116]))))
    {
        self::ml_string::Text::from_units(vec![97, 98, 115, 101, 110, 116])
    } else if ((*kind) == (self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110])))
        && ((((*element) == (self::ml_string::Text::from_units(vec![102, 108, 111, 97, 116])))
            || ((*element) == (self::ml_string::Text::from_units(vec![105, 110, 116]))))
            || ((*element) == (self::ml_string::Text::from_units(vec![110, 97, 116]))))
    {
        self::ml_string::Text::from_units(vec![111, 112, 116, 105, 111, 110, 97, 108])
    } else {
        self::ml_string::Text::from_units(vec![114, 101, 102, 117, 115, 101])
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_length_key() -> self::JavaScriptSemanticValue {
    self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![
        108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
    ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_length_key(key: self::JavaScriptSemanticValue) -> bool {
    self::test_java_script_semantic_same_value(
        key.clone(),
        self::read_java_script_array_length_key(),
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_index(key: self::JavaScriptSemanticValue) -> f64 {
    {
        let length = self::read_java_script_semantic_text_length(key.clone());
        if (length < 0f64) || (length > 10f64) {
            -1f64
        } else {
            self::read_java_script_literal_property_index(
                self::construct_java_script_semantic_text_units(key.clone(), 10f64),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_array_property(
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source1_s358 =
                self::source_ml_temporary_find_java_script_array_property_loop9(
                    properties.clone(),
                    key.clone(),
                    at,
                );
            match source_ml_source1_s358 {
                self::MlTemporaryFindJavaScriptArrayPropertyLoop9Result::MlTemporaryFindJavaScriptArrayPropertyLoop9Done => {
                    -1f64
                }
                self::MlTemporaryFindJavaScriptArrayPropertyLoop9Result::MlTemporaryFindJavaScriptArrayPropertyLoop9Return(source_ml_temporary_result9) => {
                    source_ml_temporary_result9
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_length(object: self::JavaScriptArrayObject) -> f64 {
    {
        let property = {
            let source_ml_source1_array_read_value361 = match object.clone() {
                self::JavaScriptArrayObject::ArrayObject(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value359,
                ) => source_ml_source1_data_property_value359.clone(),
            };
            {
                let source_ml_source1_array_read_index362 = self::find_java_script_array_property(
                    match object.clone() {
                        self::JavaScriptArrayObject::ArrayObject(
                            _,
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value360,
                        ) => source_ml_source1_data_property_value360.clone(),
                    },
                    self::read_java_script_array_length_key(),
                );
                {
                    let ml_reference_array126 = source_ml_source1_array_read_value361;
                    let ml_reference_position126 = source_ml_source1_array_read_index362;
                    self::ml_array::optional_at(
                        &*ml_reference_array126.borrow(),
                        self::ml_array::number_index(ml_reference_position126),
                    )
                }
            }
        };
        if match (property.clone(), None::<()>) {
            (Some(_ml_left), Some(_ml_right)) => false,
            (None, None) => true,
            _ => false,
        } {
            -1f64
        } else {
            self::read_java_script_array_length_value(
                ({
                    (match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            _,
                            source_ml_source1_data_property_value363,
                        ) => source_ml_source1_data_property_value363.clone(),
                    })
                    .borrow()
                    .clone()
                })
                .8,
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_length_value(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => -1f64,
        self::JavaScriptSemanticValue::Null => -1f64,
        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f364) => -1f64,
        self::JavaScriptSemanticValue::Number(value_value) => value_value,
        self::JavaScriptSemanticValue::Integer(source_ml_source1_f365) => -1f64,
        self::JavaScriptSemanticValue::Text(source_ml_source1_f366) => -1f64,
        self::JavaScriptSemanticValue::SourceText(
            source_ml_source1_f367,
            source_ml_source1_f368,
        ) => -1f64,
        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f369) => -1f64,
        self::JavaScriptSemanticValue::Object(source_ml_source1_f370, source_ml_source1_f371) => {
            -1f64
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_length(length: f64) -> bool {
    ((self::ml_math::is_integer(length) && (length >= 0f64)) && (length <= 4_294_967_295f64))
        && !({
            let source_ml_source1_same_value_operand372 = length;
            {
                let source_ml_source1_same_value_operand373 = -0f64;
                ((((source_ml_source1_same_value_operand372)
                    .partial_cmp(&(source_ml_source1_same_value_operand373))
                    == Some(std::cmp::Ordering::Equal))
                    && ((source_ml_source1_same_value_operand372 != 0f64)
                        || ((1f64 / source_ml_source1_same_value_operand372)
                            == (1f64 / source_ml_source1_same_value_operand373))))
                    || ((source_ml_source1_same_value_operand372
                        != source_ml_source1_same_value_operand372)
                        && (source_ml_source1_same_value_operand373
                            != source_ml_source1_same_value_operand373)))
            }
        })
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_object_error(
    object: self::JavaScriptArrayObject,
) -> self::ml_string::Text {
    if match object.clone() {
        self::JavaScriptArrayObject::ArrayObject(
            _,
            _,
            _,
            source_ml_source1_data_property_value374,
            _,
        ) => source_ml_source1_data_property_value374,
    } {
        self::ml_string::Text::from_units(vec![
            97, 110, 32, 65, 114, 114, 97, 121, 32, 101, 120, 111, 116, 105, 99, 32, 111, 98, 106,
            101, 99, 116, 32, 105, 115, 32, 110, 111, 116, 32, 99, 97, 108, 108, 97, 98, 108, 101,
        ])
    } else {
        {
            let property = {
                let source_ml_source1_array_read_value377 = match object.clone() {
                    self::JavaScriptArrayObject::ArrayObject(
                        _,
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value375,
                    ) => source_ml_source1_data_property_value375.clone(),
                };
                {
                    let source_ml_source1_array_read_index378 =
                        self::find_java_script_array_property(
                            match object.clone() {
                                self::JavaScriptArrayObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source1_data_property_value376,
                                ) => source_ml_source1_data_property_value376.clone(),
                            },
                            self::read_java_script_array_length_key(),
                        );
                    {
                        let ml_reference_array128 = source_ml_source1_array_read_value377;
                        let ml_reference_position128 = source_ml_source1_array_read_index378;
                        self::ml_array::optional_at(
                            &*ml_reference_array128.borrow(),
                            self::ml_array::number_index(ml_reference_position128),
                        )
                    }
                }
            };
            {
                let invalid = self::ml_string::Text::from_units(vec![
                    97, 110, 32, 65, 114, 114, 97, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 101, 110, 117,
                    109, 101, 114, 97, 98, 108, 101, 32, 110, 111, 110, 99, 111, 110, 102, 105,
                    103, 117, 114, 97, 98, 108, 101, 32, 105, 110, 116, 101, 103, 114, 97, 108, 32,
                    108, 101, 110, 103, 116, 104, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112,
                    101, 114, 116, 121,
                ]);
                if ((((match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } || !(({
                    ({
                        let source_ml_source1_nullish_field_receiver379 = (property.clone(),);
                        if match (source_ml_source1_nullish_field_receiver379.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present380 =
                                    ((source_ml_source1_nullish_field_receiver379.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present380 {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        _,
                                        source_ml_source1_data_property_value381,
                                    ) => source_ml_source1_data_property_value381.clone(),
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105,
                                    112, 116, 111, 114, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    })
                    .borrow()
                    .clone()
                })
                .9)) || ({
                    ({
                        let source_ml_source1_nullish_field_receiver382 = (property.clone(),);
                        if match (source_ml_source1_nullish_field_receiver382.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present383 =
                                    ((source_ml_source1_nullish_field_receiver382.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present383 {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        _,
                                        source_ml_source1_data_property_value384,
                                    ) => source_ml_source1_data_property_value384.clone(),
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105,
                                    112, 116, 111, 114, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    })
                    .borrow()
                    .clone()
                })
                .5) || ({
                    ({
                        let source_ml_source1_nullish_field_receiver385 = (property.clone(),);
                        if match (source_ml_source1_nullish_field_receiver385.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present386 =
                                    ((source_ml_source1_nullish_field_receiver385.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present386 {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        _,
                                        source_ml_source1_data_property_value387,
                                    ) => source_ml_source1_data_property_value387.clone(),
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105,
                                    112, 116, 111, 114, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    })
                    .borrow()
                    .clone()
                })
                .7) || ({
                    ({
                        let source_ml_source1_nullish_field_receiver388 = (property.clone(),);
                        if match (source_ml_source1_nullish_field_receiver388.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present389 =
                                    ((source_ml_source1_nullish_field_receiver388.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present389 {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        _,
                                        source_ml_source1_data_property_value390,
                                    ) => source_ml_source1_data_property_value390.clone(),
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105,
                                    112, 116, 111, 114, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    })
                    .borrow()
                    .clone()
                })
                .2) || ({
                    ({
                        let source_ml_source1_nullish_field_receiver391 = (property.clone(),);
                        if match (source_ml_source1_nullish_field_receiver391.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            {
                                let source_ml_source1_nullish_field_present392 =
                                    ((source_ml_source1_nullish_field_receiver391.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_nullish_field_present392 {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        _,
                                        source_ml_source1_data_property_value393,
                                    ) => source_ml_source1_data_property_value393.clone(),
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException {
                                class_name: String::from("TypeError"),
                                message_units: (self::ml_string::Text::from_units(vec![
                                    67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112,
                                    114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32,
                                    117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101,
                                    97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105,
                                    112, 116, 111, 114, 39, 41,
                                ]))
                                .0,
                                message_present: true,
                            })
                            .raise()
                        }
                    })
                    .borrow()
                    .clone()
                })
                .0
                {
                    invalid.clone()
                } else {
                    {
                        let length = self::read_java_script_array_length_value(({ ({
                            let source_ml_source1_nullish_field_receiver394 = (property.clone(),);
                            if match (source_ml_source1_nullish_field_receiver394.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                {
                                    let source_ml_source1_nullish_field_present395 = ((source_ml_source1_nullish_field_receiver394.clone()).0).clone().expect("a proven present value");
                                    match source_ml_source1_nullish_field_present395 {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value396) => {
                                            source_ml_source1_data_property_value396.clone()
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                            }
                        }).borrow().clone() }).8);
                        if (self::test_java_script_array_length(length)) {
                            {
                                let source_ml_temporary_values10 = match object.clone() {
                                    self::JavaScriptArrayObject::ArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source1_data_property_value397,
                                    ) => source_ml_source1_data_property_value397.clone(),
                                };
                                {
                                    let source_ml_temporary_index10 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source1_s398 = self::source_ml_temporary_read_java_script_array_object_error_loop11(length, source_ml_temporary_values10, source_ml_temporary_index10);
                                        match source_ml_source1_s398 {
                                            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop11Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop11Done => {
                                                self::ml_string::Text::from_units(vec![])
                                            }
                                            self::MlTemporaryReadJavaScriptArrayObjectErrorLoop11Result::MlTemporaryReadJavaScriptArrayObjectErrorLoop11Return(source_ml_temporary_result11) => {
                                                source_ml_temporary_result11.clone()
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            invalid.clone()
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_array_object(
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
    length: f64,
) -> self::JavaScriptArrayObject {
    {
        let descriptor = self::create_java_script_property_descriptor(
            &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
            true,
            false,
        );
        {
            let source_ml_temporary_construct_java_script_array_object_written12 = {
                let ml_field_owner135 = descriptor.clone();
                let ml_field_value135 = false;
                ml_field_owner135.borrow_mut().0 = ml_field_value135;
                ()
            };
            {
                let source_ml_temporary_construct_java_script_array_object_written13 = {
                    let ml_field_owner136 = descriptor.clone();
                    let ml_field_value136 = true;
                    ml_field_owner136.borrow_mut().10 = ml_field_value136;
                    ()
                };
                {
                    let source_ml_temporary_construct_java_script_array_object_written14 = {
                        let ml_field_owner137 = descriptor.clone();
                        let ml_field_value137 = self::JavaScriptSemanticValue::Number(length);
                        ml_field_owner137.borrow_mut().8 = ml_field_value137;
                        ()
                    };
                    self::JavaScriptArrayObject::ArrayObject(
                        identity.clone(),
                        Box::new(prototype.clone()),
                        true,
                        false,
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                Box::new(self::read_java_script_array_length_key()),
                                descriptor.clone(),
                            ),
                        ])),
                    )
                }
            }
        }
    }
}
