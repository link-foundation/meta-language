// Generated from complete js/src/translation/javascript-error-construction-rules.js. Do not edit by hand.
use super::JavaScriptErrorAllocationDirectory;
use super::JavaScriptErrorAllocationIdentity;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result;
use super::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result;
use super::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result;
use super::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result;
use super::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result;
use super::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_semantic_value_error;
use super::source_ml_source1_temporary_find_java_script_heap_directory_object_loop8;
use super::source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop6;
use super::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop12;
use super::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop13;
use super::source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop9;
use super::source_ml_source1_temporary_test_java_script_semantic_text_equal_loop2;

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
            let source_ml_source2_nullish_field_receiver78 = (match value.clone() {
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
                self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value77) => {
                    Some(source_ml_source2_data_property_value77.clone())
                }
                self::JavaScriptSemanticValue::Symbol(_) => {
                    None::<self::ml_string::Text>
                }
                self::JavaScriptSemanticValue::Object(_, _) => {
                    None::<self::ml_string::Text>
                }
            },);
            if ((match (source_ml_source2_nullish_field_receiver78.clone()).0 { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                {
                    let source_ml_source2_nullish_field_present79 = ((source_ml_source2_nullish_field_receiver78.clone()).0).clone().expect("a proven present value");
                    source_ml_source2_nullish_field_present79.length()
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
                let source_ml_source2_nullish_field_receiver90 = (match value.clone() {
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
                    self::JavaScriptSemanticValue::Text(source_ml_source2_data_property_value89) => {
                        Some(source_ml_source2_data_property_value89)
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
                if match (source_ml_source2_nullish_field_receiver90.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                    {
                        let source_ml_source2_nullish_field_present91 = ((source_ml_source2_nullish_field_receiver90.clone()).0).clone().expect("a proven present value");
                        (source_ml_source2_nullish_field_present91.len() as f64)
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
                    let source_ml_source2_comparison_operand146 = match left.clone() {
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
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value144) => {
                            Some(source_ml_source2_data_property_value144.clone())
                        }
                        self::JavaScriptSemanticValue::Symbol(_) => {
                            None::<self::ml_string::Text>
                        }
                        self::JavaScriptSemanticValue::Object(_, _) => {
                            None::<self::ml_string::Text>
                        }
                    };
                    {
                        let source_ml_source2_comparison_operand147 = match right.clone() {
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
                            self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_data_property_value145) => {
                                Some(source_ml_source2_data_property_value145.clone())
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                None::<self::ml_string::Text>
                            }
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                None::<self::ml_string::Text>
                            }
                        };
                        match (source_ml_source2_comparison_operand146.clone(), source_ml_source2_comparison_operand147.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false }
                    }
                }
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s148 = self::source_ml_source1_temporary_test_java_script_semantic_text_equal_loop2(left.clone(), right.clone(), length, at);
                        match source_ml_source2_s148.clone() {
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Done => {
                                true
                            }
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop2Return(source_ml_source1_temporary_result2) => {
                                source_ml_source1_temporary_result2
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
pub fn find_java_script_property_descriptor_entry(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source2_s302 = self::source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop6(properties, key.clone(), at);
            match source_ml_source2_s302.clone() {
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(source_ml_source1_temporary_result6) => {
                    source_ml_source1_temporary_result6
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
                let source_ml_source2_s344 = self::source_ml_source1_temporary_find_java_script_heap_directory_object_loop8(objects, identity.clone(), low, high);
                match source_ml_source2_s344.clone() {
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(source_ml_source1_temporary_result8) => {
                        source_ml_source1_temporary_result8
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_heap_directory_symbol(symbols: Vec<self::ml::Big>, identity: self::ml::Big) -> bool {
    {
        let low = 0f64;
        {
            let high = (symbols.len() as f64);
            {
                let source_ml_source2_s345 = self::source_ml_source1_temporary_test_java_script_heap_directory_symbol_loop9(symbols, identity.clone(), low, high);
                match source_ml_source2_s345.clone() {
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Done => {
                        false
                    }
                    self::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Result::MlSource1TemporaryTestJavaScriptHeapDirectorySymbolLoop9Return(source_ml_source1_temporary_result9) => {
                        source_ml_source1_temporary_result9
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_heap_directory_value_error(heap: self::JavaScriptOrdinaryHeap, value: self::JavaScriptSemanticValue) -> self::ml_string::Text {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2, _, _, _, _) => {
            {
                let error = self::read_java_script_semantic_value_error(value.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    match value.clone() {
                        self::JavaScriptSemanticValue::Symbol(identity) => {
                            if self::test_java_script_heap_directory_symbol(shared_constructor_field2, identity.clone()) {
                                self::ml_string::Text::from_units(vec![])
                            } else {
                                self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 83, 121, 109, 98, 111, 108, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f352, source_ml_source2_f353) => {
                            {
                                let at = self::find_java_script_heap_directory_object(shared_constructor_field1.clone(), source_ml_source2_f352.clone());
                                if (at < (0f64)) {
                                    self::ml_string::Text::from_units(vec![97, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 79, 98, 106, 101, 99, 116, 32, 109, 117, 115, 116, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121])
                                } else {
                                    {
                                        let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(at));
                                        if (!(match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && (({
                                            let source_ml_source2_nullish_field_receiver428 = (object.clone(),);
                                            if match (source_ml_source2_nullish_field_receiver428.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                {
                                                    let source_ml_source2_nullish_field_present429 = ((source_ml_source2_nullish_field_receiver428.clone()).0).clone().expect("a proven present value");
                                                    match source_ml_source2_nullish_field_present429.clone() {
                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value430, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value430
                                                        }
                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value431, _, _, _, _) => {
                                                            source_ml_source2_data_property_value431
                                                        }
                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value432, _) => {
                                                            source_ml_source2_data_property_value432
                                                        }
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value433, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value433
                                                        }
                                                        self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value434, _, _) => {
                                                            source_ml_source2_data_property_value434
                                                        }
                                                        self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value435, _, _) => {
                                                            source_ml_source2_data_property_value435
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value436, _, _) => {
                                                            source_ml_source2_data_property_value436
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value437, _, _) => {
                                                            source_ml_source2_data_property_value437
                                                        }
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value438, _) => {
                                                            source_ml_source2_data_property_value438
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value439, _, _, _) => {
                                                            source_ml_source2_data_property_value439
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value440, _, _, _) => {
                                                            source_ml_source2_data_property_value440
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value441, _, _, _) => {
                                                            source_ml_source2_data_property_value441
                                                        }
                                                        self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value442, _, _, _) => {
                                                            source_ml_source2_data_property_value442
                                                        }
                                                    }
                                                }
                                            } else {
                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 97, 108, 108, 97, 98, 108, 101, 39, 41])).0, message_present: true }).raise()
                                            }
                                        }) == source_ml_source2_f353)) {
                                            self::ml_string::Text::from_units(vec![])
                                        } else {
                                            self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 99, 97, 108, 108, 97, 98, 105, 108, 105, 116, 121, 32, 109, 117, 115, 116, 32, 97, 103, 114, 101, 101, 32, 119, 105, 116, 104, 32, 105, 116, 115, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 103, 108, 111, 98, 97, 108, 32, 105, 100, 101, 110, 116, 105, 116, 121])
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
pub fn find_java_script_heap_directory_property(properties: Vec<self::JavaScriptOrdinaryProperty>, key: self::JavaScriptSemanticValue) -> f64 {
    self::find_java_script_property_descriptor_entry(properties, key.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_exception_operation(name: &self::ml_string::Text) -> self::ml_string::Text {
    if ((*name) == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114]))) {
        self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 114, 114, 111, 114])
    } else {
        if ((*name) == (self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]))) {
            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 84, 121, 112, 101, 69, 114, 114, 111, 114])
        } else {
            if ((*name) == (self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) {
                self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 97, 110, 103, 101, 69, 114, 114, 111, 114])
            } else {
                if ((*name) == (self::ml_string::Text::from_units(vec![83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114]))) {
                    self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 83, 121, 110, 116, 97, 120, 69, 114, 114, 111, 114])
                } else {
                    if ((*name) == (self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114]))) {
                        self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])
                    } else {
                        if ((*name) == (self::ml_string::Text::from_units(vec![85, 82, 73, 69, 114, 114, 111, 114]))) {
                            self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 85, 82, 73, 69, 114, 114, 111, 114])
                        } else {
                            if ((*name) == (self::ml_string::Text::from_units(vec![69, 118, 97, 108, 69, 114, 114, 111, 114]))) {
                                self::ml_string::Text::from_units(vec![114, 97, 105, 115, 101, 74, 97, 118, 97, 83, 99, 114, 105, 112, 116, 69, 118, 97, 108, 69, 114, 114, 111, 114])
                            } else {
                                self::ml_string::Text::from_units(vec![])
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
pub fn test_java_script_error_constructor_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode) == (self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 69, 114, 114, 111, 114])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_error_intrinsic_mode(mode: &self::ml_string::Text) -> bool {
    ((*mode) == (self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_construction_kind_error(error_class: &self::ml_string::Text, bind_receiver: bool, mode: &self::ml_string::Text) -> self::ml_string::Text {
    if ((self::read_java_script_source_exception_operation(error_class)) == (self::ml_string::Text::from_units(vec![]))) {
        self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 97, 110, 100, 97, 114, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 99, 108, 97, 115, 115])
    } else {
        if (bind_receiver && !(self::test_java_script_error_constructor_mode(mode))) {
            self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 114, 105, 118, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 98, 105, 110, 100, 115, 32, 116, 104, 101, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_allocation_directory(heap: self::JavaScriptOrdinaryHeap) -> self::JavaScriptErrorAllocationDirectory {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, shared_constructor_field2, _, _, _, _) => {
            self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory({ let ml_values1 = shared_constructor_field1; let ml_callback1 = std::rc::Rc::new(move |object: self::JavaScriptOrdinaryObject| -> self::JavaScriptErrorAllocationIdentity { self::JavaScriptErrorAllocationIdentity::ErrorAllocationIdentity(match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value487, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value487.clone()
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value488, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value488.clone()
                }
                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value489, _, _, _, _) => {
                    source_ml_source2_data_property_value489.clone()
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value490, _, _, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value490.clone()
                }
                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value491, _, _, _, _, _) => {
                    source_ml_source2_data_property_value491.clone()
                }
                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value492, _, _, _, _, _) => {
                    source_ml_source2_data_property_value492.clone()
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value493, _, _, _, _, _) => {
                    source_ml_source2_data_property_value493.clone()
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value494, _, _, _, _, _) => {
                    source_ml_source2_data_property_value494.clone()
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value495, _, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value495.clone()
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value496, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value496.clone()
                }
                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value497, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value497.clone()
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value498, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value498.clone()
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value499, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value499.clone()
                }
            }, match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                    self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                }
            }, match object.clone() {
                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, source_ml_source2_data_property_value500, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value500
                }
                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, source_ml_source2_data_property_value501, _, _, _, _) => {
                    source_ml_source2_data_property_value501
                }
                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, source_ml_source2_data_property_value502, _) => {
                    source_ml_source2_data_property_value502
                }
                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, source_ml_source2_data_property_value503, _, _, _, _, _, _, _) => {
                    source_ml_source2_data_property_value503
                }
                self::JavaScriptOrdinaryObject::MapObject(_, _, _, source_ml_source2_data_property_value504, _, _) => {
                    source_ml_source2_data_property_value504
                }
                self::JavaScriptOrdinaryObject::SetObject(_, _, _, source_ml_source2_data_property_value505, _, _) => {
                    source_ml_source2_data_property_value505
                }
                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, source_ml_source2_data_property_value506, _, _) => {
                    source_ml_source2_data_property_value506
                }
                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, source_ml_source2_data_property_value507, _, _) => {
                    source_ml_source2_data_property_value507
                }
                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value508, _) => {
                    source_ml_source2_data_property_value508
                }
                self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, source_ml_source2_data_property_value509, _, _, _) => {
                    source_ml_source2_data_property_value509
                }
                self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, source_ml_source2_data_property_value510, _, _, _) => {
                    source_ml_source2_data_property_value510
                }
                self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, source_ml_source2_data_property_value511, _, _, _) => {
                    source_ml_source2_data_property_value511
                }
                self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, source_ml_source2_data_property_value512, _, _, _) => {
                    source_ml_source2_data_property_value512
                }
            }) }); ml_values1.iter().cloned().enumerate().map(|(ml_index1, ml_value1)| ml_callback1(ml_value1)).collect::<Vec<_>>() }, self::ml_array::append(Vec::<self::ml::Big>::new(), shared_constructor_field2))
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_error_prior_directory_error(heap: self::JavaScriptOrdinaryHeap, before: self::JavaScriptErrorAllocationDirectory) -> self::ml_string::Text {
    match before.clone() {
        self::JavaScriptErrorAllocationDirectory::ErrorAllocationDirectory(shared_constructor_field1, shared_constructor_field2) => {
            if ((((shared_constructor_field1.len() as f64)) > (4096f64)) || (((shared_constructor_field2.len() as f64)) > (16384f64))) {
                self::ml_string::Text::from_units(vec![116, 104, 101, 32, 112, 114, 105, 111, 114, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116])
            } else {
                {
                    let previous = self::ml::Big::from_i128(0);
                    {
                        let source_ml_source1_temporary_values10 = shared_constructor_field1;
                        {
                            let source_ml_source1_temporary_index10 = self::ml::Big::from_i128(0);
                            {
                                let source_ml_source2_s519 = self::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop12(heap.clone(), previous.clone(), source_ml_source1_temporary_values10, source_ml_source1_temporary_index10.clone());
                                match source_ml_source2_s519.clone() {
                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Done(previous_2) => {
                                        {
                                            let previous_3 = self::ml::Big::from_i128(0);
                                            {
                                                let source_ml_source1_temporary_values11 = shared_constructor_field2;
                                                {
                                                    let source_ml_source1_temporary_index11 = self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s521 = self::source_ml_source1_temporary_read_java_script_error_prior_directory_error_loop13(heap.clone(), before.clone(), previous_3.clone(), source_ml_source1_temporary_values11, source_ml_source1_temporary_index11.clone());
                                                        match source_ml_source2_s521.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Done => {
                                                                self::ml_string::Text::from_units(vec![])
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop13Return(source_ml_source1_temporary_result13) => {
                                                                source_ml_source1_temporary_result13.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Result::MlSource1TemporaryReadJavaScriptErrorPriorDirectoryErrorLoop12Return(source_ml_source1_temporary_result13_2) => {
                                        source_ml_source1_temporary_result13_2.clone()
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
