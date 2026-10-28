// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_literal_property_index;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_temporary_read_java_script_ordinary_heap_error_loop48;
use super::test_complete_java_script_property_descriptor;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49 {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49Next,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop47(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut at: f64) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result {
    loop {
        return if (at < (((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1941) => source_ml_source1_data_property_value1941.as_slice() }).len() as f64))) {
            {
                let property = self::ml_array::optional_at((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1942) => source_ml_source1_data_property_value1942.as_slice() }), self::ml_array::number_index(at));
                if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_key_error(heap.clone(), match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1943, _) => {
                                let source_ml_source1_data_property_value1943 = *source_ml_source1_data_property_value1943;
                                source_ml_source1_data_property_value1943.clone()
                            }
                        });
                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Return(key_error.clone())
                        } else {
                            if ((self::find_java_script_ordinary_property(match object.clone() {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1946) => {
                                    source_ml_source1_data_property_value1946
                                }
                            }, match property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1947, _) => {
                                    let source_ml_source1_data_property_value1947 = *source_ml_source1_data_property_value1947;
                                    source_ml_source1_data_property_value1947.clone()
                                }
                            })) != at) {
                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                            } else {
                                {
                                    let descriptor = match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1948) => {
                                            source_ml_source1_data_property_value1948.clone()
                                        }
                                    };
                                    if !(self::test_complete_java_script_property_descriptor(descriptor.clone())) {
                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 105, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115]))
                                    } else {
                                        {
                                            let source_ml_source1_s1949 = if (descriptor.clone()).9 {
                                                {
                                                    let error = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8);
                                                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49Return(error.clone())
                                                    } else {
                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49Next
                                                    }
                                                }
                                            } else {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49Next
                                            };
                                            match source_ml_source1_s1949.clone() {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49Next => {
                                                    {
                                                        let source_ml_source1_s1950 = if (descriptor.clone()).5 {
                                                            {
                                                                let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4);
                                                                if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50Return(error_2.clone())
                                                                } else {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50Next
                                                                }
                                                            }
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50Next
                                                        };
                                                        match source_ml_source1_s1950.clone() {
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50Next => {
                                                                if (descriptor.clone()).7 {
                                                                    {
                                                                        let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6);
                                                                        if (error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Return(error_3.clone())
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
                                                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin50Return(source_ml_temporary_result50) => {
                                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Return(source_ml_temporary_result50.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49::MlTemporaryReadJavaScriptOrdinaryHeapErrorJoin49Return(source_ml_temporary_result50_2) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Return(source_ml_temporary_result50_2.clone())
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
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop46(mut heap: self::JavaScriptOrdinaryHeap, mut property_count: f64, mut source_ml_temporary_values43: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index43: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result {
    loop {
        return if (source_ml_temporary_index43 < (self::ml::Big::from_u128(source_ml_temporary_values43.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_temporary_values43, source_ml_temporary_index43.to_index());
                if ({
                    let source_ml_source1_s1952 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1960, _, _, _) => {
                            let source_ml_source1_data_property_value1960 = *source_ml_source1_data_property_value1960;
                            source_ml_source1_data_property_value1960.clone()
                        }
                    };
                    match source_ml_source1_s1952.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            false
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1953) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1954) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1955) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1956) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1957) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1958, source_ml_source1_f1959) => {
                            true
                        }
                    }
                } && {
                    let source_ml_source1_s1962 = match object.clone() {
                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1970, _, _, _) => {
                            let source_ml_source1_data_property_value1970 = *source_ml_source1_data_property_value1970;
                            source_ml_source1_data_property_value1970.clone()
                        }
                    };
                    match source_ml_source1_s1962.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            true
                        }
                        self::JavaScriptSemanticValue::Null => {
                            true
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1963) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1964) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1965) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f1966) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1967) => {
                            true
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1968, source_ml_source1_f1969) => {
                            false
                        }
                    }
                }) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]))
                } else {
                    {
                        let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1971, _, _, _) => {
                                let source_ml_source1_data_property_value1971 = *source_ml_source1_data_property_value1971;
                                source_ml_source1_data_property_value1971.clone()
                            }
                        });
                        if (prototype_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Return(prototype_error.clone())
                        } else {
                            {
                                let property_count_2 = (property_count + ((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1972) => source_ml_source1_data_property_value1972.as_slice() }).len() as f64));
                                if ((property_count_2 > (16384f64)) || ((((match &object { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1974) => source_ml_source1_data_property_value1974.as_slice() }).len() as f64)) > (4096f64))) {
                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let source_ml_source1_s1975 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop47(heap.clone(), object.clone(), at);
                                            match source_ml_source1_s1975.clone() {
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Done => {
                                                    {
                                                        let prototype = match object.clone() {
                                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1976, _, _, _) => {
                                                                let source_ml_source1_data_property_value1976 = *source_ml_source1_data_property_value1976;
                                                                source_ml_source1_data_property_value1976.clone()
                                                            }
                                                        };
                                                        {
                                                            let depth = 0f64;
                                                            {
                                                                let source_ml_source1_s1977 = self::source_ml_temporary_read_java_script_ordinary_heap_error_loop48(heap.clone(), object.clone(), prototype.clone(), depth);
                                                                match source_ml_source1_s1977.clone() {
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Done => {
                                                                        {
                                                                            let source_ml_temporary_index43_2 = source_ml_temporary_index43.add(&self::ml::Big::from_i128(1));
                                                                            {
                                                                                (property_count, source_ml_temporary_index43) = (property_count_2, source_ml_temporary_index43_2.clone());
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop48Return(source_ml_temporary_result48) => {
                                                                        self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Return(source_ml_temporary_result48.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop47Return(source_ml_temporary_result48_2) => {
                                                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Return(source_ml_temporary_result48_2.clone())
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
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop46Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Done,
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop45(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_temporary_values42: Vec<self::ml::Big>, mut source_ml_temporary_index42: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Result {
    loop {
        return if (source_ml_temporary_index42 < (self::ml::Big::from_u128(source_ml_temporary_values42.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_temporary_values42, source_ml_temporary_index42.to_index());
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1979, _) => {
                        source_ml_source1_data_property_value1979
                    }
                }, symbol.clone())) >= (0f64))) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = symbol.clone();
                        {
                            let source_ml_temporary_index42_2 = source_ml_temporary_index42.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_temporary_index42) = (previous_2.clone(), source_ml_temporary_index42_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop45Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Result {
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Done(self::ml::Big),
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop44(mut previous: self::ml::Big, mut source_ml_temporary_values41: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index41: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Result {
    loop {
        return if (source_ml_temporary_index41 < (self::ml::Big::from_u128(source_ml_temporary_values41.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_temporary_values41, source_ml_temporary_index41.to_index());
                if (((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1981, _, _, _, _) => {
                        source_ml_source1_data_property_value1981.clone()
                    }
                }) <= previous) || ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1983, _, _, _, _) => {
                        source_ml_source1_data_property_value1983.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1984, _, _, _, _) => {
                                source_ml_source1_data_property_value1984.clone()
                            }
                        };
                        {
                            let source_ml_temporary_index41_2 = source_ml_temporary_index41.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_temporary_index41) = (previous_2.clone(), source_ml_temporary_index41_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop44Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result {
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Done,
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool, self::ml::Big)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_locate_java_script_ordinary_property_loop51(mut heap: self::JavaScriptOrdinaryHeap, mut key: self::JavaScriptSemanticValue, mut current: self::ml::Big, mut depth: f64) -> self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result {
    loop {
        return if (depth < (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1986, _) => source_ml_source1_data_property_value1986.as_slice() }).len() as f64))) {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1987, _) => {
                        source_ml_source1_data_property_value1987
                    }
                }, current.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1988, _) => source_ml_source1_data_property_value1988.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Done
                    } else {
                        {
                            let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1989) => {
                                    source_ml_source1_data_property_value1989
                                }
                            }, key.clone());
                            {
                                let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1990) => {
                                        source_ml_source1_data_property_value1990
                                    }
                                }, self::ml_array::number_index(at));
                                if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Return({
                                        let source_ml_source1_field1991 = true;
                                        {
                                            let source_ml_source1_field1992 = current.clone();
                                            {
                                                let source_ml_source1_field1994 = match property.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1993) => {
                                                        source_ml_source1_data_property_value1993.clone()
                                                    }
                                                };
                                                (source_ml_source1_field1994.clone(), source_ml_source1_field1991, source_ml_source1_field1992.clone())
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let prototype = match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1995, _, _, _) => {
                                                let source_ml_source1_data_property_value1995 = *source_ml_source1_data_property_value1995;
                                                source_ml_source1_data_property_value1995.clone()
                                            }
                                        };
                                        match prototype.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Done
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Done
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1996) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Done
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1997) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1998) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Done
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1999) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f2000) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, source_ml_source1_f2001) => {
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
            self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop51Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_define_java_script_ordinary_property_loop53(mut count: f64, mut source_ml_temporary_values52: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index52: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index52 < (self::ml::Big::from_u128(source_ml_temporary_values52.len() as u128))) {
            {
                let entry = self::ml_array::at(&source_ml_temporary_values52, source_ml_temporary_index52.to_index());
                {
                    let count_2 = (count + ((match &entry { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value2002) => source_ml_source1_data_property_value2002.as_slice() }).len() as f64));
                    {
                        let source_ml_temporary_index52_2 = source_ml_temporary_index52.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_temporary_index52) = (count_2, source_ml_temporary_index52_2.clone());
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
pub enum MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result {
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Done,
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Return((self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_set_java_script_ordinary_prototype_loop54(mut heap: self::JavaScriptOrdinaryHeap, mut identity: self::ml::Big, mut next: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result {
    loop {
        return if (depth <= (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value2004, _) => source_ml_source1_data_property_value2004.as_slice() }).len() as f64))) {
            match next.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Done
                }
                self::JavaScriptSemanticValue::Null => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Done
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f2005) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Done
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f2006) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Done
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f2007) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Done
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f2008) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Done
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f2009) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Done
                }
                self::JavaScriptSemanticValue::Object(next_identity, source_ml_source1_f2010) => {
                    if (next_identity == identity) {
                        self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Return({
                            let source_ml_source1_field2011 = heap.clone();
                            {
                                let source_ml_source1_field2012 = false;
                                {
                                    let source_ml_source1_field2013 = self::ml_string::Text::from_units(vec![]);
                                    (source_ml_source1_field2013.clone(), source_ml_source1_field2011.clone(), source_ml_source1_field2012)
                                }
                            }
                        })
                    } else {
                        {
                            let at = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value2014, _) => {
                                    source_ml_source1_data_property_value2014
                                }
                            }, next_identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value2015, _) => source_ml_source1_data_property_value2015.as_slice() }), self::ml_array::number_index(at));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Return({
                                        let source_ml_source1_field2016 = heap.clone();
                                        {
                                            let source_ml_source1_field2017 = false;
                                            {
                                                let source_ml_source1_field2018 = self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]);
                                                (source_ml_source1_field2018.clone(), source_ml_source1_field2016.clone(), source_ml_source1_field2017)
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let next_2 = match parent.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value2019, _, _, _) => {
                                                let source_ml_source1_data_property_value2019 = *source_ml_source1_data_property_value2019;
                                                source_ml_source1_data_property_value2019.clone()
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
            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop54Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop56Result {
    MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop56Done(Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop57(positions: &[f64], mut position: f64, mut at: f64) -> f64 {
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
pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop56(mut numeric: Vec<self::JavaScriptSemanticValue>, mut positions: Vec<f64>, mut text: Vec<self::JavaScriptSemanticValue>, mut symbols: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_values55: Vec<self::JavaScriptOrdinaryProperty>, mut source_ml_temporary_index55: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop56Result {
    loop {
        return if (source_ml_temporary_index55 < (self::ml::Big::from_u128(source_ml_temporary_values55.len() as u128))) {
            {
                let property = self::ml_array::at(&source_ml_temporary_values55, source_ml_temporary_index55.to_index());
                {
                    let key = match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value2020, _) => {
                            let source_ml_source1_data_property_value2020 = *source_ml_source1_data_property_value2020;
                            source_ml_source1_data_property_value2020.clone()
                        }
                    };
                    match key.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            {
                                let symbols_2 = {
                                    let source_ml_source1_concat_value2027 = symbols;
                                    {
                                        let source_ml_source1_concat_value2028 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value2027), source_ml_source1_concat_value2028)
                                    }
                                };
                                {
                                    let source_ml_temporary_index55_2 = source_ml_temporary_index55.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index55) = (symbols_2, source_ml_temporary_index55_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Null => {
                            {
                                let symbols_3 = {
                                    let source_ml_source1_concat_value2029 = symbols;
                                    {
                                        let source_ml_source1_concat_value2030 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value2029), source_ml_source1_concat_value2030)
                                    }
                                };
                                {
                                    let source_ml_temporary_index55_3 = source_ml_temporary_index55.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index55) = (symbols_3, source_ml_temporary_index55_3.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f2021) => {
                            {
                                let symbols_4 = {
                                    let source_ml_source1_concat_value2031 = symbols;
                                    {
                                        let source_ml_source1_concat_value2032 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value2031), source_ml_source1_concat_value2032)
                                    }
                                };
                                {
                                    let source_ml_temporary_index55_4 = source_ml_temporary_index55.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index55) = (symbols_4, source_ml_temporary_index55_4.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f2022) => {
                            {
                                let symbols_5 = {
                                    let source_ml_source1_concat_value2033 = symbols;
                                    {
                                        let source_ml_source1_concat_value2034 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value2033), source_ml_source1_concat_value2034)
                                    }
                                };
                                {
                                    let source_ml_temporary_index55_5 = source_ml_temporary_index55.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index55) = (symbols_5, source_ml_temporary_index55_5.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f2023) => {
                            {
                                let symbols_6 = {
                                    let source_ml_source1_concat_value2035 = symbols;
                                    {
                                        let source_ml_source1_concat_value2036 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value2035), source_ml_source1_concat_value2036)
                                    }
                                };
                                {
                                    let source_ml_temporary_index55_6 = source_ml_temporary_index55.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index55) = (symbols_6, source_ml_temporary_index55_6.clone());
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
                                            let source_ml_source1_concat_value2037 = text;
                                            {
                                                let source_ml_source1_concat_value2038 = vec![key.clone()];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value2037), source_ml_source1_concat_value2038)
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index55_7 = source_ml_temporary_index55.add(&self::ml::Big::from_i128(1));
                                            {
                                                (text, source_ml_temporary_index55) = (text_2, source_ml_temporary_index55_7.clone());
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let at_2 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop57(&(positions), position, at);
                                            {
                                                let numeric_2 = {
                                                    let source_ml_source1_concat_value2039 = self::ml_array::slice(&numeric, 0f64, at_2);
                                                    {
                                                        let source_ml_source1_concat_value2040 = vec![key.clone()];
                                                        {
                                                            let source_ml_source1_concat_value2041 = self::ml_array::slice(&numeric, at_2, f64::INFINITY);
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value2039), source_ml_source1_concat_value2040), source_ml_source1_concat_value2041)
                                                        }
                                                    }
                                                };
                                                {
                                                    let positions_2 = {
                                                        let source_ml_source1_concat_value2042 = self::ml_array::slice(&positions, 0f64, at_2);
                                                        {
                                                            let source_ml_source1_concat_value2043 = vec![position];
                                                            {
                                                                let source_ml_source1_concat_value2044 = self::ml_array::slice(&positions, at_2, f64::INFINITY);
                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value2042), source_ml_source1_concat_value2043), source_ml_source1_concat_value2044)
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_temporary_index55_8 = source_ml_temporary_index55.add(&self::ml::Big::from_i128(1));
                                                        {
                                                            (numeric, positions, source_ml_temporary_index55) = (numeric_2, positions_2, source_ml_temporary_index55_8.clone());
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
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f2024) => {
                            {
                                let symbols_7 = {
                                    let source_ml_source1_concat_value2045 = symbols;
                                    {
                                        let source_ml_source1_concat_value2046 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value2045), source_ml_source1_concat_value2046)
                                    }
                                };
                                {
                                    let source_ml_temporary_index55_9 = source_ml_temporary_index55.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index55) = (symbols_7, source_ml_temporary_index55_9.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f2025, source_ml_source1_f2026) => {
                            {
                                let symbols_8 = {
                                    let source_ml_source1_concat_value2047 = symbols;
                                    {
                                        let source_ml_source1_concat_value2048 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value2047), source_ml_source1_concat_value2048)
                                    }
                                };
                                {
                                    let source_ml_temporary_index55_10 = source_ml_temporary_index55.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index55) = (symbols_8, source_ml_temporary_index55_10.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop56Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop56Done(numeric, text, symbols)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_insert_java_script_ordinary_object_loop59(mut identity: self::ml::Big, mut at: f64, mut source_ml_temporary_values58: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index58: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index58 < (self::ml::Big::from_u128(source_ml_temporary_values58.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_temporary_values58, source_ml_temporary_index58.to_index());
                if ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value2050, _, _, _, _) => {
                        source_ml_source1_data_property_value2050.clone()
                    }
                }) < identity) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_temporary_index58_2 = source_ml_temporary_index58.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index58) = (at_2, source_ml_temporary_index58_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index58_3 = source_ml_temporary_index58.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index58 = source_ml_temporary_index58_3.clone();
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
pub fn source_ml_temporary_register_java_script_ordinary_symbol_loop61(mut identity: self::ml::Big, mut at: f64, mut source_ml_temporary_values60: Vec<self::ml::Big>, mut source_ml_temporary_index60: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index60 < (self::ml::Big::from_u128(source_ml_temporary_values60.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_temporary_values60, source_ml_temporary_index60.to_index());
                if (symbol < identity) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_temporary_index60_2 = source_ml_temporary_index60.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index60) = (at_2, source_ml_temporary_index60_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index60_3 = source_ml_temporary_index60.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index60 = source_ml_temporary_index60_3.clone();
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Result {
    MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Done,
    MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Return(self::ml_string::Text),
}
