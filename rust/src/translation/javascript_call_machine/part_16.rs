// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallHandler;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Result;
use super::accept_java_script_call_definition;
use super::accept_java_script_call_key;
use super::begin_java_script_property_definition;
use super::begin_java_script_property_key_conversion;
use super::convert_java_script_semantic_boolean;
use super::create_java_script_call_frame;
use super::execute_java_script_call_primitive_instruction;
use super::execute_java_script_ordinary_call_property;
use super::find_java_script_call_function;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_call_machine_error;
use super::read_java_script_call_program_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_source_exception_class;
use super::replace_java_script_call_frame;
use super::return_java_script_call_value;
use super::source_ml_source1_temporary_begin_java_script_call_machine_loop155;
use super::step_java_script_call_number;
use super::step_java_script_call_text;
use super::step_java_script_property_definition;
use super::step_java_script_property_key_conversion;
use super::test_java_script_call_strict_equal;
use super::test_java_script_semantic_same_value;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_call_machine(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field9, shared_constructor_field10) => {
            let shared_constructor_field9 = *shared_constructor_field9;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(shared_constructor_field7, _, shared_constructor_field8) => {
                    let shared_constructor_field8 = *shared_constructor_field8;
                    {
                        let error = self::read_java_script_call_machine_error(program.clone(), machine.clone());
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), error.clone())
                        } else {
                            {
                                let frame = self::ml_array::optional_at(&shared_constructor_field10, self::ml_array::number_index(((shared_constructor_field10.len() as f64) - 1f64)));
                                if match (frame.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101]))
                                } else {
                                    {
                                        let pending = match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2130) => {
                                                let source_ml_source2_data_property_value2130 = *source_ml_source2_data_property_value2130;
                                                source_ml_source2_data_property_value2130.clone()
                                            }
                                        };
                                        match pending.clone() {
                                            self::JavaScriptCallPending::CallIdle => {
                                                {
                                                    let fn_ = self::ml_array::optional_at(&shared_constructor_field7, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), match frame.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value2142, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value2142.clone()
                                                        }
                                                    })));
                                                    if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                                    } else {
                                                        {
                                                            let instruction = self::ml_array::optional_at(&match fn_.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value2144) => {
                                                                    source_ml_source2_data_property_value2144
                                                                }
                                                            }, self::ml_array::number_index(match frame.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2145, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value2145
                                                                }
                                                            }));
                                                            if match (instruction.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                self::return_java_script_call_value(program.clone(), machine.clone(), self::JavaScriptSemanticValue::Undefined)
                                                            } else {
                                                                {
                                                                    let next = {
                                                                        let source_ml_source2_nominal_spread2147 = frame.clone().expect("a proven present value");
                                                                        match source_ml_source2_nominal_spread2147.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                                                                let shared_constructor_field6 = *shared_constructor_field6;
                                                                                {
                                                                                    let source_ml_source2_nominal_field2148 = shared_constructor_field1.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field2150 = shared_constructor_field2;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field2152 = shared_constructor_field3;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2154 = shared_constructor_field4;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2156 = shared_constructor_field5;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2158 = shared_constructor_field6.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override2160 = (match frame.clone().expect("a proven present value") {
                                                                                                                self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value2146, _, _, _, _) => {
                                                                                                                    source_ml_source2_data_property_value2146
                                                                                                                }
                                                                                                            } + 1f64);
                                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2148.clone(), source_ml_source2_nominal_override2160, source_ml_source2_nominal_field2152, source_ml_source2_nominal_field2154, source_ml_source2_nominal_field2156, Box::new(source_ml_source2_nominal_field2158.clone()))
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
                                                                    match next.clone() {
                                                                        self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3_2, shared_constructor_field4_2, shared_constructor_field5_2, shared_constructor_field6_2) => {
                                                                            let shared_constructor_field6_2 = *shared_constructor_field6_2;
                                                                            {
                                                                                let current = self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10.clone(), next.clone());
                                                                                {
                                                                                    let registers = match frame.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2163, _, _, _) => {
                                                                                            source_ml_source2_data_property_value2163
                                                                                        }
                                                                                    };
                                                                                    match instruction.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallInstruction::Constant(destination, value) => {
                                                                                            let value = *value;
                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, self::write_java_script_call_register(next.clone(), destination, value.clone()))))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::Copy(destination_2, source) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, self::write_java_script_call_register(next.clone(), destination_2, (self::ml_array::optional_at(&registers, self::ml_array::number_index(source))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::Argument(destination_3, index) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, self::write_java_script_call_register(next.clone(), destination_3, (self::ml_array::optional_at(&match frame.clone().expect("a proven present value") {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value2183, _, _) => {
                                                                                                    source_ml_source2_data_property_value2183
                                                                                                }
                                                                                            }, self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::Jump(target) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, {
                                                                                                let source_ml_source2_nominal_field2187 = shared_constructor_field1_2.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2189 = shared_constructor_field2_2;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2191 = shared_constructor_field3_2;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2193 = shared_constructor_field4_2;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2195 = shared_constructor_field5_2;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field2197 = shared_constructor_field6_2.clone();
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2187.clone(), target, source_ml_source2_nominal_field2191, source_ml_source2_nominal_field2193, source_ml_source2_nominal_field2195, Box::new(source_ml_source2_nominal_field2197.clone()))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            })))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::Branch(condition, yes, no) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, {
                                                                                                let source_ml_source2_nominal_field2203 = shared_constructor_field1_2.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2205 = shared_constructor_field2_2;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2207 = shared_constructor_field3_2;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2209 = shared_constructor_field4_2;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2211 = shared_constructor_field5_2;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field2213 = shared_constructor_field6_2.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override2215 = if self::convert_java_script_semantic_boolean((self::ml_array::optional_at(&registers, self::ml_array::number_index(condition))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)) {
                                                                                                                            yes
                                                                                                                        } else {
                                                                                                                            no
                                                                                                                        };
                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2203.clone(), source_ml_source2_nominal_override2215, source_ml_source2_nominal_field2207, source_ml_source2_nominal_field2209, source_ml_source2_nominal_field2211, Box::new(source_ml_source2_nominal_field2213.clone()))
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            })))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::Call(destination_4, callee, receiver, arguments) => {
                                                                                            {
                                                                                                let arguments_ = { let ml_values7 = arguments; let ml_callback7 = { let registers = registers.clone(); std::rc::Rc::new(move |index_2: f64| -> self::JavaScriptSemanticValue { (self::ml_array::optional_at(&registers, self::ml_array::number_index(index_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values7.iter().cloned().enumerate().map(|(ml_index7, ml_value7)| ml_callback7(ml_value7)).collect::<Vec<_>>() };
                                                                                                self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, {
                                                                                                    let source_ml_source2_nominal_field2219 = shared_constructor_field1_2.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2221 = shared_constructor_field2_2;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2223 = shared_constructor_field3_2;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2225 = shared_constructor_field4_2;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field2227 = shared_constructor_field5_2;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field2229 = shared_constructor_field6_2.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override2231 = self::JavaScriptCallPending::WaitCall(destination_4);
                                                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2219.clone(), source_ml_source2_nominal_field2221, source_ml_source2_nominal_field2223, source_ml_source2_nominal_field2225, source_ml_source2_nominal_field2227, Box::new(source_ml_source2_nominal_override2231.clone()))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }), (self::ml_array::optional_at(&registers, self::ml_array::number_index(callee))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), arguments_)
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::Return(source_2) => {
                                                                                            self::return_java_script_call_value(program.clone(), current.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(source_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::Throw(source_3) => {
                                                                                            self::throw_java_script_call_value(current.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(source_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::PushHandler(destination_5, target_2) => {
                                                                                            if ((((match frame.clone().expect("a proven present value") {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2233, _) => {
                                                                                                    source_ml_source2_data_property_value2233
                                                                                                }
                                                                                            }).len() as f64)) >= (1024f64)) {
                                                                                                self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![104, 97, 110, 100, 108, 101, 114, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                                            } else {
                                                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, {
                                                                                                    let source_ml_source2_nominal_field2240 = shared_constructor_field1_2.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2242 = shared_constructor_field2_2;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2244 = shared_constructor_field3_2;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2246 = shared_constructor_field4_2;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field2248 = shared_constructor_field5_2;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field2250 = shared_constructor_field6_2.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override2252 = {
                                                                                                                                let source_ml_source2_concat_value2237 = match frame.clone().expect("a proven present value") {
                                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2236, _) => {
                                                                                                                                        source_ml_source2_data_property_value2236
                                                                                                                                    }
                                                                                                                                };
                                                                                                                                {
                                                                                                                                    let source_ml_source2_concat_value2238 = vec![self::JavaScriptCallHandler::CallHandler(target_2, destination_5)];
                                                                                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallHandler>::new(), source_ml_source2_concat_value2237), source_ml_source2_concat_value2238)
                                                                                                                                }
                                                                                                                            };
                                                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2240.clone(), source_ml_source2_nominal_field2242, source_ml_source2_nominal_field2244, source_ml_source2_nominal_field2246, source_ml_source2_nominal_override2252, Box::new(source_ml_source2_nominal_field2250.clone()))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                })))
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::PopHandler => {
                                                                                            if ((((match frame.clone().expect("a proven present value") {
                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2254, _) => {
                                                                                                    source_ml_source2_data_property_value2254
                                                                                                }
                                                                                            }).len() as f64)) == (0f64)) {
                                                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![97, 32, 104, 97, 110, 100, 108, 101, 114, 32, 112, 111, 112, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 104, 97, 110, 100, 108, 101, 114]))
                                                                                            } else {
                                                                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, {
                                                                                                    let source_ml_source2_nominal_field2261 = shared_constructor_field1_2.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2263 = shared_constructor_field2_2;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2265 = shared_constructor_field3_2;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field2267 = shared_constructor_field4_2;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field2269 = shared_constructor_field5_2;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field2271 = shared_constructor_field6_2.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override2273 = self::ml_array::slice(&match frame.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value2259, _) => {
                                                                                                                                    source_ml_source2_data_property_value2259
                                                                                                                                }
                                                                                                                            }, 0f64, (-1f64));
                                                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2261.clone(), source_ml_source2_nominal_field2263, source_ml_source2_nominal_field2265, source_ml_source2_nominal_field2267, source_ml_source2_nominal_override2273, Box::new(source_ml_source2_nominal_field2271.clone()))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                })))
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::NumberBinary(destination_6, instruction_left, instruction_right, operator) => {
                                                                                            {
                                                                                                let left = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_left))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                                                {
                                                                                                    let right = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_right))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                                                    match left.clone() {
                                                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Null => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Number(left_value) => {
                                                                                                            match right.clone() {
                                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                                }
                                                                                                                self::JavaScriptSemanticValue::Null => {
                                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                                }
                                                                                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                                }
                                                                                                                self::JavaScriptSemanticValue::Number(right_value) => {
                                                                                                                    if ((operator == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) || (operator == (self::ml_string::Text::from_units(vec![101, 113, 117, 97, 108])))) {
                                                                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Boolean(if (operator == (self::ml_string::Text::from_units(vec![108, 101, 115, 115, 84, 104, 97, 110]))) {
                                                                                                                            (left_value < right_value)
                                                                                                                        } else {
                                                                                                                            (left_value == right_value)
                                                                                                                        })))))
                                                                                                                    } else {
                                                                                                                        {
                                                                                                                            let value_2 = if (operator == (self::ml_string::Text::from_units(vec![97, 100, 100]))) {
                                                                                                                                (left_value + right_value)
                                                                                                                            } else {
                                                                                                                                if (operator == (self::ml_string::Text::from_units(vec![115, 117, 98, 116, 114, 97, 99, 116]))) {
                                                                                                                                    (left_value - right_value)
                                                                                                                                } else {
                                                                                                                                    if (operator == (self::ml_string::Text::from_units(vec![109, 117, 108, 116, 105, 112, 108, 121]))) {
                                                                                                                                        (left_value * right_value)
                                                                                                                                    } else {
                                                                                                                                        if (operator == (self::ml_string::Text::from_units(vec![100, 105, 118, 105, 100, 101]))) {
                                                                                                                                            (left_value / right_value)
                                                                                                                                        } else {
                                                                                                                                            if (operator == (self::ml_string::Text::from_units(vec![114, 101, 109, 97, 105, 110, 100, 101, 114]))) {
                                                                                                                                                (left_value % right_value)
                                                                                                                                            } else {
                                                                                                                                                self::ml_big_integer_conversion::power_java_script_number(left_value,right_value)
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, self::write_java_script_call_register(next.clone(), destination_6, self::JavaScriptSemanticValue::Number(value_2)))))
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                                }
                                                                                                                self::JavaScriptSemanticValue::Text(_) => {
                                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                                }
                                                                                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                                }
                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Integer(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![116, 121, 112, 101, 100, 32, 110, 117, 109, 98, 101, 114, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 119, 111, 32, 97, 99, 116, 117, 97, 108, 32, 78, 117, 109, 98, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::SameValue(destination_7, left_2, right_2) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, self::write_java_script_call_register(next.clone(), destination_7, self::JavaScriptSemanticValue::Boolean(self::test_java_script_semantic_same_value((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::StrictEqual(destination_8, left_3, right_3) => {
                                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field9.clone(), shared_constructor_field10, self::write_java_script_call_register(next.clone(), destination_8, self::JavaScriptSemanticValue::Boolean(self::test_java_script_call_strict_equal((self::ml_array::optional_at(&registers, self::ml_array::number_index(left_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(right_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))))))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::RequireWrite(_) => {
                                                                                            self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                                                                            self::execute_java_script_call_primitive_instruction(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                                                                            self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                                                                            self::execute_java_script_ordinary_call_property(program.clone(), machine.clone(), frame.clone().expect("a proven present value"), instruction.clone().expect("a proven present value"))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::PropertyKey(destination_9, input, realm) => {
                                                                                            self::accept_java_script_call_key(program.clone(), current.clone(), destination_9, realm.clone(), self::begin_java_script_property_key_conversion(shared_constructor_field9.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(input))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field8.clone()))
                                                                                        }
                                                                                        self::JavaScriptCallInstruction::PropertyDefinition(destination_10, target_3, key, attributes, realm_2) => {
                                                                                            self::accept_java_script_call_definition(program.clone(), current.clone(), destination_10, realm_2.clone(), self::begin_java_script_property_definition(shared_constructor_field9.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(target_3))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(attributes))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), shared_constructor_field8.clone()))
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
                                            self::JavaScriptCallPending::WaitCall(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptCallPending::WaitGet(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptCallPending::WaitSet(_) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptCallPending::WaitKey(destination_11, realm_3, state) => {
                                                let state = *state;
                                                self::accept_java_script_call_key(program.clone(), machine.clone(), destination_11, realm_3.clone(), self::step_java_script_property_key_conversion(shared_constructor_field9.clone(), state.clone()))
                                            }
                                            self::JavaScriptCallPending::WaitDefinition(destination_12, realm_4, state_2) => {
                                                let state_2 = *state_2;
                                                self::accept_java_script_call_definition(program.clone(), machine.clone(), destination_12, realm_4.clone(), self::step_java_script_property_definition(shared_constructor_field9.clone(), state_2.clone()))
                                            }
                                            self::JavaScriptCallPending::WaitNumber(destination_13, realm_5, state_3) => {
                                                let state_3 = *state_3;
                                                self::step_java_script_call_number(program.clone(), machine.clone(), destination_13, realm_5.clone(), state_3.clone())
                                            }
                                            self::JavaScriptCallPending::WaitText(destination_14, realm_6, state_4) => {
                                                let state_4 = *state_4;
                                                self::step_java_script_call_text(program.clone(), machine.clone(), destination_14, realm_6.clone(), state_4.clone())
                                            }
                                            self::JavaScriptCallPending::WaitSourceError(_, _, _, _) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                            }
                                            self::JavaScriptCallPending::WaitError(_, _, _) => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field9.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 97, 108, 108, 32, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
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
pub fn begin_java_script_call_machine(program: self::JavaScriptCallProgram, heap: self::JavaScriptOrdinaryHeap, identity: self::ml::Big, receiver: self::JavaScriptSemanticValue, arguments_: Vec<self::JavaScriptSemanticValue>) -> self::JavaScriptCallMachineTransition {
    {
        let error = self::read_java_script_call_program_error(program.clone(), heap.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), error.clone())
        } else {
            {
                let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value2319, _, _) => source_ml_source2_data_property_value2319.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), identity.clone())));
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
                                    let source_ml_source1_temporary_index154 = self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s2320 = self::source_ml_source1_temporary_begin_java_script_call_machine_loop155(heap.clone(), arguments_.clone(), source_ml_source1_temporary_index154.clone());
                                        match source_ml_source2_s2320.clone() {
                                            self::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Done => {
                                                self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(heap.clone()), vec![self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_)])))
                                            }
                                            self::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Result::MlSource1TemporaryBeginJavaScriptCallMachineLoop155Return(source_ml_source1_temporary_result155) => {
                                                let source_ml_source1_temporary_result155 = *source_ml_source1_temporary_result155;
                                                source_ml_source1_temporary_result155.clone()
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
                                let source_ml_source2_s2326 = match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2349) => {
                                        let source_ml_source2_data_property_value2349 = *source_ml_source2_data_property_value2349;
                                        source_ml_source2_data_property_value2349.clone()
                                    }
                                };
                                match source_ml_source2_s2326.clone() {
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
                                    let source_ml_source2_comparison_operand2358 = match match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value2351) => {
                                            let source_ml_source2_data_property_value2351 = *source_ml_source2_data_property_value2351;
                                            source_ml_source2_data_property_value2351.clone()
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
                                        self::JavaScriptCallPending::WaitKey(_, source_ml_source2_data_property_value2352, _) => {
                                            Some(source_ml_source2_data_property_value2352.clone())
                                        }
                                        self::JavaScriptCallPending::WaitDefinition(_, source_ml_source2_data_property_value2353, _) => {
                                            Some(source_ml_source2_data_property_value2353.clone())
                                        }
                                        self::JavaScriptCallPending::WaitNumber(_, source_ml_source2_data_property_value2354, _) => {
                                            Some(source_ml_source2_data_property_value2354.clone())
                                        }
                                        self::JavaScriptCallPending::WaitText(_, source_ml_source2_data_property_value2355, _) => {
                                            Some(source_ml_source2_data_property_value2355.clone())
                                        }
                                        self::JavaScriptCallPending::WaitSourceError(source_ml_source2_data_property_value2356, _, _, _) => {
                                            Some(source_ml_source2_data_property_value2356.clone())
                                        }
                                        self::JavaScriptCallPending::WaitError(source_ml_source2_data_property_value2357, _, _) => {
                                            Some(source_ml_source2_data_property_value2357.clone())
                                        }
                                    };
                                    !(match (source_ml_source2_comparison_operand2358.clone(), realm.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false })
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_call_source_error(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, frame: self::JavaScriptCallFrame, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8) => {
            let shared_constructor_field8 = *shared_constructor_field8;
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, shared_constructor_field2) => {
                    let shared_constructor_field1 = *shared_constructor_field1;
                    match instruction.clone() {
                        self::JavaScriptCallInstruction::Constant(_, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::Copy(_, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::Argument(_, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::Jump(_) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::Branch(_, _, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::Return(_) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::Throw(_) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::PushHandler(_, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::PopHandler => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::RequireWrite(_) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::Primitive(_, operation, arguments) => {
                            {
                                let error_class = self::read_java_script_source_exception_class(&(operation));
                                if ((error_class == (self::ml_string::Text::from_units(vec![]))) || (((arguments.len() as f64)) != (2f64))) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 108, 97, 115, 115, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115]))
                                } else {
                                    {
                                        let message = (self::ml_array::optional_at(&shared_constructor_field5, self::ml_array::number_index((self::ml_array::optional_at(&arguments, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| (-1f64))))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                        {
                                            let present = (self::ml_array::optional_at(&shared_constructor_field5, self::ml_array::number_index((self::ml_array::optional_at(&arguments, self::ml_array::number_index(1f64))).clone().unwrap_or_else(|| (-1f64))))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                            match message.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 85, 84, 70, 49, 54, 32, 109, 101, 115, 115, 97, 103, 101]))
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 85, 84, 70, 49, 54, 32, 109, 101, 115, 115, 97, 103, 101]))
                                                }
                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 85, 84, 70, 49, 54, 32, 109, 101, 115, 115, 97, 103, 101]))
                                                }
                                                self::JavaScriptSemanticValue::Number(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 85, 84, 70, 49, 54, 32, 109, 101, 115, 115, 97, 103, 101]))
                                                }
                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 85, 84, 70, 49, 54, 32, 109, 101, 115, 115, 97, 103, 101]))
                                                }
                                                self::JavaScriptSemanticValue::Text(_) => {
                                                    {
                                                        let message_error = self::read_java_script_ordinary_value_error(shared_constructor_field1.clone(), message.clone());
                                                        if (message_error != (self::ml_string::Text::from_units(vec![]))) {
                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), message_error.clone())
                                                        } else {
                                                            match present.clone() {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101]))
                                                                }
                                                                self::JavaScriptSemanticValue::Null => {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101]))
                                                                }
                                                                self::JavaScriptSemanticValue::Boolean(value) => {
                                                                    {
                                                                        let fn_ = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value2464, _, _) => source_ml_source2_data_property_value2464.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field3.clone())));
                                                                        if match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                                        } else {
                                                                            {
                                                                                let next = {
                                                                                    let source_ml_source2_nominal_field2470 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field2472 = shared_constructor_field4;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field2474 = shared_constructor_field5;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2476 = shared_constructor_field6;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2478 = shared_constructor_field7;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2480 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override2482 = (shared_constructor_field4 + 1f64);
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override2483 = self::JavaScriptCallPending::WaitSourceError(match fn_.clone().expect("a proven present value") {
                                                                                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value2468, _, _, _, _, _, _) => {
                                                                                                                        source_ml_source2_data_property_value2468.clone()
                                                                                                                    }
                                                                                                                }, error_class.clone(), Box::new(message.clone()), value);
                                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2470.clone(), source_ml_source2_nominal_override2482, source_ml_source2_nominal_field2474, source_ml_source2_nominal_field2476, source_ml_source2_nominal_field2478, Box::new(source_ml_source2_nominal_override2483.clone()))
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                };
                                                                                self::JavaScriptCallMachineTransition::MachineAllocateSourceError(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2, next.clone())), match fn_.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value2486, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value2486.clone()
                                                                                    }
                                                                                }, error_class.clone(), Box::new(message.clone()), value)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::JavaScriptSemanticValue::Number(_) => {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101]))
                                                                }
                                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101]))
                                                                }
                                                                self::JavaScriptSemanticValue::Text(_) => {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101]))
                                                                }
                                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101]))
                                                                }
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 109, 101, 115, 115, 97, 103, 101, 32, 112, 114, 101, 115, 101, 110, 99, 101]))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 85, 84, 70, 49, 54, 32, 109, 101, 115, 115, 97, 103, 101]))
                                                }
                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 85, 84, 70, 49, 54, 32, 109, 101, 115, 115, 97, 103, 101]))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                        self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        }
                    }
                }
            }
        }
    }
}
