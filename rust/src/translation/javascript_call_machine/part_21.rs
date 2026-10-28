// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result;
use super::apply_java_script_descriptor_field;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_descriptor_field;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::test_complete_java_script_property_descriptor;
use super::test_java_script_call_coordinate;
use super::test_java_script_descriptor_field_presence;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_ordinary_property_loop98(mut properties: Vec<self::JavaScriptOrdinaryProperty>, mut key: self::JavaScriptSemanticValue, mut at: f64) -> self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result {
    loop {
        return if (at < ((properties.len() as f64))) {
            {
                let property = self::ml_array::optional_at(&properties, self::ml_array::number_index(at));
                if (!(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && self::test_java_script_semantic_same_value({
                    let source_ml_source2_nullish_field_receiver2025 = (property.clone(),);
                    if match (source_ml_source2_nullish_field_receiver2025.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                        {
                            let source_ml_source2_nullish_field_present2026 = ((source_ml_source2_nullish_field_receiver2025.clone()).0).clone().expect("a proven present value");
                            match source_ml_source2_nullish_field_present2026.clone() {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2027, _) => {
                                    let source_ml_source2_data_property_value2027 = *source_ml_source2_data_property_value2027;
                                    source_ml_source2_data_property_value2027.clone()
                                }
                            }
                        }
                    } else {
                        (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])).0, message_present: true }).raise()
                    }
                }, key.clone())) {
                    self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Return(at)
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
            self::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Result::MlSource1TemporaryFindJavaScriptOrdinaryPropertyLoop98Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop106(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut prototype: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result {
    loop {
        return match prototype.clone() {
            self::JavaScriptSemanticValue::Undefined => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Null => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2028) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Number(source_ml_source2_f2029) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Integer(source_ml_source2_f2030) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Text(source_ml_source2_f2031) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2032) => {
                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done
            }
            self::JavaScriptSemanticValue::Object(identity, source_ml_source2_f2033) => {
                {
                    let depth_2 = (depth + 1f64);
                    if ((identity == (match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2035, _, _, _, _) => {
                            source_ml_source2_data_property_value2035.clone()
                        }
                    })) || (depth_2 > (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2037, _) => source_ml_source2_data_property_value2037.as_slice() }).len() as f64)))) {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                    } else {
                        {
                            let index = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2038, _) => {
                                    source_ml_source2_data_property_value2038
                                }
                            }, identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2039, _) => source_ml_source2_data_property_value2039.as_slice() }), self::ml_array::number_index(index));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                } else {
                                    {
                                        let prototype_2 = match parent.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2040, _, _, _) => {
                                                let source_ml_source2_data_property_value2040 = *source_ml_source2_data_property_value2040;
                                                source_ml_source2_data_property_value2040.clone()
                                            }
                                        };
                                        {
                                            (prototype, depth) = (prototype_2.clone(), depth_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop105(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result {
    loop {
        return if (at < (((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2042) => source_ml_source2_data_property_value2042.as_slice() }).len() as f64))) {
            {
                let property = self::ml_array::optional_at((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2043) => source_ml_source2_data_property_value2043.as_slice() }), self::ml_array::number_index(at));
                if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_key_error(heap.clone(), match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2044, _) => {
                                let source_ml_source2_data_property_value2044 = *source_ml_source2_data_property_value2044;
                                source_ml_source2_data_property_value2044.clone()
                            }
                        });
                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(key_error.clone())
                        } else {
                            if ((self::find_java_script_ordinary_property(match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2047) => {
                                    source_ml_source2_data_property_value2047
                                }
                            }, match property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2048, _) => {
                                    let source_ml_source2_data_property_value2048 = *source_ml_source2_data_property_value2048;
                                    source_ml_source2_data_property_value2048.clone()
                                }
                            })) != at) {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                            } else {
                                {
                                    let descriptor = match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2049) => {
                                            source_ml_source2_data_property_value2049.clone()
                                        }
                                    };
                                    if !(self::test_complete_java_script_property_descriptor(descriptor.clone())) {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 105, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115]))
                                    } else {
                                        {
                                            let source_ml_source2_s2050 = if (descriptor.clone()).9 {
                                                {
                                                    let error = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8);
                                                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Return(error.clone())
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Next
                                                    }
                                                }
                                            } else {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Next
                                            };
                                            match source_ml_source2_s2050.clone() {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Next => {
                                                    {
                                                        let source_ml_source2_s2051 = if (descriptor.clone()).5 {
                                                            {
                                                                let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4);
                                                                if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(error_2.clone())
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next
                                                        };
                                                        match source_ml_source2_s2051.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Next => {
                                                                if (descriptor.clone()).7 {
                                                                    {
                                                                        let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6);
                                                                        if (error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(error_3.clone())
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
                                                                    {
                                                                        let at_3 = (at + 1f64);
                                                                        {
                                                                            at = at_3;
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin108Return(source_ml_source1_temporary_result108) => {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(source_ml_source1_temporary_result108.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin107Return(source_ml_source1_temporary_result108_2) => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(source_ml_source1_temporary_result108_2.clone())
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
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop104(mut heap: self::JavaScriptOrdinaryHeap, mut property_count: f64, mut source_ml_source1_temporary_values101: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index101: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result {
    loop {
        return if (source_ml_source1_temporary_index101 < (self::ml::Big::from_u128(source_ml_source1_temporary_values101.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values101, source_ml_source1_temporary_index101.to_index());
                if ({
                    let source_ml_source2_s2053 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2061, _, _, _) => {
                            let source_ml_source2_data_property_value2061 = *source_ml_source2_data_property_value2061;
                            source_ml_source2_data_property_value2061.clone()
                        }
                    };
                    match source_ml_source2_s2053.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            false
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2054) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f2055) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f2056) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f2057) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2058) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f2059, source_ml_source2_f2060) => {
                            true
                        }
                    }
                } && {
                    let source_ml_source2_s2063 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2071, _, _, _) => {
                            let source_ml_source2_data_property_value2071 = *source_ml_source2_data_property_value2071;
                            source_ml_source2_data_property_value2071.clone()
                        }
                    };
                    match source_ml_source2_s2063.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2064) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f2065) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f2066) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source2_f2067) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2068) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f2069, source_ml_source2_f2070) => {
                            false
                        }
                    }
                }) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]))
                } else {
                    {
                        let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2072, _, _, _) => {
                                let source_ml_source2_data_property_value2072 = *source_ml_source2_data_property_value2072;
                                source_ml_source2_data_property_value2072.clone()
                            }
                        });
                        if (prototype_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(prototype_error.clone())
                        } else {
                            {
                                let property_count_2 = (property_count + ((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2073) => source_ml_source2_data_property_value2073.as_slice() }).len() as f64));
                                if ((property_count_2 > (16384f64)) || ((((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2075) => source_ml_source2_data_property_value2075.as_slice() }).len() as f64)) > (4096f64))) {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source2_s2076 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop105(heap.clone(), object.clone(), at);
                                            match source_ml_source2_s2076.clone() {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Done => {
                                                    {
                                                        let prototype = match object.clone() {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2077, _, _, _) => {
                                                                let source_ml_source2_data_property_value2077 = *source_ml_source2_data_property_value2077;
                                                                source_ml_source2_data_property_value2077.clone()
                                                            }
                                                        };
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source2_s2078 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop106(heap.clone(), object.clone(), prototype.clone(), depth);
                                                                match source_ml_source2_s2078.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Done => {
                                                                        {
                                                                            let source_ml_source1_temporary_index101_2 = source_ml_source1_temporary_index101.add(&self::ml::Big::from_i128(1));
                                                                            {
                                                                                (property_count, source_ml_source1_temporary_index101) = (property_count_2, source_ml_source1_temporary_index101_2.clone());
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop106Return(source_ml_source1_temporary_result106) => {
                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(source_ml_source1_temporary_result106.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop105Return(source_ml_source1_temporary_result106_2) => {
                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Return(source_ml_source1_temporary_result106_2.clone())
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
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop104Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop103(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_source1_temporary_values100: Vec<self::ml::Big>, mut source_ml_source1_temporary_index100: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result {
    loop {
        return if (source_ml_source1_temporary_index100 < (self::ml::Big::from_u128(source_ml_source1_temporary_values100.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_source1_temporary_values100, source_ml_source1_temporary_index100.to_index());
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2080, _) => {
                        source_ml_source2_data_property_value2080
                    }
                }, symbol.clone())) >= (0f64))) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = symbol.clone();
                        {
                            let source_ml_source1_temporary_index100_2 = source_ml_source1_temporary_index100.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_source1_temporary_index100) = (previous_2.clone(), source_ml_source1_temporary_index100_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop102(mut previous: self::ml::Big, mut source_ml_source1_temporary_values99: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index99: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result {
    loop {
        return if (source_ml_source1_temporary_index99 < (self::ml::Big::from_u128(source_ml_source1_temporary_values99.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values99, source_ml_source1_temporary_index99.to_index());
                if (((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2082, _, _, _, _) => {
                        source_ml_source2_data_property_value2082.clone()
                    }
                }) <= previous) || ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2084, _, _, _, _) => {
                        source_ml_source2_data_property_value2084.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2085, _, _, _, _) => {
                                source_ml_source2_data_property_value2085.clone()
                            }
                        };
                        {
                            let source_ml_source1_temporary_index99_2 = source_ml_source1_temporary_index99.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_source1_temporary_index99) = (previous_2.clone(), source_ml_source1_temporary_index99_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result {
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done,
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool, self::ml::Big)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_locate_java_script_ordinary_property_loop109(mut heap: self::JavaScriptOrdinaryHeap, mut key: self::JavaScriptSemanticValue, mut current: self::ml::Big, mut depth: f64) -> self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result {
    loop {
        return if (depth < (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2087, _) => source_ml_source2_data_property_value2087.as_slice() }).len() as f64))) {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2088, _) => {
                        source_ml_source2_data_property_value2088
                    }
                }, current.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2089, _) => source_ml_source2_data_property_value2089.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                    } else {
                        {
                            let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2090) => {
                                    source_ml_source2_data_property_value2090
                                }
                            }, key.clone());
                            {
                                let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2091) => {
                                        source_ml_source2_data_property_value2091
                                    }
                                }, self::ml_array::number_index(at));
                                if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Return({
                                        let source_ml_source2_field2092 = true;
                                        {
                                            let source_ml_source2_field2093 = current.clone();
                                            {
                                                let source_ml_source2_field2095 = match property.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2094) => {
                                                        source_ml_source2_data_property_value2094.clone()
                                                    }
                                                };
                                                (source_ml_source2_field2095.clone(), source_ml_source2_field2092, source_ml_source2_field2093.clone())
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let prototype = match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2096, _, _, _) => {
                                                let source_ml_source2_data_property_value2096 = *source_ml_source2_data_property_value2096;
                                                source_ml_source2_data_property_value2096.clone()
                                            }
                                        };
                                        match prototype.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2097) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source2_f2098) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source2_f2099) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source2_f2100) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2101) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, source_ml_source2_f2102) => {
                                                {
                                                    let depth_2 = (depth + 1f64);
                                                    {
                                                        (current, depth) = (current_2.clone(), depth_2);
                                                        continue;
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
            }
        } else {
            self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_define_java_script_ordinary_property_loop111(mut count: f64, mut source_ml_source1_temporary_values110: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index110: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index110 < (self::ml::Big::from_u128(source_ml_source1_temporary_values110.len() as u128))) {
            {
                let entry = self::ml_array::at(&source_ml_source1_temporary_values110, source_ml_source1_temporary_index110.to_index());
                {
                    let count_2 = (count + ((match &entry { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2103) => source_ml_source2_data_property_value2103.as_slice() }).len() as f64));
                    {
                        let source_ml_source1_temporary_index110_2 = source_ml_source1_temporary_index110.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index110) = (count_2, source_ml_source1_temporary_index110_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            count
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result {
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Done,
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop120(mut state: self::JavaScriptDescriptorConversionState, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result {
    loop {
        return if (index < (6f64)) {
            if self::test_java_script_descriptor_field_presence(match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2152, _) => {
                    source_ml_source2_data_property_value2152.clone()
                }
            }, index) {
                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Return(self::ml_string::Text::from_units(vec![117, 110, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 102, 97, 98, 114, 105, 99, 97, 116, 101, 100, 32, 105, 110, 32, 97, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
            } else {
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result {
    MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Done(Box<self::JavaScriptDescriptorConversionState>),
    MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box<self::JavaScriptDescriptorConversionTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_descriptor_conversion_loop121(mut heap: self::JavaScriptOrdinaryHeap, mut current: self::JavaScriptDescriptorConversionState) -> self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result {
    loop {
        return if ((match current.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2154, _, _) => {
                source_ml_source2_data_property_value2154
            }
        }) < (6f64)) {
            {
                let field = self::read_java_script_descriptor_field(match current.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2155, _, _) => {
                        source_ml_source2_data_property_value2155
                    }
                });
                {
                    let key = self::JavaScriptSemanticValue::Text((field.clone()).2);
                    {
                        let receiver = match current.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value2156, _, _, _) => {
                                let source_ml_source2_data_property_value2156 = *source_ml_source2_data_property_value2156;
                                source_ml_source2_data_property_value2156.clone()
                            }
                        };
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2157) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source2_f2158) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source2_f2159) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source2_f2160) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2161) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Object(identity, source_ml_source2_f2162) => {
                                {
                                    let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                    if !((property.clone()).1) {
                                        {
                                            let current_2 = self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), (match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2163, _, _) => {
                                                    source_ml_source2_data_property_value2163
                                                }
                                            } + 1f64), match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2164, _) => {
                                                    source_ml_source2_data_property_value2164.clone()
                                                }
                                            }, false);
                                            {
                                                current = current_2.clone();
                                                continue;
                                            }
                                        }
                                    } else {
                                        {
                                            let plan = self::plan_java_script_ordinary_get(heap.clone(), identity.clone(), key.clone(), receiver.clone());
                                            if (((plan.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid((plan.clone()).3)))
                                            } else {
                                                if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorCall(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2165, _, _) => {
                                                            source_ml_source2_data_property_value2165
                                                        }
                                                    }, match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2166, _) => {
                                                            source_ml_source2_data_property_value2166.clone()
                                                        }
                                                    }, true)), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)))
                                                } else {
                                                    if (((plan.clone()).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                        self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))))
                                                    } else {
                                                        {
                                                            let applied = self::apply_java_script_descriptor_field(current.clone(), (plan.clone()).5);
                                                            match applied.clone() {
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorContinuation(current_3) => {
                                                                    let current_3 = *current_3;
                                                                    {
                                                                        current = current_3.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source2_f2167, source_ml_source2_f2168, source_ml_source2_f2169, source_ml_source2_f2170) => {
                                                                    let source_ml_source2_f2167 = *source_ml_source2_f2167;
                                                                    let source_ml_source2_f2168 = *source_ml_source2_f2168;
                                                                    let source_ml_source2_f2169 = *source_ml_source2_f2169;
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source2_f2171) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source2_f2172) => {
                                                                    let source_ml_source2_f2172 = *source_ml_source2_f2172;
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source2_f2173, source_ml_source2_f2174) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source2_f2175) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(applied.clone()))
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
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Done(Box::new(current.clone()))
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_finish_java_script_property_key_conversion_loop122(spelling: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (spelling.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source2_string_unit_operand2176 = spelling.clone();
                    {
                        let source_ml_source2_string_unit_operand2177 = index;
                        (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand2183 = {
                                        let source_ml_source2_string_range_operand2178 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand2179 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand2180 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position2181 = source_ml_source2_string_range_operand2179;
                                                    {
                                                        let source_ml_source2_string_range_position2182 = source_ml_source2_string_range_operand2180;
                                                        source_ml_source2_string_range_operand2178.slice(source_ml_source2_string_range_position2181, source_ml_source2_string_range_position2182)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand2184 = 0f64;
                                        source_ml_source2_string_unit_operand2183.code_point_at(source_ml_source2_string_unit_operand2184)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand2176.clone(), source_ml_source2_string_unit_operand2177)
                    }
                }], units); self::ml_array::append(ml_append_left, ml_append_right) };
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop132(mut fn_: self::JavaScriptCallFunction, source_ml_source1_temporary_values127: &[f64], mut source_ml_source1_temporary_index127: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result {
    loop {
        return if (source_ml_source1_temporary_index127 < (self::ml::Big::from_u128(source_ml_source1_temporary_values127.len() as u128))) {
            {
                let register = self::ml_array::at(source_ml_source1_temporary_values127, source_ml_source1_temporary_index127.to_index());
                if !(self::test_java_script_call_coordinate(register, match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2185, _, _) => {
                        source_ml_source2_data_property_value2185
                    }
                })) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index127_2 = source_ml_source1_temporary_index127.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index127 = source_ml_source1_temporary_index127_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Done
        };
    }
}
