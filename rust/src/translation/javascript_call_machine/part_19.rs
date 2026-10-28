// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Result;
use super::apply_java_script_descriptor_field;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_call_primitive_arity_error;
use super::read_java_script_call_registers;
use super::read_java_script_descriptor_field;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_value_error;
use super::test_java_script_call_coordinate;
use super::test_java_script_descriptor_field_presence;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_numeric_primitive_loop116(mut numbers: Vec<f64>, mut source_ml_source1_temporary_values115: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index115: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Result {
    loop {
        return if (source_ml_source1_temporary_index115 < (self::ml::Big::from_u128(source_ml_source1_temporary_values115.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values115, source_ml_source1_temporary_index115.to_index());
                match value.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Boolean(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Number(value_value) => {
                        {
                            let numbers_2 = { let (ml_append_right, ml_append_left) = (vec![value_value], numbers); self::ml_array::append(ml_append_left, ml_append_right) };
                            {
                                let source_ml_source1_temporary_index115_2 = source_ml_source1_temporary_index115.add(&self::ml::Big::from_i128(1));
                                {
                                    (numbers, source_ml_source1_temporary_index115) = (numbers_2, source_ml_source1_temporary_index115_2.clone());
                                    continue;
                                }
                            }
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Text(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Symbol(_) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                    self::JavaScriptSemanticValue::Object(_, _) => {
                        self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(self::ml_string::Text::from_units(vec![110, 117, 109, 101, 114, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115, 32, 97, 102, 116, 101, 114, 32, 115, 111, 117, 114, 99, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))))
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Result::MlSource1TemporaryExecuteJavaScriptCallNumericPrimitiveLoop116Done(numbers)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Result {
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Done,
    MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Return(Box<self::JavaScriptCallPrimitiveResult>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_primitive_loop118(mut source_ml_source1_temporary_values117: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index117: self::ml::Big) -> self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Result {
    loop {
        return if (source_ml_source1_temporary_index117 < (self::ml::Big::from_u128(source_ml_source1_temporary_values117.len() as u128))) {
            {
                let value = self::ml_array::at(&source_ml_source1_temporary_values117, source_ml_source1_temporary_index117.to_index());
                {
                    let error = self::read_java_script_semantic_value_error(value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Return(Box::new(self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(error.clone())))
                    } else {
                        {
                            let source_ml_source1_temporary_index117_2 = source_ml_source1_temporary_index117.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index117 = source_ml_source1_temporary_index117_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Result::MlSource1TemporaryExecuteJavaScriptCallPrimitiveLoop118Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_execute_java_script_call_text_conversion_loop120(text: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (text.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source2_string_unit_operand2490 = text.clone();
                    {
                        let source_ml_source2_string_unit_operand2491 = index;
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
                                    let source_ml_source2_string_unit_operand2497 = {
                                        let source_ml_source2_string_range_operand2492 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand2493 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand2494 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position2495 = source_ml_source2_string_range_operand2493;
                                                    {
                                                        let source_ml_source2_string_range_position2496 = source_ml_source2_string_range_operand2494;
                                                        source_ml_source2_string_range_operand2492.slice(source_ml_source2_string_range_position2495, source_ml_source2_string_range_position2496)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand2498 = 0f64;
                                        source_ml_source2_string_unit_operand2497.code_point_at(source_ml_source2_string_unit_operand2498)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand2490.clone(), source_ml_source2_string_unit_operand2491)
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
pub enum MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119 {
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Next(self::ml_string::Text),
    MlSource1TemporaryExecuteJavaScriptCallTextConversionJoin119Return(Box<self::JavaScriptCallPrimitiveResult>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Result {
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Done,
    MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_call_code_function_loop121(mut program: self::JavaScriptCallProgram, mut identity: self::ml::Big, mut low: f64, mut high: f64) -> self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Result {
    loop {
        return if (low < high) {
            {
                let middle = (((low + high) / 2f64)).floor();
                {
                    let entry = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value2499, _, _) => source_ml_source2_data_property_value2499.as_slice() }), self::ml_array::number_index(middle));
                    if match (entry.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                        self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Return((-1f64))
                    } else {
                        if ((match entry.clone().expect("a proven present value") {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2501, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value2501.clone()
                            }
                        }) == identity) {
                            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Return(middle)
                        } else {
                            if ((match entry.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2503, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value2503.clone()
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
            self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop121Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result {
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Done,
    MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_descriptor_conversion_state_error_loop122(mut state: self::JavaScriptDescriptorConversionState, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result {
    loop {
        return if (index < (6f64)) {
            if self::test_java_script_descriptor_field_presence(match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2504, _) => {
                    source_ml_source2_data_property_value2504.clone()
                }
            }, index) {
                self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Return(self::ml_string::Text::from_units(vec![117, 110, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 102, 97, 98, 114, 105, 99, 97, 116, 101, 100, 32, 105, 110, 32, 97, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
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
            self::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Result::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop122Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result {
    MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Done(Box<self::JavaScriptDescriptorConversionState>),
    MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box<self::JavaScriptDescriptorConversionTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_descriptor_conversion_loop123(mut heap: self::JavaScriptOrdinaryHeap, mut current: self::JavaScriptDescriptorConversionState) -> self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result {
    loop {
        return if ((match current.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2506, _, _) => {
                source_ml_source2_data_property_value2506
            }
        }) < (6f64)) {
            {
                let field = self::read_java_script_descriptor_field(match current.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2507, _, _) => {
                        source_ml_source2_data_property_value2507
                    }
                });
                {
                    let key = self::JavaScriptSemanticValue::Text((field.clone()).2);
                    {
                        let receiver = match current.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value2508, _, _, _) => {
                                let source_ml_source2_data_property_value2508 = *source_ml_source2_data_property_value2508;
                                source_ml_source2_data_property_value2508.clone()
                            }
                        };
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                {
                                    let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                    if !((property.clone()).1) {
                                        {
                                            let current_2 = self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), (match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2515, _, _) => {
                                                    source_ml_source2_data_property_value2515
                                                }
                                            } + 1f64), match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2516, _) => {
                                                    source_ml_source2_data_property_value2516.clone()
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
                                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid((plan.clone()).3)))
                                            } else {
                                                if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorCall(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2517, _, _) => {
                                                            source_ml_source2_data_property_value2517
                                                        }
                                                    }, match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2518, _) => {
                                                            source_ml_source2_data_property_value2518.clone()
                                                        }
                                                    }, true)), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)))
                                                } else {
                                                    if (((plan.clone()).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                        self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))))
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
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(_) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(_) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(_, _) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(_) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Return(Box::new(applied.clone()))
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
            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop123Done(Box::new(current.clone()))
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_finish_java_script_property_key_conversion_loop124(spelling: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (spelling.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source2_string_unit_operand2528 = spelling.clone();
                    {
                        let source_ml_source2_string_unit_operand2529 = index;
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
                                    let source_ml_source2_string_unit_operand2535 = {
                                        let source_ml_source2_string_range_operand2530 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand2531 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand2532 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position2533 = source_ml_source2_string_range_operand2531;
                                                    {
                                                        let source_ml_source2_string_range_position2534 = source_ml_source2_string_range_operand2532;
                                                        source_ml_source2_string_range_operand2530.slice(source_ml_source2_string_range_position2533, source_ml_source2_string_range_position2534)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand2536 = 0f64;
                                        source_ml_source2_string_unit_operand2535.code_point_at(source_ml_source2_string_unit_operand2536)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand2528.clone(), source_ml_source2_string_unit_operand2529)
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
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop134(mut fn_: self::JavaScriptCallFunction, source_ml_source1_temporary_values129: &[f64], mut source_ml_source1_temporary_index129: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result {
    loop {
        return if (source_ml_source1_temporary_index129 < (self::ml::Big::from_u128(source_ml_source1_temporary_values129.len() as u128))) {
            {
                let register = self::ml_array::at(source_ml_source1_temporary_values129, source_ml_source1_temporary_index129.to_index());
                if !(self::test_java_script_call_coordinate(register, match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2537, _, _) => {
                        source_ml_source2_data_property_value2537
                    }
                })) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index129_2 = source_ml_source1_temporary_index129.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index129 = source_ml_source1_temporary_index129_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop133(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut fn_: self::JavaScriptCallFunction, mut source_ml_source1_temporary_values128: Vec<self::JavaScriptCallInstruction>, mut source_ml_source1_temporary_index128: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result {
    loop {
        return match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, shared_constructor_field2) => {
                match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(_, shared_constructor_field1, _) => {
                        if (source_ml_source1_temporary_index128 < (self::ml::Big::from_u128(source_ml_source1_temporary_values128.len() as u128))) {
                            {
                                let instruction = self::ml_array::at(&source_ml_source1_temporary_values128, source_ml_source1_temporary_index128.to_index());
                                {
                                    let source_ml_source1_temporary_values129 = self::read_java_script_call_registers(instruction.clone());
                                    {
                                        let source_ml_source1_temporary_index129 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s2538 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop134(fn_.clone(), &(source_ml_source1_temporary_values129), source_ml_source1_temporary_index129.clone());
                                            match source_ml_source2_s2538.clone() {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Done => {
                                                    match instruction.clone() {
                                                        self::JavaScriptCallInstruction::Constant(_, value) => {
                                                            let value = *value;
                                                            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 110, 115, 116, 97, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index128_2 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_2.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Copy(_, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index128_3 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_3.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Argument(_, index) => {
                                                            if !(self::test_java_script_call_coordinate(index, 1048576f64)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 97, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index128_4 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_4.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Jump(target) => {
                                                            if !(self::test_java_script_call_coordinate(target, (shared_constructor_field2.len() as f64))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 106, 117, 109, 112, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index128_5 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_5.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Branch(_, yes, no) => {
                                                            if (!(self::test_java_script_call_coordinate(yes, (shared_constructor_field2.len() as f64))) || !(self::test_java_script_call_coordinate(no, (shared_constructor_field2.len() as f64)))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 98, 114, 97, 110, 99, 104, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 98, 111, 116, 104, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index128_6 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_6.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index128_7 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_7.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Return(_) => {
                                                            {
                                                                let source_ml_source1_temporary_index128_8 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_8.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Throw(_) => {
                                                            {
                                                                let source_ml_source1_temporary_index128_9 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_9.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PushHandler(_, target_2) => {
                                                            if !(self::test_java_script_call_coordinate(target_2, (shared_constructor_field2.len() as f64))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index128_10 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_10.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PopHandler => {
                                                            {
                                                                let source_ml_source1_temporary_index128_11 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_11.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::NumberBinary(_, _, _, operator) => {
                                                            if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 100, 100]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]), self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]), self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index128_12 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_12.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index128_13 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_13.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index128_14 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_14.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                            {
                                                                let source_ml_source1_temporary_index128_15 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_15.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Primitive(_, operation, arguments) => {
                                                            {
                                                                let error = self::read_java_script_call_primitive_arity_error(&(operation), (arguments.len() as f64));
                                                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(error.clone())
                                                                } else {
                                                                    {
                                                                        let source_ml_source1_temporary_index128_16 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                        {
                                                                            source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_16.clone();
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index128_17 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_17.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index128_18 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_18.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PropertyKey(_, _, realm) => {
                                                            if !(self::ml_array::includes(&shared_constructor_field1, &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index128_19 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_19.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, realm_2) => {
                                                            if !(self::ml_array::includes(&shared_constructor_field1, &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index128_20 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index128 = source_ml_source1_temporary_index128_20.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Return(source_ml_source1_temporary_result134) => {
                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(source_ml_source1_temporary_result134.clone())
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Done
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop132(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values127: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index127: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result {
    loop {
        return if (source_ml_source1_temporary_index127 < (self::ml::Big::from_u128(source_ml_source1_temporary_values127.len() as u128))) {
            {
                let capture = self::ml_array::at(&source_ml_source1_temporary_values127, source_ml_source1_temporary_index127.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), capture.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop131(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut count: f64, mut source_ml_source1_temporary_values126: Vec<self::JavaScriptCallFunction>, mut source_ml_source1_temporary_index126: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result {
    loop {
        return if (source_ml_source1_temporary_index126 < (self::ml::Big::from_u128(source_ml_source1_temporary_values126.len() as u128))) {
            {
                let fn_ = self::ml_array::at(&source_ml_source1_temporary_values126, source_ml_source1_temporary_index126.to_index());
                match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8) => {
                        let shared_constructor_field4 = *shared_constructor_field4;
                        if ((shared_constructor_field1 <= previous) || (shared_constructor_field1 > (self::ml::Big::from_i128(18446744073709551615)))) {
                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                        } else {
                            {
                                let previous_2 = shared_constructor_field1.clone();
                                if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value2591, _) => source_ml_source2_data_property_value2591.as_slice() }), &shared_constructor_field2.clone(), 0f64, false, |a, b| a == b)) {
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))
                                } else {
                                    {
                                        let callable = self::read_java_script_ordinary_value_error(heap.clone(), self::JavaScriptSemanticValue::Object(shared_constructor_field1.clone(), true));
                                        if (callable != (self::ml_string::Text::from_units(vec![]))) {
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(callable.clone())
                                        } else {
                                            if ((shared_constructor_field3 != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) && (shared_constructor_field3 != (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])))) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 105, 99, 116, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103]))
                                            } else {
                                                if ((self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field4.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                                } else {
                                                    if (((!(self::test_java_script_call_coordinate(shared_constructor_field6, 4097f64)) || (shared_constructor_field6 == (0f64))) || !(self::test_java_script_call_coordinate(shared_constructor_field5, shared_constructor_field6))) || ((((shared_constructor_field5 + (shared_constructor_field7.len() as f64)) + 1f64)) > shared_constructor_field6)) {
                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101]))
                                                    } else {
                                                        {
                                                            let source_ml_source1_temporary_values127 = shared_constructor_field7;
                                                            {
                                                                let source_ml_source1_temporary_index127 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let source_ml_source2_s2612 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop132(heap.clone(), source_ml_source1_temporary_values127, source_ml_source1_temporary_index127.clone());
                                                                    match source_ml_source2_s2612.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Done => {
                                                                            {
                                                                                let count_2 = (count + (shared_constructor_field8.len() as f64));
                                                                                if ((count_2 > (65536f64)) || (((shared_constructor_field8.len() as f64)) == (0f64))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116, 32, 111, 114, 32, 104, 97, 115, 32, 110, 111, 32, 101, 110, 116, 114, 121]))
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source1_temporary_values128 = shared_constructor_field8;
                                                                                        {
                                                                                            let source_ml_source1_temporary_index128 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source2_s2617 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop133(program.clone(), heap.clone(), fn_.clone(), source_ml_source1_temporary_values128, source_ml_source1_temporary_index128.clone());
                                                                                                match source_ml_source2_s2617.clone() {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Done => {
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index126_2 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                                                                            {
                                                                                                                (previous, count, source_ml_source1_temporary_index126) = (previous_2.clone(), count_2, source_ml_source1_temporary_index126_2.clone());
                                                                                                                continue;
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(source_ml_source1_temporary_result133) => {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(source_ml_source1_temporary_result133.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Return(source_ml_source1_temporary_result134) => {
                                                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(source_ml_source1_temporary_result134.clone())
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
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop130(mut previous: self::ml::Big, mut source_ml_source1_temporary_values125: Vec<self::ml::Big>, mut source_ml_source1_temporary_index125: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result {
    loop {
        return if (source_ml_source1_temporary_index125 < (self::ml::Big::from_u128(source_ml_source1_temporary_values125.len() as u128))) {
            {
                let realm = self::ml_array::at(&source_ml_source1_temporary_values125, source_ml_source1_temporary_index125.to_index());
                if ((realm <= previous) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 97, 108, 109, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index125_2 = source_ml_source1_temporary_index125.add(&self::ml::Big::from_i128(1));
                        {
                            (previous, source_ml_source1_temporary_index125) = (realm.clone(), source_ml_source1_temporary_index125_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Done(previous.clone())
        };
    }
}
