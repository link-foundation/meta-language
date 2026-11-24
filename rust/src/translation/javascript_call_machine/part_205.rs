// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationFallback;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionState;
use super::JavaScriptObjectFromEntriesErrorRequest;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveWrapperState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::MlOptional;
use super::accept_java_script_call_array_construction;
use super::accept_java_script_call_array_creation;
use super::accept_java_script_call_construction;
use super::accept_java_script_call_constructor_check;
use super::accept_java_script_call_function_bind;
use super::accept_java_script_call_number;
use super::accept_java_script_call_object_from_entries;
use super::accept_java_script_call_primitive_wrapper;
use super::accept_java_script_call_text;
use super::advance_java_script_call_frame;
use super::begin_java_script_primitive_wrapper;
use super::begin_java_script_scalar_protocol;
use super::execute_java_script_call_primitive;
use super::ml_array;
use super::ml_source_exception;
use super::ml_string;
use super::read_java_script_call_machine_error;
use super::read_java_script_call_primitive_arity_error;
use super::read_java_script_executable_call_function;
use super::read_java_script_integer_protocol_operation_error;
use super::read_java_script_source_exception_class;
use super::read_java_script_symbol_registration_fact;
use super::replace_java_script_call_frame;
use super::request_java_script_call_array_fallback;
use super::request_java_script_call_error;
use super::request_java_script_call_source_error;
use super::resume_java_script_call_function_name_symbol_description;
use super::step_java_script_array_creation;
use super::step_java_script_call_pending_operation;
use super::step_java_script_construction;
use super::step_java_script_function_bind;
use super::step_java_script_object_from_entries;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn execute_java_script_call_primitive_instruction(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    instruction: self::JavaScriptCallInstruction,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            shared_constructor_field4,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(
                shared_constructor_field2,
                shared_constructor_field3,
            ) => {
                let shared_constructor_field2 = *shared_constructor_field2;
                match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(
                        _,
                        _,
                        shared_constructor_field1,
                        _,
                        _,
                        _,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        match instruction.clone() {
                            self::JavaScriptCallInstruction::Primitive(
                                destination,
                                operation,
                                arguments,
                            ) => {
                                let arity = self::read_java_script_call_primitive_arity_error(
                                    &(operation),
                                    {
                                        let ml_reference_array2398 = arguments.clone();
                                        let ml_reference_projection2398 =
                                            ml_reference_array2398.borrow().len() as f64;
                                        ml_reference_projection2398
                                    },
                                );
                                if (arity != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field2.clone()),
                                        arity.clone(),
                                    )
                                } else {
                                    {
                                        let arguments_ = std::rc::Rc::new(std::cell::RefCell::new(
                                            {
                                                let ml_values2399 = {
                                                    let ml_reference_value2400 =
                                                        arguments.borrow().clone();
                                                    ml_reference_value2400
                                                };
                                                let ml_callback2399 = {
                                                    let frame = frame.clone();
                                                    std::rc::Rc::new(move |index: f64| -> self::JavaScriptSemanticValue { ({
                                                    let source_ml_source2_array_read_value36291 = match frame.clone() {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value36290, _, _, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value36290.clone()
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_array_read_index36292 = index;
                                                        { let ml_reference_array2401 = source_ml_source2_array_read_value36291.clone(); let ml_reference_position2401 = source_ml_source2_array_read_index36292; let ml_reference_projection2401 = self::ml_array::optional_at(&*ml_reference_array2401.borrow(), self::ml_array::number_index(ml_reference_position2401)); ml_reference_projection2401 }
                                                    }
                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined) })
                                                };
                                                ml_values2399
                                                    .iter()
                                                    .cloned()
                                                    .enumerate()
                                                    .map(|(ml_index2399, ml_value2399)| {
                                                        ml_callback2399(ml_value2399)
                                                    })
                                                    .collect::<Vec<_>>()
                                            },
                                        ));
                                        {
                                            let input = ({
                                                let source_ml_source2_array_read_index36294 = 0f64;
                                                {
                                                    let ml_reference_array2402 = arguments_.clone();
                                                    let ml_reference_position2402 =
                                                        source_ml_source2_array_read_index36294;
                                                    let ml_reference_projection2402 =
                                                        self::ml_array::optional_at(
                                                            &*ml_reference_array2402.borrow(),
                                                            self::ml_array::number_index(
                                                                ml_reference_position2402,
                                                            ),
                                                        );
                                                    ml_reference_projection2402
                                                }
                                            })
                                            .clone()
                                            .unwrap_or_else(|| {
                                                self::JavaScriptSemanticValue::Undefined
                                            });
                                            if ((self::read_java_script_source_exception_class(
                                                &(operation),
                                            )) != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                self::request_java_script_call_source_error(
                                                    program.clone(),
                                                    machine.clone(),
                                                    frame.clone(),
                                                    instruction.clone(),
                                                )
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
                                                                let owner = self::read_java_script_executable_call_function(program.clone(), shared_constructor_field2.clone(), shared_constructor_field4.clone());
                                                                if match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![97, 32, 110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                                } else {
                                                                    {
                                                                        let current = self::replace_java_script_call_frame(shared_constructor_field2.clone(), shared_constructor_field3.clone(), self::advance_java_script_call_frame(frame.clone()));
                                                                        self::accept_java_script_call_number(program.clone(), current.clone(), destination, match owner.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value36302, _, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value36302.clone()
                                                                            }
                                                                        }, self::begin_java_script_scalar_protocol(shared_constructor_field2.clone(), input.clone(), shared_constructor_field1.clone(), &(operation)))
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
                                                                    let owner_2 = self::read_java_script_executable_call_function(program.clone(), shared_constructor_field2.clone(), shared_constructor_field4.clone());
                                                                    if match (owner_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![97, 32, 116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                                    } else {
                                                                        {
                                                                            let current_2 = self::replace_java_script_call_frame(shared_constructor_field2.clone(), shared_constructor_field3.clone(), self::advance_java_script_call_frame(frame.clone()));
                                                                            self::accept_java_script_call_text(program.clone(), current_2.clone(), destination, match owner_2.clone().expect("a proven present value") {
                                                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value36312, _, _, _, _, _, _, _, _) => {
                                                                                    source_ml_source2_data_property_value36312.clone()
                                                                                }
                                                                            }, self::begin_java_script_scalar_protocol(shared_constructor_field2.clone(), input.clone(), shared_constructor_field1.clone(), &(operation)))
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let result = self::execute_java_script_call_primitive(&(operation), arguments_.clone());
                                                                    match result.clone() {
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveValue(value) => {
                                                                            let value = *value;
                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field2.clone(), shared_constructor_field3.clone(), self::write_java_script_call_register(self::advance_java_script_call_frame(frame.clone()), destination, value.clone()))))
                                                                        }
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(reason) => {
                                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), reason.clone())
                                                                        }
                                                                        self::JavaScriptCallPrimitiveResult::PrimitiveError(error_class, reason_2) => {
                                                                            {
                                                                                let owner_3 = self::read_java_script_executable_call_function(program.clone(), shared_constructor_field2.clone(), shared_constructor_field4.clone());
                                                                                if match (owner_3.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field2.clone()), self::ml_string::Text::from_units(vec![97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 102, 97, 105, 108, 117, 114, 101, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101, 32, 111, 119, 110, 101, 114]))
                                                                                } else {
                                                                                    self::request_java_script_call_error(self::replace_java_script_call_frame(shared_constructor_field2.clone(), shared_constructor_field3.clone(), self::advance_java_script_call_frame(frame.clone())), match owner_3.clone().expect("a proven present value") {
                                                                                        self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value36323, _, _, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value36323.clone()
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
                            _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field2.clone()),
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 101,
                                    120, 101, 99, 117, 116, 111, 114, 32, 114, 101, 113, 117, 105,
                                    114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 105, 110,
                                    115, 116, 114, 117, 99, 116, 105, 111, 110,
                                ]),
                            ),
                        }
                    }
                }
            }
        },
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_call_machine(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field11 = *shared_constructor_field11;
            {
                let error =
                    self::read_java_script_call_machine_error(program.clone(), machine.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field11.clone()),
                        error.clone(),
                    )
                } else {
                    {
                        let frame = {
                            let source_ml_source2_array_read_value36328 =
                                shared_constructor_field12.clone();
                            {
                                let source_ml_source2_array_read_index36329 = ({
                                    let ml_reference_array2403 = shared_constructor_field12.clone();
                                    let ml_reference_projection2403 =
                                        ml_reference_array2403.borrow().len() as f64;
                                    ml_reference_projection2403
                                } - 1f64);
                                {
                                    let ml_reference_array2404 =
                                        source_ml_source2_array_read_value36328.clone();
                                    let ml_reference_position2404 =
                                        source_ml_source2_array_read_index36329;
                                    let ml_reference_projection2404 = self::ml_array::optional_at(
                                        &*ml_reference_array2404.borrow(),
                                        self::ml_array::number_index(ml_reference_position2404),
                                    );
                                    ml_reference_projection2404
                                }
                            }
                        };
                        if match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field11.clone()),
                                self::ml_string::Text::from_units(vec![
                                    97, 32, 99, 97, 108, 108, 32, 115, 116, 101, 112, 32, 110, 101,
                                    101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                    108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109,
                                    101,
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
                                        source_ml_source2_data_property_value36331,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value36331 =
                                            *source_ml_source2_data_property_value36331;
                                        source_ml_source2_data_property_value36331.clone()
                                    }
                                };
                                match pending.clone() {
                                    self::JavaScriptCallPending::WaitArrayCreation(
                                        destination,
                                        realm,
                                        state,
                                        pending_construction,
                                        pending_fallback,
                                        error_2,
                                    ) => {
                                        let state = *state;
                                        if !(match (
                                            ((error_2.clone().unwrap_or_else(|| {
                                                (None::<
                                                    self::JavaScriptObjectFromEntriesErrorRequest,
                                                >,)
                                            }))
                                            .0)
                                                .clone(),
                                            None::<()>,
                                        ) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        }) {
                                            self::JavaScriptCallMachineTransition::MachineAllocateError(Box::new(machine.clone()), {
                                                let source_ml_source2_nullish_field_receiver36337 = (((error_2.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(),);
                                                if match (source_ml_source2_nullish_field_receiver36337.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present36338 = ((source_ml_source2_nullish_field_receiver36337.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source2_nullish_field_present36338.clone() {
                                                            self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(source_ml_source2_data_property_value36339, _, _) => {
                                                                source_ml_source2_data_property_value36339.clone()
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 97, 108, 109, 39, 41])).0, message_present: true }).raise()
                                                }
                                            }, {
                                                let source_ml_source2_nullish_field_receiver36340 = (((error_2.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(),);
                                                if match (source_ml_source2_nullish_field_receiver36340.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present36341 = ((source_ml_source2_nullish_field_receiver36340.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source2_nullish_field_present36341.clone() {
                                                            self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(_, source_ml_source2_data_property_value36342, _) => {
                                                                source_ml_source2_data_property_value36342.clone()
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 114, 114, 111, 114, 67, 108, 97, 115, 115, 39, 41])).0, message_present: true }).raise()
                                                }
                                            }, {
                                                let source_ml_source2_nullish_field_receiver36343 = (((error_2.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(),);
                                                if match (source_ml_source2_nullish_field_receiver36343.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present36344 = ((source_ml_source2_nullish_field_receiver36343.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source2_nullish_field_present36344.clone() {
                                                            self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(_, _, source_ml_source2_data_property_value36345) => {
                                                                source_ml_source2_data_property_value36345.clone()
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 97, 115, 111, 110, 39, 41])).0, message_present: true }).raise()
                                                }
                                            })
                                        } else {
                                            {
                                                let construction = ((pending_construction
                                                    .clone()
                                                    .unwrap_or_else(|| {
                                                        (None::<self::JavaScriptConstructionState>,)
                                                    }))
                                                .0)
                                                    .clone();
                                                if !(match (construction.clone(), None::<()>) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                }) {
                                                    {
                                                        let directory = match program.clone() {
                                                            self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value36346, _) => {
                                                                ((source_ml_source2_data_property_value36346.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                            }
                                                        };
                                                        if match (directory.clone(), None::<()>) {
                                                            (Some(_ml_left), Some(_ml_right)) => {
                                                                false
                                                            }
                                                            (None, None) => true,
                                                            _ => false,
                                                        } {
                                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                                        } else {
                                                            self::accept_java_script_call_array_construction(program.clone(), machine.clone(), self::step_java_script_construction(program.clone(), shared_constructor_field11.clone(), directory.clone().expect("a proven present value"), construction.clone().expect("a proven present value")))
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        let fallback = ((pending_fallback.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCreationFallback>,))).0).clone();
                                                        if !(match (fallback.clone(), None::<()>) {
                                                            (Some(_ml_left), Some(_ml_right)) => {
                                                                false
                                                            }
                                                            (None, None) => true,
                                                            _ => false,
                                                        }) {
                                                            self::request_java_script_call_array_fallback(program.clone(), machine.clone(), destination, realm.clone(), state.clone(), fallback.clone().expect("a proven present value"))
                                                        } else {
                                                            self::accept_java_script_call_array_creation(program.clone(), machine.clone(), destination, realm.clone(), self::step_java_script_array_creation(shared_constructor_field11.clone(), state.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitConstruction(
                                        destination_2,
                                        realm_2,
                                        state_2,
                                        _,
                                    ) => {
                                        let state_2 = *state_2;
                                        {
                                            let directory_2 = match program.clone() {
                                                self::JavaScriptCallProgram::CallProgram(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source2_data_property_value36350,
                                                    _,
                                                ) => ((source_ml_source2_data_property_value36350
                                                    .clone()
                                                    .unwrap_or_else(|| {
                                                        (None::<
                                                            self::JavaScriptArraySpeciesDirectory,
                                                        >,)
                                                    }))
                                                .0)
                                                    .clone(),
                                            };
                                            if match (directory_2.clone(), None::<()>) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]))
                                            } else {
                                                self::accept_java_script_call_construction(
                                                    program.clone(),
                                                    machine.clone(),
                                                    destination_2,
                                                    realm_2.clone(),
                                                    self::step_java_script_construction(
                                                        program.clone(),
                                                        shared_constructor_field11.clone(),
                                                        directory_2
                                                            .clone()
                                                            .expect("a proven present value"),
                                                        state_2.clone(),
                                                    ),
                                                    None::<
                                                        std::rc::Rc<
                                                            std::cell::RefCell<
                                                                Vec<self::JavaScriptSemanticValue>,
                                                            >,
                                                        >,
                                                    >,
                                                )
                                            }
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitFunctionBind(
                                        destination_3,
                                        realm_3,
                                        state_3,
                                    ) => {
                                        let state_3 = *state_3;
                                        self::accept_java_script_call_function_bind(
                                            program.clone(),
                                            machine.clone(),
                                            destination_3,
                                            realm_3.clone(),
                                            self::step_java_script_function_bind(
                                                shared_constructor_field11.clone(),
                                                state_3.clone(),
                                            ),
                                        )
                                    }
                                    self::JavaScriptCallPending::WaitFunctionText(
                                        _,
                                        realm_4,
                                        input,
                                        kind,
                                    ) => {
                                        let input = *input;
                                        self::JavaScriptCallMachineTransition::MachineNeedFunctionText(Box::new(machine.clone()), Box::new(input.clone()), kind.clone(), realm_4.clone())
                                    }
                                    self::JavaScriptCallPending::WaitConstructorCheck(
                                        destination_4,
                                        input_2,
                                        required,
                                    ) => {
                                        let input_2 = *input_2;
                                        self::accept_java_script_call_constructor_check(
                                            program.clone(),
                                            machine.clone(),
                                            destination_4,
                                            input_2.clone(),
                                            required,
                                        )
                                    }
                                    self::JavaScriptCallPending::WaitObjectFromEntries(
                                        destination_5,
                                        realm_5,
                                        state_4,
                                        error_3,
                                    ) => {
                                        let state_4 = *state_4;
                                        if !(match (
                                            ((error_3.clone().unwrap_or_else(|| {
                                                (None::<
                                                    self::JavaScriptObjectFromEntriesErrorRequest,
                                                >,)
                                            }))
                                            .0)
                                                .clone(),
                                            None::<()>,
                                        ) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        }) {
                                            self::JavaScriptCallMachineTransition::MachineAllocateError(Box::new(machine.clone()), {
                                                let source_ml_source2_nullish_field_receiver36354 = (((error_3.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(),);
                                                if match (source_ml_source2_nullish_field_receiver36354.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present36355 = ((source_ml_source2_nullish_field_receiver36354.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source2_nullish_field_present36355.clone() {
                                                            self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(source_ml_source2_data_property_value36356, _, _) => {
                                                                source_ml_source2_data_property_value36356.clone()
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 97, 108, 109, 39, 41])).0, message_present: true }).raise()
                                                }
                                            }, {
                                                let source_ml_source2_nullish_field_receiver36357 = (((error_3.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(),);
                                                if match (source_ml_source2_nullish_field_receiver36357.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present36358 = ((source_ml_source2_nullish_field_receiver36357.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source2_nullish_field_present36358.clone() {
                                                            self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(_, source_ml_source2_data_property_value36359, _) => {
                                                                source_ml_source2_data_property_value36359.clone()
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 101, 114, 114, 111, 114, 67, 108, 97, 115, 115, 39, 41])).0, message_present: true }).raise()
                                                }
                                            }, {
                                                let source_ml_source2_nullish_field_receiver36360 = (((error_3.clone().unwrap_or_else(|| (None::<self::JavaScriptObjectFromEntriesErrorRequest>,))).0).clone(),);
                                                if match (source_ml_source2_nullish_field_receiver36360.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                    {
                                                        let source_ml_source2_nullish_field_present36361 = ((source_ml_source2_nullish_field_receiver36360.clone()).0).clone().expect("a proven present value");
                                                        match source_ml_source2_nullish_field_present36361.clone() {
                                                            self::JavaScriptObjectFromEntriesErrorRequest::ObjectFromEntriesErrorRequest(_, _, source_ml_source2_data_property_value36362) => {
                                                                source_ml_source2_data_property_value36362.clone()
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 97, 115, 111, 110, 39, 41])).0, message_present: true }).raise()
                                                }
                                            })
                                        } else {
                                            self::accept_java_script_call_object_from_entries(
                                                program.clone(),
                                                machine.clone(),
                                                destination_5,
                                                realm_5.clone(),
                                                self::step_java_script_object_from_entries(
                                                    shared_constructor_field11.clone(),
                                                    state_4.clone(),
                                                ),
                                            )
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitPrimitiveWrapper(
                                        destination_6,
                                        state_5,
                                    ) => {
                                        let state_5 = *state_5;
                                        match state_5.clone() {
                                            self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(shared_constructor_field1, shared_constructor_field2) => {
                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                self::accept_java_script_call_primitive_wrapper(machine.clone(), destination_6, self::begin_java_script_primitive_wrapper(shared_constructor_field11.clone(), shared_constructor_field1.clone(), shared_constructor_field2.clone()))
                                            }
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitSymbolAllocation(
                                        _,
                                        description,
                                        registry_key,
                                    ) => {
                                        let description = *description;
                                        if (({
                                            let ml_reference_array2405 =
                                                match shared_constructor_field11.clone() {
                                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                        _,
                                                        source_ml_source2_data_property_value36370,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source2_data_property_value36370
                                                        .clone(),
                                                };
                                            let ml_reference_projection2405 =
                                                ml_reference_array2405.borrow().len() as f64;
                                            ml_reference_projection2405
                                        }) >= (16384f64))
                                        {
                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 49, 54, 51, 56, 52, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 112, 114, 111, 102, 105, 108, 101]))
                                        } else {
                                            if match (
                                                ((registry_key.clone().unwrap_or_else(|| {
                                                    (None::<self::JavaScriptSemanticValue>,)
                                                }))
                                                .0)
                                                    .clone(),
                                                None::<()>,
                                            ) {
                                                (Some(_ml_left), Some(_ml_right)) => false,
                                                (None, None) => true,
                                                _ => false,
                                            } {
                                                self::JavaScriptCallMachineTransition::MachineAllocateSymbol(Box::new(machine.clone()), Box::new(description.clone()), None::<(Option<self::JavaScriptSemanticValue>,)>)
                                            } else {
                                                {
                                                    let source_ml_source2_constructor_field36375 =
                                                        ((registry_key.clone().unwrap_or_else(
                                                            || {
                                                                (None::<
                                                                    self::JavaScriptSemanticValue,
                                                                >,)
                                                            },
                                                        ))
                                                        .0)
                                                            .clone();
                                                    self::JavaScriptCallMachineTransition::MachineAllocateSymbol(Box::new(machine.clone()), Box::new(description.clone()), Some((source_ml_source2_constructor_field36375.clone(),)))
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitSymbolDescription(
                                        destination_7,
                                        symbol,
                                    ) => {
                                        let symbol = *symbol;
                                        match symbol.clone() {
                                            self::JavaScriptSemanticValue::Symbol(identity) => {
                                                {
                                                    let fact = self::read_java_script_symbol_registration_fact(shared_constructor_field11.clone(), identity.clone());
                                                    {
                                                        let key = if (!(match (fact.clone(), None::<()>) { (self::MlOptional::Value(_ml_left), Some(_ml_right)) => false, (self::MlOptional::Null, None) => false, (self::MlOptional::Undefined, None) => true, _ => false }) && {
                                                            let source_ml_source2_nullish_field_receiver36377 = (fact.clone(),);
                                                            if match (source_ml_source2_nullish_field_receiver36377.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                                                                {
                                                                    let source_ml_source2_nullish_field_present36378 = match (source_ml_source2_nullish_field_receiver36377.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                                                    match source_ml_source2_nullish_field_present36378.clone() {
                                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source2_data_property_value36379, _) => {
                                                                            source_ml_source2_data_property_value36379
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source2_nullish_field_receiver36377.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                                                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 39, 41])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 39, 41])
                                                                }).0, message_present: true }).raise()
                                                            }
                                                        }) {
                                                            {
                                                                let source_ml_source2_nullish_field_receiver36380 = (fact.clone(),);
                                                                if match (source_ml_source2_nullish_field_receiver36380.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                                                                    {
                                                                        let source_ml_source2_nullish_field_present36381 = match (source_ml_source2_nullish_field_receiver36380.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                                                        match source_ml_source2_nullish_field_present36381.clone() {
                                                                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source2_data_property_value36382) => {
                                                                                ((source_ml_source2_data_property_value36382.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source2_nullish_field_receiver36380.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                                                                        self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])
                                                                    }).0, message_present: true }).raise()
                                                                }
                                                            }
                                                        } else {
                                                            None::<self::JavaScriptSemanticValue>
                                                        };
                                                        if !(match (key.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), self::write_java_script_call_register({
                                                                let source_ml_source2_nominal_spread36385 = frame.clone().expect("a proven present value");
                                                                match source_ml_source2_nominal_spread36385.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1_2, shared_constructor_field2_2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                        let shared_constructor_field6 = *shared_constructor_field6;
                                                                        {
                                                                            let source_ml_source2_nominal_field36386 = shared_constructor_field1_2.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field36388 = shared_constructor_field2_2;
                                                                                {
                                                                                    let source_ml_source2_nominal_field36390 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field36392 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field36394 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field36396 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field36398 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field36400 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field36402 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field36404 = shared_constructor_field10.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override36406 = self::JavaScriptCallPending::CallIdle;
                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field36386.clone(), source_ml_source2_nominal_field36388, source_ml_source2_nominal_field36390.clone(), source_ml_source2_nominal_field36392.clone(), source_ml_source2_nominal_field36394.clone(), Box::new(source_ml_source2_nominal_override36406.clone()), source_ml_source2_nominal_field36398.clone(), source_ml_source2_nominal_field36400.clone(), source_ml_source2_nominal_field36402.clone(), source_ml_source2_nominal_field36404.clone())
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
                                                            }, destination_7, key.clone().expect("a proven present value")))))
                                                        } else {
                                                            self::JavaScriptCallMachineTransition::MachineNeedSymbolDescription(Box::new(machine.clone()), Box::new(symbol.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 83, 121, 109, 98, 111, 108]))
                                            }
                                        }
                                    }
                                    self::JavaScriptCallPending::WaitFunctionNameSymbol(
                                        _,
                                        object,
                                        symbol_2,
                                        prefix,
                                        _,
                                    ) => {
                                        let object = *object;
                                        let symbol_2 = *symbol_2;
                                        match symbol_2.clone() {
                                            self::JavaScriptSemanticValue::Symbol(identity_2) => {
                                                {
                                                    let fact_2 = self::read_java_script_symbol_registration_fact(shared_constructor_field11.clone(), identity_2.clone());
                                                    {
                                                        let key_2 = if (!(match (fact_2.clone(), None::<()>) { (self::MlOptional::Value(_ml_left), Some(_ml_right)) => false, (self::MlOptional::Null, None) => false, (self::MlOptional::Undefined, None) => true, _ => false }) && {
                                                            let source_ml_source2_nullish_field_receiver36409 = (fact_2.clone(),);
                                                            if match (source_ml_source2_nullish_field_receiver36409.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                                                                {
                                                                    let source_ml_source2_nullish_field_present36410 = match (source_ml_source2_nullish_field_receiver36409.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                                                    match source_ml_source2_nullish_field_present36410.clone() {
                                                                        self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source2_data_property_value36411, _) => {
                                                                            source_ml_source2_data_property_value36411
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source2_nullish_field_receiver36409.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                                                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 39, 41])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 39, 41])
                                                                }).0, message_present: true }).raise()
                                                            }
                                                        }) {
                                                            {
                                                                let source_ml_source2_nullish_field_receiver36412 = (fact_2.clone(),);
                                                                if match (source_ml_source2_nullish_field_receiver36412.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present.clone(); true }, self::MlOptional::Null => false, self::MlOptional::Undefined => false } {
                                                                    {
                                                                        let source_ml_source2_nullish_field_present36413 = match (source_ml_source2_nullish_field_receiver36412.clone()).0 { self::MlOptional::Value(ml_present) => ml_present, _ => unreachable!("a proven present value") };
                                                                        match source_ml_source2_nullish_field_present36413.clone() {
                                                                            self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, _, source_ml_source2_data_property_value36414) => {
                                                                                ((source_ml_source2_data_property_value36414.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                            }
                                                                        }
                                                                    }
                                                                } else {
                                                                    (self::ml_source_exception::SourceException { class_name: String::from("TypeError"), message_units: (if ((match (source_ml_source2_nullish_field_receiver36412.clone()).0 { self::MlOptional::Value(ml_present) => { let _ = ml_present; self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]) }, self::MlOptional::Null => self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]), self::MlOptional::Undefined => self::ml_string::Text::from_units(vec![117, 110, 100, 101, 102, 105, 110, 101, 100]) }) == (self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116]))) {
                                                                        self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 110, 117, 108, 108, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![67, 97, 110, 110, 111, 116, 32, 114, 101, 97, 100, 32, 112, 114, 111, 112, 101, 114, 116, 105, 101, 115, 32, 111, 102, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 40, 114, 101, 97, 100, 105, 110, 103, 32, 39, 107, 101, 121, 39, 41])
                                                                    }).0, message_present: true }).raise()
                                                                }
                                                            }
                                                        } else {
                                                            None::<self::JavaScriptSemanticValue>
                                                        };
                                                        if !(match (key_2.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                            self::resume_java_script_call_function_name_symbol_description(program.clone(), machine.clone(), key_2.clone().expect("a proven present value"))
                                                        } else {
                                                            self::JavaScriptCallMachineTransition::MachineNeedFunctionNameSymbolDescription(Box::new(machine.clone()), Box::new(object.clone()), Box::new(symbol_2.clone()), prefix.clone())
                                                        }
                                                    }
                                                }
                                            }
                                            _ => {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![83, 121, 109, 98, 111, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 105, 110, 103, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 83, 121, 109, 98, 111, 108]))
                                            }
                                        }
                                    }
                                    _ => self::step_java_script_call_pending_operation(
                                        program.clone(),
                                        machine.clone(),
                                        frame.clone().expect("a proven present value"),
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
