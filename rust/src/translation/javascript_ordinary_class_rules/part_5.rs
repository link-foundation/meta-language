// Generated from complete js/src/translation/javascript-ordinary-class-rules.js. Do not edit by hand.
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_source_exception;
use super::ml_string;
use super::ml_utf16_text;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_text_equal_loop2(mut left: self::JavaScriptSemanticValue, mut right: self::JavaScriptSemanticValue, mut length: f64, mut at: f64) -> self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result {
    loop {
        return if (at < length) {
            {
                let a = if match left.clone() {
                    self::JavaScriptSemanticValue::Text(_) => {
                        true
                    }
                    _ => {
                        false
                    }
                } {
                    ({
                        let source_ml_source1_nullish_field_receiver840 = (match left.clone() {
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
                            self::JavaScriptSemanticValue::Text(source_ml_source1_data_property_value839) => {
                                Some(source_ml_source1_data_property_value839)
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
                        }, at);
                        {
                            let source_ml_source1_computed_property_key841 = (source_ml_source1_nullish_field_receiver840.clone()).1;
                            if match (source_ml_source1_nullish_field_receiver840.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                {
                                    let source_ml_source1_nullish_field_present842 = ((source_ml_source1_nullish_field_receiver840.clone()).0).clone().expect("a proven present value");
                                    self::ml_array::optional_at(&source_ml_source1_nullish_field_present842, self::ml_array::number_index(source_ml_source1_computed_property_key841))
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number((source_ml_source1_nullish_field_receiver840.clone()).1)))).concat(&self::ml_string::Text::from_units(vec![39, 41]))).0, message_present: true }).raise()
                            }
                        }
                    }).clone().unwrap_or_else(|| (-1f64))
                } else {
                    if match left.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                            true
                        }
                        _ => {
                            false
                        }
                    } {
                        {
                            let source_ml_source1_nullish_field_receiver853 = (match left.clone() {
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
                                self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value852) => {
                                    Some(source_ml_source1_data_property_value852.clone())
                                }
                                self::JavaScriptSemanticValue::Symbol(_) => {
                                    None::<self::ml_string::Text>
                                }
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    None::<self::ml_string::Text>
                                }
                            },);
                            if ((match (source_ml_source1_nullish_field_receiver853.clone()).0 { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                                {
                                    let source_ml_source1_nullish_field_present854 = ((source_ml_source1_nullish_field_receiver853.clone()).0).clone().expect("a proven present value");
                                    {
                                        let source_ml_source1_string_unit_operand855 = source_ml_source1_nullish_field_present854.clone();
                                        {
                                            let source_ml_source1_string_unit_operand856 = at;
                                            (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if ((index < (0f64)) || (index >= (text.length()))) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                        let source_ml_source1_string_unit_operand862 = {
                                                            let source_ml_source1_string_range_operand857 = text.clone();
                                                            {
                                                                let source_ml_source1_string_range_operand858 = index;
                                                                {
                                                                    let source_ml_source1_string_range_operand859 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source1_string_range_position860 = source_ml_source1_string_range_operand858;
                                                                        {
                                                                            let source_ml_source1_string_range_position861 = source_ml_source1_string_range_operand859;
                                                                            source_ml_source1_string_range_operand857.slice(source_ml_source1_string_range_position860, source_ml_source1_string_range_position861)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source1_string_unit_operand863 = 0f64;
                                                            source_ml_source1_string_unit_operand862.code_point_at(source_ml_source1_string_unit_operand863)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source1_string_unit_operand855.clone(), source_ml_source1_string_unit_operand856)
                                        }
                                    }
                                }
                            } else {
                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 39, 41])).0, message_present: true }).raise()
                            }
                        }
                    } else {
                        (-1f64)
                    }
                };
                {
                    let b = if match right.clone() {
                        self::JavaScriptSemanticValue::Text(_) => {
                            true
                        }
                        _ => {
                            false
                        }
                    } {
                        ({
                            let source_ml_source1_nullish_field_receiver874 = (match right.clone() {
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
                                self::JavaScriptSemanticValue::Text(source_ml_source1_data_property_value873) => {
                                    Some(source_ml_source1_data_property_value873)
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
                            }, at);
                            {
                                let source_ml_source1_computed_property_key875 = (source_ml_source1_nullish_field_receiver874.clone()).1;
                                if match (source_ml_source1_nullish_field_receiver874.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                    {
                                        let source_ml_source1_nullish_field_present876 = ((source_ml_source1_nullish_field_receiver874.clone()).0).clone().expect("a proven present value");
                                        self::ml_array::optional_at(&source_ml_source1_nullish_field_present876, self::ml_array::number_index(source_ml_source1_computed_property_key875))
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (((self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39])).concat(&self::ml_string::Text::from_text(&self::ml_number::js_number((source_ml_source1_nullish_field_receiver874.clone()).1)))).concat(&self::ml_string::Text::from_units(vec![39, 41]))).0, message_present: true }).raise()
                                }
                            }
                        }).clone().unwrap_or_else(|| (-1f64))
                    } else {
                        if match right.clone() {
                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                true
                            }
                            _ => {
                                false
                            }
                        } {
                            {
                                let source_ml_source1_nullish_field_receiver887 = (match right.clone() {
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
                                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source1_data_property_value886) => {
                                        Some(source_ml_source1_data_property_value886.clone())
                                    }
                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                        None::<self::ml_string::Text>
                                    }
                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                        None::<self::ml_string::Text>
                                    }
                                },);
                                if ((match (source_ml_source1_nullish_field_receiver887.clone()).0 { Some(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]) }, None => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103]))) {
                                    {
                                        let source_ml_source1_nullish_field_present888 = ((source_ml_source1_nullish_field_receiver887.clone()).0).clone().expect("a proven present value");
                                        {
                                            let source_ml_source1_string_unit_operand889 = source_ml_source1_nullish_field_present888.clone();
                                            {
                                                let source_ml_source1_string_unit_operand890 = at;
                                                (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                    let index = if position.is_nan() {
                                                        0f64
                                                    } else {
                                                        position.trunc()
                                                    };
                                                    if ((index < (0f64)) || (index >= (text.length()))) {
                                                        f64::NAN
                                                    } else {
                                                        ({
                                                            let source_ml_source1_string_unit_operand896 = {
                                                                let source_ml_source1_string_range_operand891 = text.clone();
                                                                {
                                                                    let source_ml_source1_string_range_operand892 = index;
                                                                    {
                                                                        let source_ml_source1_string_range_operand893 = (index + 1f64);
                                                                        {
                                                                            let source_ml_source1_string_range_position894 = source_ml_source1_string_range_operand892;
                                                                            {
                                                                                let source_ml_source1_string_range_position895 = source_ml_source1_string_range_operand893;
                                                                                source_ml_source1_string_range_operand891.slice(source_ml_source1_string_range_position894, source_ml_source1_string_range_position895)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source1_string_unit_operand897 = 0f64;
                                                                source_ml_source1_string_unit_operand896.code_point_at(source_ml_source1_string_unit_operand897)
                                                            }
                                                        }).clone().unwrap_or_else(|| f64::NAN)
                                                    }
                                                } }))(source_ml_source1_string_unit_operand889.clone(), source_ml_source1_string_unit_operand890)
                                            }
                                        }
                                    }
                                } else {
                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 99, 104, 97, 114, 67, 111, 100, 101, 65, 116, 39, 41])).0, message_present: true }).raise()
                                }
                            }
                        } else {
                            (-1f64)
                        }
                    };
                    if (a != b) {
                        self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Return(false)
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
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Result::MlTemporaryTestJavaScriptSemanticTextEqualLoop2Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_text_string_loop4(mut text: self::ml_string::Text, source_ml_temporary_values3: &[f64], mut source_ml_temporary_index3: self::ml::Big) -> self::ml_string::Text {
    loop {
        return if (source_ml_temporary_index3 < (self::ml::Big::from_u128(source_ml_temporary_values3.len() as u128))) {
            {
                let unit = self::ml_array::at(source_ml_temporary_values3, source_ml_temporary_index3.to_index());
                {
                    let text_2 = text.concat(&self::ml_string::Text::from_units(self::ml_utf16_text::character_units_utf16_text(&vec![unit]).into_iter().map(|ml_unit| ml_unit as u16).collect()));
                    {
                        let source_ml_temporary_index3_2 = source_ml_temporary_index3.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_temporary_index3) = (text_2.clone(), source_ml_temporary_index3_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            text.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done,
    MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_property_descriptor_entry_loop6(mut properties: Vec<self::JavaScriptOrdinaryProperty>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result {
    loop {
        return if (at < ((properties.len() as f64))) {
            {
                let property = self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value({
                    let source_ml_source1_nullish_field_receiver899 = (property.clone(),);
                    if match (source_ml_source1_nullish_field_receiver899.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source1_nullish_field_present900 = ((source_ml_source1_nullish_field_receiver899.clone()).0).clone().expect("a proven present value");
                            match source_ml_source1_nullish_field_present900.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value901, _) => {
                                    let source_ml_source1_data_property_value901 = *source_ml_source1_data_property_value901;
                                    source_ml_source1_data_property_value901.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }, key.clone())) {
                    self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Return(at)
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
            self::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Result::MlTemporaryFindJavaScriptPropertyDescriptorEntryLoop6Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Done,
    MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_heap_directory_object_loop8(mut objects: Vec<self::JavaScriptOrdinaryObject>, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let object = self::ml_array::optional_at(&objects, self::ml_array::number_index(middle));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return((-1f64))
                    } else {
                        if ((match object.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value915, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value915.clone()
                            }
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value916, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value916.clone()
                            }
                            self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value917, _, _, _, _) => {
                                source_ml_source1_data_property_value917.clone()
                            }
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value918, _, _, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value918.clone()
                            }
                            self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value919, _, _, _, _, _) => {
                                source_ml_source1_data_property_value919.clone()
                            }
                            self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value920, _, _, _, _, _) => {
                                source_ml_source1_data_property_value920.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value921, _, _, _, _, _) => {
                                source_ml_source1_data_property_value921.clone()
                            }
                            self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value922, _, _, _, _, _) => {
                                source_ml_source1_data_property_value922.clone()
                            }
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value923, _, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value923.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value924, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value924.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value925, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value925.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value926, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value926.clone()
                            }
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value927, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value927.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Return(middle)
                        } else {
                            if ((match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value941, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value941.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value942, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value942.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value943, _, _, _, _) => {
                                    source_ml_source1_data_property_value943.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value944, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value944.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value945, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value945.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value946, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value946.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value947, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value947.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value948, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value948.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value949, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value949.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value950, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value950.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value951, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value951.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value952, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value952.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value953, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value953.clone()
                                }
                            }) < identity) {
                                {
                                    let low_2 = (middle + 1f64);
                                    {
                                        low = low_2;
                                        continue;
                                    }
                                }
                            } else {
                                {
                                    high = middle;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Result::MlTemporaryFindJavaScriptHeapDirectoryObjectLoop8Done
        };
    }
}
