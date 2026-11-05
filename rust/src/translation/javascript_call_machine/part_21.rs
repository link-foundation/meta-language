// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result;
use super::accept_java_script_whitespace_unit;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::index_utf16_text;
use super::match_utf16_text;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_semantic_value_error;

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
                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2693) => {
                                            source_ml_source2_data_property_value2693
                                        }
                                    }, key.clone());
                                    {
                                        let property = self::ml_array::optional_at(&match object.clone().expect("a proven present value") {
                                            self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2694) => {
                                                source_ml_source2_data_property_value2694
                                            }
                                        }, self::ml_array::number_index(at));
                                        if !(match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                            self::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Result::MlSource1TemporaryLocateJavaScriptOrdinaryPropertyLoop81Return({
                                                let source_ml_source2_field2698 = match property.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value2697) => {
                                                        source_ml_source2_data_property_value2697.clone()
                                                    }
                                                };
                                                (source_ml_source2_field2698.clone(), true, current.clone())
                                            })
                                        } else {
                                            {
                                                let prototype = match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value2699, _, _, _) => {
                                                        let source_ml_source2_data_property_value2699 = *source_ml_source2_data_property_value2699;
                                                        source_ml_source2_data_property_value2699.clone()
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
                    let count_2 = (count + ((match &entry { self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, source_ml_source2_data_property_value2706) => source_ml_source2_data_property_value2706.as_slice() }).len() as f64));
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
pub fn source_ml_source1_temporary_construct_utf16_text_repetition_loop92(text: &[f64], mut length: f64, mut result: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < length) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(text, self::ml_array::number_index((index % (text.len() as f64))))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (result, index) = (result_2, index_2);
                        continue;
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
pub fn source_ml_source1_temporary_construct_utf16_text_padding_loop93(text: &[f64], filler: &[f64], mut length: f64, mut start: bool, mut added: f64, mut result: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < length) {
            {
                let original = if start {
                    (index >= added)
                } else {
                    (index < ((text.len() as f64)))
                };
                {
                    let position = if original {
                        if start {
                            (index - added)
                        } else {
                            index
                        }
                    } else {
                        if start {
                            index
                        } else {
                            (index - (text.len() as f64))
                        }
                    };
                    {
                        let result_2 = { let (ml_append_right, ml_append_left) = (vec![(if original {
                            self::ml_array::optional_at(text, self::ml_array::number_index(position))
                        } else {
                            self::ml_array::optional_at(filler, self::ml_array::number_index((position % (filler.len() as f64))))
                        }).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (result, index) = (result_2, index_2);
                                continue;
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
pub fn source_ml_source1_temporary_slice_utf16_text_loop94(text: &[f64], mut last: f64, mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
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
pub enum MlSource1TemporaryMatchUtf16TextLoop98Result {
    MlSource1TemporaryMatchUtf16TextLoop98Done,
    MlSource1TemporaryMatchUtf16TextLoop98Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_match_utf16_text_loop98(text: &[f64], search: &[f64], mut start: f64, mut at: f64) -> self::MlSource1TemporaryMatchUtf16TextLoop98Result {
    loop {
        return if (at < ((search.len() as f64))) {
            if {
                let source_ml_source2_comparison_operand2755 = self::ml_array::optional_at(text, self::ml_array::number_index((start + at)));
                {
                    let source_ml_source2_comparison_operand2756 = self::ml_array::optional_at(search, self::ml_array::number_index(at));
                    !(match (source_ml_source2_comparison_operand2755.clone(), source_ml_source2_comparison_operand2756.clone()) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false })
                }
            } {
                self::MlSource1TemporaryMatchUtf16TextLoop98Result::MlSource1TemporaryMatchUtf16TextLoop98Return(false)
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
            self::MlSource1TemporaryMatchUtf16TextLoop98Result::MlSource1TemporaryMatchUtf16TextLoop98Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryIndexUtf16TextLoop99Result {
    MlSource1TemporaryIndexUtf16TextLoop99Done,
    MlSource1TemporaryIndexUtf16TextLoop99Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_index_utf16_text_loop99(text: &[f64], search: &[f64], mut at: f64) -> self::MlSource1TemporaryIndexUtf16TextLoop99Result {
    loop {
        return if (at <= (((text.len() as f64) - (search.len() as f64)))) {
            if self::match_utf16_text(text.to_vec(), search.to_vec(), at) {
                self::MlSource1TemporaryIndexUtf16TextLoop99Result::MlSource1TemporaryIndexUtf16TextLoop99Return(at)
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
            self::MlSource1TemporaryIndexUtf16TextLoop99Result::MlSource1TemporaryIndexUtf16TextLoop99Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLastIndexUtf16TextLoop100Result {
    MlSource1TemporaryLastIndexUtf16TextLoop100Done,
    MlSource1TemporaryLastIndexUtf16TextLoop100Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_last_index_utf16_text_loop100(text: &[f64], search: &[f64], mut at: f64) -> self::MlSource1TemporaryLastIndexUtf16TextLoop100Result {
    loop {
        return if (at >= (0f64)) {
            if self::match_utf16_text(text.to_vec(), search.to_vec(), at) {
                self::MlSource1TemporaryLastIndexUtf16TextLoop100Result::MlSource1TemporaryLastIndexUtf16TextLoop100Return(at)
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
            self::MlSource1TemporaryLastIndexUtf16TextLoop100Result::MlSource1TemporaryLastIndexUtf16TextLoop100Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompareUtf16TextLoop101Result {
    MlSource1TemporaryCompareUtf16TextLoop101Done,
    MlSource1TemporaryCompareUtf16TextLoop101Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compare_utf16_text_loop101(left: &[f64], right: &[f64], mut count: f64, mut at: f64) -> self::MlSource1TemporaryCompareUtf16TextLoop101Result {
    loop {
        return if (at < count) {
            {
                let a = (self::ml_array::optional_at(left, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                {
                    let b = (self::ml_array::optional_at(right, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64);
                    if (a != b) {
                        self::MlSource1TemporaryCompareUtf16TextLoop101Result::MlSource1TemporaryCompareUtf16TextLoop101Return(if (a < b) {
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
            self::MlSource1TemporaryCompareUtf16TextLoop101Result::MlSource1TemporaryCompareUtf16TextLoop101Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_trim_utf16_text_loop105(text: &[f64], mut first: f64, mut last: f64) -> f64 {
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
pub fn source_ml_source1_temporary_trim_utf16_text_loop106(text: &[f64], mut first: f64, mut last: f64) -> f64 {
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
pub fn source_ml_source1_temporary_character_units_utf16_text_loop107(values: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
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
pub enum MlSource1TemporaryPlanUtf16TextSubstitutionLoop112Result {
    MlSource1TemporaryPlanUtf16TextSubstitutionLoop112Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_utf16_text_substitution_loop112(replacement: &[f64], mut result: Vec<f64>, mut start: f64, mut at: f64) -> self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop112Result {
    loop {
        return if (at < ((replacement.len() as f64))) {
            {
                let next = (self::ml_array::optional_at(replacement, self::ml_array::number_index((at + 1f64)))).clone().unwrap_or_else(|| (-1f64));
                if ((((self::ml_array::optional_at(replacement, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)) == (36f64)) && ((((next == (36f64)) || (next == (38f64))) || (next == (96f64))) || (next == (39f64)))) {
                    {
                        let result_3 = if (start < at) {
                            {
                                let result_2 = { let (ml_append_right, ml_append_left) = (vec![0f64, start, at], result); self::ml_array::append(ml_append_left, ml_append_right) };
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let result_6 = if (next == (36f64)) {
                                {
                                    let result_4 = { let (ml_append_right, ml_append_left) = (vec![0f64, at, (at + 1f64)], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                    result_4
                                }
                            } else {
                                {
                                    let result_5 = { let (ml_append_right, ml_append_left) = (vec![if (next == (38f64)) {
                                        1f64
                                    } else {
                                        if (next == (96f64)) {
                                            2f64
                                        } else {
                                            3f64
                                        }
                                    }, 0f64, 0f64], result_3); self::ml_array::append(ml_append_left, ml_append_right) };
                                    result_5
                                }
                            };
                            {
                                let at_2 = (at + 2f64);
                                {
                                    (result, start, at) = (result_6, at_2, at_2);
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
        } else {
            self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop112Result::MlSource1TemporaryPlanUtf16TextSubstitutionLoop112Done(result, start)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_replace_utf16_text_loop114(text: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((text.len() as f64))) {
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

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReplaceUtf16TextLoop113Result {
    MlSource1TemporaryReplaceUtf16TextLoop113Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_replace_utf16_text_loop115(text: &[f64], mut result: Vec<f64>, mut found: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < found) {
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_replace_utf16_text_loop117(mut result: Vec<f64>, mut last: f64, source: &[f64], mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![(self::ml_array::optional_at(source, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)], result); self::ml_array::append(ml_append_left, ml_append_right) };
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_replace_utf16_text_loop116(text: &[f64], search: &[f64], replacement: &[f64], mut result: Vec<f64>, plan: &[f64], mut found: f64, mut instruction: f64) -> Vec<f64> {
    loop {
        return if (instruction < ((plan.len() as f64))) {
            {
                let kind = (self::ml_array::optional_at(plan, self::ml_array::number_index(instruction))).clone().unwrap_or_else(|| 0f64);
                {
                    let first = if (kind == (0f64)) {
                        (self::ml_array::optional_at(plan, self::ml_array::number_index((instruction + 1f64)))).clone().unwrap_or_else(|| 0f64)
                    } else {
                        if (kind == (2f64)) {
                            0f64
                        } else {
                            if (kind == (1f64)) {
                                found
                            } else {
                                (found + (search.len() as f64))
                            }
                        }
                    };
                    {
                        let last = if (kind == (0f64)) {
                            (self::ml_array::optional_at(plan, self::ml_array::number_index((instruction + 2f64)))).clone().unwrap_or_else(|| 0f64)
                        } else {
                            if (kind == (2f64)) {
                                found
                            } else {
                                if (kind == (1f64)) {
                                    (found + (search.len() as f64))
                                } else {
                                    (text.len() as f64)
                                }
                            }
                        };
                        {
                            let source = if (kind == (0f64)) {
                                replacement.to_vec()
                            } else {
                                text.to_vec()
                            };
                            {
                                let result_2 = self::source_ml_source1_temporary_replace_utf16_text_loop117(result, last, &(source), first);
                                {
                                    let instruction_2 = (instruction + 3f64);
                                    {
                                        (result, instruction) = (result_2, instruction_2);
                                        continue;
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
pub fn source_ml_source1_temporary_replace_utf16_text_loop113(text: &[f64], search: &[f64], replacement: &[f64], mut all: bool, mut result: Vec<f64>, plan: &[f64], mut start: f64, mut from: f64, mut searching: bool) -> self::MlSource1TemporaryReplaceUtf16TextLoop113Result {
    loop {
        return if searching {
            {
                let found = self::index_utf16_text(text.to_vec(), search.to_vec(), from);
                if (found < (0f64)) {
                    {
                        searching = false;
                        continue;
                    }
                } else {
                    {
                        let at = start;
                        {
                            let result_2 = self::source_ml_source1_temporary_replace_utf16_text_loop115(text, result, found, at);
                            {
                                let instruction = 0f64;
                                {
                                    let result_3 = self::source_ml_source1_temporary_replace_utf16_text_loop116(text, search, replacement, result_2, plan, found, instruction);
                                    {
                                        let start_2 = (found + (search.len() as f64));
                                        {
                                            let searching_2 = (all && !(((((search.len() as f64)) == (0f64)) && (found == ((text.len() as f64))))));
                                            {
                                                let from_2 = (start_2 + if (((search.len() as f64)) == (0f64)) {
                                                    1f64
                                                } else {
                                                    0f64
                                                });
                                                {
                                                    (result, start, from, searching) = (result_3, start_2, from_2, searching_2);
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
        } else {
            self::MlSource1TemporaryReplaceUtf16TextLoop113Result::MlSource1TemporaryReplaceUtf16TextLoop113Done(result, start)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result {
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Done(Vec<f64>),
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop119(mut numbers: Vec<f64>, mut source_ml_source1_temporary_values118: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index118: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result {
    loop {
        return if (source_ml_source1_temporary_index118 < (self::ml::Big::from_u128(source_ml_source1_temporary_values118.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values118, source_ml_source1_temporary_index118.to_index());
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        {
                            let numbers_2 = { let (ml_append_right, ml_append_left) = (vec![value_value], numbers); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index118_2 = source_ml_source1_temporary_index118.add(&self::ml::Big::from_i128(1));
                                {
                                    (numbers, source_ml_source1_temporary_index118) = (numbers_2, source_ml_source1_temporary_index118_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop119Done(numbers)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_text_replacement_work_error_loop120(mut length: f64, mut search: f64, plan: &[f64], mut width: f64, mut index: f64) -> f64 {
    loop {
        return if (index < ((plan.len() as f64))) {
            {
                let kind = (self::ml_array::optional_at(plan, self::ml_array::number_index(index))).clone().unwrap_or_else(|| 0f64);
                {
                    let width_2 = (width + if (kind == (0f64)) {
                        ((self::ml_array::optional_at(plan, self::ml_array::number_index((index + 2f64)))).clone().unwrap_or_else(|| 0f64) - (self::ml_array::optional_at(plan, self::ml_array::number_index((index + 1f64)))).clone().unwrap_or_else(|| 0f64))
                    } else {
                        if (kind == (1f64)) {
                            search
                        } else {
                            length
                        }
                    });
                    {
                        let index_2 = (index + 3f64);
                        {
                            (width, index) = (width_2, index_2);
                            continue;
                        }
                    }
                }
            }
        } else {
            width
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Result {
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Done,
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_primitive_loop122(mut source_ml_source1_temporary_values121: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index121: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Result {
    loop {
        return if (source_ml_source1_temporary_index121 < (self::ml::Big::from_u128(source_ml_source1_temporary_values121.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values121, source_ml_source1_temporary_index121.to_index());
                {
                    let error = self::read_java_script_semantic_value_error(value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index121_2 = source_ml_source1_temporary_index121.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index121 = source_ml_source1_temporary_index121_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop122Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_text_conversion_loop124(text: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (text.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source2_string_unit_operand2764 = text.clone();
                    {
                        let source_ml_source2_string_unit_operand2765 = index;
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
                                    let source_ml_source2_string_unit_operand2771 = {
                                        let source_ml_source2_string_range_operand2766 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand2767 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand2768 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position2769 = source_ml_source2_string_range_operand2767;
                                                    {
                                                        let source_ml_source2_string_range_position2770 = source_ml_source2_string_range_operand2768;
                                                        source_ml_source2_string_range_operand2766.slice(source_ml_source2_string_range_position2769, source_ml_source2_string_range_position2770)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand2772 = 0f64;
                                        source_ml_source2_string_unit_operand2771.code_point_at(source_ml_source2_string_unit_operand2772)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand2764.clone(), source_ml_source2_string_unit_operand2765)
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
pub enum MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123 {
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Next(self::ml_string::Text),
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin123Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Result {
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Done,
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_code_function_loop125(mut program: self::JavaScriptCallProgram, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value2773, _, _) => source_ml_source2_data_property_value2773.as_slice() }), self::ml_array::number_index(middle));
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2775, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value2775.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2777, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value2777.clone()
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
            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop125Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop126Result {
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop126Done,
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop126Return(self::ml_string::Text),
}
