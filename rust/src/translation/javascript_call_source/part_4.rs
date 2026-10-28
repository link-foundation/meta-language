// Generated from complete js/src/translation/javascript-call-source.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptCallSource;
use super::JavaScriptCallSourceBinding;
use super::JavaScriptCallSourceBuilder;
use super::JavaScriptCallSourceCompilation;
use super::JavaScriptCallSourceResult;
use super::allocate_java_script_call_source_register;
use super::append_java_script_call_source_instruction;
use super::lower_java_script_call_source_expression;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::test_java_script_call_code_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn compile_java_script_call_source(source: self::JavaScriptCallSource, parameters: &[f64]) -> self::JavaScriptCallSourceCompilation {
    if (((parameters.len() as f64)) >= (4096f64)) {
        self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, String::from("source parameters exceed their explicit register budget"))
    } else {
        {
            let index = 0f64;
            {
                let source_ml_source1_s428 = self::source_ml_temporary_compile_java_script_call_source_loop10(parameters, index);
                match source_ml_source1_s428.clone() {
                    self::MlTemporaryCompileJavaScriptCallSourceLoop10Result::MlTemporaryCompileJavaScriptCallSourceLoop10Done => {
                        {
                            let bindings = { let ml_values3 = parameters.to_vec(); let ml_callback3 = std::rc::Rc::new(move |identity: f64, index_2: f64| -> self::JavaScriptCallSourceBinding { self::JavaScriptCallSourceBinding::SourceBindingRegister(identity, (index_2 + 1f64)) }); ml_values3.iter().cloned().enumerate().map(|(ml_index3, ml_value3)| ml_callback3(ml_value3, (ml_index3 as f64))).collect::<Vec<_>>() };
                            {
                                let result = self::lower_java_script_call_source_expression(source.clone(), bindings, self::JavaScriptCallSourceBuilder::SourceBuilder(Vec::<self::JavaScriptCallInstruction>::new(), ((parameters.len() as f64) + 1f64), 0f64, (parameters.len() as f64), String::from("")), 0f64);
                                {
                                    let builder = self::append_java_script_call_source_instruction(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value429, _) => {
                                            let source_ml_source1_data_property_value429 = *source_ml_source1_data_property_value429;
                                            source_ml_source1_data_property_value429.clone()
                                        }
                                    }, self::JavaScriptCallInstruction::Return(match result.clone() {
                                        self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value430) => {
                                            source_ml_source1_data_property_value430
                                        }
                                    }));
                                    self::JavaScriptCallSourceCompilation::SourceCompiled(if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value432) => {
                                            source_ml_source1_data_property_value432.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(source_ml_source1_data_property_value433, _, _, _, _) => {
                                                source_ml_source1_data_property_value433
                                            }
                                        }
                                    } else {
                                        Vec::<self::JavaScriptCallInstruction>::new()
                                    }, if ((match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value435) => {
                                            source_ml_source1_data_property_value435.clone()
                                        }
                                    }) == "") {
                                        match builder.clone() {
                                            self::JavaScriptCallSourceBuilder::SourceBuilder(_, source_ml_source1_data_property_value436, _, _, _) => {
                                                source_ml_source1_data_property_value436
                                            }
                                        }
                                    } else {
                                        0f64
                                    }, match builder.clone() {
                                        self::JavaScriptCallSourceBuilder::SourceBuilder(_, _, _, _, source_ml_source1_data_property_value437) => {
                                            source_ml_source1_data_property_value437.clone()
                                        }
                                    })
                                }
                            }
                        }
                    }
                    self::MlTemporaryCompileJavaScriptCallSourceLoop10Result::MlTemporaryCompileJavaScriptCallSourceLoop10Return(source_ml_temporary_result10) => {
                        let source_ml_temporary_result10 = *source_ml_temporary_result10;
                        source_ml_temporary_result10.clone()
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String::from("semantic text requires complete ordered UTF16 code units"))
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
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_same_value_loop2(left_units: &[f64], right_units: &[f64], mut at: f64) -> self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if (at < ((left_units.len() as f64))) {
            if (((self::ml_array::optional_at(left_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(right_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)))) {
                self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
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
            self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result {
    MlTemporaryFindJavaScriptCallCodeFunctionLoop3Done,
    MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_find_java_script_call_code_function_loop3(mut program: self::JavaScriptCallProgram, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source1_data_property_value438, _, _) => source_ml_source1_data_property_value438.as_slice() }), self::ml_array::number_index(middle));
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value440, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value440.clone()
                            }
                        }) == identity) {
                            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value442, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value442.clone()
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
                                    let high_2 = middle;
                                    {
                                        high = high_2;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Result::MlTemporaryFindJavaScriptCallCodeFunctionLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result {
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Done,
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Return(Box<self::JavaScriptCallSourceResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_call_source_expression_loop6(mut source: self::JavaScriptCallSource, mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut state: self::JavaScriptCallSourceBuilder, mut index: f64) -> self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result {
    loop {
        return if (index >= (0f64)) {
            {
                let entry = self::ml_array::optional_at(&bindings, self::ml_array::number_index(index));
                if (!(match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                    let source_ml_source1_comparison_operand448 = {
                        let source_ml_source1_nullish_field_receiver443 = (entry.clone(),);
                        if match (source_ml_source1_nullish_field_receiver443.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source1_nullish_field_present444 = ((source_ml_source1_nullish_field_receiver443.clone()).0).clone().expect("a proven present value");
                                match source_ml_source1_nullish_field_present444.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(source_ml_source1_data_property_value445, _) => {
                                        source_ml_source1_data_property_value445
                                    }
                                }
                            }
                        } else {
                            (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (String::from("Cannot read properties of undefined (reading 'identity')")).encode_utf16().collect(), message_present: true }).raise()
                        }
                    };
                    {
                        let source_ml_source1_comparison_operand449 = match source.clone() {
                            self::JavaScriptCallSource::SourceConstant(_) => {
                                None::<f64>
                            }
                            self::JavaScriptCallSource::SourceBinding(source_ml_source1_data_property_value446) => {
                                Some(source_ml_source1_data_property_value446)
                            }
                            self::JavaScriptCallSource::SourceLet(source_ml_source1_data_property_value447, _, _) => {
                                Some(source_ml_source1_data_property_value447)
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
                        match (source_ml_source1_comparison_operand448, source_ml_source1_comparison_operand449.clone()) { (ml_left, Some(ml_right)) => ml_left == ml_right, _ => false }
                    }
                }) {
                    self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Return(Box::new(self::JavaScriptCallSourceResult::SourceResult(Box::new(state.clone()), {
                        let source_ml_source1_nullish_field_receiver450 = (entry.clone(),);
                        if match (source_ml_source1_nullish_field_receiver450.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                            {
                                let source_ml_source1_nullish_field_present451 = ((source_ml_source1_nullish_field_receiver450.clone()).0).clone().expect("a proven present value");
                                match source_ml_source1_nullish_field_present451.clone() {
                                    self::JavaScriptCallSourceBinding::SourceBindingRegister(_, source_ml_source1_data_property_value452) => {
                                        source_ml_source1_data_property_value452
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
            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop6Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_call_source_expression_loop8(mut current: self::JavaScriptCallSourceBuilder, arguments_: &[f64], mut index: f64) -> self::JavaScriptCallSourceBuilder {
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
pub enum MlTemporaryLowerJavaScriptCallSourceExpressionLoop7Result {
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop7Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_call_source_expression_loop7(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_temporary_values4: Vec<self::JavaScriptCallSource>, mut source_ml_temporary_index4: self::ml::Big) -> self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop7Result {
    loop {
        return if (source_ml_temporary_index4 < (self::ml::Big::from_u128(source_ml_temporary_values4.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_temporary_values4, source_ml_temporary_index4.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let captured = self::allocate_java_script_call_source_register(match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value453, _) => {
                                let source_ml_source1_data_property_value453 = *source_ml_source1_data_property_value453;
                                source_ml_source1_data_property_value453.clone()
                            }
                        });
                        {
                            let current_2 = self::append_java_script_call_source_instruction(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value454, _) => {
                                    let source_ml_source1_data_property_value454 = *source_ml_source1_data_property_value454;
                                    source_ml_source1_data_property_value454.clone()
                                }
                            }, self::JavaScriptCallInstruction::Copy(match captured.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value455) => {
                                    source_ml_source1_data_property_value455
                                }
                            }, match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value456) => {
                                    source_ml_source1_data_property_value456
                                }
                            }));
                            {
                                let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match captured.clone() {
                                    self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value457) => {
                                        source_ml_source1_data_property_value457
                                    }
                                }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                                {
                                    let source_ml_temporary_index4_2 = source_ml_temporary_index4.add(&self::ml::Big::from_i128(1));
                                    {
                                        (current, arguments_, source_ml_temporary_index4) = (current_2.clone(), arguments__2, source_ml_temporary_index4_2.clone());
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop7Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop7Done(Box::new(current.clone()), arguments_)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryLowerJavaScriptCallSourceExpressionLoop9Result {
    MlTemporaryLowerJavaScriptCallSourceExpressionLoop9Done(Box<self::JavaScriptCallSourceBuilder>, Vec<f64>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_lower_java_script_call_source_expression_loop9(mut bindings: Vec<self::JavaScriptCallSourceBinding>, mut depth: f64, mut current: self::JavaScriptCallSourceBuilder, mut arguments_: Vec<f64>, mut source_ml_temporary_values5: Vec<self::JavaScriptCallSource>, mut source_ml_temporary_index5: self::ml::Big) -> self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop9Result {
    loop {
        return if (source_ml_temporary_index5 < (self::ml::Big::from_u128(source_ml_temporary_values5.len() as u128))) {
            {
                let argument = self::ml_array::at(&source_ml_temporary_values5, source_ml_temporary_index5.to_index());
                {
                    let value = self::lower_java_script_call_source_expression(argument.clone(), bindings.clone(), current.clone(), (depth + 1f64));
                    {
                        let current_2 = match value.clone() {
                            self::JavaScriptCallSourceResult::SourceResult(source_ml_source1_data_property_value458, _) => {
                                let source_ml_source1_data_property_value458 = *source_ml_source1_data_property_value458;
                                source_ml_source1_data_property_value458.clone()
                            }
                        };
                        {
                            let arguments__2 = { let (ml_append_right, ml_append_left) = (vec![match value.clone() {
                                self::JavaScriptCallSourceResult::SourceResult(_, source_ml_source1_data_property_value459) => {
                                    source_ml_source1_data_property_value459
                                }
                            }], arguments_); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_temporary_index5_2 = source_ml_temporary_index5.add(&self::ml::Big::from_i128(1));
                                {
                                    (current, arguments_, source_ml_temporary_index5) = (current_2.clone(), arguments__2, source_ml_temporary_index5_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryLowerJavaScriptCallSourceExpressionLoop9Result::MlTemporaryLowerJavaScriptCallSourceExpressionLoop9Done(Box::new(current.clone()), arguments_)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryCompileJavaScriptCallSourceLoop10Result {
    MlTemporaryCompileJavaScriptCallSourceLoop10Done,
    MlTemporaryCompileJavaScriptCallSourceLoop10Return(Box<self::JavaScriptCallSourceCompilation>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_compile_java_script_call_source_loop10(parameters: &[f64], mut index: f64) -> self::MlTemporaryCompileJavaScriptCallSourceLoop10Result {
    loop {
        return if (index < ((parameters.len() as f64))) {
            {
                let owner = (self::ml_array::optional_at(parameters, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (!(self::test_java_script_call_code_coordinate(owner, 65536f64)) || ((self::ml_array::index_of(parameters, &owner, 0f64, false, |a, b| a == b)) != index)) {
                    self::MlTemporaryCompileJavaScriptCallSourceLoop10Result::MlTemporaryCompileJavaScriptCallSourceLoop10Return(Box::new(self::JavaScriptCallSourceCompilation::SourceCompiled(Vec::<self::JavaScriptCallInstruction>::new(), 0f64, String::from("source parameters require distinct actual lexical owners"))))
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
            self::MlTemporaryCompileJavaScriptCallSourceLoop10Result::MlTemporaryCompileJavaScriptCallSourceLoop10Done
        };
    }
}
