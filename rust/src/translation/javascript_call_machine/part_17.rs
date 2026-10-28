// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryBeginJavaScriptCallMachineLoop151Result;
use super::accept_java_script_whitespace_unit;
use super::create_java_script_call_frame;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_call_machine_error;
use super::read_java_script_call_program_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_begin_java_script_call_machine_loop151;
use super::throw_java_script_call_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_call_machine(program: self::JavaScriptCallProgram, heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallMachineTransition {
    {
        let error = self::read_java_script_call_program_error(program.clone(), heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), error.clone())
        } else {
            {
                let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value2081, _, _) => source_ml_source2_data_property_value2081.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
                if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 101, 110, 116, 114, 121, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 101, 99, 117, 116, 97, 98, 108, 101, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110]))
                } else {
                    if (((arguments_.len() as f64)) > (1048576f64)) {
                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![101, 110, 116, 114, 121, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 99, 97, 108, 108, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                    } else {
                        {
                            let receiver_error = self::read_java_script_ordinary_value_error(heap.clone(), receiver.clone());
                            if (receiver_error != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), receiver_error.clone())
                            } else {
                                {
                                    let source_ml_source1_temporary_values150 = arguments_.clone();
                                    {
                                        let source_ml_source1_temporary_index150 = self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s2082 = self::source_ml_source1_temporary_begin_java_script_call_machine_loop151(heap.clone(), source_ml_source1_temporary_values150, source_ml_source1_temporary_index150.clone());
                                            match source_ml_source2_s2082.clone() {
                                                self::MlSource1TemporaryBeginJavaScriptCallMachineLoop151Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop151Done => {
                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)])))
                                                }
                                                self::MlSource1TemporaryBeginJavaScriptCallMachineLoop151Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop151Return(source_ml_source1_temporary_result151) => {
                                                    let source_ml_source1_temporary_result151 = *source_ml_source1_temporary_result151;
                                                    source_ml_source1_temporary_result151.clone()
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_error_allocation(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, realm: self::ml::Big, value: self::JavaScriptSemanticValue) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let error = self::read_java_script_call_machine_error(program.clone(), machine.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), error.clone())
                } else {
                    {
                        let frame = self::ml_array::optional_at(&shared_constructor_field2, self::ml_array::number_index(((shared_constructor_field2.len() as f64) - 1f64)));
                        if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                        } else {
                            if {
                                let source_ml_source2_s2088 = match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2104) => {
                                        let source_ml_source2_data_property_value2104 = *source_ml_source2_data_property_value2104;
                                        source_ml_source2_data_property_value2104.clone()
                                    }
                                };
                                match source_ml_source2_s2088.clone() {
                                    self::JavaScriptCallPending::WaitError(_, _, _) => {
                                        false
                                    }
                                    _ => {
                                        true
                                    }
                                }
                            } {
                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))
                            } else {
                                if {
                                    let source_ml_source2_comparison_operand2111 = match match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2106) => {
                                            let source_ml_source2_data_property_value2106 = *source_ml_source2_data_property_value2106;
                                            source_ml_source2_data_property_value2106.clone()
                                        }
                                    } {
                                        self::JavaScriptCallPending::CallIdle => {
                                            None::<self::ml::Big>
                                        }
                                        self::JavaScriptCallPending::WaitCall(_) => {
                                            None::<self::ml::Big>
                                        }
                                        self::JavaScriptCallPending::WaitGet(_) => {
                                            None::<self::ml::Big>
                                        }
                                        self::JavaScriptCallPending::WaitSet(_) => {
                                            None::<self::ml::Big>
                                        }
                                        self::JavaScriptCallPending::WaitKey(_, source_ml_source2_data_property_value2107, _) => {
                                            Some(source_ml_source2_data_property_value2107.clone())
                                        }
                                        self::JavaScriptCallPending::WaitDefinition(_, source_ml_source2_data_property_value2108, _) => {
                                            Some(source_ml_source2_data_property_value2108.clone())
                                        }
                                        self::JavaScriptCallPending::WaitNumber(_, source_ml_source2_data_property_value2109, _) => {
                                            Some(source_ml_source2_data_property_value2109.clone())
                                        }
                                        self::JavaScriptCallPending::WaitError(source_ml_source2_data_property_value2110, _, _) => {
                                            Some(source_ml_source2_data_property_value2110.clone())
                                        }
                                    };
                                    {
                                        let source_ml_source2_comparison_operand2112 = realm.clone();
                                        !(match (source_ml_source2_comparison_operand2111.clone(), source_ml_source2_comparison_operand2112.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
                                    }
                                } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 114, 101, 97, 108, 109]))
                                } else {
                                    {
                                        let value_error = self::read_java_script_ordinary_value_error(shared_constructor_field1.clone(), value.clone());
                                        if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), value_error.clone())
                                        } else {
                                            match value.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Number(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Text(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                }
                                                self::JavaScriptSemanticValue::Object(_, callable) => {
                                                    if callable {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                    } else {
                                                        self::throw_java_script_call_value(machine.clone(), value.clone())
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop1(units: &[f64], mut at: f64) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if (at < ((units.len() as f64))) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64))) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
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
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_semantic_same_value_loop2(left_units: &[f64], right_units: &[f64], mut at: f64) -> self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if (at < ((left_units.len() as f64))) {
            if (((self::ml_array::optional_at(left_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(right_units, self::ml_array::number_index(at))).clone().unwrap_or_else(|| (-1f64)))) {
                self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
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
            self::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Result::MlSource1TemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop6(mut count: self::ml::Big, mut value: self::ml::Big, mut factor: self::ml::Big) -> self::ml::Big {
    loop {
        return if (count > (self::ml::Big::from_i128(0))) {
            {
                let value_3 = if ((self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let value_2 = value.mul(&factor);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let count_2 = self::ml::divide(&count, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (count_2 > (self::ml::Big::from_i128(0))) {
                        {
                            let factor_2 = factor.mul(&factor);
                            {
                                (count, value, factor) = (count_2.clone(), value_3.clone(), factor_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (count, value) = (count_2.clone(), value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop7(mut count: self::ml::Big, mut value: self::ml::Big) -> self::ml::Big {
    loop {
        return if (((count > (self::ml::Big::from_i128(0))) && (value != (self::ml::Big::from_i128(0)))) && (value != (self::ml::Big::from_i128(-1)))) {
            {
                let value_2 = if (value < (self::ml::Big::from_i128(0))) {
                    self::ml::divide(&value.sub(&self::ml::Big::from_i128(1)), &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                } else {
                    self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false)
                };
                {
                    let count_2 = count.sub(&self::ml::Big::from_i128(1));
                    {
                        (count, value) = (count_2.clone(), value_2.clone());
                        continue;
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5 {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationJoin5Next(self::ml::Big, bool),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Result {
    MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_big_integer_bitwise_operation_loop8(mut operation: f64, mut negative_left: bool, mut negative_right: bool, mut first: self::ml::Big, mut second: self::ml::Big, mut weight: self::ml::Big, mut result: self::ml::Big) -> self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Result {
    loop {
        return if ((first > (self::ml::Big::from_i128(0))) || (second > (self::ml::Big::from_i128(0)))) {
            {
                let a = if negative_left {
                    (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                } else {
                    self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                };
                {
                    let b = if negative_right {
                        (self::ml::Big::from_i128(1)).sub(&self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true))
                    } else {
                        self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)
                    };
                    {
                        let selected = if (operation == (0f64)) {
                            ((a == (self::ml::Big::from_i128(1))) && (b == (self::ml::Big::from_i128(1))))
                        } else {
                            if (operation == (1f64)) {
                                ((a == (self::ml::Big::from_i128(1))) || (b == (self::ml::Big::from_i128(1))))
                            } else {
                                (a != b)
                            }
                        };
                        {
                            let result_3 = if selected {
                                {
                                    let result_2 = result.add(&weight);
                                    result_2.clone()
                                }
                            } else {
                                result.clone()
                            };
                            {
                                let first_2 = self::ml::divide(&first, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                {
                                    let second_2 = self::ml::divide(&second, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                                    {
                                        let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                                        {
                                            (first, second, weight, result) = (first_2.clone(), second_2.clone(), weight_2.clone(), result_3.clone());
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Result::MlSource1TemporaryApplyBigIntegerBitwiseOperationLoop8Done(weight.clone(), result.clone())
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop10(mut count: f64, mut factor: f64, mut position: f64) -> f64 {
    loop {
        return if (position < count) {
            {
                let factor_2 = (factor * 2f64);
                {
                    let position_2 = (position + 1f64);
                    {
                        (factor, position) = (factor_2, position_2);
                        continue;
                    }
                }
            }
        } else {
            factor
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_apply_java_script_number_bitwise_operation_loop11(mut operation: f64, mut first: f64, mut second: f64, mut result: f64, mut weight: f64, mut position: f64) -> f64 {
    loop {
        return if (position < (32f64)) {
            {
                let first_bit = (first % 2f64);
                {
                    let second_bit = (second % 2f64);
                    {
                        let result_3 = if (((((operation == (0f64)) && (first_bit == (1f64))) && (second_bit == (1f64))) || ((operation == (1f64)) && ((first_bit == (1f64)) || (second_bit == (1f64))))) || ((operation == (2f64)) && (first_bit != second_bit))) {
                            {
                                let result_2 = (result + weight);
                                result_2
                            }
                        } else {
                            result
                        };
                        {
                            let first_2 = ((first - first_bit) / 2f64);
                            {
                                let second_2 = ((second - second_bit) / 2f64);
                                {
                                    let weight_2 = (weight * 2f64);
                                    {
                                        let position_2 = (position + 1f64);
                                        {
                                            (first, second, result, weight, position) = (first_2, second_2, result_3, weight_2, position_2);
                                            continue;
                                        }
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
pub fn source_ml_source1_temporary_read_string_number_decimal_power_loop16(mut exponent: f64, mut value: self::ml::Big, mut base: self::ml::Big) -> self::ml::Big {
    loop {
        return if (exponent > (0f64)) {
            {
                let value_3 = if (((exponent % 2f64)) == (1f64)) {
                    {
                        let value_2 = value.mul(&base);
                        value_2.clone()
                    }
                } else {
                    value.clone()
                };
                {
                    let exponent_2 = ((exponent / 2f64)).floor();
                    if (exponent_2 > (0f64)) {
                        {
                            let base_2 = base.mul(&base);
                            {
                                (exponent, value, base) = (exponent_2, value_3.clone(), base_2.clone());
                                continue;
                            }
                        }
                    } else {
                        {
                            (exponent, value) = (exponent_2, value_3.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            value.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_exact_mantissa_loop17(mut value: self::ml::Big, mut result: f64, mut scale: f64) -> f64 {
    loop {
        return if (value > (self::ml::Big::from_i128(0))) {
            {
                let result_3 = if ((self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), true)) == (self::ml::Big::from_i128(1))) {
                    {
                        let result_2 = (result + scale);
                        result_2
                    }
                } else {
                    result
                };
                {
                    let value_2 = self::ml::divide(&value, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    {
                        let scale_2 = (scale * 2f64);
                        {
                            (value, result, scale) = (value_2.clone(), result_3, scale_2);
                            continue;
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
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop19(mut exponent: f64, mut result: f64) -> f64 {
    loop {
        return if (exponent < (0f64)) {
            {
                let result_2 = (result / 2f64);
                {
                    let exponent_2 = (exponent + 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
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
pub enum MlSource1TemporaryReadStringNumberBinaryPowerLoop18Result {
    MlSource1TemporaryReadStringNumberBinaryPowerLoop18Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_string_number_binary_power_loop18(mut exponent: f64, mut result: f64) -> self::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Result {
    loop {
        return if (exponent > (0f64)) {
            {
                let result_2 = (result * 2f64);
                {
                    let exponent_2 = (exponent - 1f64);
                    {
                        (exponent, result) = (exponent_2, result_2);
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Result::MlSource1TemporaryReadStringNumberBinaryPowerLoop18Done(exponent, result)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop23Result {
    MlSource1TemporaryConvertStringNumberRationalLoop23Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop23(mut quotient: self::ml::Big, mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop23Result {
    loop {
        return if (weight > (self::ml::Big::from_i128(1))) {
            {
                let scale_2 = self::ml::divide(&scale, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                {
                    let weight_2 = self::ml::divide(&weight, &self::ml::Big::from_i128(2), "trunc", Some("Division by zero"), false);
                    if (remainder >= scale_2) {
                        {
                            let remainder_2 = remainder.sub(&scale_2);
                            {
                                let quotient_2 = quotient.add(&weight_2);
                                {
                                    (quotient, remainder, scale, weight) = (quotient_2.clone(), remainder_2.clone(), scale_2.clone(), weight_2.clone());
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            (scale, weight) = (scale_2.clone(), weight_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop23Result::MlSource1TemporaryConvertStringNumberRationalLoop23Done(quotient.clone(), remainder.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop22Result {
    MlSource1TemporaryConvertStringNumberRationalLoop22Done(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop22(mut remainder: self::ml::Big, mut scale: self::ml::Big, mut weight: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop22Result {
    loop {
        return if (scale <= remainder) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let weight_2 = weight.mul(&self::ml::Big::from_i128(2));
                    {
                        (scale, weight) = (scale_2.clone(), weight_2.clone());
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop22Result::MlSource1TemporaryConvertStringNumberRationalLoop22Done(scale.clone(), weight.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin21 {
    MlSource1TemporaryConvertStringNumberRationalJoin21Next(self::ml::Big, self::ml::Big),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop24(mut numerator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < shift) {
            {
                let numerator_2 = numerator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (numerator, index) = (numerator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            numerator.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop25(mut denominator: self::ml::Big, mut shift: f64, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < ((-shift))) {
            {
                let denominator_2 = denominator.mul(&self::ml::Big::from_i128(2));
                {
                    let index_2 = (index + 1f64);
                    {
                        (denominator, index) = (denominator_2.clone(), index_2);
                        continue;
                    }
                }
            }
        } else {
            denominator.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalJoin20 {
    MlSource1TemporaryConvertStringNumberRationalJoin20Next(f64),
    MlSource1TemporaryConvertStringNumberRationalJoin20Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop26Result {
    MlSource1TemporaryConvertStringNumberRationalLoop26Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop26Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop26(mut numerator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop26Result {
    loop {
        return if (numerator >= (scale.mul(&self::ml::Big::from_i128(2)))) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent + 1f64);
                    if (exponent_2 >= (1024f64)) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop26Result::MlSource1TemporaryConvertStringNumberRationalLoop26Return(f64::INFINITY)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop26Result::MlSource1TemporaryConvertStringNumberRationalLoop26Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertStringNumberRationalLoop27Result {
    MlSource1TemporaryConvertStringNumberRationalLoop27Done(f64),
    MlSource1TemporaryConvertStringNumberRationalLoop27Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_string_number_rational_loop27(mut denominator: self::ml::Big, mut exponent: f64, mut scale: self::ml::Big) -> self::MlSource1TemporaryConvertStringNumberRationalLoop27Result {
    loop {
        return if (scale < denominator) {
            {
                let scale_2 = scale.mul(&self::ml::Big::from_i128(2));
                {
                    let exponent_2 = (exponent - 1f64);
                    if (exponent_2 < ((-1075f64))) {
                        self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Return(0f64)
                    } else {
                        {
                            (exponent, scale) = (exponent_2, scale_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertStringNumberRationalLoop27Result::MlSource1TemporaryConvertStringNumberRationalLoop27Done(exponent)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryAcceptStringNumberInfinityLoop28Result {
    MlSource1TemporaryAcceptStringNumberInfinityLoop28Done,
    MlSource1TemporaryAcceptStringNumberInfinityLoop28Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_accept_string_number_infinity_loop28(units: &[f64], mut start: f64, expected: &[f64], mut index: f64) -> self::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result {
    loop {
        return if (index < ((expected.len() as f64))) {
            if (((self::ml_array::optional_at(units, self::ml_array::number_index((start + index)))).clone().unwrap_or_else(|| (-1f64))) != ((self::ml_array::optional_at(expected, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-2f64)))) {
                self::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result::MlSource1TemporaryAcceptStringNumberInfinityLoop28Return(false)
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
            self::MlSource1TemporaryAcceptStringNumberInfinityLoop28Result::MlSource1TemporaryAcceptStringNumberInfinityLoop28Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop30(units: &[f64], mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((start < end) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(units, self::ml_array::number_index(start))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let start_2 = (start + 1f64);
                {
                    start = start_2;
                    continue;
                }
            }
        } else {
            start
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop31(units: &[f64], mut start: f64, mut end: f64) -> f64 {
    loop {
        return if ((end > start) && self::accept_java_script_whitespace_unit((self::ml_array::optional_at(units, self::ml_array::number_index((end - 1f64)))).clone().unwrap_or_else(|| (-1f64)))) {
            {
                let end_2 = (end - 1f64);
                {
                    end = end_2;
                    continue;
                }
            }
        } else {
            end
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop37(units: &[f64], mut integer_digits: Vec<self::ml::Big>, mut mantissa_end: f64, mut mantissa: self::ml::Big, mut index: f64) -> self::ml::Big {
    loop {
        return if (index < mantissa_end) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(index))).clone().unwrap_or_else(|| (-1f64));
                if (unit != (46f64)) {
                    {
                        let mantissa_2 = (mantissa.mul(&self::ml::Big::from_i128(10))).add(&(self::ml_array::optional_at(&integer_digits, self::ml_array::number_index((unit - 48f64)))).clone().unwrap_or_else(|| self::ml::Big::from_i128(0)));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (mantissa, index) = (mantissa_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                } else {
                    {
                        let index_3 = (index + 1f64);
                        {
                            index = index_3;
                            continue;
                        }
                    }
                }
            }
        } else {
            mantissa.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Done(f64, f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_convert_java_script_string_number_units_loop36(units: &[f64], mut start: f64, mut exponent: f64, mut mantissa_end: f64) -> self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Result {
    loop {
        return if (mantissa_end > start) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index((mantissa_end - 1f64)))).clone().unwrap_or_else(|| (-1f64));
                if (unit == (46f64)) {
                    {
                        let mantissa_end_2 = (mantissa_end - 1f64);
                        {
                            mantissa_end = mantissa_end_2;
                            continue;
                        }
                    }
                } else {
                    if (unit != (48f64)) {
                        self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Done(exponent, mantissa_end)
                    } else {
                        {
                            let mantissa_end_3 = (mantissa_end - 1f64);
                            {
                                let exponent_2 = (exponent + 1f64);
                                {
                                    (exponent, mantissa_end) = (exponent_2, mantissa_end_3);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Result::MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop36Done(exponent, mantissa_end)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35 {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35Next(f64, f64),
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsJoin35Return(f64),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Result {
    MlSource1TemporaryConvertJavaScriptStringNumberUnitsLoop39Done(f64, f64),
}
