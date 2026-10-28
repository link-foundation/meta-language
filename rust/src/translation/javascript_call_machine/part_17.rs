// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop15;
use super::test_complete_java_script_property_descriptor;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16 {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop14(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut at: f64) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result {
    loop {
        return match object.clone() {
            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, shared_constructor_field1) => {
                if (at < ((shared_constructor_field1.len() as f64))) {
                    {
                        let property = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(at));
                        if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_key_error(heap.clone(), match property.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2370, _) => {
                                        let source_ml_source2_data_property_value2370 = *source_ml_source2_data_property_value2370;
                                        source_ml_source2_data_property_value2370.clone()
                                    }
                                });
                                if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(key_error.clone())
                                } else {
                                    if ((self::find_java_script_ordinary_property(shared_constructor_field1, match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2374, _) => {
                                            let source_ml_source2_data_property_value2374 = *source_ml_source2_data_property_value2374;
                                            source_ml_source2_data_property_value2374.clone()
                                        }
                                    })) != at) {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                                    } else {
                                        {
                                            let descriptor = match property.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2375) => {
                                                    source_ml_source2_data_property_value2375.clone()
                                                }
                                            };
                                            if !(self::test_complete_java_script_property_descriptor(descriptor.clone())) {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97, 108, 105, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115]))
                                            } else {
                                                {
                                                    let source_ml_source2_s2376 = if (descriptor.clone()).9 {
                                                        {
                                                            let error = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).8);
                                                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(error.clone())
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next
                                                            }
                                                        }
                                                    } else {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next
                                                    };
                                                    match source_ml_source2_s2376.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Next => {
                                                            {
                                                                let source_ml_source2_s2377 = if (descriptor.clone()).5 {
                                                                    {
                                                                        let error_2 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).4);
                                                                        if (error_2 != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(error_2.clone())
                                                                        } else {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                                                        }
                                                                    }
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next
                                                                };
                                                                match source_ml_source2_s2377.clone() {
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Next => {
                                                                        if (descriptor.clone()).7 {
                                                                            {
                                                                                let error_3 = self::read_java_script_ordinary_value_error(heap.clone(), (descriptor.clone()).6);
                                                                                if (error_3 != (self::ml_string::Text::from_units(vec![]))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(error_3.clone())
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
                                                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin17Return(source_ml_source1_temporary_result17) => {
                                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_source1_temporary_result17.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorJoin16Return(source_ml_source1_temporary_result17_2) => {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_source1_temporary_result17_2.clone())
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
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop13(mut heap: self::JavaScriptOrdinaryHeap, mut property_count: f64, mut source_ml_source1_temporary_values10: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index10: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result {
    loop {
        return if (source_ml_source1_temporary_index10 < (self::ml::Big::from_u128(source_ml_source1_temporary_values10.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values10, source_ml_source1_temporary_index10.to_index());
                match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, shared_constructor_field1, _, _, shared_constructor_field2) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        if ({
                            let source_ml_source2_s2379 = shared_constructor_field1.clone();
                            match source_ml_source2_s2379.clone() {
                                self::JavaScriptSemanticValue::Null => {
                                    false
                                }
                                _ => {
                                    true
                                }
                            }
                        } && {
                            let source_ml_source2_s2389 = shared_constructor_field1.clone();
                            match source_ml_source2_s2389.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    false
                                }
                                _ => {
                                    true
                                }
                            }
                        }) {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 110, 117, 108, 108, 32, 111, 114, 32, 97, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 79, 98, 106, 101, 99, 116]))
                        } else {
                            {
                                let prototype_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                                if (prototype_error != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(prototype_error.clone())
                                } else {
                                    {
                                        let property_count_2 = (property_count + (shared_constructor_field2.len() as f64));
                                        if ((property_count_2 > (16384f64)) || (((shared_constructor_field2.len() as f64)) > (4096f64))) {
                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            {
                                                let at = 0f64;
                                                {
                                                    let source_ml_source2_s2402 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop14(heap.clone(), object.clone(), at);
                                                    match source_ml_source2_s2402.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Done => {
                                                            {
                                                                let prototype = shared_constructor_field1.clone();
                                                                {
                                                                    let depth = 0f64;
                                                                    {
                                                                        let source_ml_source2_s2404 = self::source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop15(heap.clone(), object.clone(), prototype.clone(), depth);
                                                                        match source_ml_source2_s2404.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Done => {
                                                                                {
                                                                                    let source_ml_source1_temporary_index10_2 = source_ml_source1_temporary_index10.add(&self::ml::Big::from_i128(1));
                                                                                    {
                                                                                        (property_count, source_ml_source1_temporary_index10) = (property_count_2, source_ml_source1_temporary_index10_2.clone());
                                                                                        continue;
                                                                                    }
                                                                                }
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop15Return(source_ml_source1_temporary_result15) => {
                                                                                self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_source1_temporary_result15.clone())
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop14Return(source_ml_source1_temporary_result15_2) => {
                                                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(source_ml_source1_temporary_result15_2.clone())
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
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop13Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop12(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_source1_temporary_values9: Vec<self::ml::Big>, mut source_ml_source1_temporary_index9: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    loop {
        return if (source_ml_source1_temporary_index9 < (self::ml::Big::from_u128(source_ml_source1_temporary_values9.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_source1_temporary_values9, source_ml_source1_temporary_index9.to_index());
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2406, _) => {
                        source_ml_source2_data_property_value2406
                    }
                }, symbol.clone())) >= (0f64))) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index9_2 = source_ml_source1_temporary_index9.add(&self::ml::Big::from_i128(1));
                        {
                            (previous, source_ml_source1_temporary_index9) = (symbol.clone(), source_ml_source1_temporary_index9_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop11(mut previous: self::ml::Big, mut source_ml_source1_temporary_values8: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index8: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result {
    loop {
        return if (source_ml_source1_temporary_index8 < (self::ml::Big::from_u128(source_ml_source1_temporary_values8.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values8, source_ml_source1_temporary_index8.to_index());
                match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(shared_constructor_field1, _, _, _, _) => {
                        if ((shared_constructor_field1 <= previous) || (shared_constructor_field1 > (self::ml::Big::from_i128(18446744073709551615)))) {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                        } else {
                            {
                                let previous_2 = shared_constructor_field1.clone();
                                {
                                    let source_ml_source1_temporary_index8_2 = source_ml_source1_temporary_index8.add(&self::ml::Big::from_i128(1));
                                    {
                                        (previous, source_ml_source1_temporary_index8) = (previous_2.clone(), source_ml_source1_temporary_index8_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop11Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result {
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done,
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool, self::ml::Big)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_locate_java_script_ordinary_property_loop18(mut heap: self::JavaScriptOrdinaryHeap, mut key: self::JavaScriptSemanticValue, mut current: self::ml::Big, mut depth: f64) -> self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _) => {
                if (depth < ((shared_constructor_field1.len() as f64))) {
                    {
                        let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), current.clone());
                        {
                            let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(index));
                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2416) => {
                                            source_ml_source2_data_property_value2416
                                        }
                                    }, key.clone());
                                    {
                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2417) => {
                                                source_ml_source2_data_property_value2417
                                            }
                                        }, self::ml_array::number_index(at));
                                        if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                            self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Return({
                                                let source_ml_source2_field2418 = true;
                                                {
                                                    let source_ml_source2_field2421 = match property.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2420) => {
                                                            source_ml_source2_data_property_value2420.clone()
                                                        }
                                                    };
                                                    (source_ml_source2_field2421.clone(), source_ml_source2_field2418, current.clone())
                                                }
                                            })
                                        } else {
                                            {
                                                let prototype = match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2422, _, _, _) => {
                                                        let source_ml_source2_data_property_value2422 = *source_ml_source2_data_property_value2422;
                                                        source_ml_source2_data_property_value2422.clone()
                                                    }
                                                };
                                                match prototype.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                                                    }
                                                    self::JavaScriptSemanticValue::Object(current_2, _) => {
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
                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop18Done
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_define_java_script_ordinary_property_loop20(mut count: f64, mut source_ml_source1_temporary_values19: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index19: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index19 < (self::ml::Big::from_u128(source_ml_source1_temporary_values19.len() as u128))) {
            {
                let entry = self::ml_array::at(&source_ml_source1_temporary_values19, source_ml_source1_temporary_index19.to_index());
                {
                    let count_2 = (count + ((match &entry { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2429) => source_ml_source2_data_property_value2429.as_slice() }).len() as f64));
                    {
                        let source_ml_source1_temporary_index19_2 = source_ml_source1_temporary_index19.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index19) = (count_2, source_ml_source1_temporary_index19_2.clone());
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop32(mut count: self::ml::Big, mut value: self::ml::Big, mut factor: self::ml::Big) -> self::ml::Big {
    loop {
        return if (count > (self::ml::Big::from_i128(0))) {
            {
                let value_3 = if ((self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let value_2 = value.mul(&factor);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let count_2 = self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (count_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let factor_2 = factor.mul(&factor);
                            {
                                (count, value, factor) = (count_2.clone(), value_3.clone(), factor_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (count, value) = (count_2.clone(), value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop33(mut count: self::ml::Big, mut value: self::ml::Big) -> self::ml::Big {
    loop {
        return if (((count > (self::ml::Big::from_i128(0))) && (value != (self::ml::Big::from_i128(0)))) && (value != (self::ml::Big::from_i128(-1)))) {
            {
                let value_2 = if (value < (self::ml::Big::from_i128(0))) {
                    self::ml::divide(&value.sub(&self::ml::Big::from_i128(1)), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                } else {
                    self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                };
                {
                    let count_2 = count.sub(&self::ml::Big::from_i128(1));
                    {
                        (count, value) = (count_2.clone(), value_2.clone());
                        continue;
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31 {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin31Next(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop34Result {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop34Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop34(mut operation: f64, mut negative_left: bool, mut negative_right: bool, mut first: self::ml::Big, mut second: self::ml::Big, mut weight: self::ml::Big, mut result: self::ml::Big) -> self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop34Result {
    loop {
        return if ((first > (self::ml::Big::from_i128(0))) || (second > (self::ml::Big::from_i128(0)))) {
            {
                let a = if negative_left {
                    (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                } else {
                    self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                };
                {
                    let b = if negative_right {
                        (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                    } else {
                        self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                    };
                    {
                        let selected = if (operation == (0f64)) {
                            ((a == (self::ml::Big::from_i128(1))) && (b == (self::ml::Big::from_i128(1))))
                        } else {
                            if (operation == (1f64)) {
                                ((a == (self::ml::Big::from_i128(1))) || (b == (self::ml::Big::from_i128(1))))
                            } else {
                                (a != b)
                            }
                        };
                        {
                            let result_3 = if selected {
                                {
                                    let result_2 = result.add(&weight);
                                    result_2.clone()
                                }
                            } else {
                                result.clone()
                            };
                            {
                                let first_2 = self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                {
                                    let second_2 = self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                    {
                                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                                        {
                                            (first, second, weight, result) = (first_2.clone(), second_2.clone(), weight_2.clone(), result_3.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop34Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop34Done(weight.clone(), result.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop36(mut count: f64, mut factor: f64, mut position: f64) -> f64 {
    loop {
        return if (position < count) {
            {
                let factor_2 = (factor * 2f64);
                {
                    let position_2 = (position + 1f64);
                    {
                        (factor, position) = (factor_2, position_2);
                        continue;
                    }
                }
            }
        } else {
            factor
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop37(mut operation: f64, mut first: f64, mut second: f64, mut result: f64, mut weight: f64, mut position: f64) -> f64 {
    loop {
        return if (position < (32f64)) {
            {
                let first_bit = (first % 2f64);
                {
                    let second_bit = (second % 2f64);
                    {
                        let result_3 = if (((((operation == (0f64)) && (first_bit == (1f64))) && (second_bit == (1f64))) || ((operation == (1f64)) && ((first_bit == (1f64)) || (second_bit == (1f64))))) || ((operation == (2f64)) && (first_bit != second_bit))) {
                            {
                                let result_2 = (result + weight);
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let first_2 = ((first - first_bit) / 2f64);
                            {
                                let second_2 = ((second - second_bit) / 2f64);
                                {
                                    let weight_2 = (weight * 2f64);
                                    {
                                        let position_2 = (position + 1f64);
                                        {
                                            (first, second, result, weight, position) = (first_2, second_2, result_3, weight_2, position_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop42(mut exponent: f64, mut value: self::ml::Big, mut base: self::ml::Big) -> self::ml::Big {
    loop {
        return if (exponent > (0f64)) {
            {
                let value_3 = if (((exponent % 2f64)) == (1f64)) {
                    {
                        let value_2 = value.mul(&base);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let exponent_2 = ((exponent / 2f64)).floor();
                    if (exponent_2 > (0f64)) {
                        {
                            let base_2 = base.mul(&base);
                            {
                                (exponent, value, base) = (exponent_2, value_3.clone(), base_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (exponent, value) = (exponent_2, value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop43(mut value: self::ml::Big, mut result: f64, mut scale: f64) -> f64 {
    loop {
        return if (value > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = (result + scale);
                        result_2
                    }
                } else {
                    result
                };
                {
                    let value_2 = self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let scale_2 = (scale * 2f64);
                        {
                            (value, result, scale) = (value_2.clone(), result_3, scale_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            result
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop45(mut exponent: f64, mut result: f64) -> f64 {
    loop {
        return if (exponent < (0f64)) {
            {
                let result_2 = (result / 2f64);
                {
                    let exponent_2 = (exponent + 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop44Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop44Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop44(mut exponent: f64, mut result: f64) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop44Result {
    loop {
        return if (exponent > (0f64)) {
            {
                let result_2 = (result * 2f64);
                {
                    let exponent_2 = (exponent - 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop44Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop44Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop49Result {
    MlSource1TemporaryConvertStringNumberRationalLoop49Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop49(mut quotient: self::ml::Big, mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop49Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let scale_2 = self::ml::divide(&scale, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = self::ml::divide(&weight, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remainder >= scale_2) {
                        {
                            let remainder_2 = remainder.sub(&scale_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (quotient, remainder, scale, weight) = (quotient_2.clone(), remainder_2.clone(), scale_2.clone(), weight_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (scale, weight) = (scale_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop49Result::MlSource1TemporaryConvertStringNumberRationalLoop49Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop48Result {
    MlSource1TemporaryConvertStringNumberRationalLoop48Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop48(mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop48Result {
    loop {
        return if (scale <= remainder) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (scale, weight) = (scale_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop48Result::MlSource1TemporaryConvertStringNumberRationalLoop48Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin47 {
    MlSource1TemporaryConvertStringNumberRationalJoin47Next(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop50(mut numerator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < shift) {
            {
                let numerator_2 = numerator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (numerator, index) = (numerator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            numerator.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop51(mut denominator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < ((-shift))) {
            {
                let denominator_2 = denominator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (denominator, index) = (denominator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            denominator.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin46 {
    MlSource1TemporaryConvertStringNumberRationalJoin46Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin46Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop52Result {
    MlSource1TemporaryConvertStringNumberRationalLoop52Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop52Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop52(mut numerator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop52Result {
    loop {
        return if (numerator >= (scale.mul(&self::ml::Big::from_i128(2)))) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent + 1f64);
                    if (exponent_2 >= (1024f64)) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop52Result::MlSource1TemporaryConvertStringNumberRationalLoop52Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop52Result::MlSource1TemporaryConvertStringNumberRationalLoop52Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop53Result {
    MlSource1TemporaryConvertStringNumberRationalLoop53Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop53Return(f64),
}
