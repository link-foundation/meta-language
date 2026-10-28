// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptDescriptorConversionStateErrorLoop120Result;
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
use super::test_java_script_call_coordinate;
use super::test_java_script_descriptor_field_presence;

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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop131(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut fn_: self::JavaScriptCallFunction, mut source_ml_source1_temporary_values126: Vec<self::JavaScriptCallInstruction>, mut source_ml_source1_temporary_index126: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result {
    loop {
        return if (source_ml_source1_temporary_index126 < (self::ml::Big::from_u128(source_ml_source1_temporary_values126.len() as u128))) {
            {
                let instruction = self::ml_array::at(&source_ml_source1_temporary_values126, source_ml_source1_temporary_index126.to_index());
                {
                    let source_ml_source1_temporary_values127 = self::read_java_script_call_registers(instruction.clone());
                    {
                        let source_ml_source1_temporary_index127 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source2_s2182 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop132(fn_.clone(), &(source_ml_source1_temporary_values127), source_ml_source1_temporary_index127.clone());
                            match source_ml_source2_s2182.clone() {
                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Done => {
                                    match instruction.clone() {
                                        self::JavaScriptCallInstruction::Constant(source_ml_source2_f2183, value) => {
                                            let value = *value;
                                            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 110, 115, 116, 97, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index126_2 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_2.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Copy(source_ml_source2_f2184, source_ml_source2_f2185) => {
                                            {
                                                let source_ml_source1_temporary_index126_3 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_3.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Argument(source_ml_source2_f2186, index) => {
                                            if !(self::test_java_script_call_coordinate(index, 1048576f64)) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 97, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index126_4 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_4.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Jump(target) => {
                                            if !(self::test_java_script_call_coordinate(target, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2221) => source_ml_source2_data_property_value2221.as_slice() }).len() as f64))) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 106, 117, 109, 112, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index126_5 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_5.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Branch(source_ml_source2_f2187, yes, no) => {
                                            if (!(self::test_java_script_call_coordinate(yes, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2222) => source_ml_source2_data_property_value2222.as_slice() }).len() as f64))) || !(self::test_java_script_call_coordinate(no, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2223) => source_ml_source2_data_property_value2223.as_slice() }).len() as f64)))) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 98, 114, 97, 110, 99, 104, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 98, 111, 116, 104, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index126_6 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_6.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Call(source_ml_source2_f2188, source_ml_source2_f2189, source_ml_source2_f2190, source_ml_source2_f2191) => {
                                            {
                                                let source_ml_source1_temporary_index126_7 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_7.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Return(source_ml_source2_f2192) => {
                                            {
                                                let source_ml_source1_temporary_index126_8 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_8.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Throw(source_ml_source2_f2193) => {
                                            {
                                                let source_ml_source1_temporary_index126_9 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_9.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PushHandler(source_ml_source2_f2194, target_2) => {
                                            if !(self::test_java_script_call_coordinate(target_2, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2224) => source_ml_source2_data_property_value2224.as_slice() }).len() as f64))) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index126_10 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_10.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PopHandler => {
                                            {
                                                let source_ml_source1_temporary_index126_11 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_11.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::NumberBinary(source_ml_source2_f2195, source_ml_source2_f2196, source_ml_source2_f2197, operator) => {
                                            if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 100, 100]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]), self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]), self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index126_12 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_12.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::SameValue(source_ml_source2_f2198, source_ml_source2_f2199, source_ml_source2_f2200) => {
                                            {
                                                let source_ml_source1_temporary_index126_13 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_13.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::StrictEqual(source_ml_source2_f2201, source_ml_source2_f2202, source_ml_source2_f2203) => {
                                            {
                                                let source_ml_source1_temporary_index126_14 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_14.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::RequireWrite(source_ml_source2_f2204) => {
                                            {
                                                let source_ml_source1_temporary_index126_15 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_15.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Primitive(source_ml_source2_f2205, operation, arguments) => {
                                            {
                                                let error = self::read_java_script_call_primitive_arity_error(&(operation), (arguments.len() as f64));
                                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(error.clone())
                                                } else {
                                                    {
                                                        let source_ml_source1_temporary_index126_16 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                        {
                                                            source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_16.clone();
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source2_f2206, source_ml_source2_f2207, source_ml_source2_f2208, source_ml_source2_f2209) => {
                                            {
                                                let source_ml_source1_temporary_index126_17 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_17.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinarySet(source_ml_source2_f2210, source_ml_source2_f2211, source_ml_source2_f2212, source_ml_source2_f2213, source_ml_source2_f2214) => {
                                            {
                                                let source_ml_source1_temporary_index126_18 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_18.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PropertyKey(source_ml_source2_f2215, source_ml_source2_f2216, realm) => {
                                            if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value2226, _) => source_ml_source2_data_property_value2226.as_slice() }), &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index126_19 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_19.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source2_f2217, source_ml_source2_f2218, source_ml_source2_f2219, source_ml_source2_f2220, realm_2) => {
                                            if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value2228, _) => source_ml_source2_data_property_value2228.as_slice() }), &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index126_20 = source_ml_source1_temporary_index126.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_source1_temporary_index126 = source_ml_source1_temporary_index126_20.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Return(source_ml_source1_temporary_result132) => {
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(source_ml_source1_temporary_result132.clone())
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
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop130(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values125: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index125: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result {
    loop {
        return if (source_ml_source1_temporary_index125 < (self::ml::Big::from_u128(source_ml_source1_temporary_values125.len() as u128))) {
            {
                let capture = self::ml_array::at(&source_ml_source1_temporary_values125, source_ml_source1_temporary_index125.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), capture.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index125_2 = source_ml_source1_temporary_index125.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index125 = source_ml_source1_temporary_index125_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop129(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut count: f64, mut source_ml_source1_temporary_values124: Vec<self::JavaScriptCallFunction>, mut source_ml_source1_temporary_index124: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result {
    loop {
        return if (source_ml_source1_temporary_index124 < (self::ml::Big::from_u128(source_ml_source1_temporary_values124.len() as u128))) {
            {
                let fn_ = self::ml_array::at(&source_ml_source1_temporary_values124, source_ml_source1_temporary_index124.to_index());
                if (((match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2230, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value2230.clone()
                    }
                }) <= previous) || ((match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2232, _, _, _, _, _, _, _) => {
                        source_ml_source2_data_property_value2232.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2233, _, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value2233.clone()
                            }
                        };
                        if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value2235, _) => source_ml_source2_data_property_value2235.as_slice() }), &match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value2236, _, _, _, _, _, _) => {
                                source_ml_source2_data_property_value2236.clone()
                            }
                        }, 0f64, false, |a, b| a == b)) {
                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))
                        } else {
                            {
                                let callable = self::read_java_script_ordinary_value_error(heap.clone(), self::JavaScriptSemanticValue::Object(match fn_.clone() {
                                    self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value2237, _, _, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value2237.clone()
                                    }
                                }, true));
                                if (callable != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(callable.clone())
                                } else {
                                    if (((match fn_.clone() {
                                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value2239, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value2239.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) && ((match fn_.clone() {
                                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value2241, _, _, _, _, _) => {
                                            source_ml_source2_data_property_value2241.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])))) {
                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 105, 99, 116, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103]))
                                    } else {
                                        if ((self::read_java_script_ordinary_value_error(heap.clone(), match fn_.clone() {
                                            self::JavaScriptCallFunction::CallFunction(_, _, _, source_ml_source2_data_property_value2243, _, _, _, _) => {
                                                let source_ml_source2_data_property_value2243 = *source_ml_source2_data_property_value2243;
                                                source_ml_source2_data_property_value2243.clone()
                                            }
                                        })) != (self::ml_string::Text::from_units(vec![]))) {
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                        } else {
                                            if (((!(self::test_java_script_call_coordinate(match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2244, _, _) => {
                                                    source_ml_source2_data_property_value2244
                                                }
                                            }, 4097f64)) || ((match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2246, _, _) => {
                                                    source_ml_source2_data_property_value2246
                                                }
                                            }) == (0f64))) || !(self::test_java_script_call_coordinate(match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source2_data_property_value2247, _, _, _) => {
                                                    source_ml_source2_data_property_value2247
                                                }
                                            }, match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2248, _, _) => {
                                                    source_ml_source2_data_property_value2248
                                                }
                                            }))) || ((((match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source2_data_property_value2252, _, _, _) => {
                                                    source_ml_source2_data_property_value2252
                                                }
                                            } + ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source2_data_property_value2253, _) => source_ml_source2_data_property_value2253.as_slice() }).len() as f64)) + 1f64)) > (match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2254, _, _) => {
                                                    source_ml_source2_data_property_value2254
                                                }
                                            }))) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101]))
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_values125 = match fn_.clone() {
                                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source2_data_property_value2255, _) => {
                                                            source_ml_source2_data_property_value2255
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source1_temporary_index125 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s2256 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop130(heap.clone(), source_ml_source1_temporary_values125, source_ml_source1_temporary_index125.clone());
                                                            match source_ml_source2_s2256.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Done => {
                                                                    {
                                                                        let count_2 = (count + ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2257) => source_ml_source2_data_property_value2257.as_slice() }).len() as f64));
                                                                        if ((count_2 > (65536f64)) || ((((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2259) => source_ml_source2_data_property_value2259.as_slice() }).len() as f64)) == (0f64))) {
                                                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116, 32, 111, 114, 32, 104, 97, 115, 32, 110, 111, 32, 101, 110, 116, 114, 121]))
                                                                        } else {
                                                                            {
                                                                                let source_ml_source1_temporary_values126 = match fn_.clone() {
                                                                                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2260) => {
                                                                                        source_ml_source2_data_property_value2260
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source1_temporary_index126 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source2_s2261 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop131(program.clone(), heap.clone(), fn_.clone(), source_ml_source1_temporary_values126, source_ml_source1_temporary_index126.clone());
                                                                                        match source_ml_source2_s2261.clone() {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Done => {
                                                                                                {
                                                                                                    let source_ml_source1_temporary_index124_2 = source_ml_source1_temporary_index124.add(&self::ml::Big::from_i128(1));
                                                                                                    {
                                                                                                        (previous, count, source_ml_source1_temporary_index124) = (previous_2.clone(), count_2, source_ml_source1_temporary_index124_2.clone());
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop131Return(source_ml_source1_temporary_result131) => {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(source_ml_source1_temporary_result131.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop130Return(source_ml_source1_temporary_result132) => {
                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Return(source_ml_source1_temporary_result132.clone())
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
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop129Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop128(mut previous: self::ml::Big, mut source_ml_source1_temporary_values123: Vec<self::ml::Big>, mut source_ml_source1_temporary_index123: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result {
    loop {
        return if (source_ml_source1_temporary_index123 < (self::ml::Big::from_u128(source_ml_source1_temporary_values123.len() as u128))) {
            {
                let realm = self::ml_array::at(&source_ml_source1_temporary_values123, source_ml_source1_temporary_index123.to_index());
                if ((realm <= previous) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 97, 108, 109, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = realm.clone();
                        {
                            let source_ml_source1_temporary_index123_2 = source_ml_source1_temporary_index123.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_source1_temporary_index123) = (previous_2.clone(), source_ml_source1_temporary_index123_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop128Done(previous.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop133(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2263, _, _) => {
                source_ml_source2_data_property_value2263
            }
        })) {
            {
                let registers_2 = { let (ml_append_right, ml_append_left) = (vec![self::JavaScriptSemanticValue::Undefined], registers); self::ml_array::append(ml_append_left, ml_append_right) };
                {
                    let index_2 = (index + 1f64);
                    {
                        (registers, index) = (registers_2, index_2);
                        continue;
                    }
                }
            }
        } else {
            registers
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop134(mut fn_: self::JavaScriptCallFunction, mut arguments_: Vec<self::JavaScriptSemanticValue>, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source2_data_property_value2265, _, _, _) => {
                source_ml_source2_data_property_value2265
            }
        })) {
            {
                let value = (self::ml_array::optional_at(&arguments_, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let slot = (index + 1f64);
                    {
                        let registers_2 = { let ml_values8 = registers; let ml_callback8 = { let slot = slot; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, position: f64| -> self::JavaScriptSemanticValue { if (position == slot) {
                            value.clone()
                        } else {
                            entry.clone()
                        } }) }; ml_values8.iter().cloned().enumerate().map(|(ml_index8, ml_value8)| ml_callback8(ml_value8, (ml_index8 as f64))).collect::<Vec<_>>() };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (registers, index) = (registers_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            registers
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop135(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source2_data_property_value2267, _) => source_ml_source2_data_property_value2267.as_slice() }).len() as f64))) {
            {
                let value = (self::ml_array::optional_at((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source2_data_property_value2268, _) => source_ml_source2_data_property_value2268.as_slice() }), self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let slot = ((match fn_.clone() {
                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source2_data_property_value2269, _, _, _) => {
                            source_ml_source2_data_property_value2269
                        }
                    } + 1f64) + index);
                    {
                        let registers_2 = { let ml_values9 = registers; let ml_callback9 = { let slot = slot; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, position: f64| -> self::JavaScriptSemanticValue { if (position == slot) {
                            value.clone()
                        } else {
                            entry.clone()
                        } }) }; ml_values9.iter().cloned().enumerate().map(|(ml_index9, ml_value9)| ml_callback9(ml_value9, (ml_index9 as f64))).collect::<Vec<_>>() };
                        {
                            let index_2 = (index + 1f64);
                            {
                                (registers, index) = (registers_2, index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            registers
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop139Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144 {
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Next,
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin144Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143 {
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Next,
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin143Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text),
}
