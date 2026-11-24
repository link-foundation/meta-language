// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAuthorityUpdate;
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptSpeciesConstructionState;
use super::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityJoin1420;
use super::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_species_directory_error;
use super::read_java_script_call_array_transform_directory;
use super::read_java_script_call_machine_error;
use super::source_ml_source1_temporary_extend_java_script_call_array_authority_loop1421;
use super::step_java_script_call_machine;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn replace_java_script_instruction_species(
    instruction: self::JavaScriptCallInstruction,
    species: self::JavaScriptArraySpeciesDirectory,
) -> self::JavaScriptCallInstruction {
    match instruction.clone() {
        self::JavaScriptCallInstruction::ArrayConcatenation(
            destination,
            instruction_input,
            spreadable_key,
            _,
        ) => {
            let input = ((instruction_input.clone().unwrap_or_else(|| (None::<f64>,))).0).clone();
            if {
                let source_ml_source2_comparison_operand33252 = None::<()>;
                match (
                    input.clone(),
                    source_ml_source2_comparison_operand33252.clone(),
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }
            } {
                self::JavaScriptCallInstruction::ArrayConcatenation(
                    destination,
                    None::<(Option<f64>,)>,
                    spreadable_key,
                    Box::new(species.clone()),
                )
            } else {
                {
                    let source_ml_source2_constructor_field33259 =
                        Some(input.clone().expect("a proven present value"));
                    self::JavaScriptCallInstruction::ArrayConcatenation(
                        destination,
                        Some((source_ml_source2_constructor_field33259.clone(),)),
                        spreadable_key,
                        Box::new(species.clone()),
                    )
                }
            }
        }
        self::JavaScriptCallInstruction::ArraySlice(destination_2, instruction_input_2, _) => {
            let input_2 = ((instruction_input_2
                .clone()
                .unwrap_or_else(|| (None::<f64>,)))
            .0)
                .clone();
            if {
                let source_ml_source2_comparison_operand33261 = None::<()>;
                match (
                    input_2.clone(),
                    source_ml_source2_comparison_operand33261.clone(),
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }
            } {
                self::JavaScriptCallInstruction::ArraySlice(
                    destination_2,
                    None::<(Option<f64>,)>,
                    Box::new(species.clone()),
                )
            } else {
                {
                    let source_ml_source2_constructor_field33266 =
                        Some(input_2.clone().expect("a proven present value"));
                    self::JavaScriptCallInstruction::ArraySlice(
                        destination_2,
                        Some((source_ml_source2_constructor_field33266.clone(),)),
                        Box::new(species.clone()),
                    )
                }
            }
        }
        self::JavaScriptCallInstruction::ArrayTransform(
            destination_3,
            operation,
            _,
            instruction_input_3,
        ) => {
            let input_3 = ((instruction_input_3
                .clone()
                .unwrap_or_else(|| (None::<f64>,)))
            .0)
                .clone();
            if {
                let source_ml_source2_comparison_operand33268 = None::<()>;
                match (
                    input_3.clone(),
                    source_ml_source2_comparison_operand33268.clone(),
                ) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }
            } {
                self::JavaScriptCallInstruction::ArrayTransform(
                    destination_3,
                    operation.clone(),
                    Box::new(species.clone()),
                    None::<(Option<f64>,)>,
                )
            } else {
                {
                    let source_ml_source2_constructor_field33275 =
                        Some(input_3.clone().expect("a proven present value"));
                    self::JavaScriptCallInstruction::ArrayTransform(
                        destination_3,
                        operation.clone(),
                        Box::new(species.clone()),
                        Some((source_ml_source2_constructor_field33275.clone(),)),
                    )
                }
            }
        }
        _ => instruction.clone(),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn extend_java_script_call_array_authority(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    directory: self::JavaScriptArraySpeciesDirectory,
) -> self::JavaScriptArrayAuthorityUpdate {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field7,
            shared_constructor_field8,
        ) => {
            let shared_constructor_field7 = *shared_constructor_field7;
            match program.clone() {
                self::JavaScriptCallProgram::CallProgram(
                    shared_constructor_field1,
                    shared_constructor_field2,
                    shared_constructor_field3,
                    shared_constructor_field4,
                    shared_constructor_field5,
                    shared_constructor_field6,
                ) => {
                    let shared_constructor_field3 = *shared_constructor_field3;
                    {
                        let error = self::read_java_script_call_machine_error(
                            program.clone(),
                            machine.clone(),
                        );
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(
                                Box::new(program.clone()),
                                Box::new(machine.clone()),
                                error.clone(),
                            )
                        } else {
                            {
                                let frame = {
                                    let source_ml_source2_array_read_value33278 =
                                        shared_constructor_field8.clone();
                                    {
                                        let source_ml_source2_array_read_index33279 = ({
                                            let ml_reference_array2168 =
                                                shared_constructor_field8.clone();
                                            let ml_reference_projection2168 =
                                                ml_reference_array2168.borrow().len() as f64;
                                            ml_reference_projection2168
                                        } - 1f64);
                                        {
                                            let ml_reference_array2169 =
                                                source_ml_source2_array_read_value33278.clone();
                                            let ml_reference_position2169 =
                                                source_ml_source2_array_read_index33279;
                                            let ml_reference_projection2169 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array2169.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position2169,
                                                    ),
                                                );
                                            ml_reference_projection2169
                                        }
                                    }
                                };
                                if match (frame.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(
                                        Box::new(program.clone()),
                                        Box::new(machine.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                                            32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32,
                                            114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116,
                                            115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115,
                                            112, 101, 110, 100, 101, 100, 32, 115, 111, 117, 114,
                                            99, 101, 32, 102, 114, 97, 109, 101,
                                        ]),
                                    )
                                } else {
                                    {
                                        let pending =
                                            match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source2_data_property_value33280,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => {
                                                    let source_ml_source2_data_property_value33280 =
                                                        *source_ml_source2_data_property_value33280;
                                                    source_ml_source2_data_property_value33280
                                                        .clone()
                                                }
                                            };
                                        {
                                            let global = ((match pending.clone() {
                                                self::JavaScriptCallPending::WaitConstruction(_, _, _, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            } || match pending.clone() {
                                                self::JavaScriptCallPending::WaitConstructorCheck(_, _, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            }) || match pending.clone() {
                                                self::JavaScriptCallPending::WaitArrayCreation(_, _, _, _, _, _) => {
                                                    true
                                                }
                                                _ => {
                                                    false
                                                }
                                            });
                                            if (((!global && match pending.clone() {
                                                self::JavaScriptCallPending::WaitArrayTransform(_, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) && match pending.clone() {
                                                self::JavaScriptCallPending::WaitArraySlice(_, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) && match pending.clone() {
                                                self::JavaScriptCallPending::WaitArrayConcatenation(_, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) {
                                                self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(Box::new(program.clone()), Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 101, 120, 116, 101, 110, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111, 110, 32, 111, 114, 32, 114, 101, 97, 108, 109, 32, 108, 111, 111, 107, 117, 112]))
                                            } else {
                                                {
                                                    let request = self::step_java_script_call_machine(program.clone(), machine.clone());
                                                    match request.clone() {
                                                        self::JavaScriptCallMachineTransition::MachineNeedConstructorAuthority(_, _) => {
                                                            {
                                                                let prior = if ((match pending.clone() {
                                                                    self::JavaScriptCallPending::WaitArrayTransform(_, _, _, _) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                } || match pending.clone() {
                                                                    self::JavaScriptCallPending::WaitArraySlice(_, _, _, _) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                }) || match pending.clone() {
                                                                    self::JavaScriptCallPending::WaitArrayConcatenation(_, _, _, _) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                }) {
                                                                    self::read_java_script_call_array_transform_directory(program.clone(), machine.clone(), ((match pending.clone() {
                                                                        self::JavaScriptCallPending::WaitArrayConcatenation(source_ml_source2_data_property_value33320, _, _, _) => {
                                                                            Some((source_ml_source2_data_property_value33320,))
                                                                        }
                                                                        self::JavaScriptCallPending::WaitArraySlice(source_ml_source2_data_property_value33321, _, _, _) => {
                                                                            Some((source_ml_source2_data_property_value33321,))
                                                                        }
                                                                        self::JavaScriptCallPending::WaitArrayTransform(source_ml_source2_data_property_value33322, _, _, _) => {
                                                                            Some((source_ml_source2_data_property_value33322,))
                                                                        }
                                                                        _ => {
                                                                            None::<(f64,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0, (((match pending.clone() {
                                                                        self::JavaScriptCallPending::WaitArrayConcatenation(_, source_ml_source2_data_property_value33323, _, _) => {
                                                                            Some((source_ml_source2_data_property_value33323.clone(),))
                                                                        }
                                                                        self::JavaScriptCallPending::WaitArraySlice(_, source_ml_source2_data_property_value33324, _, _) => {
                                                                            Some((source_ml_source2_data_property_value33324.clone(),))
                                                                        }
                                                                        self::JavaScriptCallPending::WaitArrayTransform(_, source_ml_source2_data_property_value33325, _, _) => {
                                                                            Some((source_ml_source2_data_property_value33325.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::ml::Big,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone())
                                                                } else {
                                                                    match program.clone() {
                                                                        self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value33326, _) => {
                                                                            ((source_ml_source2_data_property_value33326.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                        }
                                                                    }
                                                                };
                                                                if match (prior.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(Box::new(program.clone()), Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 105, 111, 114, 32, 115, 111, 117, 114, 99, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                                                } else {
                                                                    {
                                                                        let invalid = self::read_java_script_array_species_directory_error(shared_constructor_field7.clone(), directory.clone());
                                                                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(Box::new(program.clone()), Box::new(machine.clone()), invalid.clone())
                                                                        } else {
                                                                            {
                                                                                let root_directory = match program.clone() {
                                                                                    self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value33328, _) => {
                                                                                        ((source_ml_source2_data_property_value33328.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let snapshots = std::rc::Rc::new(std::cell::RefCell::new(vec![Some(prior.clone().expect("a proven present value"))]));
                                                                                    {
                                                                                        let source_ml_source2_s33329 = if (!(match (root_directory.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !global) {
                                                                                            {
                                                                                                let source_ml_source1_temporary_extend_java_script_call_array_authority_ignored1425 = { let ml_method_owner2170 = snapshots.clone(); let ml_method_argument2170_0 = Some(root_directory.clone().expect("a proven present value")); let ml_inserted = vec![ml_method_argument2170_0]; let mut ml_method_values2170 = ml_method_owner2170.borrow_mut(); let ml_length = ml_method_values2170.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values2170.extend(ml_inserted); ml_length as f64 };
                                                                                                self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityJoin1420::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityJoin1420Next
                                                                                            }
                                                                                        } else {
                                                                                            self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityJoin1420::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityJoin1420Next
                                                                                        };
                                                                                        match source_ml_source2_s33329.clone() {
                                                                                            self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityJoin1420::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityJoin1420Next => {
                                                                                                {
                                                                                                    let source_ml_source1_temporary_values1417 = snapshots.clone();
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_index1417 = self::ml::Big::from_i128(0);
                                                                                                        {
                                                                                                            let source_ml_source2_s33330 = self::source_ml_source1_temporary_extend_java_script_call_array_authority_loop1421(program.clone(), machine.clone(), directory.clone(), source_ml_source1_temporary_values1417.clone(), source_ml_source1_temporary_index1417.clone());
                                                                                                            match source_ml_source2_s33330.clone() {
                                                                                                                self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Done => {
                                                                                                                    {
                                                                                                                        let functions = if global {
                                                                                                                            shared_constructor_field1.clone()
                                                                                                                        } else {
                                                                                                                            std::rc::Rc::new(std::cell::RefCell::new({ let ml_values2171 = { let ml_reference_value2172 = shared_constructor_field1.borrow().clone(); ml_reference_value2172 }; let ml_callback2171 = { let frame = frame.clone(); let directory = directory.clone(); std::rc::Rc::new(move |fn_: self::JavaScriptCallFunction| -> self::JavaScriptCallFunction { if ((match fn_.clone() {
                                                                                                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value33335, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value33335.clone()
                                                                                                                                }
                                                                                                                            }) != (match frame.clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value33336, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                    source_ml_source2_data_property_value33336.clone()
                                                                                                                                }
                                                                                                                            })) {
                                                                                                                                fn_.clone()
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_spread33340 = fn_.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field33341 = match source_ml_source2_nominal_spread33340.clone() {
                                                                                                                                            self::JavaScriptCallFunction::CallFunction(source_ml_source2_data_property_value33342, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                                source_ml_source2_data_property_value33342.clone()
                                                                                                                                            }
                                                                                                                                        };
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field33343 = match source_ml_source2_nominal_spread33340.clone() {
                                                                                                                                                self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value33344, _, _, _, _, _, _, _, _) => {
                                                                                                                                                    source_ml_source2_data_property_value33344.clone()
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field33345 = match source_ml_source2_nominal_spread33340.clone() {
                                                                                                                                                    self::JavaScriptCallFunction::CallFunction(_, _, source_ml_source2_data_property_value33346, _, _, _, _, _, _, _) => {
                                                                                                                                                        source_ml_source2_data_property_value33346.clone()
                                                                                                                                                    }
                                                                                                                                                };
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field33347 = match source_ml_source2_nominal_spread33340.clone() {
                                                                                                                                                        self::JavaScriptCallFunction::CallFunction(_, _, _, source_ml_source2_data_property_value33348, _, _, _, _, _, _) => {
                                                                                                                                                            let source_ml_source2_data_property_value33348 = *source_ml_source2_data_property_value33348;
                                                                                                                                                            source_ml_source2_data_property_value33348.clone()
                                                                                                                                                        }
                                                                                                                                                    };
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field33349 = match source_ml_source2_nominal_spread33340.clone() {
                                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, source_ml_source2_data_property_value33350, _, _, _, _, _) => {
                                                                                                                                                                source_ml_source2_data_property_value33350
                                                                                                                                                            }
                                                                                                                                                        };
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field33351 = match source_ml_source2_nominal_spread33340.clone() {
                                                                                                                                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, source_ml_source2_data_property_value33352, _, _, _, _) => {
                                                                                                                                                                    source_ml_source2_data_property_value33352
                                                                                                                                                                }
                                                                                                                                                            };
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field33353 = match source_ml_source2_nominal_spread33340.clone() {
                                                                                                                                                                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, source_ml_source2_data_property_value33354, _, _, _) => {
                                                                                                                                                                        source_ml_source2_data_property_value33354.clone()
                                                                                                                                                                    }
                                                                                                                                                                };
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field33355 = match source_ml_source2_nominal_spread33340.clone() {
                                                                                                                                                                        self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value33356, _, _) => {
                                                                                                                                                                            source_ml_source2_data_property_value33356.clone()
                                                                                                                                                                        }
                                                                                                                                                                    };
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field33357 = match source_ml_source2_nominal_spread33340.clone() {
                                                                                                                                                                            self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value33358, _) => {
                                                                                                                                                                                source_ml_source2_data_property_value33358.clone()
                                                                                                                                                                            }
                                                                                                                                                                        };
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_field33359 = match source_ml_source2_nominal_spread33340.clone() {
                                                                                                                                                                                self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value33360) => {
                                                                                                                                                                                    source_ml_source2_data_property_value33360.clone()
                                                                                                                                                                                }
                                                                                                                                                                            };
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_nominal_override33361 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values2173 = { let ml_reference_value2174 = (match fn_.clone() {
                                                                                                                                                                                    self::JavaScriptCallFunction::CallFunction(_, _, _, _, _, _, _, source_ml_source2_data_property_value33337, _, _) => {
                                                                                                                                                                                        source_ml_source2_data_property_value33337.clone()
                                                                                                                                                                                    }
                                                                                                                                                                                }).borrow().clone(); ml_reference_value2174 }; let ml_callback2173 = { let frame = frame.clone(); let directory = directory.clone(); std::rc::Rc::new(move |instruction: self::JavaScriptCallInstruction, index: f64| -> self::JavaScriptCallInstruction { if (index == ((match frame.clone().expect("a proven present value") {
                                                                                                                                                                                    self::JavaScriptCallFrame::CallFrame(_, source_ml_source2_data_property_value33339, _, _, _, _, _, _, _, _) => {
                                                                                                                                                                                        source_ml_source2_data_property_value33339
                                                                                                                                                                                    }
                                                                                                                                                                                } - 1f64))) {
                                                                                                                                                                                    self::replace_java_script_instruction_species(instruction.clone(), directory.clone())
                                                                                                                                                                                } else {
                                                                                                                                                                                    instruction.clone()
                                                                                                                                                                                } }) }; ml_values2173.iter().cloned().enumerate().map(|(ml_index2173, ml_value2173)| ml_callback2173(ml_value2173, (ml_index2173 as f64))).collect::<Vec<_>>() }));
                                                                                                                                                                                self::JavaScriptCallFunction::CallFunction(source_ml_source2_nominal_field33341.clone(), source_ml_source2_nominal_field33343.clone(), source_ml_source2_nominal_field33345.clone(), Box::new(source_ml_source2_nominal_field33347.clone()), source_ml_source2_nominal_field33349, source_ml_source2_nominal_field33351, source_ml_source2_nominal_field33353.clone(), source_ml_source2_nominal_override33361.clone(), source_ml_source2_nominal_field33357.clone(), source_ml_source2_nominal_field33359.clone())
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
                                                                                                                            } }) }; ml_values2171.iter().cloned().enumerate().map(|(ml_index2171, ml_value2171)| ml_callback2171(ml_value2171)).collect::<Vec<_>>() }))
                                                                                                                        };
                                                                                                                        {
                                                                                                                            let next = if match (match program.clone() {
                                                                                                                                self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value33362, _) => {
                                                                                                                                    ((source_ml_source2_data_property_value33362.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                                                                                                }
                                                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field33364 = shared_constructor_field1.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field33366 = shared_constructor_field2.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field33368 = shared_constructor_field3.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field33370 = shared_constructor_field4.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field33372 = shared_constructor_field5.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field33374 = shared_constructor_field6.clone();
                                                                                                                                                        self::JavaScriptCallProgram::CallProgram(functions.clone(), source_ml_source2_nominal_field33366.clone(), Box::new(source_ml_source2_nominal_field33368.clone()), source_ml_source2_nominal_field33370.clone(), source_ml_source2_nominal_field33372.clone(), source_ml_source2_nominal_field33374.clone())
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            } else {
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field33378 = shared_constructor_field1.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field33380 = shared_constructor_field2.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field33382 = shared_constructor_field3.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field33384 = shared_constructor_field4.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field33386 = shared_constructor_field5.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field33388 = shared_constructor_field6.clone();
                                                                                                                                                        self::JavaScriptCallProgram::CallProgram(functions.clone(), source_ml_source2_nominal_field33380.clone(), Box::new(source_ml_source2_nominal_field33382.clone()), source_ml_source2_nominal_field33384.clone(), Some((Some(directory.clone()),)), source_ml_source2_nominal_field33388.clone())
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            };
                                                                                                                            {
                                                                                                                                let state_error = self::read_java_script_call_machine_error(next.clone(), machine.clone());
                                                                                                                                if (state_error == (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                                    self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(Box::new(next.clone()), Box::new(machine.clone()), self::ml_string::Text::from_units(vec![]))
                                                                                                                                } else {
                                                                                                                                    self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(Box::new(program.clone()), Box::new(machine.clone()), state_error.clone())
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                                self::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Result::MlSource1TemporaryExtendJavaScriptCallArrayAuthorityLoop1421Return(source_ml_source1_temporary_result1421) => {
                                                                                                                    let source_ml_source1_temporary_result1421 = *source_ml_source1_temporary_result1421;
                                                                                                                    source_ml_source1_temporary_result1421.clone()
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
                                                        _ => {
                                                            self::JavaScriptArrayAuthorityUpdate::ArrayAuthorityUpdate(Box::new(program.clone()), Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121, 32, 99, 97, 110, 110, 111, 116, 32, 114, 101, 112, 108, 97, 99, 101, 32, 97, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 98, 111, 100, 121, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110]))
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_pending_species_construction(
    pending: self::JavaScriptCallPending,
) -> Option<self::JavaScriptSpeciesConstructionState> {
    match pending.clone() {
        self::JavaScriptCallPending::WaitArrayConcatenation(_, _, _, construction) => {
            ((construction
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,)))
            .0)
                .clone()
        }
        self::JavaScriptCallPending::WaitArraySlice(_, _, _, construction_2) => ((construction_2
            .clone()
            .unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,)))
        .0)
            .clone(),
        self::JavaScriptCallPending::WaitArrayTransform(_, _, _, construction_3) => {
            ((construction_3
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,)))
            .0)
                .clone()
        }
        _ => None::<self::JavaScriptSpeciesConstructionState>,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn replace_java_script_pending_species_construction(
    pending: self::JavaScriptCallPending,
    construction: self::JavaScriptSpeciesConstructionState,
) -> self::JavaScriptCallPending {
    match pending.clone() {
        self::JavaScriptCallPending::WaitArrayConcatenation(destination, realm, state, _) => {
            let state = *state;
            {
                let source_ml_source2_constructor_field33407 = Some(construction.clone());
                self::JavaScriptCallPending::WaitArrayConcatenation(
                    destination,
                    realm.clone(),
                    Box::new(state.clone()),
                    Some((source_ml_source2_constructor_field33407.clone(),)),
                )
            }
        }
        self::JavaScriptCallPending::WaitArraySlice(destination_2, realm_2, state_2, _) => {
            let state_2 = *state_2;
            {
                let source_ml_source2_constructor_field33411 = Some(construction.clone());
                self::JavaScriptCallPending::WaitArraySlice(
                    destination_2,
                    realm_2.clone(),
                    Box::new(state_2.clone()),
                    Some((source_ml_source2_constructor_field33411.clone(),)),
                )
            }
        }
        self::JavaScriptCallPending::WaitArrayTransform(destination_3, realm_3, state_3, _) => {
            let state_3 = *state_3;
            {
                let source_ml_source2_constructor_field33415 = Some(construction.clone());
                self::JavaScriptCallPending::WaitArrayTransform(
                    destination_3,
                    realm_3.clone(),
                    Box::new(state_3.clone()),
                    Some((source_ml_source2_constructor_field33415.clone(),)),
                )
            }
        }
        _ => pending.clone(),
    }
}
