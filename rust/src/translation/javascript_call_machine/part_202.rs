// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptNumberCallState;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPropertyDefinitionTransition;
use super::JavaScriptPropertyKeyConversionTransition;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::JavaScriptTextCallState;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::push_java_script_call;
use super::read_java_script_executable_call_function;
use super::read_java_script_number_call_source_operation;
use super::read_java_script_property_definition_heap;
use super::replace_java_script_call_frame;
use super::replace_java_script_call_pending;
use super::request_java_script_call_error;
use super::resume_java_script_scalar_protocol;
use super::step_java_script_scalar_protocol;
use super::throw_java_script_call_value;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_key(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptPropertyKeyConversionTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = {
                    let source_ml_source2_array_read_value35731 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_array_read_index35732 = ({
                            let ml_reference_array2299 = shared_constructor_field2.clone();
                            let ml_reference_projection2299 =
                                ml_reference_array2299.borrow().len() as f64;
                            ml_reference_projection2299
                        } - 1f64);
                        {
                            let ml_reference_array2300 =
                                source_ml_source2_array_read_value35731.clone();
                            let ml_reference_position2300 = source_ml_source2_array_read_index35732;
                            let ml_reference_projection2300 = self::ml_array::optional_at(
                                &*ml_reference_array2300.borrow(),
                                self::ml_array::number_index(ml_reference_position2300),
                            );
                            ml_reference_projection2300
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
                            107, 101, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                            110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                            108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101,
                        ]),
                    )
                } else {
                    match transition.clone() {
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state.clone()))))))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCall(state_2, callee, receiver, arguments) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitKey(destination, realm.clone(), Box::new(state_2.clone())))), callee.clone(), receiver.clone(), arguments.clone())
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyCompleted(key) => {
                            let key = *key;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, key.clone()))))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyThrow(value) => {
                            let value = *value;
                            self::throw_java_script_call_value(machine.clone(), value.clone())
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyIntrinsicError(error_class, reason) => {
                            self::request_java_script_call_error(machine.clone(), realm.clone(), &(error_class), &(reason))
                        }
                        self::JavaScriptPropertyKeyConversionTransition::PropertyKeyInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), reason_2.clone())
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_definition(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptPropertyDefinitionTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field2,
            shared_constructor_field3,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let heap = self::read_java_script_property_definition_heap(
                    transition.clone(),
                    shared_constructor_field2.clone(),
                );
                {
                    let current = self::JavaScriptCallMachine::CallMachine(
                        Box::new(heap.clone()),
                        shared_constructor_field3.clone(),
                    );
                    match current.clone() {
                        self::JavaScriptCallMachine::CallMachine(_, shared_constructor_field1) => {
                            let frame = {
                                let source_ml_source2_array_read_value35745 =
                                    shared_constructor_field1.clone();
                                {
                                    let source_ml_source2_array_read_index35746 = ({
                                        let ml_reference_array2301 =
                                            shared_constructor_field1.clone();
                                        let ml_reference_projection2301 =
                                            ml_reference_array2301.borrow().len() as f64;
                                        ml_reference_projection2301
                                    } - 1f64);
                                    {
                                        let ml_reference_array2302 =
                                            source_ml_source2_array_read_value35745.clone();
                                        let ml_reference_position2302 =
                                            source_ml_source2_array_read_index35746;
                                        let ml_reference_projection2302 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array2302.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position2302,
                                                ),
                                            );
                                        ml_reference_projection2302
                                    }
                                }
                            };
                            if match (frame.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::JavaScriptCallMachineTransition::MachineInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 101, 102,
                                        105, 110, 105, 116, 105, 111, 110, 32, 110, 101, 101, 100,
                                        115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                        97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109, 101,
                                    ]),
                                )
                            } else {
                                match transition.clone() {
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionContinuation(state, _) => {
                                            let state = *state;
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state.clone()))))))
                                        }
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionCall(state_2, callee, receiver, arguments, _) => {
                                            let state_2 = *state_2;
                                            let callee = *callee;
                                            let receiver = *receiver;
                                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state_2.clone())))), callee.clone(), receiver.clone(), arguments.clone())
                                        }
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionCompleted(heap_2, value, _) => {
                                            let heap_2 = *heap_2;
                                            let value = *value;
                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(heap_2.clone(), shared_constructor_field3.clone(), self::write_java_script_call_register(frame.clone().expect("a proven present value"), destination, value.clone()))))
                                        }
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionThrow(value_2, _) => {
                                            let value_2 = *value_2;
                                            self::throw_java_script_call_value(current.clone(), value_2.clone())
                                        }
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionIntrinsicError(error_class, reason, _, _) => {
                                            self::request_java_script_call_error(current.clone(), realm.clone(), &(error_class), &(reason))
                                        }
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionInvalid(reason_2, _) => {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(heap.clone()), reason_2.clone())
                                        }
                                        self::JavaScriptPropertyDefinitionTransition::DefinitionResourceLimit(_, state_3, reason_3) => {
                                            let state_3 = *state_3;
                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(self::replace_java_script_call_frame(heap.clone(), shared_constructor_field1.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitDefinition(destination, realm.clone(), Box::new(state_3.clone()))))), reason_3.clone())
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
pub fn accept_java_script_call_number(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptScalarConversionTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = {
                    let source_ml_source2_array_read_value35761 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_array_read_index35762 = ({
                            let ml_reference_array2303 = shared_constructor_field2.clone();
                            let ml_reference_projection2303 =
                                ml_reference_array2303.borrow().len() as f64;
                            ml_reference_projection2303
                        } - 1f64);
                        {
                            let ml_reference_array2304 =
                                source_ml_source2_array_read_value35761.clone();
                            let ml_reference_position2304 = source_ml_source2_array_read_index35762;
                            let ml_reference_projection2304 = self::ml_array::optional_at(
                                &*ml_reference_array2304.borrow(),
                                self::ml_array::number_index(ml_reference_position2304),
                            );
                            ml_reference_projection2304
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
                            110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105,
                            111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99,
                            116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97,
                            109, 101,
                        ]),
                    )
                } else {
                    match transition.clone() {
                        self::JavaScriptScalarConversionTransition::ScalarContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state.clone()))))))))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCall(
                            state_2,
                            callee,
                            receiver,
                            arguments,
                        ) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(program.clone(), self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberProtocolState(Box::new(state_2.clone())))))), callee.clone(), receiver.clone(), arguments.clone())
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCompleted(value) => {
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    shared_constructor_field1.clone(),
                                    shared_constructor_field2.clone(),
                                    self::write_java_script_call_register(
                                        frame.clone().expect("a proven present value"),
                                        destination,
                                        value.clone(),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(machine.clone(), value_2.clone())
                        }
                        self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(
                            error_class,
                            reason,
                        ) => self::request_java_script_call_error(
                            machine.clone(),
                            realm.clone(),
                            &(error_class),
                            &(reason),
                        ),
                        self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                reason_2.clone(),
                            )
                        }
                        self::JavaScriptScalarConversionTransition::ScalarResourceLimit(
                            value_3,
                            reason_3,
                        ) => {
                            let value_3 = *value_3;
                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), self::replace_java_script_call_pending(frame.clone().expect("a proven present value"), self::JavaScriptCallPending::WaitNumber(destination, realm.clone(), Box::new(self::JavaScriptNumberCallState::NumberPrimitiveValue(Box::new(value_3.clone()))))))), reason_3.clone())
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_call_number(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    state: self::JavaScriptNumberCallState,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = {
                    let source_ml_source2_array_read_value35775 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_array_read_index35776 = ({
                            let ml_reference_array2305 = shared_constructor_field2.clone();
                            let ml_reference_projection2305 =
                                ml_reference_array2305.borrow().len() as f64;
                            ml_reference_projection2305
                        } - 1f64);
                        {
                            let ml_reference_array2306 =
                                source_ml_source2_array_read_value35775.clone();
                            let ml_reference_position2306 = source_ml_source2_array_read_index35776;
                            let ml_reference_projection2306 = self::ml_array::optional_at(
                                &*ml_reference_array2306.borrow(),
                                self::ml_array::number_index(ml_reference_position2306),
                            );
                            ml_reference_projection2306
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
                            110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105,
                            111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99,
                            116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97,
                            109, 101,
                        ]),
                    )
                } else {
                    {
                        let fn_ = self::read_java_script_executable_call_function(
                            program.clone(),
                            shared_constructor_field1.clone(),
                            match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(
                                    source_ml_source2_data_property_value35779,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value35779.clone(),
                            },
                        );
                        if match (fn_.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114,
                                    115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116,
                                    115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99,
                                    101, 32, 102, 117, 110, 99, 116, 105, 111, 110,
                                ]),
                            )
                        } else {
                            {
                                let operation = self::read_java_script_number_call_source_operation(
                                    fn_.clone().expect("a proven present value"),
                                    frame.clone().expect("a proven present value"),
                                    destination,
                                    realm.clone(),
                                    match program.clone() {
                                        self::JavaScriptCallProgram::CallProgram(
                                            _,
                                            _,
                                            source_ml_source2_data_property_value35781,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let source_ml_source2_data_property_value35781 =
                                                *source_ml_source2_data_property_value35781;
                                            source_ml_source2_data_property_value35781.clone()
                                        }
                                    },
                                    state.clone(),
                                );
                                if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field1.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109,
                                            98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115,
                                            105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115,
                                            32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                            115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116,
                                            114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99,
                                            101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111,
                                            108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109,
                                        ]),
                                    )
                                } else {
                                    match state.clone() {
                                        self::JavaScriptNumberCallState::NumberProtocolState(
                                            state_state,
                                        ) => {
                                            let state_state = *state_state;
                                            self::accept_java_script_call_number(
                                                program.clone(),
                                                machine.clone(),
                                                destination,
                                                realm.clone(),
                                                self::step_java_script_scalar_protocol(
                                                    shared_constructor_field1.clone(),
                                                    state_state.clone(),
                                                    &(operation),
                                                ),
                                            )
                                        }
                                        self::JavaScriptNumberCallState::NumberPrimitiveValue(
                                            value,
                                        ) => {
                                            let value = *value;
                                            self::accept_java_script_call_number(
                                                program.clone(),
                                                machine.clone(),
                                                destination,
                                                realm.clone(),
                                                self::finish_java_script_scalar_primitive(
                                                    &(operation),
                                                    value.clone(),
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_number(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    state: self::JavaScriptNumberCallState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = {
                    let source_ml_source2_array_read_value35786 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_array_read_index35787 = ({
                            let ml_reference_array2307 = shared_constructor_field2.clone();
                            let ml_reference_projection2307 =
                                ml_reference_array2307.borrow().len() as f64;
                            ml_reference_projection2307
                        } - 1f64);
                        {
                            let ml_reference_array2308 =
                                source_ml_source2_array_read_value35786.clone();
                            let ml_reference_position2308 = source_ml_source2_array_read_index35787;
                            let ml_reference_projection2308 = self::ml_array::optional_at(
                                &*ml_reference_array2308.borrow(),
                                self::ml_array::number_index(ml_reference_position2308),
                            );
                            ml_reference_projection2308
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
                            110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105,
                            111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99,
                            116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97,
                            109, 101,
                        ]),
                    )
                } else {
                    {
                        let fn_ = self::read_java_script_executable_call_function(
                            program.clone(),
                            shared_constructor_field1.clone(),
                            match frame.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(
                                    source_ml_source2_data_property_value35790,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value35790.clone(),
                            },
                        );
                        if match (fn_.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                self::ml_string::Text::from_units(vec![
                                    110, 117, 109, 98, 101, 114, 32, 99, 111, 110, 118, 101, 114,
                                    115, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116,
                                    115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99,
                                    101, 32, 102, 117, 110, 99, 116, 105, 111, 110,
                                ]),
                            )
                        } else {
                            {
                                let operation = self::read_java_script_number_call_source_operation(
                                    fn_.clone().expect("a proven present value"),
                                    frame.clone().expect("a proven present value"),
                                    destination,
                                    realm.clone(),
                                    match program.clone() {
                                        self::JavaScriptCallProgram::CallProgram(
                                            _,
                                            _,
                                            source_ml_source2_data_property_value35792,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let source_ml_source2_data_property_value35792 =
                                                *source_ml_source2_data_property_value35792;
                                            source_ml_source2_data_property_value35792.clone()
                                        }
                                    },
                                    state.clone(),
                                );
                                if (operation == (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field1.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            112, 101, 110, 100, 105, 110, 103, 32, 110, 117, 109,
                                            98, 101, 114, 32, 99, 111, 110, 118, 101, 114, 115,
                                            105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115,
                                            32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32,
                                            115, 111, 117, 114, 99, 101, 32, 105, 110, 115, 116,
                                            114, 117, 99, 116, 105, 111, 110, 44, 32, 114, 101, 99,
                                            101, 105, 118, 101, 114, 44, 32, 83, 121, 109, 98, 111,
                                            108, 32, 97, 110, 100, 32, 114, 101, 97, 108, 109,
                                        ]),
                                    )
                                } else {
                                    match state.clone() {
                                        self::JavaScriptNumberCallState::NumberProtocolState(
                                            state_state,
                                        ) => {
                                            let state_state = *state_state;
                                            self::accept_java_script_call_number(program.clone(), machine.clone(), destination, realm.clone(), self::resume_java_script_scalar_protocol(shared_constructor_field1.clone(), state_state.clone(), self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(Box::new(value.clone())), &(operation)))
                                        }
                                        self::JavaScriptNumberCallState::NumberPrimitiveValue(
                                            _,
                                        ) => self::JavaScriptCallMachineTransition::MachineInvalid(
                                            Box::new(shared_constructor_field1.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                97, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100,
                                                32, 110, 117, 109, 101, 114, 105, 99, 32, 112, 114,
                                                105, 109, 105, 116, 105, 118, 101, 32, 99, 97, 110,
                                                110, 111, 116, 32, 104, 97, 118, 101, 32, 97, 32,
                                                115, 117, 115, 112, 101, 110, 100, 101, 100, 32,
                                                99, 104, 105, 108, 100, 32, 99, 97, 108, 108,
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_call_text(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    destination: f64,
    realm: self::ml::Big,
    transition: self::JavaScriptScalarConversionTransition,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let frame = {
                    let source_ml_source2_array_read_value35799 = shared_constructor_field2.clone();
                    {
                        let source_ml_source2_array_read_index35800 = ({
                            let ml_reference_array2309 = shared_constructor_field2.clone();
                            let ml_reference_projection2309 =
                                ml_reference_array2309.borrow().len() as f64;
                            ml_reference_projection2309
                        } - 1f64);
                        {
                            let ml_reference_array2310 =
                                source_ml_source2_array_read_value35799.clone();
                            let ml_reference_position2310 = source_ml_source2_array_read_index35800;
                            let ml_reference_projection2310 = self::ml_array::optional_at(
                                &*ml_reference_array2310.borrow(),
                                self::ml_array::number_index(ml_reference_position2310),
                            );
                            ml_reference_projection2310
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
                            116, 101, 120, 116, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                            110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                            117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102, 114, 97, 109,
                            101,
                        ]),
                    )
                } else {
                    match transition.clone() {
                        self::JavaScriptScalarConversionTransition::ScalarContinuation(state) => {
                            let state = *state;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    shared_constructor_field1.clone(),
                                    shared_constructor_field2.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitText(
                                            destination,
                                            realm.clone(),
                                            Box::new(
                                                self::JavaScriptTextCallState::TextProtocolState(
                                                    Box::new(state.clone()),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCall(
                            state_2,
                            callee,
                            receiver,
                            arguments,
                        ) => {
                            let state_2 = *state_2;
                            let callee = *callee;
                            let receiver = *receiver;
                            self::push_java_script_call(
                                program.clone(),
                                self::replace_java_script_call_frame(
                                    shared_constructor_field1.clone(),
                                    shared_constructor_field2.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitText(
                                            destination,
                                            realm.clone(),
                                            Box::new(
                                                self::JavaScriptTextCallState::TextProtocolState(
                                                    Box::new(state_2.clone()),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                                callee.clone(),
                                receiver.clone(),
                                arguments.clone(),
                            )
                        }
                        self::JavaScriptScalarConversionTransition::ScalarCompleted(value) => {
                            let value = *value;
                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                                self::replace_java_script_call_frame(
                                    shared_constructor_field1.clone(),
                                    shared_constructor_field2.clone(),
                                    self::write_java_script_call_register(
                                        frame.clone().expect("a proven present value"),
                                        destination,
                                        value.clone(),
                                    ),
                                ),
                            ))
                        }
                        self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                            let value_2 = *value_2;
                            self::throw_java_script_call_value(machine.clone(), value_2.clone())
                        }
                        self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(
                            error_class,
                            reason,
                        ) => self::request_java_script_call_error(
                            machine.clone(),
                            realm.clone(),
                            &(error_class),
                            &(reason),
                        ),
                        self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field1.clone()),
                                reason_2.clone(),
                            )
                        }
                        self::JavaScriptScalarConversionTransition::ScalarResourceLimit(
                            value_3,
                            reason_3,
                        ) => {
                            let value_3 = *value_3;
                            self::JavaScriptCallMachineTransition::MachineResourceLimit(
                                Box::new(self::replace_java_script_call_frame(
                                    shared_constructor_field1.clone(),
                                    shared_constructor_field2.clone(),
                                    self::replace_java_script_call_pending(
                                        frame.clone().expect("a proven present value"),
                                        self::JavaScriptCallPending::WaitText(
                                            destination,
                                            realm.clone(),
                                            Box::new(
                                                self::JavaScriptTextCallState::TextPrimitiveValue(
                                                    Box::new(value_3.clone()),
                                                ),
                                            ),
                                        ),
                                    ),
                                )),
                                reason_3.clone(),
                            )
                        }
                    }
                }
            }
        }
    }
}
