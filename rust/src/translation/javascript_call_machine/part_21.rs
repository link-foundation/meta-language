// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Result;
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
pub fn source_ml_temporary_read_java_script_descriptor_conversion_state_error_loop62(mut state: self::JavaScriptDescriptorConversionState, mut index: f64) -> self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Result {
    loop {
        return if (index < (6f64)) {
            if self::test_java_script_descriptor_field_presence(match state.clone() {
                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value2051, _) => {
                    source_ml_source1_data_property_value2051.clone()
                }
            }, index) {
                self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Return(self::ml_string::Text::from_units(vec![117, 110, 112, 114, 111, 99, 101, 115, 115, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 116, 116, 114, 105, 98, 117, 116, 101, 115, 32, 99, 97, 110, 110, 111, 116, 32, 98, 101, 32, 102, 97, 98, 114, 105, 99, 97, 116, 101, 100, 32, 105, 110, 32, 97, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
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
            self::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Result::MlTemporaryReadJavaScriptDescriptorConversionStateErrorLoop62Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryStepJavaScriptDescriptorConversionLoop63Result {
    MlTemporaryStepJavaScriptDescriptorConversionLoop63Done(Box<self::JavaScriptDescriptorConversionState>),
    MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box<self::JavaScriptDescriptorConversionTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_step_java_script_descriptor_conversion_loop63(mut heap: self::JavaScriptOrdinaryHeap, mut current: self::JavaScriptDescriptorConversionState) -> self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result {
    loop {
        return if ((match current.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value2053, _, _) => {
                source_ml_source1_data_property_value2053
            }
        }) < (6f64)) {
            {
                let field = self::read_java_script_descriptor_field(match current.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value2054, _, _) => {
                        source_ml_source1_data_property_value2054
                    }
                });
                {
                    let key = self::JavaScriptSemanticValue::Text((field.clone()).2);
                    {
                        let receiver = match current.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source1_data_property_value2055, _, _, _) => {
                                let source_ml_source1_data_property_value2055 = *source_ml_source1_data_property_value2055;
                                source_ml_source1_data_property_value2055.clone()
                            }
                        };
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Boolean(source_ml_source1_f2056) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Number(source_ml_source1_f2057) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Integer(source_ml_source1_f2058) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Text(source_ml_source1_f2059) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_f2060) => {
                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f2061) => {
                                {
                                    let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                    if !((property.clone()).1) {
                                        {
                                            let current_2 = self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), (match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value2062, _, _) => {
                                                    source_ml_source1_data_property_value2062
                                                }
                                            } + 1f64), match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value2063, _) => {
                                                    source_ml_source1_data_property_value2063.clone()
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
                                                self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid((plan.clone()).3)))
                                            } else {
                                                if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorCall(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source1_data_property_value2064, _, _) => {
                                                            source_ml_source1_data_property_value2064
                                                        }
                                                    }, match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source1_data_property_value2065, _) => {
                                                            source_ml_source1_data_property_value2065.clone()
                                                        }
                                                    }, true)), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)))
                                                } else {
                                                    if (((plan.clone()).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                        self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))))
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
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCall(source_ml_source1_f2066, source_ml_source1_f2067, source_ml_source1_f2068, source_ml_source1_f2069) => {
                                                                    let source_ml_source1_f2066 = *source_ml_source1_f2066;
                                                                    let source_ml_source1_f2067 = *source_ml_source1_f2067;
                                                                    let source_ml_source1_f2068 = *source_ml_source1_f2068;
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(source_ml_source1_f2070) => {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(source_ml_source1_f2071) => {
                                                                    let source_ml_source1_f2071 = *source_ml_source1_f2071;
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(source_ml_source1_f2072, source_ml_source1_f2073) => {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(source_ml_source1_f2074) => {
                                                                    self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Return(Box::new(applied.clone()))
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
            self::MlTemporaryStepJavaScriptDescriptorConversionLoop63Result::MlTemporaryStepJavaScriptDescriptorConversionLoop63Done(Box::new(current.clone()))
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_finish_java_script_property_key_conversion_loop64(spelling: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (spelling.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source1_string_unit_operand2075 = spelling.clone();
                    {
                        let source_ml_source1_string_unit_operand2076 = index;
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
                                    let source_ml_source1_string_unit_operand2082 = {
                                        let source_ml_source1_string_range_operand2077 = text.clone();
                                        {
                                            let source_ml_source1_string_range_operand2078 = index_2;
                                            {
                                                let source_ml_source1_string_range_operand2079 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source1_string_range_position2080 = source_ml_source1_string_range_operand2078;
                                                    {
                                                        let source_ml_source1_string_range_position2081 = source_ml_source1_string_range_operand2079;
                                                        source_ml_source1_string_range_operand2077.slice(source_ml_source1_string_range_position2080, source_ml_source1_string_range_position2081)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source1_string_unit_operand2083 = 0f64;
                                        source_ml_source1_string_unit_operand2082.code_point_at(source_ml_source1_string_unit_operand2083)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source1_string_unit_operand2075.clone(), source_ml_source1_string_unit_operand2076)
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
pub enum MlTemporaryReadJavaScriptCallProgramErrorLoop71Result {
    MlTemporaryReadJavaScriptCallProgramErrorLoop71Done,
    MlTemporaryReadJavaScriptCallProgramErrorLoop71Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallProgramErrorLoop73Result {
    MlTemporaryReadJavaScriptCallProgramErrorLoop73Done,
    MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallProgramErrorLoop74Result {
    MlTemporaryReadJavaScriptCallProgramErrorLoop74Done,
    MlTemporaryReadJavaScriptCallProgramErrorLoop74Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_program_error_loop74(mut fn_: self::JavaScriptCallFunction, source_ml_temporary_values69: &[f64], mut source_ml_temporary_index69: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallProgramErrorLoop74Result {
    loop {
        return if (source_ml_temporary_index69 < (self::ml::Big::from_u128(source_ml_temporary_values69.len() as u128))) {
            {
                let register = self::ml_array::at(source_ml_temporary_values69, source_ml_temporary_index69.to_index());
                if !(self::test_java_script_call_coordinate(register, match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2084, _, _) => {
                        source_ml_source1_data_property_value2084
                    }
                })) {
                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop74Result::MlTemporaryReadJavaScriptCallProgramErrorLoop74Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_temporary_index69_2 = source_ml_temporary_index69.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index69 = source_ml_temporary_index69_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallProgramErrorLoop74Result::MlTemporaryReadJavaScriptCallProgramErrorLoop74Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_program_error_loop73(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut fn_: self::JavaScriptCallFunction, mut source_ml_temporary_values68: Vec<self::JavaScriptCallInstruction>, mut source_ml_temporary_index68: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result {
    loop {
        return if (source_ml_temporary_index68 < (self::ml::Big::from_u128(source_ml_temporary_values68.len() as u128))) {
            {
                let instruction = self::ml_array::at(&source_ml_temporary_values68, source_ml_temporary_index68.to_index());
                {
                    let source_ml_temporary_values69 = self::read_java_script_call_registers(instruction.clone());
                    {
                        let source_ml_temporary_index69 = self::ml::Big::from_i128(0);
                        {
                            let source_ml_source1_s2085 = self::source_ml_temporary_read_java_script_call_program_error_loop74(fn_.clone(), &(source_ml_temporary_values69), source_ml_temporary_index69.clone());
                            match source_ml_source1_s2085.clone() {
                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop74Result::MlTemporaryReadJavaScriptCallProgramErrorLoop74Done => {
                                    match instruction.clone() {
                                        self::JavaScriptCallInstruction::Constant(source_ml_source1_f2086, value) => {
                                            let value = *value;
                                            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 110, 115, 116, 97, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index68_2 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index68 = source_ml_temporary_index68_2.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Copy(source_ml_source1_f2087, source_ml_source1_f2088) => {
                                            {
                                                let source_ml_temporary_index68_3 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index68 = source_ml_temporary_index68_3.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Argument(source_ml_source1_f2089, index) => {
                                            if !(self::test_java_script_call_coordinate(index, 1048576f64)) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 97, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index68_4 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index68 = source_ml_temporary_index68_4.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Jump(target) => {
                                            if !(self::test_java_script_call_coordinate(target, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2124) => source_ml_source1_data_property_value2124.as_slice() }).len() as f64))) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 106, 117, 109, 112, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index68_5 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index68 = source_ml_temporary_index68_5.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Branch(source_ml_source1_f2090, yes, no) => {
                                            if (!(self::test_java_script_call_coordinate(yes, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2125) => source_ml_source1_data_property_value2125.as_slice() }).len() as f64))) || !(self::test_java_script_call_coordinate(no, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2126) => source_ml_source1_data_property_value2126.as_slice() }).len() as f64)))) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 98, 114, 97, 110, 99, 104, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 98, 111, 116, 104, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index68_6 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index68 = source_ml_temporary_index68_6.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Call(source_ml_source1_f2091, source_ml_source1_f2092, source_ml_source1_f2093, source_ml_source1_f2094) => {
                                            {
                                                let source_ml_temporary_index68_7 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index68 = source_ml_temporary_index68_7.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Return(source_ml_source1_f2095) => {
                                            {
                                                let source_ml_temporary_index68_8 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index68 = source_ml_temporary_index68_8.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Throw(source_ml_source1_f2096) => {
                                            {
                                                let source_ml_temporary_index68_9 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index68 = source_ml_temporary_index68_9.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PushHandler(source_ml_source1_f2097, target_2) => {
                                            if !(self::test_java_script_call_coordinate(target_2, ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2127) => source_ml_source1_data_property_value2127.as_slice() }).len() as f64))) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index68_10 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index68 = source_ml_temporary_index68_10.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PopHandler => {
                                            {
                                                let source_ml_temporary_index68_11 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index68 = source_ml_temporary_index68_11.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::NumberBinary(source_ml_source1_f2098, source_ml_source1_f2099, source_ml_source1_f2100, operator) => {
                                            if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 100, 100]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]), self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]), self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index68_12 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index68 = source_ml_temporary_index68_12.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::SameValue(source_ml_source1_f2101, source_ml_source1_f2102, source_ml_source1_f2103) => {
                                            {
                                                let source_ml_temporary_index68_13 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index68 = source_ml_temporary_index68_13.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::StrictEqual(source_ml_source1_f2104, source_ml_source1_f2105, source_ml_source1_f2106) => {
                                            {
                                                let source_ml_temporary_index68_14 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index68 = source_ml_temporary_index68_14.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::RequireWrite(source_ml_source1_f2107) => {
                                            {
                                                let source_ml_temporary_index68_15 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index68 = source_ml_temporary_index68_15.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Primitive(source_ml_source1_f2108, operation, arguments) => {
                                            {
                                                let error = self::read_java_script_call_primitive_arity_error(&(operation), (arguments.len() as f64));
                                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(error.clone())
                                                } else {
                                                    {
                                                        let source_ml_temporary_index68_16 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                        {
                                                            source_ml_temporary_index68 = source_ml_temporary_index68_16.clone();
                                                            continue;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryGet(source_ml_source1_f2109, source_ml_source1_f2110, source_ml_source1_f2111, source_ml_source1_f2112) => {
                                            {
                                                let source_ml_temporary_index68_17 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index68 = source_ml_temporary_index68_17.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinarySet(source_ml_source1_f2113, source_ml_source1_f2114, source_ml_source1_f2115, source_ml_source1_f2116, source_ml_source1_f2117) => {
                                            {
                                                let source_ml_temporary_index68_18 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                {
                                                    source_ml_temporary_index68 = source_ml_temporary_index68_18.clone();
                                                    continue;
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PropertyKey(source_ml_source1_f2118, source_ml_source1_f2119, realm) => {
                                            if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2129, _) => source_ml_source1_data_property_value2129.as_slice() }), &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index68_19 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index68 = source_ml_temporary_index68_19.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PropertyDefinition(source_ml_source1_f2120, source_ml_source1_f2121, source_ml_source1_f2122, source_ml_source1_f2123, realm_2) => {
                                            if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2131, _) => source_ml_source1_data_property_value2131.as_slice() }), &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                            } else {
                                                {
                                                    let source_ml_temporary_index68_20 = source_ml_temporary_index68.add(&self::ml::Big::from_i128(1));
                                                    {
                                                        source_ml_temporary_index68 = source_ml_temporary_index68_20.clone();
                                                        continue;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop74Result::MlTemporaryReadJavaScriptCallProgramErrorLoop74Return(source_ml_temporary_result74) => {
                                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(source_ml_temporary_result74.clone())
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallProgramErrorLoop72Result {
    MlTemporaryReadJavaScriptCallProgramErrorLoop72Done,
    MlTemporaryReadJavaScriptCallProgramErrorLoop72Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_program_error_loop72(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_temporary_values67: Vec<self::JavaScriptSemanticValue>, mut source_ml_temporary_index67: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallProgramErrorLoop72Result {
    loop {
        return if (source_ml_temporary_index67 < (self::ml::Big::from_u128(source_ml_temporary_values67.len() as u128))) {
            {
                let capture = self::ml_array::at(&source_ml_temporary_values67, source_ml_temporary_index67.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), capture.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop72Result::MlTemporaryReadJavaScriptCallProgramErrorLoop72Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_temporary_index67_2 = source_ml_temporary_index67.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_temporary_index67 = source_ml_temporary_index67_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallProgramErrorLoop72Result::MlTemporaryReadJavaScriptCallProgramErrorLoop72Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_program_error_loop71(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut count: f64, mut source_ml_temporary_values66: Vec<self::JavaScriptCallFunction>, mut source_ml_temporary_index66: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result {
    loop {
        return if (source_ml_temporary_index66 < (self::ml::Big::from_u128(source_ml_temporary_values66.len() as u128))) {
            {
                let fn_ = self::ml_array::at(&source_ml_temporary_values66, source_ml_temporary_index66.to_index());
                if (((match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value2133, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2133.clone()
                    }
                }) <= previous) || ((match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value2135, _, _, _, _, _, _, _) => {
                        source_ml_source1_data_property_value2135.clone()
                    }
                }) > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value2136, _, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2136.clone()
                            }
                        };
                        if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source1_data_property_value2138, _) => source_ml_source1_data_property_value2138.as_slice() }), &match fn_.clone() {
                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source1_data_property_value2139, _, _, _, _, _, _) => {
                                source_ml_source1_data_property_value2139.clone()
                            }
                        }, 0f64, false, |a, b| a == b)) {
                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))
                        } else {
                            {
                                let callable = self::read_java_script_ordinary_value_error(heap.clone(), self::JavaScriptSemanticValue::Object(match fn_.clone() {
                                    self::JavaScriptCallFunction::CallFunction(source_ml_source1_data_property_value2140, _, _, _, _, _, _, _) => {
                                        source_ml_source1_data_property_value2140.clone()
                                    }
                                }, true));
                                if (callable != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Return(callable.clone())
                                } else {
                                    if (((match fn_.clone() {
                                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source1_data_property_value2142, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2142.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) && ((match fn_.clone() {
                                        self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source1_data_property_value2144, _, _, _, _, _) => {
                                            source_ml_source1_data_property_value2144.clone()
                                        }
                                    }) != (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])))) {
                                        self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 105, 99, 116, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103]))
                                    } else {
                                        if ((self::read_java_script_ordinary_value_error(heap.clone(), match fn_.clone() {
                                            self::JavaScriptCallFunction::CallFunction(_, _, _, source_ml_source1_data_property_value2146, _, _, _, _) => {
                                                let source_ml_source1_data_property_value2146 = *source_ml_source1_data_property_value2146;
                                                source_ml_source1_data_property_value2146.clone()
                                            }
                                        })) != (self::ml_string::Text::from_units(vec![]))) {
                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Return(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                        } else {
                                            if (((!(self::test_java_script_call_coordinate(match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2147, _, _) => {
                                                    source_ml_source1_data_property_value2147
                                                }
                                            }, 4097f64)) || ((match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2149, _, _) => {
                                                    source_ml_source1_data_property_value2149
                                                }
                                            }) == (0f64))) || !(self::test_java_script_call_coordinate(match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source1_data_property_value2150, _, _, _) => {
                                                    source_ml_source1_data_property_value2150
                                                }
                                            }, match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2151, _, _) => {
                                                    source_ml_source1_data_property_value2151
                                                }
                                            }))) || ((((match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source1_data_property_value2155, _, _, _) => {
                                                    source_ml_source1_data_property_value2155
                                                }
                                            } + ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source1_data_property_value2156, _) => source_ml_source1_data_property_value2156.as_slice() }).len() as f64)) + 1f64)) > (match fn_.clone() {
                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2157, _, _) => {
                                                    source_ml_source1_data_property_value2157
                                                }
                                            }))) {
                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101]))
                                            } else {
                                                {
                                                    let source_ml_temporary_values67 = match fn_.clone() {
                                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source1_data_property_value2158, _) => {
                                                            source_ml_source1_data_property_value2158
                                                        }
                                                    };
                                                    {
                                                        let source_ml_temporary_index67 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source1_s2159 = self::source_ml_temporary_read_java_script_call_program_error_loop72(heap.clone(), source_ml_temporary_values67, source_ml_temporary_index67.clone());
                                                            match source_ml_source1_s2159.clone() {
                                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop72Result::MlTemporaryReadJavaScriptCallProgramErrorLoop72Done => {
                                                                    {
                                                                        let count_2 = (count + ((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2160) => source_ml_source1_data_property_value2160.as_slice() }).len() as f64));
                                                                        if ((count_2 > (65536f64)) || ((((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2162) => source_ml_source1_data_property_value2162.as_slice() }).len() as f64)) == (0f64))) {
                                                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116, 32, 111, 114, 32, 104, 97, 115, 32, 110, 111, 32, 101, 110, 116, 114, 121]))
                                                                        } else {
                                                                            {
                                                                                let source_ml_temporary_values68 = match fn_.clone() {
                                                                                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source1_data_property_value2163) => {
                                                                                        source_ml_source1_data_property_value2163
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_temporary_index68 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let source_ml_source1_s2164 = self::source_ml_temporary_read_java_script_call_program_error_loop73(program.clone(), heap.clone(), fn_.clone(), source_ml_temporary_values68, source_ml_temporary_index68.clone());
                                                                                        match source_ml_source1_s2164.clone() {
                                                                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Done => {
                                                                                                {
                                                                                                    let source_ml_temporary_index66_2 = source_ml_temporary_index66.add(&self::ml::Big::from_i128(1));
                                                                                                    {
                                                                                                        (previous, count, source_ml_temporary_index66) = (previous_2.clone(), count_2, source_ml_temporary_index66_2.clone());
                                                                                                        continue;
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                            self::MlTemporaryReadJavaScriptCallProgramErrorLoop73Result::MlTemporaryReadJavaScriptCallProgramErrorLoop73Return(source_ml_temporary_result73) => {
                                                                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Return(source_ml_temporary_result73.clone())
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlTemporaryReadJavaScriptCallProgramErrorLoop72Result::MlTemporaryReadJavaScriptCallProgramErrorLoop72Return(source_ml_temporary_result74) => {
                                                                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Return(source_ml_temporary_result74.clone())
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
            self::MlTemporaryReadJavaScriptCallProgramErrorLoop71Result::MlTemporaryReadJavaScriptCallProgramErrorLoop71Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallProgramErrorLoop70Result {
    MlTemporaryReadJavaScriptCallProgramErrorLoop70Done(self::ml::Big),
    MlTemporaryReadJavaScriptCallProgramErrorLoop70Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_call_program_error_loop70(mut previous: self::ml::Big, mut source_ml_temporary_values65: Vec<self::ml::Big>, mut source_ml_temporary_index65: self::ml::Big) -> self::MlTemporaryReadJavaScriptCallProgramErrorLoop70Result {
    loop {
        return if (source_ml_temporary_index65 < (self::ml::Big::from_u128(source_ml_temporary_values65.len() as u128))) {
            {
                let realm = self::ml_array::at(&source_ml_temporary_values65, source_ml_temporary_index65.to_index());
                if ((realm <= previous) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlTemporaryReadJavaScriptCallProgramErrorLoop70Result::MlTemporaryReadJavaScriptCallProgramErrorLoop70Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 97, 108, 109, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let previous_2 = realm.clone();
                        {
                            let source_ml_temporary_index65_2 = source_ml_temporary_index65.add(&self::ml::Big::from_i128(1));
                            {
                                (previous, source_ml_temporary_index65) = (previous_2.clone(), source_ml_temporary_index65_2.clone());
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptCallProgramErrorLoop70Result::MlTemporaryReadJavaScriptCallProgramErrorLoop70Done(previous.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_call_frame_loop75(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source1_data_property_value2166, _, _) => {
                source_ml_source1_data_property_value2166
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
pub fn source_ml_temporary_create_java_script_call_frame_loop76(mut fn_: self::JavaScriptCallFunction, mut arguments_: Vec<self::JavaScriptSemanticValue>, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source1_data_property_value2168, _, _, _) => {
                source_ml_source1_data_property_value2168
            }
        })) {
            {
                let value = (self::ml_array::optional_at(&arguments_, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let slot = (index + 1f64);
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_create_java_script_call_frame_loop77(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source1_data_property_value2170, _) => source_ml_source1_data_property_value2170.as_slice() }).len() as f64))) {
            {
                let value = (self::ml_array::optional_at((match &fn_ { self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source1_data_property_value2171, _) => source_ml_source1_data_property_value2171.as_slice() }), self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let slot = ((match fn_.clone() {
                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source1_data_property_value2172, _, _, _) => {
                            source_ml_source1_data_property_value2172
                        }
                    } + 1f64) + index);
                    {
                        let registers_2 = { let ml_values10 = registers; let ml_callback10 = { let slot = slot; let value = value.clone(); std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, position: f64| -> self::JavaScriptSemanticValue { if (position == slot) {
                            value.clone()
                        } else {
                            entry.clone()
                        } }) }; ml_values10.iter().cloned().enumerate().map(|(ml_index10, ml_value10)| ml_callback10(ml_value10, (ml_index10 as f64))).collect::<Vec<_>>() };
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
pub enum MlTemporaryReadJavaScriptCallMachineErrorLoop81Result {
    MlTemporaryReadJavaScriptCallMachineErrorLoop81Done,
    MlTemporaryReadJavaScriptCallMachineErrorLoop81Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallMachineErrorJoin86 {
    MlTemporaryReadJavaScriptCallMachineErrorJoin86Next,
    MlTemporaryReadJavaScriptCallMachineErrorJoin86Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallMachineErrorJoin85 {
    MlTemporaryReadJavaScriptCallMachineErrorJoin85Next,
    MlTemporaryReadJavaScriptCallMachineErrorJoin85Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCallMachineErrorLoop84Result {
    MlTemporaryReadJavaScriptCallMachineErrorLoop84Done,
    MlTemporaryReadJavaScriptCallMachineErrorLoop84Return(self::ml_string::Text),
}
