// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result;
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
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop13(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_temporary_values10: Vec<self::ml::Big>, mut source_ml_temporary_index10: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result {
    loop {
        return if (source_ml_temporary_index10 < (self::ml::Big::from_u128(source_ml_temporary_values10.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_temporary_values10, source_ml_temporary_index10.to_index());
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1803, _) => {
                        source_ml_source1_data_property_value1803
                    }
                }, symbol.clone())) >= (0f64))) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop13Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = symbol.clone();
                        {
                            let source_ml_temporary_index10_2 = source_ml_temporary_index10.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_temporary_index10) = (previous_2.clone(), source_ml_temporary_index10_2.clone());
                                continue;
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
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done(self::ml::Big),
    MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_heap_error_loop12(mut previous: self::ml::Big, mut source_ml_temporary_values9: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index9: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result {
    loop {
        return if (source_ml_temporary_index9 < (self::ml::Big::from_u128(source_ml_temporary_values9.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_temporary_values9, source_ml_temporary_index9.to_index());
                if (((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1805, _, _, _, _) => {
                        source_ml_source1_data_property_value1805.clone()
                    }
                }) <= previous) || ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1807, _, _, _, _) => {
                        source_ml_source1_data_property_value1807.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match object.clone() {
                            self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1808, _, _, _, _) => {
                                source_ml_source1_data_property_value1808.clone()
                            }
                        };
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
            self::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Result::MlTemporaryReadJavaScriptOrdinaryHeapErrorLoop12Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result {
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Done,
    MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool, self::ml::Big)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_locate_java_script_ordinary_property_loop19(mut heap: self::JavaScriptOrdinaryHeap, mut key: self::JavaScriptSemanticValue, mut current: self::ml::Big, mut depth: f64) -> self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result {
    loop {
        return if (depth < (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1810, _) => source_ml_source1_data_property_value1810.as_slice() }).len() as f64))) {
            {
                let index = self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1811, _) => {
                        source_ml_source1_data_property_value1811
                    }
                }, current.clone());
                {
                    let object = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1812, _) => source_ml_source1_data_property_value1812.as_slice() }), self::ml_array::number_index(index));
                    if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Done
                    } else {
                        {
                            let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1813) => {
                                    source_ml_source1_data_property_value1813
                                }
                            }, key.clone());
                            {
                                let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1814) => {
                                        source_ml_source1_data_property_value1814
                                    }
                                }, self::ml_array::number_index(at));
                                if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Return({
                                        let source_ml_source1_field1815 = true;
                                        {
                                            let source_ml_source1_field1816 = current.clone();
                                            {
                                                let source_ml_source1_field1818 = match property.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value1817) => {
                                                        source_ml_source1_data_property_value1817.clone()
                                                    }
                                                };
                                                (source_ml_source1_field1818.clone(), source_ml_source1_field1815, source_ml_source1_field1816.clone())
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let prototype = match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1819, _, _, _) => {
                                                let source_ml_source1_data_property_value1819 = *source_ml_source1_data_property_value1819;
                                                source_ml_source1_data_property_value1819.clone()
                                            }
                                        };
                                        match prototype.clone() {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Done
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Done
                                            }
                                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1820) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Done
                                            }
                                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1821) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Done
                                            }
                                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1822) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Done
                                            }
                                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1823) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Done
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1824) => {
                                                self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Done
                                            }
                                            self::JavaScriptSemanticValue::Object(current_2, source_ml_source1_f1825) => {
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
            self::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Result::MlTemporaryLocateJavaScriptOrdinaryPropertyLoop19Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_define_java_script_ordinary_property_loop21(mut count: f64, mut source_ml_temporary_values20: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index20: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index20 < (self::ml::Big::from_u128(source_ml_temporary_values20.len() as u128))) {
            {
                let entry = self::ml_array::at(&source_ml_temporary_values20, source_ml_temporary_index20.to_index());
                {
                    let count_2 = (count + ((match &entry { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source1_data_property_value1826) => source_ml_source1_data_property_value1826.as_slice() }).len() as f64));
                    {
                        let source_ml_temporary_index20_2 = source_ml_temporary_index20.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_temporary_index20) = (count_2, source_ml_temporary_index20_2.clone());
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
pub enum MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result {
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Done,
    MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Return((self::ml_string::Text, self::JavaScriptOrdinaryHeap, bool)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_set_java_script_ordinary_prototype_loop22(mut heap: self::JavaScriptOrdinaryHeap, mut identity: self::ml::Big, mut next: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result {
    loop {
        return if (depth <= (((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1828, _) => source_ml_source1_data_property_value1828.as_slice() }).len() as f64))) {
            match next.clone() {
                self::JavaScriptSemanticValue::Undefined => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Done
                }
                self::JavaScriptSemanticValue::Null => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Done
                }
                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1829) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Done
                }
                self::JavaScriptSemanticValue::Number(source_ml_source1_f1830) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Done
                }
                self::JavaScriptSemanticValue::Integer(source_ml_source1_f1831) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Done
                }
                self::JavaScriptSemanticValue::Text(source_ml_source1_f1832) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Done
                }
                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1833) => {
                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Done
                }
                self::JavaScriptSemanticValue::Object(next_identity, source_ml_source1_f1834) => {
                    if (next_identity == identity) {
                        self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Return({
                            let source_ml_source1_field1835 = heap.clone();
                            {
                                let source_ml_source1_field1836 = false;
                                {
                                    let source_ml_source1_field1837 = self::ml_string::Text::from_units(vec![]);
                                    (source_ml_source1_field1837.clone(), source_ml_source1_field1835.clone(), source_ml_source1_field1836)
                                }
                            }
                        })
                    } else {
                        {
                            let at = self::find_java_script_ordinary_object(match heap.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1838, _) => {
                                    source_ml_source1_data_property_value1838
                                }
                            }, next_identity.clone());
                            {
                                let parent = self::ml_array::optional_at((match &heap { self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source1_data_property_value1839, _) => source_ml_source1_data_property_value1839.as_slice() }), self::ml_array::number_index(at));
                                if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Return({
                                        let source_ml_source1_field1840 = heap.clone();
                                        {
                                            let source_ml_source1_field1841 = false;
                                            {
                                                let source_ml_source1_field1842 = self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]);
                                                (source_ml_source1_field1842.clone(), source_ml_source1_field1840.clone(), source_ml_source1_field1841)
                                            }
                                        }
                                    })
                                } else {
                                    {
                                        let next_2 = match parent.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value1843, _, _, _) => {
                                                let source_ml_source1_data_property_value1843 = *source_ml_source1_data_property_value1843;
                                                source_ml_source1_data_property_value1843.clone()
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
            self::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Result::MlTemporarySetJavaScriptOrdinaryPrototypeLoop22Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop24Result {
    MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop24Done(Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>, Vec<self::JavaScriptSemanticValue>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop25(positions: &[f64], mut position: f64, mut at: f64) -> f64 {
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
pub fn source_ml_temporary_read_java_script_ordinary_own_keys_loop24(mut numeric: Vec<self::JavaScriptSemanticValue>, mut positions: Vec<f64>, mut text: Vec<self::JavaScriptSemanticValue>, mut symbols: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_values23: Vec<self::JavaScriptOrdinaryProperty>, mut source_ml_temporary_index23: self::ml::Big) -> self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop24Result {
    loop {
        return if (source_ml_temporary_index23 < (self::ml::Big::from_u128(source_ml_temporary_values23.len() as u128))) {
            {
                let property = self::ml_array::at(&source_ml_temporary_values23, source_ml_temporary_index23.to_index());
                {
                    let key = match property.clone() {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value1844, _) => {
                            let source_ml_source1_data_property_value1844 = *source_ml_source1_data_property_value1844;
                            source_ml_source1_data_property_value1844.clone()
                        }
                    };
                    match key.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            {
                                let symbols_2 = {
                                    let source_ml_source1_concat_value1851 = symbols;
                                    {
                                        let source_ml_source1_concat_value1852 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1851), source_ml_source1_concat_value1852)
                                    }
                                };
                                {
                                    let source_ml_temporary_index23_2 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index23) = (symbols_2, source_ml_temporary_index23_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Null => {
                            {
                                let symbols_3 = {
                                    let source_ml_source1_concat_value1853 = symbols;
                                    {
                                        let source_ml_source1_concat_value1854 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1853), source_ml_source1_concat_value1854)
                                    }
                                };
                                {
                                    let source_ml_temporary_index23_3 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index23) = (symbols_3, source_ml_temporary_index23_3.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1845) => {
                            {
                                let symbols_4 = {
                                    let source_ml_source1_concat_value1855 = symbols;
                                    {
                                        let source_ml_source1_concat_value1856 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1855), source_ml_source1_concat_value1856)
                                    }
                                };
                                {
                                    let source_ml_temporary_index23_4 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index23) = (symbols_4, source_ml_temporary_index23_4.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f1846) => {
                            {
                                let symbols_5 = {
                                    let source_ml_source1_concat_value1857 = symbols;
                                    {
                                        let source_ml_source1_concat_value1858 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1857), source_ml_source1_concat_value1858)
                                    }
                                };
                                {
                                    let source_ml_temporary_index23_5 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index23) = (symbols_5, source_ml_temporary_index23_5.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f1847) => {
                            {
                                let symbols_6 = {
                                    let source_ml_source1_concat_value1859 = symbols;
                                    {
                                        let source_ml_source1_concat_value1860 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1859), source_ml_source1_concat_value1860)
                                    }
                                };
                                {
                                    let source_ml_temporary_index23_6 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index23) = (symbols_6, source_ml_temporary_index23_6.clone());
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
                                            let source_ml_source1_concat_value1861 = text;
                                            {
                                                let source_ml_source1_concat_value1862 = vec![key.clone()];
                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1861), source_ml_source1_concat_value1862)
                                            }
                                        };
                                        {
                                            let source_ml_temporary_index23_7 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                                            {
                                                (text, source_ml_temporary_index23) = (text_2, source_ml_temporary_index23_7.clone());
                                                continue;
                                            }
                                        }
                                    }
                                } else {
                                    {
                                        let at = 0f64;
                                        {
                                            let at_2 = self::source_ml_temporary_read_java_script_ordinary_own_keys_loop25(&(positions), position, at);
                                            {
                                                let numeric_2 = {
                                                    let source_ml_source1_concat_value1863 = self::ml_array::slice(&numeric, 0f64, at_2);
                                                    {
                                                        let source_ml_source1_concat_value1864 = vec![key.clone()];
                                                        {
                                                            let source_ml_source1_concat_value1865 = self::ml_array::slice(&numeric, at_2, f64::INFINITY);
                                                            self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1863), source_ml_source1_concat_value1864), source_ml_source1_concat_value1865)
                                                        }
                                                    }
                                                };
                                                {
                                                    let positions_2 = {
                                                        let source_ml_source1_concat_value1866 = self::ml_array::slice(&positions, 0f64, at_2);
                                                        {
                                                            let source_ml_source1_concat_value1867 = vec![position];
                                                            {
                                                                let source_ml_source1_concat_value1868 = self::ml_array::slice(&positions, at_2, f64::INFINITY);
                                                                self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<f64>::new(), source_ml_source1_concat_value1866), source_ml_source1_concat_value1867), source_ml_source1_concat_value1868)
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_temporary_index23_8 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                                                        {
                                                            (numeric, positions, source_ml_temporary_index23) = (numeric_2, positions_2, source_ml_temporary_index23_8.clone());
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
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1848) => {
                            {
                                let symbols_7 = {
                                    let source_ml_source1_concat_value1869 = symbols;
                                    {
                                        let source_ml_source1_concat_value1870 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1869), source_ml_source1_concat_value1870)
                                    }
                                };
                                {
                                    let source_ml_temporary_index23_9 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index23) = (symbols_7, source_ml_temporary_index23_9.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                        self::JavaScriptSemanticValue::Object(source_ml_source1_f1849, source_ml_source1_f1850) => {
                            {
                                let symbols_8 = {
                                    let source_ml_source1_concat_value1871 = symbols;
                                    {
                                        let source_ml_source1_concat_value1872 = vec![key.clone()];
                                        self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptSemanticValue>::new(), source_ml_source1_concat_value1871), source_ml_source1_concat_value1872)
                                    }
                                };
                                {
                                    let source_ml_temporary_index23_10 = source_ml_temporary_index23.add(&self::ml::Big::from_i128(1));
                                    {
                                        (symbols, source_ml_temporary_index23) = (symbols_8, source_ml_temporary_index23_10.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop24Result::MlTemporaryReadJavaScriptOrdinaryOwnKeysLoop24Done(numeric, text, symbols)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_insert_java_script_ordinary_object_loop27(mut identity: self::ml::Big, mut at: f64, mut source_ml_temporary_values26: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_temporary_index26: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index26 < (self::ml::Big::from_u128(source_ml_temporary_values26.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_temporary_values26, source_ml_temporary_index26.to_index());
                if ((match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value1874, _, _, _, _) => {
                        source_ml_source1_data_property_value1874.clone()
                    }
                }) < identity) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_temporary_index26_2 = source_ml_temporary_index26.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index26) = (at_2, source_ml_temporary_index26_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index26_3 = source_ml_temporary_index26.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index26 = source_ml_temporary_index26_3.clone();
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
pub fn source_ml_temporary_register_java_script_ordinary_symbol_loop29(mut identity: self::ml::Big, mut at: f64, mut source_ml_temporary_values28: Vec<self::ml::Big>, mut source_ml_temporary_index28: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_temporary_index28 < (self::ml::Big::from_u128(source_ml_temporary_values28.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_temporary_values28, source_ml_temporary_index28.to_index());
                if (symbol < identity) {
                    {
                        let at_2 = (at + 1f64);
                        {
                            let source_ml_temporary_index28_2 = source_ml_temporary_index28.add(&self::ml::Big::from_i128(1));
                            {
                                (at, source_ml_temporary_index28) = (at_2, source_ml_temporary_index28_2.clone());
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let source_ml_temporary_index28_3 = source_ml_temporary_index28.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index28 = source_ml_temporary_index28_3.clone();
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
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value1875, _) => {
                    source_ml_source1_data_property_value1875.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_step_java_script_descriptor_conversion_loop31(mut heap: self::JavaScriptOrdinaryHeap, mut current: self::JavaScriptDescriptorConversionState) -> self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result {
    loop {
        return if ((match current.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1877, _, _) => {
                source_ml_source1_data_property_value1877
            }
        }) < (6f64)) {
            {
                let field = self::read_java_script_descriptor_field(match current.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1878, _, _) => {
                        source_ml_source1_data_property_value1878
                    }
                });
                {
                    let key = self::JavaScriptSemanticValue::Text((field.clone()).2);
                    {
                        let receiver = match current.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value1879, _, _, _) => {
                                let source_ml_source1_data_property_value1879 = *source_ml_source1_data_property_value1879;
                                source_ml_source1_data_property_value1879.clone()
                            }
                        };
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f1880) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f1881) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f1882) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f1883) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f1884) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f1885) => {
                                {
                                    let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                    if !((property.clone()).1) {
                                        {
                                            let current_2 = self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), (match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1886, _, _) => {
                                                    source_ml_source1_data_property_value1886
                                                }
                                            } + 1f64), match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value1887, _) => {
                                                    source_ml_source1_data_property_value1887.clone()
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
                                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid((plan.clone()).3)))
                                            } else {
                                                if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorCall(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value1888, _, _) => {
                                                            source_ml_source1_data_property_value1888
                                                        }
                                                    }, match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value1889, _) => {
                                                            source_ml_source1_data_property_value1889.clone()
                                                        }
                                                    }, true)), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)))
                                                } else {
                                                    if (((plan.clone()).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                        self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))))
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
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source1_f1890, source_ml_source1_f1891, source_ml_source1_f1892, source_ml_source1_f1893) => {
                                                                    let source_ml_source1_f1890 = *source_ml_source1_f1890;
                                                                    let source_ml_source1_f1891 = *source_ml_source1_f1891;
                                                                    let source_ml_source1_f1892 = *source_ml_source1_f1892;
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source1_f1894) => {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source1_f1895) => {
                                                                    let source_ml_source1_f1895 = *source_ml_source1_f1895;
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source1_f1896, source_ml_source1_f1897) => {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source1_f1898) => {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Return(Box::new(applied.clone()))
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
            self::MlTemporaryStepJavaScriptDescriptorConversionLoop31Result::MlTemporaryStepJavaScriptDescriptorConversionLoop31Done(Box::new(current.clone()))
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_finish_java_script_property_key_conversion_loop32(spelling: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (spelling.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source1_string_unit_operand1899 = spelling.clone();
                    {
                        let source_ml_source1_string_unit_operand1900 = index;
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
                                    let source_ml_source1_string_unit_operand1906 = {
                                        let source_ml_source1_string_range_operand1901 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand1902 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand1903 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position1904 = source_ml_source1_string_range_operand1902;
                                                    {
                                                        let source_ml_source1_string_range_position1905 = source_ml_source1_string_range_operand1903;
                                                        source_ml_source1_string_range_operand1901.slice(source_ml_source1_string_range_position1904, source_ml_source1_string_range_position1905)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand1907 = 0f64;
                                        source_ml_source1_string_unit_operand1906.code_point_at(source_ml_source1_string_unit_operand1907)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand1899.clone(), source_ml_source1_string_unit_operand1900)
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
pub enum MlTemporaryReadJavaScriptCallProgramErrorLoop39Result {
    MlTemporaryReadJavaScriptCallProgramErrorLoop39Done,
    MlTemporaryReadJavaScriptCallProgramErrorLoop39Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallProgramErrorLoop41Result {
    MlTemporaryReadJavaScriptCallProgramErrorLoop41Done,
    MlTemporaryReadJavaScriptCallProgramErrorLoop41Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallProgramErrorLoop42Result {
    MlTemporaryReadJavaScriptCallProgramErrorLoop42Done,
    MlTemporaryReadJavaScriptCallProgramErrorLoop42Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_program_error_loop42(mut fn_: self::JavaScriptCallFunction, source_ml_temporary_values37: &[f64], mut source_ml_temporary_index37: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallProgramErrorLoop42Result {
    loop {
        return if (source_ml_temporary_index37 < (self::ml::Big::from_u128(source_ml_temporary_values37.len() as u128))) {
            {
                let register = self::ml_array::at(source_ml_temporary_values37, source_ml_temporary_index37.to_index());
                if !(self::test_java_script_call_coordinate(register, match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value1908, _, _) => {
                        source_ml_source1_data_property_value1908
                    }
                })) {
                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop42Result::MlTemporaryReadJavaScriptCallProgramErrorLoop42Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_temporary_index37_2 = source_ml_temporary_index37.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index37 = source_ml_temporary_index37_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallProgramErrorLoop42Result::MlTemporaryReadJavaScriptCallProgramErrorLoop42Done
        };
    }
}
