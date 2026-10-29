// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::accept_java_script_call_number;
use super::accept_java_script_call_text;
use super::begin_java_script_scalar_protocol;
use super::execute_java_script_call_primitive;
use super::find_java_script_call_function;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::plan_java_script_ordinary_set;
use super::push_java_script_call;
use super::read_java_script_call_primitive_arity_error;
use super::read_java_script_integer_protocol_operation_error;
use super::read_java_script_source_exception_class;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::request_java_script_call_source_error;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_ordinary_call_property(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, frame: self::JavaScriptCallFrame, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14) => {
            let shared_constructor_field14 = *shared_constructor_field14;
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(shared_constructor_field7, shared_constructor_field8) => {
                    let shared_constructor_field7 = *shared_constructor_field7;
                    {
                        let next = {
                            let source_ml_source2_nominal_field1818 = shared_constructor_field9.clone();
                            {
                                let source_ml_source2_nominal_field1820 = shared_constructor_field10;
                                {
                                    let source_ml_source2_nominal_field1822 = shared_constructor_field11.clone();
                                    {
                                        let source_ml_source2_nominal_field1824 = shared_constructor_field12;
                                        {
                                            let source_ml_source2_nominal_field1826 = shared_constructor_field13;
                                            {
                                                let source_ml_source2_nominal_field1828 = shared_constructor_field14.clone();
                                                {
                                                    let source_ml_source2_nominal_override1830 = (shared_constructor_field10 + 1f64);
                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1818.clone(), source_ml_source2_nominal_override1830, source_ml_source2_nominal_field1822, source_ml_source2_nominal_field1824, source_ml_source2_nominal_field1826, Box::new(source_ml_source2_nominal_field1828.clone()))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        };
                        match next.clone() {
                            self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6) => {
                                let shared_constructor_field6 = *shared_constructor_field6;
                                {
                                    let registers = shared_constructor_field11;
                                    match instruction.clone() {
                                        self::JavaScriptCallInstruction::Constant(_, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::Copy(_, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::Argument(_, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::Jump(_) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::Return(_) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::Throw(_) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::PopHandler => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::RequireWrite(instruction_success) => {
                                            {
                                                let success = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_success))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                {
                                                    let owner = self::ml_array::optional_at((match &program { self::JavaScriptCallProgram::CallProgram(source_ml_source2_data_property_value1884, _, _) => source_ml_source2_data_property_value1884.as_slice() }), self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field9.clone())));
                                                    if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                    } else {
                                                        match success.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                            }
                                                            self::JavaScriptSemanticValue::Null => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                            }
                                                            self::JavaScriptSemanticValue::Boolean(value) => {
                                                                {
                                                                    let current = self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, next.clone());
                                                                    if value {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(current.clone()))
                                                                    } else {
                                                                        self::request_java_script_call_error(current.clone(), match owner.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value1897, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value1897.clone()
                                                                            }
                                                                        }, &(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 97, 115, 115, 105, 103, 110, 109, 101, 110, 116, 32, 119, 97, 115, 32, 114, 101, 106, 101, 99, 116, 101, 100])))
                                                                    }
                                                                }
                                                            }
                                                            self::JavaScriptSemanticValue::Number(_) => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                            }
                                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                            }
                                                            self::JavaScriptSemanticValue::Text(_) => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                            }
                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                            }
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 32, 115, 116, 114, 105, 99, 116, 32, 119, 114, 105, 116, 101, 32, 103, 117, 97, 114, 100, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryGet(destination, instruction_object, key, receiver) => {
                                            {
                                                let object = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_object))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                match object.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Object(identity, _) => {
                                                        {
                                                            let plan = self::plan_java_script_ordinary_get(shared_constructor_field7.clone(), identity.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                            if (((plan.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), (plan.clone()).3)
                                                            } else {
                                                                if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, self::write_java_script_call_register(next.clone(), destination, (plan.clone()).5))))
                                                                } else {
                                                                    if (((plan.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                        self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                                                            let source_ml_source2_nominal_field1924 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field1926 = shared_constructor_field2;
                                                                                {
                                                                                    let source_ml_source2_nominal_field1928 = shared_constructor_field3;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1930 = shared_constructor_field4;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1932 = shared_constructor_field5;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field1934 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override1936 = self::JavaScriptCallPending::WaitGet(destination);
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1924.clone(), source_ml_source2_nominal_field1926, source_ml_source2_nominal_field1928, source_ml_source2_nominal_field1930, source_ml_source2_nominal_field1932, Box::new(source_ml_source2_nominal_override1936.clone()))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }), (plan.clone()).2, (plan.clone()).4, (plan.clone()).1)
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114]))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::OrdinarySet(destination_2, instruction_object_2, key_2, value_2, receiver_2) => {
                                            {
                                                let object_2 = (self::ml_array::optional_at(&registers, self::ml_array::number_index(instruction_object_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                match object_2.clone() {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Number(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 83, 101, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 111, 112, 101, 114, 97, 110, 100]))
                                                    }
                                                    self::JavaScriptSemanticValue::Object(identity_2, _) => {
                                                        {
                                                            let plan_2 = self::plan_java_script_ordinary_set(shared_constructor_field7.clone(), identity_2.clone(), (self::ml_array::optional_at(&registers, self::ml_array::number_index(key_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(value_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined), (self::ml_array::optional_at(&registers, self::ml_array::number_index(receiver_2))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined));
                                                            if (((plan_2.clone()).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), (plan_2.clone()).3)
                                                            } else {
                                                                if (((plan_2.clone()).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                    self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field7.clone(), shared_constructor_field8, {
                                                                        let source_ml_source2_nominal_field1956 = shared_constructor_field1.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field1958 = shared_constructor_field2;
                                                                            {
                                                                                let source_ml_source2_nominal_field1960 = shared_constructor_field3;
                                                                                {
                                                                                    let source_ml_source2_nominal_field1962 = shared_constructor_field4;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field1964 = shared_constructor_field5;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field1966 = shared_constructor_field6.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_override1968 = self::JavaScriptCallPending::WaitSet(destination_2);
                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field1956.clone(), source_ml_source2_nominal_field1958, source_ml_source2_nominal_field1960, source_ml_source2_nominal_field1962, source_ml_source2_nominal_field1964, Box::new(source_ml_source2_nominal_override1968.clone()))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }), (plan_2.clone()).2, (plan_2.clone()).5, (plan_2.clone()).1)
                                                                } else {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame((plan_2.clone()).4, shared_constructor_field8, self::write_java_script_call_register(next.clone(), destination_2, self::JavaScriptSemanticValue::Boolean((plan_2.clone()).6)))))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                        }
                                        self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field7.clone()), self::ml_string::Text::from_units(vec![97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
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
pub fn execute_java_script_call_primitive_instruction(program: self::JavaScriptCallProgram, machine: self::JavaScriptCallMachine, frame: self::JavaScriptCallFrame, instruction: self::JavaScriptCallInstruction) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
            let shared_constructor_field10 = *shared_constructor_field10;
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(shared_constructor_field3, shared_constructor_field4) => {
                    let shared_constructor_field3 = *shared_constructor_field3;
                    match program.clone() {
                        self::JavaScriptCallProgram::CallProgram(shared_constructor_field1, _, shared_constructor_field2) => {
                            let shared_constructor_field2 = *shared_constructor_field2;
                            match instruction.clone() {
                                self::JavaScriptCallInstruction::Constant(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Copy(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Argument(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Jump(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Return(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Throw(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PopHandler => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::RequireWrite(_) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::Primitive(destination, operation, arguments) => {
                                    {
                                        let arity = self::read_java_script_call_primitive_arity_error(&(operation), (arguments.len() as f64));
                                        if (arity != (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), arity.clone())
                                        } else {
                                            {
                                                let arguments_ = { let ml_values6 = arguments; let ml_callback6 = { let frame = frame.clone(); std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { (self::ml_array::optional_at((match &frame { self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value2033, _, _, _) => source_ml_source2_data_property_value2033.as_slice() }), self::ml_array::number_index(index))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) }) }; ml_values6.iter().cloned().enumerate().map(|(ml_index6, ml_value6)| ml_callback6(ml_value6)).collect::<Vec<_>>() };
                                                {
                                                    let input = (self::ml_array::optional_at(&arguments_, self::ml_array::number_index(0f64))).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                    if ((self::read_java_script_source_exception_class(&(operation))) != (self::ml_string::Text::from_units(vec![]))) {
                                                        self::request_java_script_call_source_error(program.clone(), machine.clone(), frame.clone(), instruction.clone())
                                                    } else {
                                                        if ((((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 110, 117, 109, 98, 101, 114]))) || (operation == (self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114])))) || ((self::read_java_script_integer_protocol_operation_error(&(operation))) == (self::ml_string::Text::from_units(vec![])))) && match input.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        }) {
                                                            {
                                                                let owner = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field5.clone())));
                                                                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                                } else {
                                                                    {
                                                                        let current = self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                            let source_ml_source2_nominal_field2048 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field2050 = shared_constructor_field6;
                                                                                {
                                                                                    let source_ml_source2_nominal_field2052 = shared_constructor_field7;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field2054 = shared_constructor_field8;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field2056 = shared_constructor_field9;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2058 = shared_constructor_field10.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override2060 = (shared_constructor_field6 + 1f64);
                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2048.clone(), source_ml_source2_nominal_override2060, source_ml_source2_nominal_field2052, source_ml_source2_nominal_field2054, source_ml_source2_nominal_field2056, Box::new(source_ml_source2_nominal_field2058.clone()))
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        });
                                                                        self::accept_java_script_call_number(program.clone(), current.clone(), destination, match owner.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value2061, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value2061.clone()
                                                                            }
                                                                        }, self::begin_java_script_scalar_protocol(shared_constructor_field3.clone(), input.clone(), shared_constructor_field2.clone(), &(operation)))
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if ((operation == (self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 116, 101, 120, 116]))) && match input.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            }) {
                                                                {
                                                                    let owner_2 = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field5.clone())));
                                                                    if match (owner_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                                    } else {
                                                                        {
                                                                            let current_2 = self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                let source_ml_source2_nominal_field2078 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field2080 = shared_constructor_field6;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field2082 = shared_constructor_field7;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field2084 = shared_constructor_field8;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2086 = shared_constructor_field9;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2088 = shared_constructor_field10.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override2090 = (shared_constructor_field6 + 1f64);
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2078.clone(), source_ml_source2_nominal_override2090, source_ml_source2_nominal_field2082, source_ml_source2_nominal_field2084, source_ml_source2_nominal_field2086, Box::new(source_ml_source2_nominal_field2088.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            });
                                                                            self::accept_java_script_call_text(program.clone(), current_2.clone(), destination, match owner_2.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value2091, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value2091.clone()
                                                                                }
                                                                            }, self::begin_java_script_scalar_protocol(shared_constructor_field3.clone(), input.clone(), shared_constructor_field2.clone(), &(operation)))
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let result = self::execute_java_script_call_primitive(&(operation), arguments_);
                                                                    match result.clone() {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                                                            let value = *value;
                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, self::write_java_script_call_register({
                                                                                let source_ml_source2_nominal_field2098 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field2100 = shared_constructor_field6;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field2102 = shared_constructor_field7;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field2104 = shared_constructor_field8;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2106 = shared_constructor_field9;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2108 = shared_constructor_field10.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override2110 = (shared_constructor_field6 + 1f64);
                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2098.clone(), source_ml_source2_nominal_override2110, source_ml_source2_nominal_field2102, source_ml_source2_nominal_field2104, source_ml_source2_nominal_field2106, Box::new(source_ml_source2_nominal_field2108.clone()))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }, destination, value.clone()))))
                                                                        }
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(reason) => {
                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), reason.clone())
                                                                        }
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(error_class, reason_2) => {
                                                                            {
                                                                                let owner_3 = self::ml_array::optional_at(&shared_constructor_field1, self::ml_array::number_index(self::find_java_script_call_function(program.clone(), shared_constructor_field5.clone())));
                                                                                if match (owner_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 102, 97, 105, 108, 117, 114, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                                                } else {
                                                                                    self::request_java_script_call_error(self::replace_java_script_call_frame(shared_constructor_field3.clone(), shared_constructor_field4, {
                                                                                        let source_ml_source2_nominal_field2119 = shared_constructor_field5.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field2121 = shared_constructor_field6;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field2123 = shared_constructor_field7;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field2125 = shared_constructor_field8;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field2127 = shared_constructor_field9;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field2129 = shared_constructor_field10.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override2131 = (shared_constructor_field6 + 1f64);
                                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field2119.clone(), source_ml_source2_nominal_override2131, source_ml_source2_nominal_field2123, source_ml_source2_nominal_field2125, source_ml_source2_nominal_field2127, Box::new(source_ml_source2_nominal_field2129.clone()))
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }), match owner_3.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value2132, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value2132.clone()
                                                                                        }
                                                                                    }, &(error_class), &(reason_2))
                                                                                }
                                                                            }
                                                                        }
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(reason_3) => {
                                                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_3.clone())
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
                                self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                                self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _) => {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field3.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101, 120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
