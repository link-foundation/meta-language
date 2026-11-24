// Generated from complete js/src/translation/javascript-property-definition.js. Do not edit by hand.
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_literal_property_index;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop14;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop15;
use super::test_java_script_descriptor_field_presence;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop13(mut heap: self::JavaScriptOrdinaryHeap, mut property_count: f64, mut source_ml_temporary_values10: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index10: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result {
    loop {
        return if (source_ml_temporary_index10 < (self::ml::Big::from_u128(source_ml_temporary_values10.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_temporary_values10, source_ml_temporary_index10.to_index());
                if ({
                    let source_ml_source1_s1071 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1079, _, _, _) => {
                            let source_ml_source1_data_property_value1079 = *source_ml_source1_data_property_value1079;
                            source_ml_source1_data_property_value1079.clone()
                        }
                    };
                    match source_ml_source1_s1071.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            false
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1072) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1073) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1074) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1075) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1076) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1077, source_ml_source1_f1078) => {
                            true
                        }
                    }
                } && {
                    let source_ml_source1_s1081 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1089, _, _, _) => {
                            let source_ml_source1_data_property_value1089 = *source_ml_source1_data_property_value1089;
                            source_ml_source1_data_property_value1089.clone()
                        }
                    };
                    match source_ml_source1_s1081.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1082) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1083) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1084) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1085) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1086) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1087, source_ml_source1_f1088) => {
                            false
                        }
                    }
                }) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]))
                } else {
                    {
                        let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1090, _, _, _) => {
                                let source_ml_source1_data_property_value1090 = *source_ml_source1_data_property_value1090;
                                source_ml_source1_data_property_value1090.clone()
                            }
                        });
                        if (prototype_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(prototype_error.clone())
                        } else {
                            {
                                let property_count_2 = (property_count + ((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1091) => source_ml_source1_data_property_value1091.as_slice() }).len() as f64));
                                if ((property_count_2 > (16384f64)) || ((((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1093) => source_ml_source1_data_property_value1093.as_slice() }).len() as f64)) > (4096f64))) {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source1_s1094 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop14(heap.clone(), object.clone(), at);
                                            match source_ml_source1_s1094.clone() {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done => {
                                                    {
                                                        let prototype = match object.clone() {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1095, _, _, _) => {
                                                                let source_ml_source1_data_property_value1095 = *source_ml_source1_data_property_value1095;
                                                                source_ml_source1_data_property_value1095.clone()
                                                            }
                                                        };
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source1_s1096 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop15(heap.clone(), object.clone(), prototype.clone(), depth);
                                                                match source_ml_source1_s1096.clone() {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done => {
                                                                        {
                                                                            let source_ml_temporary_index10_2 = source_ml_temporary_index10.add(&self::ml::Big::from_i128(1));
                                                                            {
                                                                                (property_count, source_ml_temporary_index10) = (property_count_2, source_ml_temporary_index10_2.clone());
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(source_ml_temporary_result15) => {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_temporary_result15.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_temporary_result15_2) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_temporary_result15_2.clone())
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
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop12(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_temporary_values9: Vec<self::ml::Big>, mut source_ml_temporary_index9: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    loop {
        return if (source_ml_temporary_index9 < (self::ml::Big::from_u128(source_ml_temporary_values9.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_temporary_values9, source_ml_temporary_index9.to_index());
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1098, _) => {
                        source_ml_source1_data_property_value1098
                    }
                }, symbol.clone())) >= (0f64))) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = symbol.clone();
                        {
                            let source_ml_temporary_index9_2 = source_ml_temporary_index9.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_temporary_index9) = (previous_2.clone(), source_ml_temporary_index9_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(self::ml::Big),
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop11(mut previous: self::ml::Big, mut source_ml_temporary_values8: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index8: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result {
    loop {
        return if (source_ml_temporary_index8 < (self::ml::Big::from_u128(source_ml_temporary_values8.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_temporary_values8, source_ml_temporary_index8.to_index());
                if (((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1100, _, _, _, _) => {
                        source_ml_source1_data_property_value1100.clone()
                    }
                }) <= previous) || ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1102, _, _, _, _) => {
                        source_ml_source1_data_property_value1102.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1103, _, _, _, _) => {
                                source_ml_source1_data_property_value1103.clone()
                            }
                        };
                        {
                            let source_ml_temporary_index8_2 = source_ml_temporary_index8.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_temporary_index8) = (previous_2.clone(), source_ml_temporary_index8_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result {
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done,
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool, self::ml::Big)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_locate_java_script_ordinary_property_loop18(mut heap: self::JavaScriptOrdinaryHeap, mut key: self::JavaScriptSemanticValue, mut current: self::ml::Big, mut depth: f64) -> self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result {
    loop {
        return if (depth < (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1105, _) => source_ml_source1_data_property_value1105.as_slice() }).len() as f64))) {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1106, _) => {
                        source_ml_source1_data_property_value1106
                    }
                }, current.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1107, _) => source_ml_source1_data_property_value1107.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                    } else {
                        {
                            let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1108) => {
                                    source_ml_source1_data_property_value1108
                                }
                            }, key.clone());
                            {
                                let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1109) => {
                                        source_ml_source1_data_property_value1109
                                    }
                                }, self::ml_array::number_index(at));
                                if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Return({
                                        let source_ml_source1_field1110 = true;
                                        {
                                            let source_ml_source1_field1111 = current.clone();
                                            {
                                                let source_ml_source1_field1113 = match property.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1112) => {
                                                        source_ml_source1_data_property_value1112.clone()
                                                    }
                                                };
                                                (source_ml_source1_field1113.clone(), source_ml_source1_field1110, source_ml_source1_field1111.clone())
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let prototype = match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1114, _, _, _) => {
                                                let source_ml_source1_data_property_value1114 = *source_ml_source1_data_property_value1114;
                                                source_ml_source1_data_property_value1114.clone()
                                            }
                                        };
                                        match prototype.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1115) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1116) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1117) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1118) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1119) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, source_ml_source1_f1120) => {
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
            self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_define_java_script_ordinary_property_loop20(mut count: f64, mut source_ml_temporary_values19: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index19: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index19 < (self::ml::Big::from_u128(source_ml_temporary_values19.len() as u128))) {
            {
                let entry = self::ml_array::at(&source_ml_temporary_values19, source_ml_temporary_index19.to_index());
                {
                    let count_2 = (count + ((match &entry { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1121) => source_ml_source1_data_property_value1121.as_slice() }).len() as f64));
                    {
                        let source_ml_temporary_index19_2 = source_ml_temporary_index19.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_temporary_index19) = (count_2, source_ml_temporary_index19_2.clone());
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result {
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done,
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return((self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_set_java_script_ordinary_prototype_loop21(mut heap: self::JavaScriptOrdinaryHeap, mut identity: self::ml::Big, mut next: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result {
    loop {
        return if (depth <= (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1123, _) => source_ml_source1_data_property_value1123.as_slice() }).len() as f64))) {
            match next.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Null => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1124) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f1125) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1126) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f1127) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1128) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
                }
                self::JavaScriptSemanticValue::Object(next_identity, source_ml_source1_f1129) => {
                    if (next_identity == identity) {
                        self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                            let source_ml_source1_field1130 = heap.clone();
                            {
                                let source_ml_source1_field1131 = false;
                                {
                                    let source_ml_source1_field1132 = self::ml_string::Text::from_units(vec![]);
                                    (source_ml_source1_field1132.clone(), source_ml_source1_field1130.clone(), source_ml_source1_field1131)
                                }
                            }
                        })
                    } else {
                        {
                            let at = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1133, _) => {
                                    source_ml_source1_data_property_value1133
                                }
                            }, next_identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1134, _) => source_ml_source1_data_property_value1134.as_slice() }), self::ml_array::number_index(at));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Return({
                                        let source_ml_source1_field1135 = heap.clone();
                                        {
                                            let source_ml_source1_field1136 = false;
                                            {
                                                let source_ml_source1_field1137 = self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]);
                                                (source_ml_source1_field1137.clone(), source_ml_source1_field1135.clone(), source_ml_source1_field1136)
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let next_2 = match parent.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1138, _, _, _) => {
                                                let source_ml_source1_data_property_value1138 = *source_ml_source1_data_property_value1138;
                                                source_ml_source1_data_property_value1138.clone()
                                            }
                                        };
                                        {
                                            let depth_2 = (depth + 1f64);
                                            {
                                                (next, depth) = (next_2.clone(), depth_2);
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
        } else {
            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop21Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result {
    MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Done(Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop24(positions: &[f64], mut position: f64, mut at: f64) -> f64 {
    loop {
        return if ((at < ((positions.len() as f64))) && (((self::ml_array::optional_at(positions, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) < position)) {
            {
                let at_2 = (at + 1f64);
                {
                    at = at_2;
                    continue;
                }
            }
        } else {
            at
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop23(mut numeric: Vec<self::JavaScriptSemanticValue>, mut positions: Vec<f64>, mut text: Vec<self::JavaScriptSemanticValue>, mut symbols: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_values22: Vec<self::JavaScriptOrdinaryProperty>, mut source_ml_temporary_index22: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result {
    loop {
        return if (source_ml_temporary_index22 < (self::ml::Big::from_u128(source_ml_temporary_values22.len() as u128))) {
            {
                let property = self::ml_array::at(&source_ml_temporary_values22, source_ml_temporary_index22.to_index());
                {
                    let key = match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1139, _) => {
                            let source_ml_source1_data_property_value1139 = *source_ml_source1_data_property_value1139;
                            source_ml_source1_data_property_value1139.clone()
                        }
                    };
                    match key.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            {
                                let symbols_2 = {
                                    let source_ml_source1_concat_value1146 = symbols;
                                    {
                                        let source_ml_source1_concat_value1147 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1146), source_ml_source1_concat_value1147)
                                    }
                                };
                                {
                                    let source_ml_temporary_index22_2 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index22) = (symbols_2, source_ml_temporary_index22_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Null => {
                            {
                                let symbols_3 = {
                                    let source_ml_source1_concat_value1148 = symbols;
                                    {
                                        let source_ml_source1_concat_value1149 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1148), source_ml_source1_concat_value1149)
                                    }
                                };
                                {
                                    let source_ml_temporary_index22_3 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index22) = (symbols_3, source_ml_temporary_index22_3.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1140) => {
                            {
                                let symbols_4 = {
                                    let source_ml_source1_concat_value1150 = symbols;
                                    {
                                        let source_ml_source1_concat_value1151 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1150), source_ml_source1_concat_value1151)
                                    }
                                };
                                {
                                    let source_ml_temporary_index22_4 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index22) = (symbols_4, source_ml_temporary_index22_4.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1141) => {
                            {
                                let symbols_5 = {
                                    let source_ml_source1_concat_value1152 = symbols;
                                    {
                                        let source_ml_source1_concat_value1153 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1152), source_ml_source1_concat_value1153)
                                    }
                                };
                                {
                                    let source_ml_temporary_index22_5 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index22) = (symbols_5, source_ml_temporary_index22_5.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1142) => {
                            {
                                let symbols_6 = {
                                    let source_ml_source1_concat_value1154 = symbols;
                                    {
                                        let source_ml_source1_concat_value1155 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1154), source_ml_source1_concat_value1155)
                                    }
                                };
                                {
                                    let source_ml_temporary_index22_6 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index22) = (symbols_6, source_ml_temporary_index22_6.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Text(units) => {
                            {
                                let position = self::read_java_script_literal_property_index(&(units));
                                if (position < (0f64)) {
                                    {
                                        let text_2 = {
                                            let source_ml_source1_concat_value1156 = text;
                                            {
                                                let source_ml_source1_concat_value1157 = vec![key.clone()];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1156), source_ml_source1_concat_value1157)
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index22_7 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                            {
                                                (text, source_ml_temporary_index22) = (text_2, source_ml_temporary_index22_7.clone());
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let at_2 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop24(&(positions), position, at);
                                            {
                                                let numeric_2 = {
                                                    let source_ml_source1_concat_value1158 = self::ml_array::slice(&numeric, 0f64, at_2);
                                                    {
                                                        let source_ml_source1_concat_value1159 = vec![key.clone()];
                                                        {
                                                            let source_ml_source1_concat_value1160 = self::ml_array::slice(&numeric, at_2, f64::INFINITY);
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1158), source_ml_source1_concat_value1159), source_ml_source1_concat_value1160)
                                                        }
                                                    }
                                                };
                                                {
                                                    let positions_2 = {
                                                        let source_ml_source1_concat_value1161 = self::ml_array::slice(&positions, 0f64, at_2);
                                                        {
                                                            let source_ml_source1_concat_value1162 = vec![position];
                                                            {
                                                                let source_ml_source1_concat_value1163 = self::ml_array::slice(&positions, at_2, f64::INFINITY);
                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value1161), source_ml_source1_concat_value1162), source_ml_source1_concat_value1163)
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_temporary_index22_8 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                                        {
                                                            (numeric, positions, source_ml_temporary_index22) = (numeric_2, positions_2, source_ml_temporary_index22_8.clone());
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
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1143) => {
                            {
                                let symbols_7 = {
                                    let source_ml_source1_concat_value1164 = symbols;
                                    {
                                        let source_ml_source1_concat_value1165 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1164), source_ml_source1_concat_value1165)
                                    }
                                };
                                {
                                    let source_ml_temporary_index22_9 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index22) = (symbols_7, source_ml_temporary_index22_9.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1144, source_ml_source1_f1145) => {
                            {
                                let symbols_8 = {
                                    let source_ml_source1_concat_value1166 = symbols;
                                    {
                                        let source_ml_source1_concat_value1167 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1166), source_ml_source1_concat_value1167)
                                    }
                                };
                                {
                                    let source_ml_temporary_index22_10 = source_ml_temporary_index22.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index22) = (symbols_8, source_ml_temporary_index22_10.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop23Done(numeric, text, symbols)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_insert_java_script_ordinary_object_loop26(mut identity: self::ml::Big, mut at: f64, mut source_ml_temporary_values25: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index25: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index25 < (self::ml::Big::from_u128(source_ml_temporary_values25.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_temporary_values25, source_ml_temporary_index25.to_index());
                if ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1169, _, _, _, _) => {
                        source_ml_source1_data_property_value1169.clone()
                    }
                }) < identity) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_temporary_index25_2 = source_ml_temporary_index25.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index25) = (at_2, source_ml_temporary_index25_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index25_3 = source_ml_temporary_index25.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index25 = source_ml_temporary_index25_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            at
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_register_java_script_ordinary_symbol_loop28(mut identity: self::ml::Big, mut at: f64, mut source_ml_temporary_values27: Vec<self::ml::Big>, mut source_ml_temporary_index27: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index27 < (self::ml::Big::from_u128(source_ml_temporary_values27.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_temporary_values27, source_ml_temporary_index27.to_index());
                if (symbol < identity) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_temporary_index27_2 = source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index27) = (at_2, source_ml_temporary_index27_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index27_3 = source_ml_temporary_index27.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index27 = source_ml_temporary_index27_3.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            at
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_finish_java_script_property_key_conversion_loop29(spelling: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (spelling.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source1_string_unit_operand1170 = spelling.clone();
                    {
                        let source_ml_source1_string_unit_operand1171 = index;
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
                                    let source_ml_source1_string_unit_operand1177 = {
                                        let source_ml_source1_string_range_operand1172 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand1173 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand1174 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position1175 = source_ml_source1_string_range_operand1173;
                                                    {
                                                        let source_ml_source1_string_range_position1176 = source_ml_source1_string_range_operand1174;
                                                        source_ml_source1_string_range_operand1172.slice(source_ml_source1_string_range_position1175, source_ml_source1_string_range_position1176)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand1178 = 0f64;
                                        source_ml_source1_string_unit_operand1177.code_point_at(source_ml_source1_string_unit_operand1178)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand1170.clone(), source_ml_source1_string_unit_operand1171)
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
pub enum MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Result {
    MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Done,
    MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop30(mut state: self::JavaScriptDescriptorConversionState, mut index: f64) -> self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Result {
    loop {
        return if (index < (6f64)) {
            if self::test_java_script_descriptor_field_presence(match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value1179, _) => {
                    source_ml_source1_data_property_value1179.clone()
                }
            }, index) {
                self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Return(self::ml_string::Text::from_units(vec![117, 110, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 102, 97, 98, 114, 105, 99, 97, 116, 101, 100, 32, 105, 110, 32, 97, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
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
            self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop30Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryStepJavaScriptDescriptorConversionLoop31Result {
    MlTemporaryStepJavaScriptDescriptorConversionLoop31Done(Box<self::JavaScriptDescriptorConversionState>),
    MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box<self::JavaScriptDescriptorConversionTransition>),
}
