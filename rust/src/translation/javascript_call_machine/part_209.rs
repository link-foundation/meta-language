// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptCollectionCompletion;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryPushJavaScriptCollectionConstructorLoop1527Result;
use super::MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529;
use super::advance_java_script_call_frame;
use super::create_java_script_call_frame;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_machine_error;
use super::read_java_script_error_allocation_directory;
use super::read_java_script_error_allocation_result_error;
use super::read_java_script_error_construction_operands_error;
use super::read_java_script_executable_call_function;
use super::replace_java_script_call_frame;
use super::request_java_script_call_error;
use super::source_ml_source1_temporary_push_java_script_collection_constructor_loop1527;
use super::test_java_script_collection_constructor_mode;
use super::write_java_script_call_register;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn push_java_script_collection_constructor(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    callee: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field11 = *shared_constructor_field11;
            {
                let parent = {
                    let source_ml_source2_array_read_value36949 =
                        shared_constructor_field12.clone();
                    {
                        let source_ml_source2_array_read_index36950 = ({
                            let ml_reference_array2428 = shared_constructor_field12.clone();
                            let ml_reference_projection2428 =
                                ml_reference_array2428.borrow().len() as f64;
                            ml_reference_projection2428
                        } - 1f64);
                        {
                            let ml_reference_array2429 =
                                source_ml_source2_array_read_value36949.clone();
                            let ml_reference_position2429 = source_ml_source2_array_read_index36950;
                            let ml_reference_projection2429 = self::ml_array::optional_at(
                                &*ml_reference_array2429.borrow(),
                                self::ml_array::number_index(ml_reference_position2429),
                            );
                            ml_reference_projection2429
                        }
                    }
                };
                {
                    let owner = if match (parent.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        None::<self::JavaScriptCallFunction>
                    } else {
                        self::read_java_script_executable_call_function(
                            program.clone(),
                            shared_constructor_field11.clone(),
                            match parent.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(
                                    source_ml_source2_data_property_value36952,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value36952.clone(),
                            },
                        )
                    };
                    if match (owner.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::JavaScriptCallMachineTransition::MachineInvalid(
                            Box::new(shared_constructor_field11.clone()),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110,
                                115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 110, 101, 101, 100,
                                115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115,
                                117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 97, 108, 108, 101,
                                114,
                            ]),
                        )
                    } else {
                        if (match callee.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        } || !(((match callee.clone() {
                            self::JavaScriptSemanticValue::Object(
                                _,
                                source_ml_source2_data_property_value36956,
                            ) => Some((source_ml_source2_data_property_value36956,)),
                            _ => None::<(bool,)>,
                        })
                        .clone()
                        .expect("a proven present value"))
                        .0))
                        {
                            self::request_java_script_call_error(
                                machine.clone(),
                                match owner.clone().expect("a proven present value") {
                                    self::JavaScriptCallFunction::CallFunction(
                                        _,
                                        source_ml_source2_data_property_value36957,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value36957.clone(),
                                },
                                &(self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ])),
                                &(self::ml_string::Text::from_units(vec![
                                    116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                    99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 115,
                                    32, 110, 111, 116, 32, 97, 32, 99, 111, 110, 115, 116, 114,
                                    117, 99, 116, 111, 114,
                                ])),
                            )
                        } else {
                            {
                                let fn_ = self::read_java_script_executable_call_function(
                                    program.clone(),
                                    shared_constructor_field11.clone(),
                                    (((match callee.clone() {
                                        self::JavaScriptSemanticValue::Object(
                                            source_ml_source2_data_property_value36959,
                                            _,
                                        ) => Some((
                                            source_ml_source2_data_property_value36959.clone(),
                                        )),
                                        _ => None::<(self::ml::Big,)>,
                                    })
                                    .clone()
                                    .expect("a proven present value"))
                                    .0)
                                        .clone(),
                                );
                                if (match (fn_.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } || !(self::test_java_script_collection_constructor_mode(
                                    &(match fn_.clone().expect("a proven present value") {
                                        self::JavaScriptCallFunction::CallFunction(
                                            _,
                                            _,
                                            source_ml_source2_data_property_value36960,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value36960.clone(),
                                    }),
                                ))) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field11.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99,
                                            111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110,
                                            32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116,
                                            97, 110, 100, 97, 114, 100, 32, 114, 101, 103, 105,
                                            115, 116, 101, 114, 101, 100, 32, 99, 111, 110, 115,
                                            116, 114, 117, 99, 116, 111, 114,
                                        ]),
                                    )
                                } else {
                                    {
                                        let slots =
                                            match fn_.clone().expect("a proven present value") {
                                                self::JavaScriptCallFunction::CallFunction(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source2_data_property_value36962,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => source_ml_source2_data_property_value36962,
                                            };
                                        {
                                            let argument_slots = {
                                                let ml_reference_array2430 = arguments_.clone();
                                                let ml_reference_projection2430 =
                                                    ml_reference_array2430.borrow().len() as f64;
                                                ml_reference_projection2430
                                            };
                                            {
                                                let source_ml_source1_temporary_values1525 =
                                                    shared_constructor_field12.clone();
                                                {
                                                    let source_ml_source1_temporary_index1525 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s36964 = self::source_ml_source1_temporary_push_java_script_collection_constructor_loop1527(slots, argument_slots, source_ml_source1_temporary_values1525.clone(), source_ml_source1_temporary_index1525.clone());
                                                        match source_ml_source2_s36964.clone() {
                                                            self::MlSource1TemporaryPushJavaScriptCollectionConstructorLoop1527Result::MlSource1TemporaryPushJavaScriptCollectionConstructorLoop1527Done(slots_2, argument_slots_2) => {
                                                                if (((({ let ml_reference_array2431 = shared_constructor_field12.clone(); let ml_reference_projection2431 = ml_reference_array2431.borrow().len() as f64; ml_reference_projection2431 }) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                    self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 114, 97, 109, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                } else {
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field11.clone()), std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source2_concat_value36991 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_concat_value36992 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                                let source_ml_source2_nominal_spread36969 = self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), self::JavaScriptSemanticValue::Undefined, arguments_.clone());
                                                                                match source_ml_source2_nominal_spread36969.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                                        let shared_constructor_field6 = *shared_constructor_field6;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field36970 = shared_constructor_field1.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field36972 = shared_constructor_field2;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field36974 = shared_constructor_field3.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field36976 = shared_constructor_field4.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field36978 = shared_constructor_field5.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field36980 = shared_constructor_field6.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field36982 = shared_constructor_field7.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field36984 = shared_constructor_field8.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field36986 = shared_constructor_field9.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field36988 = shared_constructor_field10.clone();
                                                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field36970.clone(), source_ml_source2_nominal_field36972, source_ml_source2_nominal_field36974.clone(), source_ml_source2_nominal_field36976.clone(), source_ml_source2_nominal_field36978.clone(), Box::new(source_ml_source2_nominal_field36980.clone()), source_ml_source2_nominal_field36982.clone(), source_ml_source2_nominal_field36984.clone(), source_ml_source2_nominal_field36986.clone(), Some((Some(callee.clone()),)))
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
                                                                            }]));
                                                                            self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), { let ml_reference_value2432 = source_ml_source2_concat_value36991.borrow().clone(); ml_reference_value2432 }), { let ml_reference_value2433 = source_ml_source2_concat_value36992.borrow().clone(); ml_reference_value2433 })
                                                                        }
                                                                    })))))
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn accept_java_script_collection_completion(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    destination: f64,
    completion: self::JavaScriptCollectionCompletion,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field1,
            shared_constructor_field2,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            {
                let current = self::JavaScriptCallMachine::CallMachine(
                    Box::new(match completion.clone() {
                        self::JavaScriptCollectionCompletion::CollectionNormal(
                            source_ml_source2_data_property_value36993,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value36993 =
                                *source_ml_source2_data_property_value36993;
                            source_ml_source2_data_property_value36993.clone()
                        }
                        self::JavaScriptCollectionCompletion::CollectionInvalid(
                            source_ml_source2_data_property_value36994,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value36994 =
                                *source_ml_source2_data_property_value36994;
                            source_ml_source2_data_property_value36994.clone()
                        }
                        self::JavaScriptCollectionCompletion::CollectionTypeError(
                            source_ml_source2_data_property_value36995,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value36995 =
                                *source_ml_source2_data_property_value36995;
                            source_ml_source2_data_property_value36995.clone()
                        }
                        self::JavaScriptCollectionCompletion::CollectionResourceLimit(
                            source_ml_source2_data_property_value36996,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value36996 =
                                *source_ml_source2_data_property_value36996;
                            source_ml_source2_data_property_value36996.clone()
                        }
                        self::JavaScriptCollectionCompletion::CollectionNeedIterable(
                            source_ml_source2_data_property_value36997,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value36997 =
                                *source_ml_source2_data_property_value36997;
                            source_ml_source2_data_property_value36997.clone()
                        }
                    }),
                    shared_constructor_field2.clone(),
                );
                match completion.clone() {
                    self::JavaScriptCollectionCompletion::CollectionNormal(heap, value) => {
                        let heap = *heap;
                        let value = *value;
                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                            self::replace_java_script_call_frame(
                                heap.clone(),
                                shared_constructor_field2.clone(),
                                self::write_java_script_call_register(
                                    self::advance_java_script_call_frame(frame.clone()),
                                    destination,
                                    value.clone(),
                                ),
                            ),
                        ))
                    }
                    self::JavaScriptCollectionCompletion::CollectionInvalid(heap_2, reason) => {
                        let heap_2 = *heap_2;
                        self::JavaScriptCallMachineTransition::MachineInvalid(
                            Box::new(heap_2.clone()),
                            reason.clone(),
                        )
                    }
                    self::JavaScriptCollectionCompletion::CollectionTypeError(heap_3, reason_2) => {
                        let heap_3 = *heap_3;
                        {
                            let owner = self::read_java_script_executable_call_function(
                                program.clone(),
                                shared_constructor_field1.clone(),
                                match frame.clone() {
                                    self::JavaScriptCallFrame::CallFrame(
                                        source_ml_source2_data_property_value37003,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value37003.clone(),
                                },
                            );
                            if match (owner.clone(), None::<()>) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::JavaScriptCallMachineTransition::MachineInvalid(
                                    Box::new(heap_3.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101,
                                        114, 114, 111, 114, 115, 32, 114, 101, 113, 117, 105, 114,
                                        101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97,
                                        108, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32,
                                        114, 101, 97, 108, 109,
                                    ]),
                                )
                            } else {
                                self::request_java_script_call_error(
                                    current.clone(),
                                    match owner.clone().expect("a proven present value") {
                                        self::JavaScriptCallFunction::CallFunction(
                                            _,
                                            source_ml_source2_data_property_value37004,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value37004.clone(),
                                    },
                                    &(self::ml_string::Text::from_units(vec![
                                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                                    ])),
                                    &(reason_2),
                                )
                            }
                        }
                    }
                    self::JavaScriptCollectionCompletion::CollectionResourceLimit(_, reason_3) => {
                        self::JavaScriptCallMachineTransition::MachineResourceLimit(
                            Box::new(current.clone()),
                            reason_3.clone(),
                        )
                    }
                    self::JavaScriptCollectionCompletion::CollectionNeedIterable(
                        _,
                        target,
                        input,
                    ) => {
                        let target = *target;
                        let input = *input;
                        self::JavaScriptCallMachineTransition::MachineNeedCollectionIterable(
                            Box::new(current.clone()),
                            Box::new(target.clone()),
                            Box::new(input.clone()),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn request_java_script_error_value(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    instruction: self::JavaScriptCallInstruction,
) -> self::JavaScriptCallMachineTransition {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            shared_constructor_field13,
            _,
            shared_constructor_field14,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(
                shared_constructor_field11,
                shared_constructor_field12,
            ) => {
                let shared_constructor_field11 = *shared_constructor_field11;
                match instruction.clone() {
                    self::JavaScriptCallInstruction::ConstructErrorValue(
                        destination,
                        instruction_constructor,
                        instruction_new_target,
                        instruction_prototype,
                        instruction_message,
                        instruction_present,
                        realm,
                        error_class,
                        bind_receiver,
                    ) => {
                        let constructor = ({
                            let source_ml_source2_array_read_value37006 =
                                shared_constructor_field14.clone();
                            {
                                let ml_reference_array2434 =
                                    source_ml_source2_array_read_value37006.clone();
                                let ml_reference_position2434 = instruction_constructor;
                                let ml_reference_projection2434 = self::ml_array::optional_at(
                                    &*ml_reference_array2434.borrow(),
                                    self::ml_array::number_index(ml_reference_position2434),
                                );
                                ml_reference_projection2434
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let new_target = ({
                                let source_ml_source2_array_read_value37009 =
                                    shared_constructor_field14.clone();
                                {
                                    let ml_reference_array2435 =
                                        source_ml_source2_array_read_value37009.clone();
                                    let ml_reference_position2435 = instruction_new_target;
                                    let ml_reference_projection2435 = self::ml_array::optional_at(
                                        &*ml_reference_array2435.borrow(),
                                        self::ml_array::number_index(ml_reference_position2435),
                                    );
                                    ml_reference_projection2435
                                }
                            })
                            .clone()
                            .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                            {
                                let prototype = ({
                                    let source_ml_source2_array_read_value37012 =
                                        shared_constructor_field14.clone();
                                    {
                                        let ml_reference_array2436 =
                                            source_ml_source2_array_read_value37012.clone();
                                        let ml_reference_position2436 = instruction_prototype;
                                        let ml_reference_projection2436 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array2436.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position2436,
                                                ),
                                            );
                                        ml_reference_projection2436
                                    }
                                })
                                .clone()
                                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                {
                                    let message = ({
                                        let source_ml_source2_array_read_value37015 =
                                            shared_constructor_field14.clone();
                                        {
                                            let ml_reference_array2437 =
                                                source_ml_source2_array_read_value37015.clone();
                                            let ml_reference_position2437 = instruction_message;
                                            let ml_reference_projection2437 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array2437.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position2437,
                                                    ),
                                                );
                                            ml_reference_projection2437
                                        }
                                    })
                                    .clone()
                                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                    {
                                        let present = ({
                                            let source_ml_source2_array_read_value37018 =
                                                shared_constructor_field14.clone();
                                            {
                                                let ml_reference_array2438 =
                                                    source_ml_source2_array_read_value37018.clone();
                                                let ml_reference_position2438 = instruction_present;
                                                let ml_reference_projection2438 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array2438.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position2438,
                                                        ),
                                                    );
                                                ml_reference_projection2438
                                            }
                                        })
                                        .clone()
                                        .unwrap_or_else(|| {
                                            self::JavaScriptSemanticValue::Undefined
                                        });
                                        {
                                            let error = self::read_java_script_error_construction_operands_error(shared_constructor_field11.clone(), constructor.clone(), new_target.clone(), prototype.clone(), message.clone(), present.clone());
                                            if (error
                                                != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), error.clone())
                                            } else {
                                                match present.clone() {
                                                            self::JavaScriptSemanticValue::Boolean(value) => {
                                                                if (bind_receiver && (match new_target.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        false
                                                                    }
                                                                    _ => {
                                                                        true
                                                                    }
                                                                } || (((((match new_target.clone() {
                                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value37026, _) => {
                                                                        Some((source_ml_source2_data_property_value37026.clone(),))
                                                                    }
                                                                    _ => {
                                                                        None::<(self::ml::Big,)>
                                                                    }
                                                                }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field13))) {
                                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![116, 104, 101, 32, 100, 105, 114, 101, 99, 116, 32, 100, 101, 114, 105, 118, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121]))
                                                                } else {
                                                                    {
                                                                        let pending = self::JavaScriptCallPending::WaitErrorValue(destination, realm.clone(), error_class.clone(), Box::new(constructor.clone()), Box::new(new_target.clone()), Box::new(prototype.clone()), Box::new(message.clone()), value, bind_receiver, Box::new(self::read_java_script_error_allocation_directory(shared_constructor_field11.clone())));
                                                                        self::JavaScriptCallMachineTransition::MachineAllocateErrorValue(Box::new(self::replace_java_script_call_frame(shared_constructor_field11.clone(), shared_constructor_field12.clone(), {
                                                                            let source_ml_source2_nominal_spread37032 = self::advance_java_script_call_frame(frame.clone());
                                                                            match source_ml_source2_nominal_spread37032.clone() {
                                                                                self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                                    let shared_constructor_field6 = *shared_constructor_field6;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field37033 = shared_constructor_field1.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field37035 = shared_constructor_field2;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field37037 = shared_constructor_field3.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field37039 = shared_constructor_field4.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field37041 = shared_constructor_field5.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field37043 = shared_constructor_field6.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field37045 = shared_constructor_field7.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field37047 = shared_constructor_field8.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field37049 = shared_constructor_field9.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field37051 = shared_constructor_field10.clone();
                                                                                                                            self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field37033.clone(), source_ml_source2_nominal_field37035, source_ml_source2_nominal_field37037.clone(), source_ml_source2_nominal_field37039.clone(), source_ml_source2_nominal_field37041.clone(), Box::new(pending.clone()), source_ml_source2_nominal_field37045.clone(), source_ml_source2_nominal_field37047.clone(), source_ml_source2_nominal_field37049.clone(), source_ml_source2_nominal_field37051.clone())
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
                                                                        })), realm.clone(), error_class.clone(), Box::new(constructor.clone()), Box::new(new_target.clone()), Box::new(prototype.clone()), Box::new(message.clone()), value)
                                                                    }
                                                                }
                                                            }
                                                            _ => {
                                                                self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field11.clone()), self::ml_string::Text::from_units(vec![69, 114, 114, 111, 114, 32, 112, 114, 101, 115, 101, 110, 99, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 66, 111, 111, 108, 101, 97, 110, 32, 111, 112, 101, 114, 97, 110, 100]))
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
                        Box::new(shared_constructor_field11.clone()),
                        self::ml_string::Text::from_units(vec![
                            69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111,
                            110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                            117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 99, 111, 110, 115, 116,
                            114, 117, 99, 116, 105, 111, 110, 32, 105, 110, 115, 116, 114, 117, 99,
                            116, 105, 111, 110,
                        ]),
                    ),
                }
            }
        },
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_error_value_allocation(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    realm: self::ml::Big,
    value: self::JavaScriptSemanticValue,
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
                            let source_ml_source2_array_read_value37059 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index37060 = ({
                                    let ml_reference_array2439 = shared_constructor_field2.clone();
                                    let ml_reference_projection2439 =
                                        ml_reference_array2439.borrow().len() as f64;
                                    ml_reference_projection2439
                                } - 1f64);
                                {
                                    let ml_reference_array2440 =
                                        source_ml_source2_array_read_value37059.clone();
                                    let ml_reference_position2440 =
                                        source_ml_source2_array_read_index37060;
                                    let ml_reference_projection2440 = self::ml_array::optional_at(
                                        &*ml_reference_array2440.borrow(),
                                        self::ml_array::number_index(ml_reference_position2440),
                                    );
                                    ml_reference_projection2440
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
                                    69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32, 97,
                                    108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113,
                                    117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                    117, 97, 108, 32, 112, 101, 110, 100, 105, 110, 103, 32, 99,
                                    111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 99, 111,
                                    110, 116, 105, 110, 117, 97, 116, 105, 111, 110,
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
                                        source_ml_source2_data_property_value37062,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value37062 =
                                            *source_ml_source2_data_property_value37062;
                                        source_ml_source2_data_property_value37062.clone()
                                    }
                                };
                                match pending.clone() {
                                    self::JavaScriptCallPending::WaitErrorValue(
                                        destination,
                                        pending_realm,
                                        _,
                                        _,
                                        _,
                                        prototype,
                                        _,
                                        _,
                                        bind_receiver,
                                        before_directory,
                                    ) => {
                                        let prototype = *prototype;
                                        let before_directory = *before_directory;
                                        if (pending_realm != realm) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                                Box::new(shared_constructor_field1.clone()),
                                                self::ml_string::Text::from_units(vec![
                                                    69, 114, 114, 111, 114, 32, 118, 97, 108, 117,
                                                    101, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                                                    111, 110, 32, 114, 101, 116, 97, 105, 110, 115,
                                                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                                    108, 32, 114, 101, 113, 117, 101, 115, 116,
                                                    101, 100, 32, 105, 110, 116, 114, 105, 110,
                                                    115, 105, 99, 32, 114, 101, 97, 108, 109,
                                                ]),
                                            )
                                        } else {
                                            {
                                                let invalid = self::read_java_script_error_allocation_result_error(shared_constructor_field1.clone(), value.clone(), prototype.clone(), before_directory.clone());
                                                if (invalid
                                                    != (self::ml_string::Text::from_units(vec![])))
                                                {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), invalid.clone())
                                                } else {
                                                    {
                                                        let next =
                                                            self::write_java_script_call_register(
                                                                frame.clone().expect(
                                                                    "a proven present value",
                                                                ),
                                                                destination,
                                                                value.clone(),
                                                            );
                                                        {
                                                            let source_ml_source2_s37074 =
                                                                if bind_receiver {
                                                                    {
                                                                        let previous = ({
                                                                        let source_ml_source2_array_read_value37076 = match frame.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, source_ml_source2_data_property_value37075, _, _, _, _, _, _, _) => {
                                                                                source_ml_source2_data_property_value37075.clone()
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_array_read_index37077 = 0f64;
                                                                            { let ml_reference_array2441 = source_ml_source2_array_read_value37076.clone(); let ml_reference_position2441 = source_ml_source2_array_read_index37077; let ml_reference_projection2441 = self::ml_array::optional_at(&*ml_reference_array2441.borrow(), self::ml_array::number_index(ml_reference_position2441)); ml_reference_projection2441 }
                                                                        }
                                                                    }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                                                                        match previous.clone() {
                                                                        self::JavaScriptSemanticValue::Undefined => {
                                                                            {
                                                                                let next_2 = self::write_java_script_call_register(next.clone(), 0f64, value.clone());
                                                                                self::MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529::MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529Next(Box::new(next_2.clone()))
                                                                            }
                                                                        }
                                                                        _ => {
                                                                            self::MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529::MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529Return(Box::new(self::request_java_script_call_error(machine.clone(), realm.clone(), &(self::ml_string::Text::from_units(vec![82, 101, 102, 101, 114, 101, 110, 99, 101, 69, 114, 114, 111, 114])), &(self::ml_string::Text::from_units(vec![116, 104, 101, 32, 100, 101, 114, 105, 118, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 119, 97, 115, 32, 97, 108, 114, 101, 97, 100, 121, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 98, 121, 32, 97, 32, 112, 114, 101, 118, 105, 111, 117, 115, 32, 115, 117, 112, 101, 114, 32, 99, 97, 108, 108])))))
                                                                        }
                                                                    }
                                                                    }
                                                                } else {
                                                                    self::MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529::MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529Next(Box::new(next.clone()))
                                                                };
                                                            match source_ml_source2_s37074.clone() {
                                                                self::MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529::MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529Next(next_3) => {
                                                                    let next_3 = *next_3;
                                                                    self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::replace_java_script_call_frame(shared_constructor_field1.clone(), shared_constructor_field2.clone(), next_3.clone())))
                                                                }
                                                                self::MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529::MlSource1TemporaryResumeJavaScriptErrorValueAllocationJoin1529Return(source_ml_source1_temporary_result1529) => {
                                                                    let source_ml_source1_temporary_result1529 = *source_ml_source1_temporary_result1529;
                                                                    source_ml_source1_temporary_result1529.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    _ => self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field1.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            69, 114, 114, 111, 114, 32, 118, 97, 108, 117, 101, 32,
                                            97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114,
                                            101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115,
                                            32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100,
                                            105, 110, 103, 32, 99, 111, 110, 115, 116, 114, 117,
                                            99, 116, 111, 114, 32, 99, 111, 110, 116, 105, 110,
                                            117, 97, 116, 105, 111, 110,
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
