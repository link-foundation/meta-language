// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result;
use super::apply_java_script_descriptor_field;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_descriptor_field;
use super::read_java_script_literal_property_index;
use super::test_java_script_call_coordinate;
use super::test_java_script_descriptor_field_presence;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop102(mut previous: self::ml::Big, mut source_ml_source1_temporary_values99: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index99: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result {
    loop {
        return if (source_ml_source1_temporary_index99 < (self::ml::Big::from_u128(source_ml_source1_temporary_values99.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values99, source_ml_source1_temporary_index99.to_index());
                if (((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2078, _, _, _, _) => {
                        source_ml_source2_data_property_value2078.clone()
                    }
                }) <= previous) || ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2080, _, _, _, _) => {
                        source_ml_source2_data_property_value2080.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2081, _, _, _, _) => {
                                source_ml_source2_data_property_value2081.clone()
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
        return if (depth < (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2083, _) => source_ml_source2_data_property_value2083.as_slice() }).len() as f64))) {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2084, _) => {
                        source_ml_source2_data_property_value2084
                    }
                }, current.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2085, _) => source_ml_source2_data_property_value2085.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                    } else {
                        {
                            let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2086) => {
                                    source_ml_source2_data_property_value2086
                                }
                            }, key.clone());
                            {
                                let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2087) => {
                                        source_ml_source2_data_property_value2087
                                    }
                                }, self::ml_array::number_index(at));
                                if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Return({
                                        let source_ml_source2_field2088 = true;
                                        {
                                            let source_ml_source2_field2089 = current.clone();
                                            {
                                                let source_ml_source2_field2091 = match property.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2090) => {
                                                        source_ml_source2_data_property_value2090.clone()
                                                    }
                                                };
                                                (source_ml_source2_field2091.clone(), source_ml_source2_field2088, source_ml_source2_field2089.clone())
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let prototype = match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2092, _, _, _) => {
                                                let source_ml_source2_data_property_value2092 = *source_ml_source2_data_property_value2092;
                                                source_ml_source2_data_property_value2092.clone()
                                            }
                                        };
                                        match prototype.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2093) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source2_f2094) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source2_f2095) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source2_f2096) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2097) => {
                                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop109Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, source_ml_source2_f2098) => {
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
                    let count_2 = (count + ((match &entry { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2099) => source_ml_source2_data_property_value2099.as_slice() }).len() as f64));
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result {
    MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Done,
    MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Return((self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_set_java_script_ordinary_prototype_loop112(mut heap: self::JavaScriptOrdinaryHeap, mut identity: self::ml::Big, mut next: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result {
    loop {
        return if (depth <= (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2101, _) => source_ml_source2_data_property_value2101.as_slice() }).len() as f64))) {
            match next.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Done
                }
                self::JavaScriptSemanticValue::Null => {
                    self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Done
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2102) => {
                    self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Done
                }
                self::JavaScriptSemanticValue::Number(source_ml_source2_f2103) => {
                    self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Done
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source2_f2104) => {
                    self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Done
                }
                self::JavaScriptSemanticValue::Text(source_ml_source2_f2105) => {
                    self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Done
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2106) => {
                    self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Done
                }
                self::JavaScriptSemanticValue::Object(next_identity, source_ml_source2_f2107) => {
                    if (next_identity == identity) {
                        self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Return({
                            let source_ml_source2_field2108 = heap.clone();
                            {
                                let source_ml_source2_field2109 = false;
                                {
                                    let source_ml_source2_field2110 = self::ml_string::Text::from_units(vec![]);
                                    (source_ml_source2_field2110.clone(), source_ml_source2_field2108.clone(), source_ml_source2_field2109)
                                }
                            }
                        })
                    } else {
                        {
                            let at = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2111, _) => {
                                    source_ml_source2_data_property_value2111
                                }
                            }, next_identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2112, _) => source_ml_source2_data_property_value2112.as_slice() }), self::ml_array::number_index(at));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Return({
                                        let source_ml_source2_field2113 = heap.clone();
                                        {
                                            let source_ml_source2_field2114 = false;
                                            {
                                                let source_ml_source2_field2115 = self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]);
                                                (source_ml_source2_field2115.clone(), source_ml_source2_field2113.clone(), source_ml_source2_field2114)
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let next_2 = match parent.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2116, _, _, _) => {
                                                let source_ml_source2_data_property_value2116 = *source_ml_source2_data_property_value2116;
                                                source_ml_source2_data_property_value2116.clone()
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
            self::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Result::MlSource1TemporarySetJavaScriptOrdinaryPrototypeLoop112Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop114Result {
    MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop114Done(Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_own_keys_loop115(positions: &[f64], mut position: f64, mut at: f64) -> f64 {
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
pub fn source_ml_source1_temporary_read_java_script_ordinary_own_keys_loop114(mut numeric: Vec<self::JavaScriptSemanticValue>, mut positions: Vec<f64>, mut text: Vec<self::JavaScriptSemanticValue>, mut symbols: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_values113: Vec<self::JavaScriptOrdinaryProperty>, mut source_ml_source1_temporary_index113: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop114Result {
    loop {
        return if (source_ml_source1_temporary_index113 < (self::ml::Big::from_u128(source_ml_source1_temporary_values113.len() as u128))) {
            {
                let property = self::ml_array::at(&source_ml_source1_temporary_values113, source_ml_source1_temporary_index113.to_index());
                {
                    let key = match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value2117, _) => {
                            let source_ml_source2_data_property_value2117 = *source_ml_source2_data_property_value2117;
                            source_ml_source2_data_property_value2117.clone()
                        }
                    };
                    match key.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            {
                                let symbols_2 = {
                                    let source_ml_source2_concat_value2124 = symbols;
                                    {
                                        let source_ml_source2_concat_value2125 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source2_concat_value2124), source_ml_source2_concat_value2125)
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_index113_2 = source_ml_source1_temporary_index113.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_source1_temporary_index113) = (symbols_2, source_ml_source1_temporary_index113_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Null => {
                            {
                                let symbols_3 = {
                                    let source_ml_source2_concat_value2126 = symbols;
                                    {
                                        let source_ml_source2_concat_value2127 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source2_concat_value2126), source_ml_source2_concat_value2127)
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_index113_3 = source_ml_source1_temporary_index113.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_source1_temporary_index113) = (symbols_3, source_ml_source1_temporary_index113_3.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2118) => {
                            {
                                let symbols_4 = {
                                    let source_ml_source2_concat_value2128 = symbols;
                                    {
                                        let source_ml_source2_concat_value2129 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source2_concat_value2128), source_ml_source2_concat_value2129)
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_index113_4 = source_ml_source1_temporary_index113.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_source1_temporary_index113) = (symbols_4, source_ml_source1_temporary_index113_4.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source2_f2119) => {
                            {
                                let symbols_5 = {
                                    let source_ml_source2_concat_value2130 = symbols;
                                    {
                                        let source_ml_source2_concat_value2131 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source2_concat_value2130), source_ml_source2_concat_value2131)
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_index113_5 = source_ml_source1_temporary_index113.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_source1_temporary_index113) = (symbols_5, source_ml_source1_temporary_index113_5.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source2_f2120) => {
                            {
                                let symbols_6 = {
                                    let source_ml_source2_concat_value2132 = symbols;
                                    {
                                        let source_ml_source2_concat_value2133 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source2_concat_value2132), source_ml_source2_concat_value2133)
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_index113_6 = source_ml_source1_temporary_index113.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_source1_temporary_index113) = (symbols_6, source_ml_source1_temporary_index113_6.clone());
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
                                            let source_ml_source2_concat_value2134 = text;
                                            {
                                                let source_ml_source2_concat_value2135 = vec![key.clone()];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source2_concat_value2134), source_ml_source2_concat_value2135)
                                            }
                                        };
                                        {
                                            let source_ml_source1_temporary_index113_7 = source_ml_source1_temporary_index113.add(&self::ml::Big::from_i128(1));
                                            {
                                                (text, source_ml_source1_temporary_index113) = (text_2, source_ml_source1_temporary_index113_7.clone());
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let at_2 = self::source_ml_source1_temporary_read_java_script_ordinary_own_keys_loop115(&(positions), position, at);
                                            {
                                                let numeric_2 = {
                                                    let source_ml_source2_concat_value2136 = self::ml_array::slice(&numeric, 0f64, at_2);
                                                    {
                                                        let source_ml_source2_concat_value2137 = vec![key.clone()];
                                                        {
                                                            let source_ml_source2_concat_value2138 = self::ml_array::slice(&numeric, at_2, f64::INFINITY);
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source2_concat_value2136), source_ml_source2_concat_value2137), source_ml_source2_concat_value2138)
                                                        }
                                                    }
                                                };
                                                {
                                                    let positions_2 = {
                                                        let source_ml_source2_concat_value2139 = self::ml_array::slice(&positions, 0f64, at_2);
                                                        {
                                                            let source_ml_source2_concat_value2140 = vec![position];
                                                            {
                                                                let source_ml_source2_concat_value2141 = self::ml_array::slice(&positions, at_2, f64::INFINITY);
                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source2_concat_value2139), source_ml_source2_concat_value2140), source_ml_source2_concat_value2141)
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_temporary_index113_8 = source_ml_source1_temporary_index113.add(&self::ml::Big::from_i128(1));
                                                        {
                                                            (numeric, positions, source_ml_source1_temporary_index113) = (numeric_2, positions_2, source_ml_source1_temporary_index113_8.clone());
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
                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2121) => {
                            {
                                let symbols_7 = {
                                    let source_ml_source2_concat_value2142 = symbols;
                                    {
                                        let source_ml_source2_concat_value2143 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source2_concat_value2142), source_ml_source2_concat_value2143)
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_index113_9 = source_ml_source1_temporary_index113.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_source1_temporary_index113) = (symbols_7, source_ml_source1_temporary_index113_9.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source2_f2122, source_ml_source2_f2123) => {
                            {
                                let symbols_8 = {
                                    let source_ml_source2_concat_value2144 = symbols;
                                    {
                                        let source_ml_source2_concat_value2145 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source2_concat_value2144), source_ml_source2_concat_value2145)
                                    }
                                };
                                {
                                    let source_ml_source1_temporary_index113_10 = source_ml_source1_temporary_index113.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_source1_temporary_index113) = (symbols_8, source_ml_source1_temporary_index113_10.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop114Result::MlSource1TemporaryReadJavaScriptOrdinaryOwnKeysLoop114Done(numeric, text, symbols)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_insert_java_script_ordinary_object_loop117(mut identity: self::ml::Big, mut at: f64, mut source_ml_source1_temporary_values116: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index116: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index116 < (self::ml::Big::from_u128(source_ml_source1_temporary_values116.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values116, source_ml_source1_temporary_index116.to_index());
                if ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value2147, _, _, _, _) => {
                        source_ml_source2_data_property_value2147.clone()
                    }
                }) < identity) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_source1_temporary_index116_2 = source_ml_source1_temporary_index116.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_source1_temporary_index116) = (at_2, source_ml_source1_temporary_index116_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index116_3 = source_ml_source1_temporary_index116.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index116 = source_ml_source1_temporary_index116_3.clone();
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
pub fn source_ml_source1_temporary_register_java_script_ordinary_symbol_loop119(mut identity: self::ml::Big, mut at: f64, mut source_ml_source1_temporary_values118: Vec<self::ml::Big>, mut source_ml_source1_temporary_index118: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index118 < (self::ml::Big::from_u128(source_ml_source1_temporary_values118.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_source1_temporary_values118, source_ml_source1_temporary_index118.to_index());
                if (symbol < identity) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_source1_temporary_index118_2 = source_ml_source1_temporary_index118.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_source1_temporary_index118) = (at_2, source_ml_source1_temporary_index118_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_source1_temporary_index118_3 = source_ml_source1_temporary_index118.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index118 = source_ml_source1_temporary_index118_3.clone();
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2148, _) => {
                    source_ml_source2_data_property_value2148.clone()
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
            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2150, _, _) => {
                source_ml_source2_data_property_value2150
            }
        }) < (6f64)) {
            {
                let field = self::read_java_script_descriptor_field(match current.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2151, _, _) => {
                        source_ml_source2_data_property_value2151
                    }
                });
                {
                    let key = self::JavaScriptSemanticValue::Text((field.clone()).2);
                    {
                        let receiver = match current.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value2152, _, _, _) => {
                                let source_ml_source2_data_property_value2152 = *source_ml_source2_data_property_value2152;
                                source_ml_source2_data_property_value2152.clone()
                            }
                        };
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source2_f2153) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source2_f2154) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source2_f2155) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source2_f2156) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_f2157) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Object(identity, source_ml_source2_f2158) => {
                                {
                                    let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                    if !((property.clone()).1) {
                                        {
                                            let current_2 = self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), (match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2159, _, _) => {
                                                    source_ml_source2_data_property_value2159
                                                }
                                            } + 1f64), match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2160, _) => {
                                                    source_ml_source2_data_property_value2160.clone()
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
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2161, _, _) => {
                                                            source_ml_source2_data_property_value2161
                                                        }
                                                    }, match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2162, _) => {
                                                            source_ml_source2_data_property_value2162.clone()
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
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source2_f2163, source_ml_source2_f2164, source_ml_source2_f2165, source_ml_source2_f2166) => {
                                                                    let source_ml_source2_f2163 = *source_ml_source2_f2163;
                                                                    let source_ml_source2_f2164 = *source_ml_source2_f2164;
                                                                    let source_ml_source2_f2165 = *source_ml_source2_f2165;
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source2_f2167) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source2_f2168) => {
                                                                    let source_ml_source2_f2168 = *source_ml_source2_f2168;
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source2_f2169, source_ml_source2_f2170) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop121Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source2_f2171) => {
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
                    let source_ml_source2_string_unit_operand2172 = spelling.clone();
                    {
                        let source_ml_source2_string_unit_operand2173 = index;
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
                                    let source_ml_source2_string_unit_operand2179 = {
                                        let source_ml_source2_string_range_operand2174 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand2175 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand2176 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position2177 = source_ml_source2_string_range_operand2175;
                                                    {
                                                        let source_ml_source2_string_range_position2178 = source_ml_source2_string_range_operand2176;
                                                        source_ml_source2_string_range_operand2174.slice(source_ml_source2_string_range_position2177, source_ml_source2_string_range_position2178)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand2180 = 0f64;
                                        source_ml_source2_string_unit_operand2179.code_point_at(source_ml_source2_string_unit_operand2180)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand2172.clone(), source_ml_source2_string_unit_operand2173)
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
                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2181, _, _) => {
                        source_ml_source2_data_property_value2181
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
