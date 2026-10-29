// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallProgram;
use super::JavaScriptDescriptorConversionState;
use super::JavaScriptDescriptorConversionTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result;
use super::apply_java_script_descriptor_field;
use super::find_java_script_call_function;
use super::locate_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_call_frame_error;
use super::read_java_script_call_primitive_arity_error;
use super::read_java_script_call_registers;
use super::read_java_script_descriptor_field;
use super::read_java_script_ordinary_value_error;
use super::test_java_script_call_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_descriptor_conversion_loop125(mut heap: self::JavaScriptOrdinaryHeap, mut current: self::JavaScriptDescriptorConversionState) -> self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result {
    loop {
        return if ((match current.clone() {
            self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2761, _, _) => {
                source_ml_source2_data_property_value2761
            }
        }) < (6f64)) {
            {
                let field = self::read_java_script_descriptor_field(match current.clone() {
                    self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2762, _, _) => {
                        source_ml_source2_data_property_value2762
                    }
                });
                {
                    let key = self::JavaScriptSemanticValue::Text((field.clone()).2);
                    {
                        let receiver = match current.clone() {
                            self::JavaScriptDescriptorConversionState::DescriptorConversion(source_ml_source2_data_property_value2763, _, _, _) => {
                                let source_ml_source2_data_property_value2763 = *source_ml_source2_data_property_value2763;
                                source_ml_source2_data_property_value2763.clone()
                            }
                        };
                        match receiver.clone() {
                            self::JavaScriptSemanticValue::Undefined => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Null => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Boolean(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Number(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Integer(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Text(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Symbol(_) => {
                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114]))))
                            }
                            self::JavaScriptSemanticValue::Object(identity, _) => {
                                {
                                    let property = self::locate_java_script_ordinary_property(heap.clone(), identity.clone(), key.clone());
                                    if !((property.clone()).1) {
                                        {
                                            let current_2 = self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), (match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2770, _, _) => {
                                                    source_ml_source2_data_property_value2770
                                                }
                                            } + 1f64), match current.clone() {
                                                self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2771, _) => {
                                                    source_ml_source2_data_property_value2771.clone()
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
                                                self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid((plan.clone()).3)))
                                            } else {
                                                if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorCall(Box::new(self::JavaScriptDescriptorConversionState::DescriptorConversion(Box::new(receiver.clone()), match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, source_ml_source2_data_property_value2772, _, _) => {
                                                            source_ml_source2_data_property_value2772
                                                        }
                                                    }, match current.clone() {
                                                        self::JavaScriptDescriptorConversionState::DescriptorConversion(_, _, source_ml_source2_data_property_value2773, _) => {
                                                            source_ml_source2_data_property_value2773.clone()
                                                        }
                                                    }, true)), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)))
                                                } else {
                                                    if (((plan.clone()).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                        self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108]))))
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
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorCompleted(_) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorThrow(_) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorIntrinsicError(_, _) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(applied.clone()))
                                                                }
                                                                self::JavaScriptDescriptorConversionTransition::DescriptorInvalid(_) => {
                                                                    self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Return(Box::new(applied.clone()))
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
            self::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Result::MlSource1TemporaryStepJavaScriptDescriptorConversionLoop125Done(Box::new(current.clone()))
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_finish_java_script_property_key_conversion_loop126(spelling: &self::ml_string::Text, mut units: Vec<f64>, mut index: f64) -> Vec<f64> {
    loop {
        return if (index < (spelling.length())) {
            {
                let units_2 = { let (ml_append_right, ml_append_left) = (vec![{
                    let source_ml_source2_string_unit_operand2783 = spelling.clone();
                    {
                        let source_ml_source2_string_unit_operand2784 = index;
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
                                    let source_ml_source2_string_unit_operand2790 = {
                                        let source_ml_source2_string_range_operand2785 = text.clone();
                                        {
                                            let source_ml_source2_string_range_operand2786 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand2787 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position2788 = source_ml_source2_string_range_operand2786;
                                                    {
                                                        let source_ml_source2_string_range_position2789 = source_ml_source2_string_range_operand2787;
                                                        source_ml_source2_string_range_operand2785.slice(source_ml_source2_string_range_position2788, source_ml_source2_string_range_position2789)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand2791 = 0f64;
                                        source_ml_source2_string_unit_operand2790.code_point_at(source_ml_source2_string_unit_operand2791)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        } }))(source_ml_source2_string_unit_operand2783.clone(), source_ml_source2_string_unit_operand2784)
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
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop136(mut fn_: self::JavaScriptCallFunction, source_ml_source1_temporary_values131: &[f64], mut source_ml_source1_temporary_index131: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Result {
    loop {
        return if (source_ml_source1_temporary_index131 < (self::ml::Big::from_u128(source_ml_source1_temporary_values131.len() as u128))) {
            {
                let register = self::ml_array::at(source_ml_source1_temporary_values131, source_ml_source1_temporary_index131.to_index());
                if !(self::test_java_script_call_coordinate(register, match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2792, _, _) => {
                        source_ml_source2_data_property_value2792
                    }
                })) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index131_2 = source_ml_source1_temporary_index131.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index131 = source_ml_source1_temporary_index131_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop135(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut fn_: self::JavaScriptCallFunction, mut source_ml_source1_temporary_values130: Vec<self::JavaScriptCallInstruction>, mut source_ml_source1_temporary_index130: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result {
    loop {
        return match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, shared_constructor_field2) => {
                match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(_, shared_constructor_field1, _) => {
                        if (source_ml_source1_temporary_index130 < (self::ml::Big::from_u128(source_ml_source1_temporary_values130.len() as u128))) {
                            {
                                let instruction = self::ml_array::at(&source_ml_source1_temporary_values130, source_ml_source1_temporary_index130.to_index());
                                {
                                    let source_ml_source1_temporary_values131 = self::read_java_script_call_registers(instruction.clone());
                                    {
                                        let source_ml_source1_temporary_index131 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s2793 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop136(fn_.clone(), &(source_ml_source1_temporary_values131), source_ml_source1_temporary_index131.clone());
                                            match source_ml_source2_s2793.clone() {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Done => {
                                                    match instruction.clone() {
                                                        self::JavaScriptCallInstruction::Constant(_, value) => {
                                                            let value = *value;
                                                            if ((self::read_java_script_ordinary_value_error(heap.clone(), value.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 110, 115, 116, 97, 110, 116, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
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
                                                        self::JavaScriptCallInstruction::Copy(_, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index130_3 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_3.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Argument(_, index) => {
                                                            if !(self::test_java_script_call_coordinate(index, 1048576f64)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 97, 100, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 101, 120, 97, 99, 116, 32, 98, 111, 117, 110, 100, 101, 100, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index130_4 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_4.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Jump(target) => {
                                                            if !(self::test_java_script_call_coordinate(target, (shared_constructor_field2.len() as f64))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 106, 117, 109, 112, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index130_5 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_5.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Branch(_, yes, no) => {
                                                            if (!(self::test_java_script_call_coordinate(yes, (shared_constructor_field2.len() as f64))) || !(self::test_java_script_call_coordinate(no, (shared_constructor_field2.len() as f64)))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 98, 114, 97, 110, 99, 104, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 98, 111, 116, 104, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index130_6 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_6.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index130_7 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_7.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Return(_) => {
                                                            {
                                                                let source_ml_source1_temporary_index130_8 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_8.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Throw(_) => {
                                                            {
                                                                let source_ml_source1_temporary_index130_9 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_9.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PushHandler(_, target_2) => {
                                                            if !(self::test_java_script_call_coordinate(target_2, (shared_constructor_field2.len() as f64))) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 104, 97, 110, 100, 108, 101, 114, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 116, 97, 114, 103, 101, 116, 115]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index130_10 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_10.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PopHandler => {
                                                            {
                                                                let source_ml_source1_temporary_index130_11 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_11.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::NumberBinary(_, _, _, operator) => {
                                                            if !(self::ml_array::includes(&vec![self::ml_string::Text::from_units(vec![97, 100, 100]), self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]), self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]), self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]), self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]), self::ml_string::Text::from_units(vec![112, 111, 119, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]), self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])], &operator.clone(), 0f64, false, |a, b| a == b)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index130_12 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_12.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index130_13 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_13.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index130_14 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_14.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                            {
                                                                let source_ml_source1_temporary_index130_15 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_15.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::Primitive(_, operation, arguments) => {
                                                            {
                                                                let error = self::read_java_script_call_primitive_arity_error(&(operation), (arguments.len() as f64));
                                                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(error.clone())
                                                                } else {
                                                                    {
                                                                        let source_ml_source1_temporary_index130_16 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                        {
                                                                            source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_16.clone();
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index130_17 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_17.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                            {
                                                                let source_ml_source1_temporary_index130_18 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                {
                                                                    source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_18.clone();
                                                                    continue;
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PropertyKey(_, _, realm) => {
                                                            if !(self::ml_array::includes(&shared_constructor_field1, &realm.clone(), 0f64, false, |a, b| a == b)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index130_19 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_19.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, realm_2) => {
                                                            if !(self::ml_array::includes(&shared_constructor_field1, &realm_2.clone(), 0f64, false, |a, b| a == b)) {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(self::ml_string::Text::from_units(vec![97, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 114, 101, 97, 108, 109]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index130_20 = source_ml_source1_temporary_index130.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index130 = source_ml_source1_temporary_index130_20.clone();
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop136Return(source_ml_source1_temporary_result136) => {
                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(source_ml_source1_temporary_result136.clone())
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Done
                        }
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop134(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_source1_temporary_values129: Vec<self::JavaScriptSemanticValue>, mut source_ml_source1_temporary_index129: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result {
    loop {
        return if (source_ml_source1_temporary_index129 < (self::ml::Big::from_u128(source_ml_source1_temporary_values129.len() as u128))) {
            {
                let capture = self::ml_array::at(&source_ml_source1_temporary_values129, source_ml_source1_temporary_index129.to_index());
                if ((self::read_java_script_ordinary_value_error(heap.clone(), capture.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
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
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop133(mut program: self::JavaScriptCallProgram, mut heap: self::JavaScriptOrdinaryHeap, mut previous: self::ml::Big, mut count: f64, mut source_ml_source1_temporary_values128: Vec<self::JavaScriptCallFunction>, mut source_ml_source1_temporary_index128: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result {
    loop {
        return if (source_ml_source1_temporary_index128 < (self::ml::Big::from_u128(source_ml_source1_temporary_values128.len() as u128))) {
            {
                let fn_ = self::ml_array::at(&source_ml_source1_temporary_values128, source_ml_source1_temporary_index128.to_index());
                match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8) => {
                        let shared_constructor_field4 = *shared_constructor_field4;
                        if ((shared_constructor_field1 <= previous) || (shared_constructor_field1 > (self::ml::Big::from_i128(18446744073709551615)))) {
                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                        } else {
                            {
                                let previous_2 = shared_constructor_field1.clone();
                                if !(self::ml_array::includes((match &program { self::JavaScriptCallProgram::CallProgram(_, source_ml_source2_data_property_value2846, _) => source_ml_source2_data_property_value2846.as_slice() }), &shared_constructor_field2.clone(), 0f64, false, |a, b| a == b)) {
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 114, 101, 97, 108, 109]))
                                } else {
                                    {
                                        let callable = self::read_java_script_ordinary_value_error(heap.clone(), self::JavaScriptSemanticValue::Object(shared_constructor_field1.clone(), true));
                                        if (callable != (self::ml_string::Text::from_units(vec![]))) {
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(callable.clone())
                                        } else {
                                            if ((shared_constructor_field3 != (self::ml_string::Text::from_units(vec![115, 116, 114, 105, 99, 116]))) && (shared_constructor_field3 != (self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108])))) {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 115, 116, 114, 105, 99, 116, 32, 111, 114, 32, 108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 98, 105, 110, 100, 105, 110, 103]))
                                            } else {
                                                if ((self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field4.clone())) != (self::ml_string::Text::from_units(vec![]))) {
                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                                                } else {
                                                    if (((!(self::test_java_script_call_coordinate(shared_constructor_field6, 4097f64)) || (shared_constructor_field6 == (0f64))) || !(self::test_java_script_call_coordinate(shared_constructor_field5, shared_constructor_field6))) || ((((shared_constructor_field5 + (shared_constructor_field7.len() as f64)) + 1f64)) > shared_constructor_field6)) {
                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 112, 97, 114, 97, 109, 101, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101, 116, 101]))
                                                    } else {
                                                        {
                                                            let source_ml_source1_temporary_values129 = shared_constructor_field7;
                                                            {
                                                                let source_ml_source1_temporary_index129 = self::ml::Big::from_i128(0);
                                                                {
                                                                    let source_ml_source2_s2867 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop134(heap.clone(), source_ml_source1_temporary_values129, source_ml_source1_temporary_index129.clone());
                                                                    match source_ml_source2_s2867.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Done => {
                                                                            {
                                                                                let count_2 = (count + (shared_constructor_field8.len() as f64));
                                                                                if ((count_2 > (65536f64)) || (((shared_constructor_field8.len() as f64)) == (0f64))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116, 32, 111, 114, 32, 104, 97, 115, 32, 110, 111, 32, 101, 110, 116, 114, 121]))
                                                                                } else {
                                                                                    {
                                                                                        let source_ml_source1_temporary_values130 = shared_constructor_field8;
                                                                                        {
                                                                                            let source_ml_source1_temporary_index130 = self::ml::Big::from_i128(0);
                                                                                            {
                                                                                                let source_ml_source2_s2872 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop135(program.clone(), heap.clone(), fn_.clone(), source_ml_source1_temporary_values130, source_ml_source1_temporary_index130.clone());
                                                                                                match source_ml_source2_s2872.clone() {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Done => {
                                                                                                        {
                                                                                                            let source_ml_source1_temporary_index128_2 = source_ml_source1_temporary_index128.add(&self::ml::Big::from_i128(1));
                                                                                                            {
                                                                                                                (previous, count, source_ml_source1_temporary_index128) = (previous_2.clone(), count_2, source_ml_source1_temporary_index128_2.clone());
                                                                                                                continue;
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop135Return(source_ml_source1_temporary_result135) => {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(source_ml_source1_temporary_result135.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop134Return(source_ml_source1_temporary_result136) => {
                                                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Return(source_ml_source1_temporary_result136.clone())
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
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop133Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop132(mut previous: self::ml::Big, mut source_ml_source1_temporary_values127: Vec<self::ml::Big>, mut source_ml_source1_temporary_index127: self::ml::Big) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result {
    loop {
        return if (source_ml_source1_temporary_index127 < (self::ml::Big::from_u128(source_ml_source1_temporary_values127.len() as u128))) {
            {
                let realm = self::ml_array::at(&source_ml_source1_temporary_values127, source_ml_source1_temporary_index127.to_index());
                if ((realm <= previous) || (realm > (self::ml::Big::from_i128(18446744073709551615)))) {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 97, 108, 109, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index127_2 = source_ml_source1_temporary_index127.add(&self::ml::Big::from_i128(1));
                        {
                            (previous, source_ml_source1_temporary_index127) = (realm.clone(), source_ml_source1_temporary_index127_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop132Done(previous.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop137(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2874, _, _) => {
                source_ml_source2_data_property_value2874
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
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop138(mut fn_: self::JavaScriptCallFunction, mut arguments_: Vec<self::JavaScriptSemanticValue>, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return if (index < (match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source2_data_property_value2876, _, _, _) => {
                source_ml_source2_data_property_value2876
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
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop139(mut fn_: self::JavaScriptCallFunction, mut registers: Vec<self::JavaScriptSemanticValue>, mut index: f64) -> Vec<self::JavaScriptSemanticValue> {
    loop {
        return match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(_, _, _, _, shared_constructor_field1, _, shared_constructor_field2, _) => {
                if (index < ((shared_constructor_field2.len() as f64))) {
                    {
                        let value = (self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let slot = ((shared_constructor_field1 + 1f64) + index);
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
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140 {
    MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140Next,
    MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin140Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141 {
    MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141Next,
    MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin141Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop142(mut program: self::JavaScriptCallProgram, mut machine: self::JavaScriptCallMachine, mut slots: f64, mut argument_slots: f64, mut index: f64) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result {
    loop {
        return match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
                let shared_constructor_field1 = *shared_constructor_field1;
                if (index < ((shared_constructor_field2.len() as f64))) {
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(index));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                        } else {
                            {
                                let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value2884, _, _) => source_ml_source2_data_property_value2884.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2885, _, _, _, _, _) => {
                                        source_ml_source2_data_property_value2885.clone()
                                    }
                                })));
                                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                } else {
                                    if (((((match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2888, _, _, _) => {
                                            source_ml_source2_data_property_value2888
                                        }
                                    }).len() as f64)) != (match fn_.clone().expect("a proven present value") {
                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value2889, _, _) => {
                                            source_ml_source2_data_property_value2889
                                        }
                                    })) || !(self::test_java_script_call_coordinate(match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2890, _, _, _, _) => {
                                            source_ml_source2_data_property_value2890
                                        }
                                    }, (((match fn_.clone().expect("a proven present value") {
                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2891) => {
                                            source_ml_source2_data_property_value2891
                                        }
                                    }).len() as f64) + 1f64)))) {
                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                    } else {
                                        {
                                            let slots_2 = (slots + ((match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2892, _, _, _) => {
                                                    source_ml_source2_data_property_value2892
                                                }
                                            }).len() as f64));
                                            {
                                                let argument_slots_2 = (argument_slots + ((match frame.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2893, _, _) => {
                                                        source_ml_source2_data_property_value2893
                                                    }
                                                }).len() as f64));
                                                if (((slots_2 > (1048576f64)) || (argument_slots_2 > (1048576f64))) || ((((match frame.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2895, _) => {
                                                        source_ml_source2_data_property_value2895
                                                    }
                                                }).len() as f64)) > (1024f64))) {
                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                } else {
                                                    {
                                                        let frame_error = self::read_java_script_call_frame_error(program.clone(), shared_constructor_field1.clone(), frame.clone().expect("a proven present value"), fn_.clone().expect("a proven present value"), (index < (((shared_constructor_field2.len() as f64) - 1f64))));
                                                        if (frame_error != (self::ml_string::Text::from_units(vec![]))) {
                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Return(frame_error.clone())
                                                        } else {
                                                            {
                                                                let index_2 = (index + 1f64);
                                                                {
                                                                    (slots, argument_slots, index) = (slots_2, argument_slots_2, index_2);
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
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop142Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorJoin149 {
    MlSource1TemporaryReadJavaScriptCallFrameErrorJoin149Next,
    MlSource1TemporaryReadJavaScriptCallFrameErrorJoin149Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorJoin150 {
    MlSource1TemporaryReadJavaScriptCallFrameErrorJoin150Next,
    MlSource1TemporaryReadJavaScriptCallFrameErrorJoin150Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151 {
    MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Next,
    MlSource1TemporaryReadJavaScriptCallFrameErrorJoin151Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Result {
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Done,
    MlSource1TemporaryReadJavaScriptCallFrameErrorLoop148Return(self::ml_string::Text),
}
