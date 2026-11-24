// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptFunctionBindTransition;
use super::JavaScriptFunctionNameResult;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::accept_java_script_call_definition;
use super::complete_java_script_property_definition_with_symbol;
use super::construct_java_script_symbol_function_name;
use super::create_java_script_property_descriptor;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_call_machine_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_function_bind(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptFunctionBindTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field2) => {
            let frame = {
                let source_ml_source2_array_read_value35629 = shared_constructor_field2.clone();
                {
                    let source_ml_source2_array_read_index35630 = ({
                        let ml_reference_array2290 = shared_constructor_field2.clone();
                        let ml_reference_projection2290 =
                            ml_reference_array2290.borrow().len() as f64;
                        ml_reference_projection2290
                    } - 1f64);
                    {
                        let ml_reference_array2291 =
                            source_ml_source2_array_read_value35629.clone();
                        let ml_reference_position2291 = source_ml_source2_array_read_index35630;
                        let ml_reference_projection2291 = self::ml_array::optional_at(
                            &*ml_reference_array2291.borrow(),
                            self::ml_array::number_index(ml_reference_position2291),
                        );
                        ml_reference_projection2291
                    }
                }
            };
            {
                let current = self::JavaScriptCallMachine::CallMachine(
                    Box::new(match transition.clone() {
                        self::JavaScriptFunctionBindTransition::FunctionBindContinue(
                            source_ml_source2_data_property_value35631,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value35631 =
                                *source_ml_source2_data_property_value35631;
                            source_ml_source2_data_property_value35631.clone()
                        }
                        self::JavaScriptFunctionBindTransition::FunctionBindAllocate(
                            source_ml_source2_data_property_value35632,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value35632 =
                                *source_ml_source2_data_property_value35632;
                            source_ml_source2_data_property_value35632.clone()
                        }
                        self::JavaScriptFunctionBindTransition::FunctionBindCall(
                            source_ml_source2_data_property_value35633,
                            _,
                            _,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value35633 =
                                *source_ml_source2_data_property_value35633;
                            source_ml_source2_data_property_value35633.clone()
                        }
                        self::JavaScriptFunctionBindTransition::FunctionBindCompleted(
                            source_ml_source2_data_property_value35634,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value35634 =
                                *source_ml_source2_data_property_value35634;
                            source_ml_source2_data_property_value35634.clone()
                        }
                        self::JavaScriptFunctionBindTransition::FunctionBindThrow(
                            source_ml_source2_data_property_value35635,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value35635 =
                                *source_ml_source2_data_property_value35635;
                            source_ml_source2_data_property_value35635.clone()
                        }
                        self::JavaScriptFunctionBindTransition::FunctionBindError(
                            source_ml_source2_data_property_value35636,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value35636 =
                                *source_ml_source2_data_property_value35636;
                            source_ml_source2_data_property_value35636.clone()
                        }
                        self::JavaScriptFunctionBindTransition::FunctionBindInvalid(
                            source_ml_source2_data_property_value35637,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value35637 =
                                *source_ml_source2_data_property_value35637;
                            source_ml_source2_data_property_value35637.clone()
                        }
                        self::JavaScriptFunctionBindTransition::FunctionBindResourceLimit(
                            source_ml_source2_data_property_value35638,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value35638 =
                                *source_ml_source2_data_property_value35638;
                            source_ml_source2_data_property_value35638.clone()
                        }
                    }),
                    shared_constructor_field2.clone(),
                );
                match current.clone() {
                    self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                        if match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(match transition.clone() {
                                    self::JavaScriptFunctionBindTransition::FunctionBindContinue(source_ml_source2_data_property_value35640, _) => {
                                        let source_ml_source2_data_property_value35640 = *source_ml_source2_data_property_value35640;
                                        source_ml_source2_data_property_value35640.clone()
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindAllocate(source_ml_source2_data_property_value35641, _, _) => {
                                        let source_ml_source2_data_property_value35641 = *source_ml_source2_data_property_value35641;
                                        source_ml_source2_data_property_value35641.clone()
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindCall(source_ml_source2_data_property_value35642, _, _, _, _) => {
                                        let source_ml_source2_data_property_value35642 = *source_ml_source2_data_property_value35642;
                                        source_ml_source2_data_property_value35642.clone()
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindCompleted(source_ml_source2_data_property_value35643, _) => {
                                        let source_ml_source2_data_property_value35643 = *source_ml_source2_data_property_value35643;
                                        source_ml_source2_data_property_value35643.clone()
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindThrow(source_ml_source2_data_property_value35644, _) => {
                                        let source_ml_source2_data_property_value35644 = *source_ml_source2_data_property_value35644;
                                        source_ml_source2_data_property_value35644.clone()
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindError(source_ml_source2_data_property_value35645, _, _) => {
                                        let source_ml_source2_data_property_value35645 = *source_ml_source2_data_property_value35645;
                                        source_ml_source2_data_property_value35645.clone()
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindInvalid(source_ml_source2_data_property_value35646, _) => {
                                        let source_ml_source2_data_property_value35646 = *source_ml_source2_data_property_value35646;
                                        source_ml_source2_data_property_value35646.clone()
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindResourceLimit(source_ml_source2_data_property_value35647, _, _) => {
                                        let source_ml_source2_data_property_value35647 = *source_ml_source2_data_property_value35647;
                                        source_ml_source2_data_property_value35647.clone()
                                    }
                                }), self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 102, 114, 97, 109, 101]))
                        } else {
                            match transition.clone() {
                                    self::JavaScriptFunctionBindTransition::FunctionBindContinue(heap, state) => {
                                        let heap = *heap;
                                        let state = *state;
                                        {
                                            let next = self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitFunctionBind(destination, realm.clone(), Box::new(state.clone()))));
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(next.clone()))
                                        }
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindAllocate(heap_2, state_2, source_ml_source2_f35648) => {
                                        let heap_2 = *heap_2;
                                        let state_2 = *state_2;
                                        let source_ml_source2_f35648 = *source_ml_source2_f35648;
                                        {
                                            let next_2 = self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitFunctionBind(destination, realm.clone(), Box::new(state_2.clone()))));
                                            self::JavaScriptCallMachineTransition::MachineAllocateObject(Box::new(next_2.clone()), Box::new(source_ml_source2_f35648.clone()))
                                        }
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindCall(heap_3, state_3, source_ml_source2_f35649, source_ml_source2_f35650, source_ml_source2_f35651) => {
                                        let heap_3 = *heap_3;
                                        let state_3 = *state_3;
                                        let source_ml_source2_f35649 = *source_ml_source2_f35649;
                                        let source_ml_source2_f35650 = *source_ml_source2_f35650;
                                        {
                                            let next_3 = self::replace_java_script_call_frame(heap_3.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitFunctionBind(destination, realm.clone(), Box::new(state_3.clone()))));
                                            self::push_java_script_call(program.clone(), next_3.clone(), source_ml_source2_f35649.clone(), source_ml_source2_f35650.clone(), source_ml_source2_f35651.clone())
                                        }
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindCompleted(heap_4, value) => {
                                        let heap_4 = *heap_4;
                                        let value = *value;
                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_4.clone(), shared_constructor_field1.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindThrow(_, value_2) => {
                                        let value_2 = *value_2;
                                        self::throw_java_script_call_value(current.clone(), value_2.clone())
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindError(_, error_class, reason) => {
                                        self::request_java_script_call_error(current.clone(), realm.clone(), &(error_class), &(reason))
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindInvalid(heap_5, reason_2) => {
                                        let heap_5 = *heap_5;
                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap_5.clone()), reason_2.clone())
                                    }
                                    self::JavaScriptFunctionBindTransition::FunctionBindResourceLimit(heap_6, state_4, source_ml_source2_f35654) => {
                                        let heap_6 = *heap_6;
                                        let state_4 = *state_4;
                                        {
                                            let next_4 = self::replace_java_script_call_frame(heap_6.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitFunctionBind(destination, realm.clone(), Box::new(state_4.clone()))));
                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(next_4.clone()), source_ml_source2_f35654.clone())
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
pub fn finish_java_script_call_function_name(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    object: self::JavaScriptSemanticValue,
    realm: self::ml::Big,
    name: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(shared_constructor_field1, _) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let entry = match object.clone() {
                    self::JavaScriptSemanticValue::Object(identity, _) => {
                        let source_ml_source2_array_read_value35685 =
                            match shared_constructor_field1.clone() {
                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                    source_ml_source2_data_property_value35682,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value35682.clone(),
                            };
                        {
                            let source_ml_source2_array_read_index35686 =
                                self::find_java_script_ordinary_object(
                                    match shared_constructor_field1.clone() {
                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                            source_ml_source2_data_property_value35684,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value35684.clone(),
                                    },
                                    identity.clone(),
                                );
                            {
                                let ml_reference_array2292 =
                                    source_ml_source2_array_read_value35685.clone();
                                let ml_reference_position2292 =
                                    source_ml_source2_array_read_index35686;
                                let ml_reference_projection2292 = self::ml_array::optional_at(
                                    &*ml_reference_array2292.borrow(),
                                    self::ml_array::number_index(ml_reference_position2292),
                                );
                                ml_reference_projection2292
                            }
                        }
                    }
                    _ => None::<self::JavaScriptOrdinaryObject>,
                };
                if ((match object.clone() {
                    self::JavaScriptSemanticValue::Object(_, _) => false,
                    _ => true,
                } || !(((match object.clone() {
                    self::JavaScriptSemanticValue::Object(
                        _,
                        source_ml_source2_data_property_value35689,
                    ) => Some((source_ml_source2_data_property_value35689,)),
                    _ => None::<(bool,)>,
                })
                .clone()
                .expect("a proven present value"))
                .0)) || {
                    let source_ml_source2_comparison_operand35691 = {
                        let source_ml_source2_optional_chain_receiver35690 = (entry.clone(),);
                        if match (source_ml_source2_optional_chain_receiver35690.clone()).0 {
                            Some(ml_present) => {
                                let _ = ml_present.clone();
                                true
                            }
                            None => false,
                        } {
                            Some({
                                let source_ml_source1_temporary_optional_chain1504 =
                                    ((source_ml_source2_optional_chain_receiver35690.clone()).0)
                                        .clone()
                                        .expect("a proven present value");
                                match source_ml_source1_temporary_optional_chain1504.clone() {
                                    self::JavaScriptOrdinaryObject::ClosureObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111,
                                        110, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116,
                                        101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            109, 97, 112, 79, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                        self::ml_string::Text::from_units(vec![
                                            115, 101, 116, 79, 98, 106, 101, 99, 116,
                                        ])
                                    }
                                    self::JavaScriptOrdinaryObject::WeakMapObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::WeakSetObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::OrdinaryObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101,
                                        99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostErrorObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106,
                                        101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostURLObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostArrayObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106,
                                        101, 99, 116,
                                    ]),
                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => self::ml_string::Text::from_units(vec![
                                        104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102,
                                        101, 114, 79, 98, 106, 101, 99, 116,
                                    ]),
                                }
                            })
                        } else {
                            None::<self::ml_string::Text>
                        }
                    };
                    !(match (
                        source_ml_source2_comparison_operand35691.clone(),
                        self::ml_string::Text::from_units(vec![
                            99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116,
                        ]),
                    ) {
                        (Some(ml_left), ml_right) => ml_left == ml_right,
                        _ => false,
                    })
                }) {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field1.clone()),
                        self::ml_string::Text::from_units(vec![
                            115, 111, 117, 114, 99, 101, 32, 102, 117, 110, 99, 116, 105, 111, 110,
                            32, 110, 97, 109, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115,
                            32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 119,
                            108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 117,
                            110, 116, 105, 109, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101,
                        ]),
                    )
                } else {
                    {
                        let descriptor = self::create_java_script_property_descriptor(
                            &(self::ml_string::Text::from_units(vec![100, 97, 116, 97])),
                            true,
                            false,
                        );
                        {
                            let source_ml_source1_temporary_finish_java_script_call_function_name_written1505 = {
                                let ml_field_owner2293 = descriptor.clone();
                                let ml_field_value2293 = name.clone();
                                ml_field_owner2293.borrow_mut().0.8 = ml_field_value2293;
                                ()
                            };
                            {
                                let source_ml_source1_temporary_finish_java_script_call_function_name_written1506 = {
                                    let ml_field_owner2294 = descriptor.clone();
                                    let ml_field_value2294 = false;
                                    ml_field_owner2294.borrow_mut().0.10 = ml_field_value2294;
                                    ()
                                };
                                self::accept_java_script_call_definition(
                                    program.clone(),
                                    machine.clone(),
                                    destination,
                                    realm.clone(),
                                    self::complete_java_script_property_definition_with_symbol(
                                        shared_constructor_field1.clone(),
                                        object.clone(),
                                        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                            std::cell::RefCell::new(vec![
                                                110f64, 97f64, 109f64, 101f64,
                                            ]),
                                        )),
                                        descriptor.clone(),
                                        Some(match program.clone() {
                                            self::JavaScriptCallProgram::CallProgram(
                                                _,
                                                _,
                                                source_ml_source2_data_property_value35695,
                                                _,
                                                _,
                                                _,
                                            ) => {
                                                let source_ml_source2_data_property_value35695 =
                                                    *source_ml_source2_data_property_value35695;
                                                source_ml_source2_data_property_value35695.clone()
                                            }
                                        }),
                                        None::<self::ml_string::Text>,
                                    ),
                                )
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
pub fn resume_java_script_call_function_name_symbol_description(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    description: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let error =
                    self::read_java_script_call_machine_error(program.clone(), machine.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field1.clone()),
                        error.clone(),
                    )
                } else {
                    {
                        let frame = {
                            let source_ml_source2_array_read_value35699 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index35700 = ({
                                    let ml_reference_array2295 = shared_constructor_field2.clone();
                                    let ml_reference_projection2295 =
                                        ml_reference_array2295.borrow().len() as f64;
                                    ml_reference_projection2295
                                } - 1f64);
                                {
                                    let ml_reference_array2296 =
                                        source_ml_source2_array_read_value35699.clone();
                                    let ml_reference_position2296 =
                                        source_ml_source2_array_read_index35700;
                                    let ml_reference_projection2296 = self::ml_array::optional_at(
                                        &*ml_reference_array2296.borrow(),
                                        self::ml_array::number_index(ml_reference_position2296),
                                    );
                                    ml_reference_projection2296
                                }
                            }
                        };
                        if match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    83, 121, 109, 98, 111, 108, 32, 102, 117, 110, 99, 116, 105,
                                    111, 110, 32, 110, 97, 109, 105, 110, 103, 32, 114, 101, 113,
                                    117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100,
                                    32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97,
                                    116, 105, 111, 110,
                                ]),
                            )
                        } else {
                            {
                                let pending = match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value35702,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value35702 =
                                            *source_ml_source2_data_property_value35702;
                                        source_ml_source2_data_property_value35702.clone()
                                    }
                                };
                                match pending.clone() {
                                    self::JavaScriptCallPending::WaitFunctionNameSymbol(
                                        destination,
                                        object,
                                        _,
                                        prefix,
                                        realm,
                                    ) => {
                                        let object = *object;
                                        {
                                            let named =
                                                self::construct_java_script_symbol_function_name(
                                                    description.clone(),
                                                    &(prefix),
                                                );
                                            match named.clone() {
                                                self::JavaScriptFunctionNameResult::FunctionNameInvalid(reason) => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), reason.clone())
                                                }
                                                self::JavaScriptFunctionNameResult::FunctionNameResourceLimit(reason_2) => {
                                                    self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), reason_2.clone())
                                                }
                                                _ => {
                                                    match named.clone() {
                                                        self::JavaScriptFunctionNameResult::FunctionNameValue(value) => {
                                                            let value = *value;
                                                            self::finish_java_script_call_function_name(program.clone(), machine.clone(), destination, object.clone(), realm.clone(), value.clone())
                                                        }
                                                        _ => {
                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 110, 97, 109, 101, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 109, 117, 115, 116, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 97, 115, 115, 101, 109, 98, 108, 121]))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field1.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            83, 121, 109, 98, 111, 108, 32, 102, 117, 110, 99, 116,
                                            105, 111, 110, 32, 110, 97, 109, 105, 110, 103, 32,
                                            114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116,
                                            115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115,
                                            112, 101, 110, 100, 101, 100, 32, 115, 111, 117, 114,
                                            99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111,
                                            110,
                                        ]),
                                    ),
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
pub fn resume_java_script_call_function_text(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let error = {
                    let source_ml_source2_logical35708 =
                        self::read_java_script_call_machine_error(program.clone(), machine.clone());
                    if !source_ml_source2_logical35708.0.is_empty() {
                        source_ml_source2_logical35708.clone()
                    } else {
                        self::read_java_script_ordinary_value_error(
                            shared_constructor_field1.clone(),
                            value.clone(),
                        )
                    }
                };
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field1.clone()),
                        error.clone(),
                    )
                } else {
                    {
                        let frame = {
                            let source_ml_source2_array_read_value35712 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index35713 = ({
                                    let ml_reference_array2297 = shared_constructor_field2.clone();
                                    let ml_reference_projection2297 =
                                        ml_reference_array2297.borrow().len() as f64;
                                    ml_reference_projection2297
                                } - 1f64);
                                {
                                    let ml_reference_array2298 =
                                        source_ml_source2_array_read_value35712.clone();
                                    let ml_reference_position2298 =
                                        source_ml_source2_array_read_index35713;
                                    let ml_reference_projection2298 = self::ml_array::optional_at(
                                        &*ml_reference_array2298.borrow(),
                                        self::ml_array::number_index(ml_reference_position2298),
                                    );
                                    ml_reference_projection2298
                                }
                            }
                        };
                        if (((match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } || {
                            let source_ml_source2_s35715 =
                                match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value35720,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value35720 =
                                            *source_ml_source2_data_property_value35720;
                                        source_ml_source2_data_property_value35720.clone()
                                    }
                                };
                            match source_ml_source2_s35715.clone() {
                                self::JavaScriptCallPending::WaitFunctionText(_, _, _, _) => false,
                                _ => true,
                            }
                        }) || (match value.clone() {
                            self::JavaScriptSemanticValue::Text(_) => false,
                            _ => true,
                        } && match value.clone() {
                            self::JavaScriptSemanticValue::SourceText(_, _) => false,
                            _ => true,
                        })) || ((self::read_java_script_semantic_text_length(value.clone()))
                            <= (0f64)))
                        {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    102, 117, 110, 99, 116, 105, 111, 110, 32, 116, 101, 120, 116,
                                    32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 115,
                                    117, 109, 101, 115, 32, 111, 110, 108, 121, 32, 105, 116, 115,
                                    32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101,
                                    110, 100, 101, 100, 32, 114, 101, 113, 117, 101, 115, 116, 32,
                                    119, 105, 116, 104, 32, 99, 111, 109, 112, 108, 101, 116, 101,
                                    32, 110, 111, 110, 101, 109, 112, 116, 121, 32, 116, 101, 120,
                                    116,
                                ]),
                            )
                        } else {
                            self::JavaScriptCallMachineTransition::MachineContinue(
                                Box::new(
                                    self::replace_java_script_call_frame(
                                        shared_constructor_field1.clone(),
                                        shared_constructor_field2.clone(),
                                        self::write_java_script_call_register(
                                            frame.clone().expect("a proven present value"),
                                            ((match match frame
                                                .clone()
                                                .expect("a proven present value")
                                            {
                                                self::JavaScriptCallFrame::CallFrame(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source2_data_property_value35727,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => {
                                                    let source_ml_source2_data_property_value35727 =
                                                        *source_ml_source2_data_property_value35727;
                                                    source_ml_source2_data_property_value35727
                                                        .clone()
                                                }
                                            } {
                                                self::JavaScriptCallPending::WaitFunctionText(
                                                    source_ml_source2_data_property_value35728,
                                                    _,
                                                    _,
                                                    _,
                                                ) => Some((
                                                    source_ml_source2_data_property_value35728,
                                                )),
                                                _ => None::<(f64,)>,
                                            })
                                            .clone()
                                            .expect("a proven present value"))
                                            .0,
                                            value.clone(),
                                        ),
                                    ),
                                ),
                            )
                        }
                    }
                }
            }
        }
    }
}
