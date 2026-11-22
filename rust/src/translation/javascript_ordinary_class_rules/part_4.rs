// Generated from complete js/src/translation/javascript-ordinary-class-rules.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result;
use super::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_value_error;
use super::source_ml_temporary_find_java_script_heap_directory_object_loop8;
use super::source_ml_temporary_find_java_script_property_descriptor_entry_loop6;
use super::source_ml_temporary_read_java_script_semantic_text_string_loop4;
use super::source_ml_temporary_test_java_script_semantic_text_equal_loop2;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_same_value(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone())) != (self::ml_string::Text::from_units(vec![]))) || ((self::read_java_script_semantic_value_error(right.clone())) != (self::ml_string::Text::from_units(vec![])))) {
        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![83, 97, 109, 101, 86, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 119, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 117, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 109, 111, 100, 101, 108])).0, message_present: true }).raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Null => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Null => {
                        true
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Boolean(left_value) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Boolean(value) => {
                        (left_value == value)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Number(left_value_2) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Number(value_2) => {
                        if (left_value_2 == value_2) {
                            ((left_value_2 != (0f64)) || (((1f64 / left_value_2)) == ((1f64 / value_2))))
                        } else {
                            (left_value_2.is_nan() && value_2.is_nan())
                        }
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Integer(left_value_3) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Integer(value_3) => {
                        (left_value_3 == value_3)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Text(_) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::SourceText(_, _) => {
                self::test_java_script_semantic_text_equal(left.clone(), right.clone())
            }
            self::JavaScriptSemanticValue::Symbol(left_identity) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Symbol(identity) => {
                        (left_identity == identity)
                    }
                    _ => {
                        false
                    }
                }
            }
            self::JavaScriptSemanticValue::Object(left_identity_2, _) => {
                match right.clone() {
                    self::JavaScriptSemanticValue::Object(identity_2, _) => {
                        (left_identity_2 == identity_2)
                    }
                    _ => {
                        false
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_length(value: self::JavaScriptSemanticValue) -> f64 {
    if match value.clone() {
        self::JavaScriptSemanticValue::SourceText(_, _) => {
            true
        }
        _ => {
            false
        }
    } {
        {
            let source_ml_source1_nullish_field_receiver78 = (match value.clone() {
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
                self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value77) => {
                    Some(source_ml_source1_data_property_value77.clone())
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    None::<self::ml_string::Text>
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    None::<self::ml_string::Text>
                }
            },);
            if ((match (source_ml_source1_nullish_field_receiver78.clone()).0 { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                {
                    let source_ml_source1_nullish_field_present79 = ((source_ml_source1_nullish_field_receiver78.clone()).0).clone().expect("a proven present value");
                    source_ml_source1_nullish_field_present79.length()
                }
            } else {
                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
            }
        }
    } else {
        if match value.clone() {
            self::JavaScriptSemanticValue::Text(_) => {
                true
            }
            _ => {
                false
            }
        } {
            {
                let source_ml_source1_nullish_field_receiver90 = (match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Null => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Number(_) => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_data_property_value89) => {
                        Some(source_ml_source1_data_property_value89)
                    }
                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        None::<Vec<f64>>
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        None::<Vec<f64>>
                    }
                },);
                if match (source_ml_source1_nullish_field_receiver90.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                    {
                        let source_ml_source1_nullish_field_present91 = ((source_ml_source1_nullish_field_receiver90.clone()).0).clone().expect("a proven present value");
                        (source_ml_source1_nullish_field_present91.len() as f64)
                    }
                } else {
                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 108, 101, 110, 103, 116, 104, 39, 41])).0, message_present: true }).raise()
                }
            }
        } else {
            (-1f64)
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_text_equal(left: self::JavaScriptSemanticValue, right: self::JavaScriptSemanticValue) -> bool {
    {
        let length = self::read_java_script_semantic_text_length(left.clone());
        if ((length < (0f64)) || (length != (self::read_java_script_semantic_text_length(right.clone())))) {
            false
        } else {
            if (match left.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    true
                }
                _ => {
                    false
                }
            } && match right.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => {
                    true
                }
                _ => {
                    false
                }
            }) {
                {
                    let source_ml_source1_comparison_operand146 = match left.clone() {
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
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value144) => {
                            Some(source_ml_source1_data_property_value144.clone())
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand147 = match right.clone() {
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
                            self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value145) => {
                                Some(source_ml_source1_data_property_value145.clone())
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        };
                        match (source_ml_source1_comparison_operand146.clone(), source_ml_source1_comparison_operand147.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false }
                    }
                }
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source1_s148 = self::source_ml_temporary_test_java_script_semantic_text_equal_loop2(left.clone(), right.clone(), length, at);
                        match source_ml_source1_s148.clone() {
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done => {
                                true
                            }
                            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(source_ml_temporary_result2) => {
                                source_ml_temporary_result2
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
pub fn read_java_script_semantic_text_string(value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Undefined => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Null => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Boolean(_) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Number(_) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Integer(_) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Text(source_ml_source1_f152) => {
            {
                let text = self::ml_string::Text::from_units(vec![]);
                {
                    let source_ml_temporary_index3 = self::ml::Big::from_i128(0);
                    {
                        let text_2 = self::source_ml_temporary_read_java_script_semantic_text_string_loop4(text.clone(), &(source_ml_source1_f152), source_ml_temporary_index3.clone());
                        text_2.clone()
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(_, text_3) => {
            text_3.clone()
        }
        self::JavaScriptSemanticValue::Symbol(_) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
        }
        self::JavaScriptSemanticValue::Object(_, _) => {
            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97, 110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101])).0, message_present: true }).raise()
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_property_descriptor_entry(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source1_s302 = self::source_ml_temporary_find_java_script_property_descriptor_entry_loop6(properties, key.clone(), at);
            match source_ml_source1_s302.clone() {
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done => {
                    (-1f64)
                }
                self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(source_ml_temporary_result6) => {
                    source_ml_temporary_result6
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_heap_directory_object(objects: Vec<self::JavaScriptOrdinaryObject>, identity: self::ml::Big) -> f64 {
    {
        let low = 0f64;
        {
            let high = (objects.len() as f64);
            {
                let source_ml_source1_s344 = self::source_ml_temporary_find_java_script_heap_directory_object_loop8(objects, identity.clone(), low, high);
                match source_ml_source1_s344.clone() {
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Done => {
                        (-1f64)
                    }
                    self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(source_ml_temporary_result8) => {
                        source_ml_temporary_result8
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_heap_directory_property(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_property_descriptor_entry(properties, key.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_class_constructor_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode) == (self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_ordinary_class_receiver_result(mode: &self::ml_string::Text, result: &self::ml_string::Text) -> bool {
    (self::test_java_script_ordinary_class_constructor_mode(mode) && ((*result) != (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_class_call_reason(heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            {
                let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_heap_directory_object(shared_constructor_field1, identity.clone())));
                if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::ml_string::Text::from_units(vec![])
                } else {
                    match object.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, properties) => {
                            {
                                let field = self::ml_array::optional_at(&properties, self::ml_array::number_index(self::find_java_script_heap_directory_property(properties, self::JavaScriptSemanticValue::Text(vec![110f64, 97f64, 109f64, 101f64]))));
                                if (((match (field.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(({
                                    let source_ml_source1_nullish_field_receiver566 = (field.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver566.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source1_nullish_field_present567 = ((source_ml_source1_nullish_field_receiver566.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source1_nullish_field_present567.clone() {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value568) => {
                                                    source_ml_source1_data_property_value568.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                    }
                                }).9)) || ({
                                    let source_ml_source1_nullish_field_receiver569 = (field.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver569.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source1_nullish_field_present570 = ((source_ml_source1_nullish_field_receiver569.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source1_nullish_field_present570.clone() {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value571) => {
                                                    source_ml_source1_data_property_value571.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                    }
                                }).5) || ({
                                    let source_ml_source1_nullish_field_receiver572 = (field.clone(),);
                                    if match (source_ml_source1_nullish_field_receiver572.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                        {
                                            let source_ml_source1_nullish_field_present573 = ((source_ml_source1_nullish_field_receiver572.clone()).0).clone().expect("a proven present value");
                                            match source_ml_source1_nullish_field_present573.clone() {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value574) => {
                                                    source_ml_source1_data_property_value574.clone()
                                                }
                                            }
                                        }
                                    } else {
                                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                    }
                                }).7) {
                                    self::ml_string::Text::from_units(vec![])
                                } else {
                                    {
                                        let value = ({
                                            let source_ml_source1_nullish_field_receiver575 = (field.clone(),);
                                            if match (source_ml_source1_nullish_field_receiver575.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source1_nullish_field_present576 = ((source_ml_source1_nullish_field_receiver575.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source1_nullish_field_present576.clone() {
                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value577) => {
                                                            source_ml_source1_data_property_value577.clone()
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }).8;
                                        if (match value.clone() {
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        } && match value.clone() {
                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                false
                                            }
                                            _ => {
                                                true
                                            }
                                        }) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            ((self::ml_string::Text::from_units(vec![67, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32])).concat(&self::read_java_script_semantic_text_string(value.clone()))).concat(&self::ml_string::Text::from_units(vec![32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 105, 110, 118, 111, 107, 101, 100, 32, 119, 105, 116, 104, 111, 117, 116, 32, 39, 110, 101, 119, 39]))
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            self::ml_string::Text::from_units(vec![])
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_ordinary_class_receiver_error(heap: self::JavaScriptOrdinaryHeap, callee: self::JavaScriptSemanticValue, receiver: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            match callee.clone() {
                self::JavaScriptSemanticValue::Object(callee_identity, callee_callable) => {
                    if !callee_callable {
                        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 110, 100, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                    } else {
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Object(identity, callable) => {
                                if callable {
                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 110, 100, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                                } else {
                                    {
                                        let constructor = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_heap_directory_object(shared_constructor_field1.clone(), callee_identity.clone())));
                                        {
                                            let instance = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_heap_directory_object(shared_constructor_field1, identity.clone())));
                                            if match (constructor.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 111, 112, 97, 113, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 107, 105, 110, 100, 115])
                                            } else {
                                                match constructor.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, properties) => {
                                                        if match (instance.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                            self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 111, 112, 97, 113, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 107, 105, 110, 100, 115])
                                                        } else {
                                                            match instance.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, prototype, _, _, _, _, _, _, _) => {
                                                                    let prototype = *prototype;
                                                                    {
                                                                        let field = self::ml_array::optional_at(&properties, self::ml_array::number_index(self::find_java_script_heap_directory_property(properties, self::JavaScriptSemanticValue::Text(vec![112f64, 114f64, 111f64, 116f64, 111f64, 116f64, 121f64, 112f64, 101f64]))));
                                                                        if (((((!(match (field.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && ({
                                                                            let source_ml_source1_nullish_field_receiver802 = (field.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver802.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present803 = ((source_ml_source1_nullish_field_receiver802.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present803.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value804) => {
                                                                                            source_ml_source1_data_property_value804.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).9) && !(({
                                                                            let source_ml_source1_nullish_field_receiver805 = (field.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver805.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present806 = ((source_ml_source1_nullish_field_receiver805.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present806.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value807) => {
                                                                                            source_ml_source1_data_property_value807.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).5)) && !(({
                                                                            let source_ml_source1_nullish_field_receiver808 = (field.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver808.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present809 = ((source_ml_source1_nullish_field_receiver808.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present809.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value810) => {
                                                                                            source_ml_source1_data_property_value810.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).7)) && {
                                                                            let source_ml_source1_s814 = ({
                                                                                let source_ml_source1_nullish_field_receiver824 = (field.clone(),);
                                                                                if match (source_ml_source1_nullish_field_receiver824.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    {
                                                                                        let source_ml_source1_nullish_field_present825 = ((source_ml_source1_nullish_field_receiver824.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source1_nullish_field_present825.clone() {
                                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value826) => {
                                                                                                source_ml_source1_data_property_value826.clone()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                                }
                                                                            }).8;
                                                                            match source_ml_source1_s814.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    true
                                                                                }
                                                                                _ => {
                                                                                    false
                                                                                }
                                                                            }
                                                                        }) && self::test_java_script_semantic_same_value(({
                                                                            let source_ml_source1_nullish_field_receiver827 = (field.clone(),);
                                                                            if match (source_ml_source1_nullish_field_receiver827.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                {
                                                                                    let source_ml_source1_nullish_field_present828 = ((source_ml_source1_nullish_field_receiver827.clone()).0).clone().expect("a proven present value");
                                                                                    match source_ml_source1_nullish_field_present828.clone() {
                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value829) => {
                                                                                            source_ml_source1_data_property_value829.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 39, 41])).0, message_present: true }).raise()
                                                                            }
                                                                        }).8, prototype.clone())) {
                                                                            self::ml_string::Text::from_units(vec![])
                                                                        } else {
                                                                            self::ml_string::Text::from_units(vec![116, 104, 101, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 99, 108, 97, 115, 115, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 119, 110, 32, 100, 97, 116, 97, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                                        }
                                                                    }
                                                                }
                                                                _ => {
                                                                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 111, 112, 97, 113, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 107, 105, 110, 100, 115])
                                                                }
                                                            }
                                                        }
                                                    }
                                                    _ => {
                                                        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 111, 112, 97, 113, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 107, 105, 110, 100, 115])
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            _ => {
                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 110, 100, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                            }
                        }
                    }
                }
                _ => {
                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 110, 100, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done,
    MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(bool),
}
