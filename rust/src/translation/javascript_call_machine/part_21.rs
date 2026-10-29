// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Result;
use super::accept_java_script_whitespace_unit;
use super::apply_java_script_descriptor_field;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::locate_java_script_ordinary_property;
use super::match_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_descriptor_field;
use super::read_java_script_semantic_value_error;
use super::test_java_script_call_coordinate;
use super::test_java_script_descriptor_field_presence;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop75(mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut source_ml_source1_temporary_values72: Vec<self::ml::Big>, mut source_ml_source1_temporary_index72: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Result {
    loop {
        return if (source_ml_source1_temporary_index72 < (self::ml::Big::from_u128(source_ml_source1_temporary_values72.len() as u128))) {
            {
                let symbol = self::ml_array::at(&source_ml_source1_temporary_values72, source_ml_source1_temporary_index72.to_index());
                if (((symbol <= previous) || (symbol > (self::ml::Big::from_i128(18446744073709551615)))) || ((self::find_java_script_ordinary_object(match heap.clone() {
                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value2630, _) => {
                        source_ml_source2_data_property_value2630
                    }
                }, symbol.clone())) >= (0f64))) {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Return(self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 115, 32, 97, 110, 100, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index72_2 = source_ml_source1_temporary_index72.add(&self::ml::Big::from_i128(1));
                        {
                            (previous, source_ml_source1_temporary_index72) = (symbol.clone(), source_ml_source1_temporary_index72_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop75Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop74(mut previous: self::ml::Big, mut source_ml_source1_temporary_values71: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index71: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Result {
    loop {
        return if (source_ml_source1_temporary_index71 < (self::ml::Big::from_u128(source_ml_source1_temporary_values71.len() as u128))) {
            {
                let object = self::ml_array::at(&source_ml_source1_temporary_values71, source_ml_source1_temporary_index71.to_index());
                match object.clone() {
                    self::JavaScriptOrdinaryObject::OrdinaryObject(shared_constructor_field1, _, _, _, _) => {
                        if ((shared_constructor_field1 <= previous) || (shared_constructor_field1 > (self::ml::Big::from_i128(18446744073709551615)))) {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 79, 98, 106, 101, 99, 116, 115, 32, 110, 101, 101, 100, 32, 115, 116, 114, 105, 99, 116, 108, 121, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                        } else {
                            {
                                let previous_2 = shared_constructor_field1.clone();
                                {
                                    let source_ml_source1_temporary_index71_2 = source_ml_source1_temporary_index71.add(&self::ml::Big::from_i128(1));
                                    {
                                        (previous, source_ml_source1_temporary_index71) = (previous_2.clone(), source_ml_source1_temporary_index71_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop74Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result {
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Done,
    MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Return(((bool, bool, bool, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, self::JavaScriptSemanticValue, bool, bool, bool), bool, self::ml::Big)),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_locate_java_script_ordinary_property_loop81(mut heap: self::JavaScriptOrdinaryHeap, mut key: self::JavaScriptSemanticValue, mut current: self::ml::Big, mut depth: f64) -> self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _) => {
                if (depth < ((shared_constructor_field1.len() as f64))) {
                    {
                        let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), current.clone());
                        {
                            let object = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(index));
                            if match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Done
                            } else {
                                {
                                    let at = self::find_java_script_ordinary_property(match object.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2640) => {
                                            source_ml_source2_data_property_value2640
                                        }
                                    }, key.clone());
                                    {
                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2641) => {
                                                source_ml_source2_data_property_value2641
                                            }
                                        }, self::ml_array::number_index(at));
                                        if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                            self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Return({
                                                let source_ml_source2_field2645 = match property.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2644) => {
                                                        source_ml_source2_data_property_value2644.clone()
                                                    }
                                                };
                                                (source_ml_source2_field2645.clone(), true, current.clone())
                                            })
                                        } else {
                                            {
                                                let prototype = match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2646, _, _, _) => {
                                                        let source_ml_source2_data_property_value2646 = *source_ml_source2_data_property_value2646;
                                                        source_ml_source2_data_property_value2646.clone()
                                                    }
                                                };
                                                match prototype.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Done
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Done
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Done
                                                    }
                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Done
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Done
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Done
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Done
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
                    self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Done
                }
            }
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_define_java_script_ordinary_property_loop83(mut count: f64, mut source_ml_source1_temporary_values82: Vec<self::JavaScriptOrdinaryObject>, mut source_ml_source1_temporary_index82: self::ml::Big) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index82 < (self::ml::Big::from_u128(source_ml_source1_temporary_values82.len() as u128))) {
            {
                let entry = self::ml_array::at(&source_ml_source1_temporary_values82, source_ml_source1_temporary_index82.to_index());
                {
                    let count_2 = (count + ((match &entry { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2653) => source_ml_source2_data_property_value2653.as_slice() }).len() as f64));
                    {
                        let source_ml_source1_temporary_index82_2 = source_ml_source1_temporary_index82.add(&self::ml::Big::from_i128(1));
                        {
                            (count, source_ml_source1_temporary_index82) = (count_2, source_ml_source1_temporary_index82_2.clone());
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
pub fn source_ml_source1_temporary_slice_utf16_text_loop92(text: &[f64], mut last: f64, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let at_2 = (at + 1f64);
                    {
                        (result, at) = (result_2, at_2);
                        continue;
                    }
                }
            }
        } else {
            result
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryMatchUtf16TextLoop96Result {
    MlSource1TemporaryMatchUtf16TextLoop96Done,
    MlSource1TemporaryMatchUtf16TextLoop96Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_match_utf16_text_loop96(text: &[f64], search: &[f64], mut start: f64, mut at: f64) -> self::MlSource1TemporaryMatchUtf16TextLoop96Result {
    loop {
        return if (at < ((search.len() as f64))) {
            if {
                let source_ml_source2_comparison_operand2702 = self::ml_array::optional_at(text, self::ml_array::number_index((start + at)));
                {
                    let source_ml_source2_comparison_operand2703 = self::ml_array::optional_at(search, self::ml_array::number_index(at));
                    !(match (source_ml_source2_comparison_operand2702.clone(), source_ml_source2_comparison_operand2703.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlSource1TemporaryMatchUtf16TextLoop96Result::MlSource1TemporaryMatchUtf16TextLoop96Return(false)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryMatchUtf16TextLoop96Result::MlSource1TemporaryMatchUtf16TextLoop96Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryIndexUtf16TextLoop97Result {
    MlSource1TemporaryIndexUtf16TextLoop97Done,
    MlSource1TemporaryIndexUtf16TextLoop97Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_index_utf16_text_loop97(text: &[f64], search: &[f64], mut at: f64) -> self::MlSource1TemporaryIndexUtf16TextLoop97Result {
    loop {
        return if (at <= (((text.len() as f64) - (search.len() as f64)))) {
            if self::match_utf16_text(text.to_vec(), search.to_vec(), at) {
                self::MlSource1TemporaryIndexUtf16TextLoop97Result::MlSource1TemporaryIndexUtf16TextLoop97Return(at)
            } else {
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryIndexUtf16TextLoop97Result::MlSource1TemporaryIndexUtf16TextLoop97Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLastIndexUtf16TextLoop98Result {
    MlSource1TemporaryLastIndexUtf16TextLoop98Done,
    MlSource1TemporaryLastIndexUtf16TextLoop98Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_last_index_utf16_text_loop98(text: &[f64], search: &[f64], mut at: f64) -> self::MlSource1TemporaryLastIndexUtf16TextLoop98Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text.to_vec(), search.to_vec(), at) {
                self::MlSource1TemporaryLastIndexUtf16TextLoop98Result::MlSource1TemporaryLastIndexUtf16TextLoop98Return(at)
            } else {
                {
                    let at_2 = (at - 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLastIndexUtf16TextLoop98Result::MlSource1TemporaryLastIndexUtf16TextLoop98Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareUtf16TextLoop99Result {
    MlSource1TemporaryCompareUtf16TextLoop99Done,
    MlSource1TemporaryCompareUtf16TextLoop99Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compare_utf16_text_loop99(left: &[f64], right: &[f64], mut count: f64, mut at: f64) -> self::MlSource1TemporaryCompareUtf16TextLoop99Result {
    loop {
        return if (at < count) {
            {
                let a = (self::ml_array::optional_at(left, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(right, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlSource1TemporaryCompareUtf16TextLoop99Result::MlSource1TemporaryCompareUtf16TextLoop99Return(if (a < b) {
                            (-1f64)
                        } else {
                            1f64
                        })
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
            self::MlSource1TemporaryCompareUtf16TextLoop99Result::MlSource1TemporaryCompareUtf16TextLoop99Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_trim_utf16_text_loop103(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((first < last) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index(first))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let first_2 = (first + 1f64);
                {
                    first = first_2;
                    continue;
                }
            }
        } else {
            first
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_trim_utf16_text_loop104(text: &[f64], mut first: f64, mut last: f64) -> f64 {
    loop {
        return if ((last > first) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(text, self::ml_array::number_index((last - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let last_2 = (last - 1f64);
                {
                    last = last_2;
                    continue;
                }
            }
        } else {
            last
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_character_units_utf16_text_loop105(values: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((values.len() as f64))) {
            {
                let value = (self::ml_array::optional_at(values, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let integer = if value.is_finite() {
                        value.trunc()
                    } else {
                        0f64
                    };
                    {
                        let residue = (integer % 65536f64);
                        {
                            let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (residue < (0f64)) {
                                (residue + 65536f64)
                            } else {
                                if (residue == (0f64)) {
                                    0f64
                                } else {
                                    residue
                                }
                            }], result); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_2 = (at + 1f64);
                                {
                                    (result, at) = (result_2, at_2);
                                    continue;
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result {
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Done(Vec<f64>),
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop117(mut numbers: Vec<f64>, mut source_ml_source1_temporary_values116: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index116: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result {
    loop {
        return if (source_ml_source1_temporary_index116 < (self::ml::Big::from_u128(source_ml_source1_temporary_values116.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values116, source_ml_source1_temporary_index116.to_index());
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        {
                            let numbers_2 = { let (ml_append_right, ml_append_left) = (vec![value_value], numbers); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index116_2 = source_ml_source1_temporary_index116.add(&self::ml::Big::from_i128(1));
                                {
                                    (numbers, source_ml_source1_temporary_index116) = (numbers_2, source_ml_source1_temporary_index116_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop117Done(numbers)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop119Result {
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop119Done,
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop119Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_primitive_loop119(mut source_ml_source1_temporary_values118: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index118: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop119Result {
    loop {
        return if (source_ml_source1_temporary_index118 < (self::ml::Big::from_u128(source_ml_source1_temporary_values118.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values118, source_ml_source1_temporary_index118.to_index());
                {
                    let error = self::read_java_script_semantic_value_error(value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index118_2 = source_ml_source1_temporary_index118.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index118 = source_ml_source1_temporary_index118_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop119Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_text_conversion_loop121(text: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (text.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source2_string_unit_operand2711 = text.clone();
                    {
                        let source_ml_source2_string_unit_operand2712 = index;
                        (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 { {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text_2.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand2718 = {
                                        let source_ml_source2_string_range_operand2713 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand2714 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand2715 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position2716 = source_ml_source2_string_range_operand2714;
                                                    {
                                                        let source_ml_source2_string_range_position2717 = source_ml_source2_string_range_operand2715;
                                                        source_ml_source2_string_range_operand2713.slice(source_ml_source2_string_range_position2716, source_ml_source2_string_range_position2717)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand2719 = 0f64;
                                        source_ml_source2_string_unit_operand2718.code_point_at(source_ml_source2_string_unit_operand2719)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand2711.clone(), source_ml_source2_string_unit_operand2712)
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin120 {
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin120Next(self::ml_string::Text),
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin120Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop122Result {
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop122Done,
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop122Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_code_function_loop122(mut program: self::JavaScriptCallProgram, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop122Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value2720, _, _) => source_ml_source2_data_property_value2720.as_slice() }), self::ml_array::number_index(middle));
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop122Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop122Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2722, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value2722.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop122Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop122Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2724, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value2724.clone()
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
            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop122Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop122Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop123Result {
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop123Done,
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop123Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop123(mut state: self::JavaScriptDescriptorConversionState, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop123Result {
    loop {
        return if (index < (6f64)) {
            if self::test_java_script_descriptor_field_presence(match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2725, _) => {
                    source_ml_source2_data_property_value2725.clone()
                }
            }, index) {
                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop123Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop123Return(self::ml_string::Text::from_units(vec![117, 110, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 102, 97, 98, 114, 105, 99, 97, 116, 101, 100, 32, 105, 110, 32, 97, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
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
            self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop123Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop123Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result {
    MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Done(Box<self::JavaScriptDescriptorConversionState>),
    MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box<self::JavaScriptDescriptorConversionTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_descriptor_conversion_loop124(mut heap: self::JavaScriptOrdinaryHeap, mut current: self::JavaScriptDescriptorConversionState) -> self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result {
    loop {
        return if ((match current.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2727, _, _) => {
                source_ml_source2_data_property_value2727
            }
        }) < (6f64)) {
            {
                let field = self::read_java_script_descriptor_field(match current.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2728, _, _) => {
                        source_ml_source2_data_property_value2728
                    }
                });
                {
                    let key = self::JavaScriptSemanticValue::Text((field.clone()).2);
                    {
                        let receiver = match current.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value2729, _, _, _) => {
                                let source_ml_source2_data_property_value2729 = *source_ml_source2_data_property_value2729;
                                source_ml_source2_data_property_value2729.clone()
                            }
                        };
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                {
                                    let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                    if !((property.clone()).1) {
                                        {
                                            let current_2 = self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), (match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2736, _, _) => {
                                                    source_ml_source2_data_property_value2736
                                                }
                                            } + 1f64), match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2737, _) => {
                                                    source_ml_source2_data_property_value2737.clone()
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
                                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid((plan.clone()).3)))
                                            } else {
                                                if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorCall(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2738, _, _) => {
                                                            source_ml_source2_data_property_value2738
                                                        }
                                                    }, match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2739, _) => {
                                                            source_ml_source2_data_property_value2739.clone()
                                                        }
                                                    }, true)), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)))
                                                } else {
                                                    if (((plan.clone()).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                        self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))))
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
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCall(_, _, _, _) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(_) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(_) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(_, _) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(_) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Return(Box::new(applied.clone()))
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
            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop124Done(Box::new(current.clone()))
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_finish_java_script_property_key_conversion_loop125(spelling: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (spelling.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source2_string_unit_operand2749 = spelling.clone();
                    {
                        let source_ml_source2_string_unit_operand2750 = index;
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
                                    let source_ml_source2_string_unit_operand2756 = {
                                        let source_ml_source2_string_range_operand2751 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand2752 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand2753 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position2754 = source_ml_source2_string_range_operand2752;
                                                    {
                                                        let source_ml_source2_string_range_position2755 = source_ml_source2_string_range_operand2753;
                                                        source_ml_source2_string_range_operand2751.slice(source_ml_source2_string_range_position2754, source_ml_source2_string_range_position2755)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand2757 = 0f64;
                                        source_ml_source2_string_unit_operand2756.code_point_at(source_ml_source2_string_unit_operand2757)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand2749.clone(), source_ml_source2_string_unit_operand2750)
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
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop135(mut fn_: self::JavaScriptCallFunction, source_ml_source1_temporary_values130: &[f64], mut source_ml_source1_temporary_index130: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result {
    loop {
        return if (source_ml_source1_temporary_index130 < (self::ml::Big::from_u128(source_ml_source1_temporary_values130.len() as u128))) {
            {
                let register = self::ml_array::at(source_ml_source1_temporary_values130, source_ml_source1_temporary_index130.to_index());
                if !(self::test_java_script_call_coordinate(register, match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2758, _, _) => {
                        source_ml_source2_data_property_value2758
                    }
                })) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index130_2 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Done
        };
    }
}
