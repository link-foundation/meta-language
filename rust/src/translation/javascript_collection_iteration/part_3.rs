// Generated from complete js/src/translation/javascript-collection-iteration.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result;
use super::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result;
use super::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result;
use super::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result;
use super::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result;
use super::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result;
use super::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_source_text_profile_error;
use super::source_ml_temporary_construct_java_script_semantic_text_units_loop7;
use super::source_ml_temporary_find_java_script_heap_directory_object_loop12;
use super::source_ml_temporary_find_java_script_property_descriptor_entry_loop9;
use super::source_ml_temporary_read_java_script_literal_property_index_loop11;
use super::source_ml_temporary_read_java_script_semantic_value_error_loop3;
use super::source_ml_temporary_test_java_script_heap_directory_symbol_loop13;
use super::source_ml_temporary_test_java_script_semantic_text_equal_loop4;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_value_error(
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Text(units) => {
            let ml_product_condition_1 = {
                let ml_reference_array3 = units.clone();
                ml_reference_array3.borrow().len() as f64
            };
            if ml_product_condition_1 {
                self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99,
                    101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105,
                    116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103,
                    101, 116,
                ])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s89 =
                            self::source_ml_temporary_read_java_script_semantic_value_error_loop3(
                                units.clone(),
                                at,
                            );
                        match source_ml_source1_s89 {
                            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop3Return(source_ml_temporary_result3) => {
                                source_ml_temporary_result3.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(profile, text) => {
            self::read_java_script_source_text_profile_error(&(profile), text.length())
        }
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if (identity > (self::ml::Big::from_i128(0)))
                && (identity <= (self::ml::Big::from_i128(18_446_744_073_709_551_615)))
            {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![
                    97, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101,
                    115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115,
                    105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105,
                    110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121,
                ])
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, _) => {
            if (identity_2 > (self::ml::Big::from_i128(0)))
                && (identity_2 <= (self::ml::Big::from_i128(18_446_744_073_709_551_615)))
            {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![
                    97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101,
                    115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115,
                    105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105,
                    110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121,
                ])
            }
        }
        _ => self::ml_string::Text::from_units(vec![]),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_same_value(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    if ((self::read_java_script_semantic_value_error(left.clone()))
        != (self::ml_string::Text::from_units(vec![])))
        || ((self::read_java_script_semantic_value_error(right.clone()))
            != (self::ml_string::Text::from_units(vec![])))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                83, 97, 109, 101, 86, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101,
                115, 32, 116, 119, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97,
                108, 117, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97,
                114, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 109, 111, 100, 101,
                108,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                _ => false,
            },
            self::JavaScriptSemanticValue::Null => match right.clone() {
                self::JavaScriptSemanticValue::Null => true,
                _ => false,
            },
            self::JavaScriptSemanticValue::Boolean(left_value) => match right.clone() {
                self::JavaScriptSemanticValue::Boolean(value) => left_value == value,
                _ => false,
            },
            self::JavaScriptSemanticValue::Number(left_value_2) => match right.clone() {
                self::JavaScriptSemanticValue::Number(value_2) => {
                    if left_value_2 == value_2 {
                        (left_value_2 != 0f64) || ((1f64 / left_value_2) == (1f64 / value_2))
                    } else {
                        left_value_2.is_nan() && value_2.is_nan()
                    }
                }
                _ => false,
            },
            self::JavaScriptSemanticValue::Integer(left_value_3) => match right.clone() {
                self::JavaScriptSemanticValue::Integer(value_3) => left_value_3 == value_3,
                _ => false,
            },
            self::JavaScriptSemanticValue::Text(_) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(left_identity) => match right.clone() {
                self::JavaScriptSemanticValue::Symbol(identity) => left_identity == identity,
                _ => false,
            },
            self::JavaScriptSemanticValue::Object(left_identity_2, _) => match right.clone() {
                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                    left_identity_2 == identity_2
                }
                _ => false,
            },
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_length(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Text(source_ml_source1_f155) => {
            let ml_reference_array4 = source_ml_source1_f155.clone();
            ml_reference_array4.borrow().len() as f64
        }
        self::JavaScriptSemanticValue::SourceText(_, text) => text.length(),
        _ => -1f64,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_unit(
    value: self::JavaScriptSemanticValue,
    index: f64,
) -> f64 {
    if (!(self::ml_math::is_safe_integer(index)) || (index < 0f64))
        || (index >= (self::read_java_script_semantic_text_length(value.clone())))
    {
        -1f64
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Text(source_ml_source1_f227) => ({
                let ml_reference_array5 = source_ml_source1_f227.clone();
                let ml_reference_position5 = index;
                self::ml_array::optional_at(
                    &*ml_reference_array5.borrow(),
                    self::ml_array::number_index(ml_reference_position5),
                )
            })
            .clone()
            .unwrap_or_else(|| -1f64),
            self::JavaScriptSemanticValue::SourceText(_, text) => {
                let source_ml_source1_string_unit_operand282 = text.clone();
                {
                    let source_ml_source1_string_unit_operand283 = index;
                    (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 {
                        {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if (index_2 < 0f64) || (index_2 >= (text_2.length())) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source1_string_unit_operand289 = {
                                        let source_ml_source1_string_range_operand284 = text_2.clone();
                                        {
                                            let source_ml_source1_string_range_operand285 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand286 = index_2 + 1f64;
                                                {
                                                    let source_ml_source1_string_range_position287 = source_ml_source1_string_range_operand285;
                                                    {
                                                        let source_ml_source1_string_range_position288 = source_ml_source1_string_range_operand286;
                                                        source_ml_source1_string_range_operand284.slice(source_ml_source1_string_range_position287, source_ml_source1_string_range_position288)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand290 = 0f64;
                                        source_ml_source1_string_unit_operand289.code_point_at(source_ml_source1_string_unit_operand290)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        }
                    }))(
                        source_ml_source1_string_unit_operand282,
                        source_ml_source1_string_unit_operand283,
                    )
                }
            }
            _ => -1f64,
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_text_equal(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    {
        let length = self::read_java_script_semantic_text_length(left.clone());
        if (length < 0f64)
            || (length != (self::read_java_script_semantic_text_length(right.clone())))
        {
            false
        } else if match left.clone() {
            self::JavaScriptSemanticValue::SourceText(_, _) => true,
            _ => false,
        } && match right.clone() {
            self::JavaScriptSemanticValue::SourceText(_, _) => true,
            _ => false,
        } {
            {
                let source_ml_source1_comparison_operand327 = match left.clone() {
                    self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::SourceText(
                        _,
                        source_ml_source1_data_property_value325,
                    ) => Some(source_ml_source1_data_property_value325.clone()),
                    self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                    self::JavaScriptSemanticValue::Object(_, _) => None::<self::ml_string::Text>,
                };
                {
                    let source_ml_source1_comparison_operand328 = match right.clone() {
                        self::JavaScriptSemanticValue::Undefined => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Null => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Number(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Integer(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Text(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::SourceText(
                            _,
                            source_ml_source1_data_property_value326,
                        ) => Some(source_ml_source1_data_property_value326.clone()),
                        self::JavaScriptSemanticValue::Symbol(_) => None::<self::ml_string::Text>,
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    match (
                        source_ml_source1_comparison_operand327,
                        source_ml_source1_comparison_operand328,
                    ) {
                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                        (None, None) => true,
                        _ => false,
                    }
                }
            }
        } else {
            {
                let at = 0f64;
                {
                    let source_ml_source1_s329 =
                        self::source_ml_temporary_test_java_script_semantic_text_equal_loop4(
                            left.clone(),
                            right.clone(),
                            length,
                            at,
                        );
                    match source_ml_source1_s329 {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Done => {
                                true
                            }
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop4Return(source_ml_temporary_result4) => {
                                source_ml_temporary_result4
                            }
                        }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_semantic_text_units(
    value: self::JavaScriptSemanticValue,
    limit: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let length = self::read_java_script_semantic_text_length(value.clone());
        if (length < 0f64) || (length > limit) {
            (self::ml_source_exception::SourceException {
                class_name: String::from("RangeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 99, 111, 110,
                    118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97,
                    116, 105, 111, 110, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    let units = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source1_s410 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units.clone(), at);
                            match source_ml_source1_s410 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Null => {
                    let units_2 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_2 = 0f64;
                        {
                            let source_ml_source1_s411 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_2.clone(), at_2);
                            match source_ml_source1_s411 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_2.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Boolean(_) => {
                    let units_3 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_3 = 0f64;
                        {
                            let source_ml_source1_s412 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_3.clone(), at_3);
                            match source_ml_source1_s412 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_3.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Number(_) => {
                    let units_4 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_4 = 0f64;
                        {
                            let source_ml_source1_s413 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_4.clone(), at_4);
                            match source_ml_source1_s413 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_4.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Integer(_) => {
                    let units_5 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_5 = 0f64;
                        {
                            let source_ml_source1_s414 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_5.clone(), at_5);
                            match source_ml_source1_s414 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_5.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Text(units_6) => units_6.clone(),
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    let units_7 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_6 = 0f64;
                        {
                            let source_ml_source1_s415 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_7.clone(), at_6);
                            match source_ml_source1_s415 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_7.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    let units_8 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_7 = 0f64;
                        {
                            let source_ml_source1_s416 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_8.clone(), at_7);
                            match source_ml_source1_s416 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_8.clone()
                                    }
                                }
                        }
                    }
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    let units_9 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at_8 = 0f64;
                        {
                            let source_ml_source1_s417 = self::source_ml_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_9.clone(), at_8);
                            match source_ml_source1_s417 {
                                    self::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlTemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_9.clone()
                                    }
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
pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    {
        let ml_library_inequality_1 = (self::read_java_script_semantic_value_error(value.clone()))
            != (self::ml_string::Text::from_units(vec![]));
        if ml_library_inequality_1 {
            false
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                self::JavaScriptSemanticValue::Object(_, source_ml_source1_f426) => {
                    source_ml_source1_f426
                }
                _ => false,
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_property_descriptor_entry(
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source1_s491 =
                self::source_ml_temporary_find_java_script_property_descriptor_entry_loop9(
                    properties.clone(),
                    key.clone(),
                    at,
                );
            match source_ml_source1_s491 {
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done => {
                    -1f64
                }
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(source_ml_temporary_result9) => {
                    source_ml_temporary_result9
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_property_descriptor_kind(
    descriptor: std::rc::Rc<
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
) -> self::ml_string::Text {
    {
        let data = ({ descriptor.borrow().clone() }).9 || ({ descriptor.borrow().clone() }).11;
        {
            let accessor =
                ({ descriptor.borrow().clone() }).5 || ({ descriptor.borrow().clone() }).7;
            if data && accessor {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else if data {
                self::ml_string::Text::from_units(vec![100, 97, 116, 97])
            } else if accessor {
                self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
            } else {
                self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn read_java_script_property_descriptor_error(
    descriptor: std::rc::Rc<
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
) -> self::ml_string::Text {
    if (self::read_java_script_property_descriptor_kind(descriptor.clone()))
        == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))
    {
        self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
    } else {
        let ml_product_condition_3 = { descriptor.borrow().clone() };
        if ml_product_condition_3 {
            self::ml_string::Text::from_units(vec![
                73, 110, 118, 97, 108, 105, 100, 83, 101, 109, 97, 110, 116, 105, 99, 86, 97, 108,
                117, 101,
            ])
        } else {
            let ml_product_condition_2 = { descriptor.borrow().clone() };
            if ml_product_condition_2 {
                self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])
            } else {
                let ml_product_condition_1 = { descriptor.borrow().clone() };
                if ml_product_condition_1 {
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ])
                } else {
                    self::ml_string::Text::from_units(vec![])
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
// The checked primitive-column interface retains every ordered source argument.
#[expect(
    clippy::too_many_arguments,
    reason = "checked source arguments and captured continuation state retain their original order"
)]
pub fn test_complete_java_script_property_descriptor(
    descriptor: std::rc::Rc<
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
) -> bool {
    if (!(({ descriptor.borrow().clone() }).1) || !(({ descriptor.borrow().clone() }).3))
        || ((self::read_java_script_property_descriptor_error(descriptor.clone()))
            != (self::ml_string::Text::from_units(vec![])))
    {
        false
    } else {
        {
            let kind = self::read_java_script_property_descriptor_kind(descriptor.clone());
            if kind == (self::ml_string::Text::from_units(vec![100, 97, 116, 97])) {
                ({ descriptor.borrow().clone() }).9 && ({ descriptor.borrow().clone() }).11
            } else {
                ((kind
                    == (self::ml_string::Text::from_units(vec![
                        97, 99, 99, 101, 115, 115, 111, 114,
                    ])))
                    && ({ descriptor.borrow().clone() }).5)
                    && ({ descriptor.borrow().clone() }).7
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_literal_property_index(
    key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
) -> f64 {
    if (({
        let ml_reference_array24 = key.clone();
        ml_reference_array24.borrow().len() as f64
    }) == 0f64)
        || (({
            let ml_reference_array25 = key.clone();
            ml_reference_array25.borrow().len() as f64
        }) > 10f64)
    {
        -1f64
    } else {
        if (({
            let ml_reference_array26 = key.clone();
            ml_reference_array26.borrow().len() as f64
        }) > 1f64)
            && {
                let source_ml_source1_comparison_operand535 = {
                    let source_ml_source1_array_read_index534 = 0f64;
                    {
                        let ml_reference_array27 = key.clone();
                        let ml_reference_position27 = source_ml_source1_array_read_index534;
                        self::ml_array::optional_at(
                            &*ml_reference_array27.borrow(),
                            self::ml_array::number_index(ml_reference_position27),
                        )
                    }
                };
                {
                    let source_ml_source1_comparison_operand536 = 48f64;
                    match (
                        source_ml_source1_comparison_operand535,
                        source_ml_source1_comparison_operand536,
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
                        let source_ml_source1_s537 = self::source_ml_temporary_read_java_script_literal_property_index_loop11(key.clone(), value, at);
                        match source_ml_source1_s537 {
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(value_2) => {
                                if value_2 <= 4_294_967_294f64 {
                                    value_2
                                } else {
                                    -1f64
                                }
                            }
                            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(source_ml_temporary_result11) => {
                                source_ml_temporary_result11
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
pub fn find_java_script_heap_directory_object(
    objects: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>>,
    identity: self::ml::Big,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array28 = objects.clone();
                ml_reference_array28.borrow().len() as f64
            };
            {
                let source_ml_source1_s538 =
                    self::source_ml_temporary_find_java_script_heap_directory_object_loop12(
                        objects.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s538 {
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Done => {
                        -1f64
                    }
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(source_ml_temporary_result12) => {
                        source_ml_temporary_result12
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_heap_directory_symbol(
    symbols: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    identity: self::ml::Big,
) -> bool {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array29 = symbols.clone();
                ml_reference_array29.borrow().len() as f64
            };
            {
                let source_ml_source1_s539 =
                    self::source_ml_temporary_test_java_script_heap_directory_symbol_loop13(
                        symbols.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source1_s539 {
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Done => {
                        false
                    }
                    self::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Result::MlTemporaryTestJavaScriptHeapDirectorySymbolLoop13Return(source_ml_temporary_result13) => {
                        source_ml_temporary_result13
                    }
                }
            }
        }
    }
}
