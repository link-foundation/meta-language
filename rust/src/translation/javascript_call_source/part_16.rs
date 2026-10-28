// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::JavaScriptSemanticValue;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::index_utf16_text;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_source_exception;
use super::read_java_script_semantic_value_error;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_display_utf8_from_utf16_text_loop83(mut bytes: Vec<f64>, points: &[f64], mut at: f64) -> Vec<f64> {
    loop {
        return if (at < ((points.len() as f64))) {
            {
                let point = (self::ml_array::optional_at(points, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 65533f64);
                if (point < (128f64)) {
                    {
                        let bytes_2 = { let (ml_append_right, ml_append_left) = (vec![point], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                        {
                            let at_2 = (at + 1f64);
                            {
                                (bytes, at) = (bytes_2, at_2);
                                continue;
                            }
                        }
                    }
                } else {
                    if (point < (2048f64)) {
                        {
                            let bytes_3 = { let (ml_append_right, ml_append_left) = (vec![(192f64 + ((point / 64f64)).floor()), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let at_3 = (at + 1f64);
                                {
                                    (bytes, at) = (bytes_3, at_3);
                                    continue;
                                }
                            }
                        }
                    } else {
                        if (point < (65536f64)) {
                            {
                                let bytes_4 = { let (ml_append_right, ml_append_left) = (vec![(224f64 + ((point / 4096f64)).floor()), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_4 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_4, at_4);
                                        continue;
                                    }
                                }
                            }
                        } else {
                            {
                                let bytes_5 = { let (ml_append_right, ml_append_left) = (vec![(240f64 + ((point / 262144f64)).floor()), (128f64 + (((point / 4096f64)).floor() % 64f64)), (128f64 + (((point / 64f64)).floor() % 64f64)), (128f64 + (point % 64f64))], bytes); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let at_5 = (at + 1f64);
                                    {
                                        (bytes, at) = (bytes_5, at_5);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            bytes
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanUtf16TextSubstitutionLoop84Result {
    MlSource1TemporaryPlanUtf16TextSubstitutionLoop84Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_plan_utf16_text_substitution_loop84(replacement: &[f64], mut result: Vec<f64>, mut start: f64, mut at: f64) -> self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop84Result {
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
                                    let start_2 = at_2;
                                    {
                                        (result, start, at) = (result_6, start_2, at_2);
                                        continue;
                                    }
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
            self::MlSource1TemporaryPlanUtf16TextSubstitutionLoop84Result::MlSource1TemporaryPlanUtf16TextSubstitutionLoop84Done(result, start)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_replace_utf16_text_loop86(text: &[f64], mut result: Vec<f64>, mut at: f64) -> Vec<f64> {
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
pub enum MlSource1TemporaryReplaceUtf16TextLoop85Result {
    MlSource1TemporaryReplaceUtf16TextLoop85Done(Vec<f64>, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_replace_utf16_text_loop87(text: &[f64], mut result: Vec<f64>, mut found: f64, mut at: f64) -> Vec<f64> {
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
pub fn source_ml_source1_temporary_replace_utf16_text_loop89(text: &[f64], replacement: &[f64], mut result: Vec<f64>, mut kind: f64, mut last: f64, mut at: f64) -> Vec<f64> {
    loop {
        return if (at < last) {
            {
                let result_2 = { let (ml_append_right, ml_append_left) = (vec![if (kind == (0f64)) {
                    (self::ml_array::optional_at(replacement, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)
                } else {
                    (self::ml_array::optional_at(text, self::ml_array::number_index(at))).clone().unwrap_or_else(|| 0f64)
                }], result); self::ml_array::append(ml_append_left, ml_append_right) };
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
pub fn source_ml_source1_temporary_replace_utf16_text_loop88(text: &[f64], search: &[f64], replacement: &[f64], mut result: Vec<f64>, plan: &[f64], mut found: f64, mut instruction: f64) -> Vec<f64> {
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
                            let at = first;
                            {
                                let result_2 = self::source_ml_source1_temporary_replace_utf16_text_loop89(text, replacement, result, kind, last, at);
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
pub fn source_ml_source1_temporary_replace_utf16_text_loop85(text: &[f64], search: &[f64], replacement: &[f64], mut all: bool, mut result: Vec<f64>, plan: &[f64], mut start: f64, mut from: f64) -> self::MlSource1TemporaryReplaceUtf16TextLoop85Result {
    loop {
        return {
            let found = self::index_utf16_text(text, search, from);
            if (found < (0f64)) {
                self::MlSource1TemporaryReplaceUtf16TextLoop85Result::MlSource1TemporaryReplaceUtf16TextLoop85Done(result, start)
            } else {
                {
                    let at = start;
                    {
                        let result_2 = self::source_ml_source1_temporary_replace_utf16_text_loop87(text, result, found, at);
                        {
                            let instruction = 0f64;
                            {
                                let result_3 = self::source_ml_source1_temporary_replace_utf16_text_loop88(text, search, replacement, result_2, plan, found, instruction);
                                {
                                    let start_2 = (found + (search.len() as f64));
                                    if (!all || ((((search.len() as f64)) == (0f64)) && (found == ((text.len() as f64))))) {
                                        self::MlSource1TemporaryReplaceUtf16TextLoop85Result::MlSource1TemporaryReplaceUtf16TextLoop85Done(result_3, start_2)
                                    } else {
                                        {
                                            let from_2 = (start_2 + if (((search.len() as f64)) == (0f64)) {
                                                1f64
                                            } else {
                                                0f64
                                            });
                                            {
                                                (result, start, from) = (result_3, start_2, from_2);
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
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result {
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Done(Vec<f64>),
    MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop91(mut numbers: Vec<f64>, mut source_ml_source1_temporary_values90: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index90: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result {
    loop {
        return if (source_ml_source1_temporary_index90 < (self::ml::Big::from_u128(source_ml_source1_temporary_values90.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values90, source_ml_source1_temporary_index90.to_index());
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source2_f596) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        {
                            let numbers_2 = { let (ml_append_right, ml_append_left) = (vec![value_value], numbers); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index90_2 = source_ml_source1_temporary_index90.add(&self::ml::Big::from_i128(1));
                                {
                                    (numbers, source_ml_source1_temporary_index90) = (numbers_2, source_ml_source1_temporary_index90_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source2_f597) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source2_f598) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_f599) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source2_f600, source_ml_source2_f601) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(String::from("numeric primitives require actual Number operands after source conversion"))))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop91Done(numbers)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Result {
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Done,
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_primitive_loop93(mut source_ml_source1_temporary_values92: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index92: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Result {
    loop {
        return if (source_ml_source1_temporary_index92 < (self::ml::Big::from_u128(source_ml_source1_temporary_values92.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values92, source_ml_source1_temporary_index92.to_index());
                {
                    let error = self::read_java_script_semantic_value_error(value.clone());
                    if (error != "") {
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index92_2 = source_ml_source1_temporary_index92.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index92 = source_ml_source1_temporary_index92_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop93Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Done,
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop97(mut source: self::JavaScriptCallSource, mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut state: self::JavaScriptCallSourceBuilder, mut index: f64) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Result {
    loop {
        return if (index >= (0f64)) {
            {
                let entry = self::ml_array::optional_at(&bindings, self::ml_array::number_index(index));
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                    let source_ml_source2_comparison_operand607 = {
                        let source_ml_source2_nullish_field_receiver602 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver602.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present603 = ((source_ml_source2_nullish_field_receiver602.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present603.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source2_data_property_value604, _) => {
                                        source_ml_source2_data_property_value604
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    };
                    {
                        let source_ml_source2_comparison_operand608 = match source.clone() {
                            self::JavaScriptCallSource::SourceConstant(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceBinding(source_ml_source2_data_property_value605) => {
                                Some(source_ml_source2_data_property_value605)
                            }
                            self::JavaScriptCallSource::SourceLet(source_ml_source2_data_property_value606, _, _) => {
                                Some(source_ml_source2_data_property_value606)
                            }
                            self::JavaScriptCallSource::SourceIf(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceNumberBinary(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceSameValue(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceStrictEqual(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourcePrimitive(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceRecur(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceSet(_, _, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceGet(_, _) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceCall(_, _, _) => {
                                None::<f64>
                            }
                        };
                        match (source_ml_source2_comparison_operand607, source_ml_source2_comparison_operand608.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                    }
                }) {
                    self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), {
                        let source_ml_source2_nullish_field_receiver609 = (entry.clone(),);
                        if match (source_ml_source2_nullish_field_receiver609.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source2_nullish_field_present610 = ((source_ml_source2_nullish_field_receiver609.clone()).0).clone().expect("a proven present value");
                                match source_ml_source2_nullish_field_present610.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source2_data_property_value611) => {
                                        source_ml_source2_data_property_value611
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'register')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    })))
                } else {
                    {
                        let index_2 = (index - 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop97Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop98(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values94: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index94: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Result {
    loop {
        return if (source_ml_source1_temporary_index94 < (self::ml::Big::from_u128(source_ml_source1_temporary_values94.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values94, source_ml_source1_temporary_index94.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value612, _) => {
                                let source_ml_source2_data_property_value612 = *source_ml_source2_data_property_value612;
                                source_ml_source2_data_property_value612.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value613) => {
                                    source_ml_source2_data_property_value613
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index94_2 = source_ml_source1_temporary_index94.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index94) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index94_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop98Done(Box::new(current.clone()), arguments_)
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop100(mut current: self::JavaScriptCallSourceBuilder, arguments_: &[f64], mut index: f64) -> self::JavaScriptCallSourceBuilder {
    loop {
        return if (index < ((arguments_.len() as f64))) {
            {
                let current_2 = self::append_java_script_call_source_instruction(current.clone(), self::JavaScriptCallInstruction::Copy((index + 1f64), (self::ml_array::optional_at(arguments_, self::ml_array::number_index(index))).clone().unwrap_or_else(|| 0f64)));
                {
                    let index_2 = (index + 1f64);
                    {
                        (current, index) = (current_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            current.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop99(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values95: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index95: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Result {
    loop {
        return if (source_ml_source1_temporary_index95 < (self::ml::Big::from_u128(source_ml_source1_temporary_values95.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values95, source_ml_source1_temporary_index95.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let captured = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value614, _) => {
                                let source_ml_source2_data_property_value614 = *source_ml_source2_data_property_value614;
                                source_ml_source2_data_property_value614.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value615, _) => {
                                    let source_ml_source2_data_property_value615 = *source_ml_source2_data_property_value615;
                                    source_ml_source2_data_property_value615.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value616) => {
                                    source_ml_source2_data_property_value616
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value617) => {
                                    source_ml_source2_data_property_value617
                                }
                            }));
                            {
                                let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match captured.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value618) => {
                                        source_ml_source2_data_property_value618
                                    }
                                }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let source_ml_source1_temporary_index95_2 = source_ml_source1_temporary_index95.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, arguments_, source_ml_source1_temporary_index95) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index95_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop99Done(Box::new(current.clone()), arguments_)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result {
    MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_lower_java_script_call_source_expression_loop101(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_source1_temporary_values96: Vec<self::JavaScriptCallSource>, mut source_ml_source1_temporary_index96: self::ml::Big) -> self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result {
    loop {
        return if (source_ml_source1_temporary_index96 < (self::ml::Big::from_u128(source_ml_source1_temporary_values96.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_source1_temporary_values96, source_ml_source1_temporary_index96.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source2_data_property_value619, _) => {
                                let source_ml_source2_data_property_value619 = *source_ml_source2_data_property_value619;
                                source_ml_source2_data_property_value619.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source2_data_property_value620) => {
                                    source_ml_source2_data_property_value620
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index96_2 = source_ml_source1_temporary_index96.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_source1_temporary_index96) = (current_2.clone(), arguments__2, source_ml_source1_temporary_index96_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Result::MlSource1TemporaryLowerJavaScriptCallSourceExpressionLoop101Done(Box::new(current.clone()), arguments_)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryCompileJavaScriptCallSourceLoop102Result {
    MlSource1TemporaryCompileJavaScriptCallSourceLoop102Done,
    MlSource1TemporaryCompileJavaScriptCallSourceLoop102Return(Box<self::JavaScriptCallSourceCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_compile_java_script_call_source_loop102(parameters: &[f64], mut index: f64) -> self::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Result {
    loop {
        return if (index < ((parameters.len() as f64))) {
            {
                let owner = (self::ml_array::optional_at(parameters, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || ((self::ml_array::index_of(parameters, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, String::from("source parameters require distinct actual lexical owners"))))
                } else {
                    {
                        let index_2 = (index + 1f64);
                        {
                            index = index_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Result::MlSource1TemporaryCompileJavaScriptCallSourceLoop102Done
        };
    }
}
