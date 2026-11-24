// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::execute_java_script_call_control_instruction;
use super::execute_java_script_call_invocation_instruction;
use super::execute_java_script_call_object_instruction;
use super::execute_java_script_call_observation_instruction;
use super::execute_java_script_call_property_instruction;
use super::ml_array;
use super::ml_string;
use super::read_java_script_executable_call_function;
use super::return_java_script_call_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_instruction(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => {
            match machine.clone() {
                self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, _) => {
                    let shared_constructor_field1 = *shared_constructor_field1;
                    {
                        let fn_ = self::read_java_script_executable_call_function(
                            program.clone(),
                            shared_constructor_field1.clone(),
                            shared_constructor_field2.clone(),
                        );
                        if match (fn_.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101,
                                    101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99,
                                    116, 105, 111, 110,
                                ]),
                            )
                        } else {
                            {
                                let instruction = {
                                    let source_ml_source2_array_read_value29170 =
                                        match fn_.clone().expect("a proven present value") {
                                            self::JavaScriptCallFunction::CallFunction(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source2_data_property_value29168,
                                                _,
                                                _,
                                            ) => source_ml_source2_data_property_value29168.clone(),
                                        };
                                    {
                                        let source_ml_source2_array_read_index29171 =
                                            shared_constructor_field3;
                                        {
                                            let ml_reference_array1630 =
                                                source_ml_source2_array_read_value29170.clone();
                                            let ml_reference_position1630 =
                                                source_ml_source2_array_read_index29171;
                                            let ml_reference_projection1630 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1630.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1630,
                                                    ),
                                                );
                                            ml_reference_projection1630
                                        }
                                    }
                                };
                                if match (instruction.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    self::return_java_script_call_value(
                                        program.clone(),
                                        machine.clone(),
                                        self::JavaScriptSemanticValue::Undefined,
                                    )
                                } else {
                                    match instruction.clone().expect("a proven present value") {
                                        self::JavaScriptCallInstruction::Constant(_, _) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29419, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29419.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::Copy(_, _) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29420, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29420.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::Argument(_, _) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29421, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29421.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArgumentCount(_) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29422, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29422.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArgumentAt(_, _) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29423, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29423.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::FunctionBind(_) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29424, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29424.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::BoundFunctionCall(_) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29425, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29425.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::FunctionToString(_) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29426, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29426.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::PushFinally(_) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29427, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29427.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::FinishFinally => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29428, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29428.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::CompleteJump(_, _) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29429, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29429.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::Jump(_) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29430, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29430.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::Branch(_, _, _) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29431, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29431.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::Call(_, _, _, _) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29432, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29432.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArgumentListOpen(_) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29433, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29433.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArgumentListAppend(_, _) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29434, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29434.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArgumentListCall(_, _, _, _, _) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29435, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29435.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::Construct(_, _, _, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29436, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29436.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ConstructorCheck(_, _, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29437, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29437.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ReadNewTarget(_) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29438, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29438.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArrayJoinEnter(_, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29439, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29439.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArrayJoinLeave(_) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29440, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29440.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ConstructErrorClass(_, _, _) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29441, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29441.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::CallClassConstructor(_, _, _, _) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29442, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29442.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ConstructCollection(_, _, _) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29443, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29443.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::CollectionAllocate(_, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29444, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29444.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29445, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29445.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::CollectionIteratorAllocate(_, _, _, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29446, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29446.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29447, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29447.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArgumentRest(_, _, _, _) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29448, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29448.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::IteratorProtocol(_, _, _, _, _, _, _, _) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29449, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29449.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::CollectionIteratorResult(_, _, _, _) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29450, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29450.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::CollectionOperation(_, _) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29451, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29451.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::PrimitiveMethod(_, _) => {
                                            self::execute_java_script_call_invocation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29452, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29452.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::CollectionForEach(_, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29453, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29453.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArrayConcatenation(_, _, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29454, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29454.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArraySlice(_, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29455, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29455.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArrayCreation(_, _, _, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29456, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29456.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArrayCallback(_, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29457, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29457.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArraySearch(_, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29458, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29458.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArrayTransform(_, _, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29459, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29459.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ObjectEnumeration(_, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29460, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29460.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::PropertyDefinitions(_, _, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29461, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29461.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::PropertyReflection(_, _, _, _, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29462, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29462.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::InstanceOf(_, _, _, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29463, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29463.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ObjectObservation(_, _, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29464, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29464.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::CollectionCopy(_, _, _, _, _, _, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29465, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29465.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29466, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29466.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ConstructErrorValue(_, _, _, _, _, _, _, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29467, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29467.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29468, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29468.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ReadDerivedReceiver(_) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29469, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29469.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29470, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29470.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::InvokeErrorIntrinsic(_) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29471, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29471.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29472, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29472.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::InvokeHostCapability(_) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29473, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29473.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29474, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29474.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::Return(_) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29475, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29475.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::Throw(_) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29476, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29476.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::PushHandler(_, _) => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29477, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29477.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::PopHandler => {
                                            self::execute_java_script_call_control_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29478, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29478.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::NumberBinary(_, _, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29479, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29479.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29480, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29480.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::SameValue(_, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29481, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29481.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29482, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29482.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::StrictEqual(_, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29483, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29483.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29484, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29484.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::RequireWrite(_) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29485, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29485.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29486, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29486.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::Primitive(_, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29487, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29487.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29488, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29488.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryAllocate(_, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29489, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29489.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29490, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29490.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::LiteralPrototype(_, _, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29491, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29491.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::LiteralAccessor(_, _, _, _, _, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29492, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29492.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ObjectFromEntries(_, _, _, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29493, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29493.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::PrimitiveWrapper(_, _, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29494, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29494.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::SymbolAllocate(_, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29495, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29495.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::SymbolDescription(_, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29496, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29496.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::SymbolRegistry(_, _, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29497, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29497.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::FunctionName(_, _, _, _, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29498, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29498.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArrayAllocate(_, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29499, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29499.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29500, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29500.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ClosureAllocate(_, _, _, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29501, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29501.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29502, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29502.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ArraySpread(_, _, _, _, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29503, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29503.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::ObjectSpread(_, _, _, _, _, _) => {
                                            self::execute_java_script_call_observation_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29504, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29504.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryDataProperty(_, _, _, _, _) => {
                                            self::execute_java_script_call_object_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29505, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29505.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryGet(_, _, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29506, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29506.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29507, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29507.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryDelete(_, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29508, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29508.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29509, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29509.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryHas(_, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29510, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29510.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29511, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29511.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::OrdinarySet(_, _, _, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29512, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29512.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29513, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29513.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::OrdinaryDefine(_, _, _, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29514, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29514.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29515, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29515.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::PropertyKey(_, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29516, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29516.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29517, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29517.clone()
                                                }
                                            })
                                        }
                                        self::JavaScriptCallInstruction::PropertyDefinition(_, _, _, _, _, _) => {
                                            self::execute_java_script_call_property_instruction(program.clone(), machine.clone(), frame.clone(), instruction.clone().expect("a proven present value"), match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value29518, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29518.clone()
                                                }
                                            }, match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value29519, _, _, _, _, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value29519.clone()
                                                }
                                            })
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
