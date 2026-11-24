// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallFrame;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptConstructionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSpeciesConstructionState;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_active_construction;
use super::read_java_script_collection_construction_arguments_error;
use super::read_java_script_collection_constructor_kind;
use super::read_java_script_constructed_collection_allocation_kind;
use super::read_java_script_construction_state_error;
use super::read_java_script_executable_call_function;
use super::read_java_script_fresh_collection_construction_receiver;
use super::source_ml_source3_record_order;
use super::test_java_script_constructed_collection_kind;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_pending_construction(
    pending: self::JavaScriptCallPending,
) -> Option<self::JavaScriptConstructionState> {
    match pending.clone() {
        self::JavaScriptCallPending::WaitArrayConcatenation(_, _, _, construction) => {
            let operation = {
                let source_ml_source2_optional_chain_receiver32285 = (((construction
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,)))
                .0)
                    .clone(),);
                match if match (source_ml_source2_optional_chain_receiver32285.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    Some({
                        let source_ml_source1_temporary_optional_chain1339 =
                            ((source_ml_source2_optional_chain_receiver32285.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        match source_ml_source1_temporary_optional_chain1339.clone() {
                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value32286,
                            ) => ((source_ml_source2_data_property_value32286
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
                            .0)
                                .clone(),
                        }
                    })
                } else {
                    None::<self::JavaScriptConstructionState>
                } {
                    Some(ml_nested) => match ml_nested {
                        Some(ml_present) => Some(ml_present),
                        None => None,
                    },
                    None => None,
                }
            };
            if match (operation.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                None::<self::JavaScriptConstructionState>
            } else {
                Some(self::read_java_script_active_construction(
                    operation.clone().expect("a proven present value"),
                ))
            }
        }
        self::JavaScriptCallPending::WaitArraySlice(_, _, _, construction_2) => {
            let operation_2 = {
                let source_ml_source2_optional_chain_receiver32287 = (((construction_2
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,)))
                .0)
                    .clone(),);
                match if match (source_ml_source2_optional_chain_receiver32287.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    Some({
                        let source_ml_source1_temporary_optional_chain1339_2 =
                            ((source_ml_source2_optional_chain_receiver32287.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        match source_ml_source1_temporary_optional_chain1339_2.clone() {
                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value32288,
                            ) => ((source_ml_source2_data_property_value32288
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
                            .0)
                                .clone(),
                        }
                    })
                } else {
                    None::<self::JavaScriptConstructionState>
                } {
                    Some(ml_nested) => match ml_nested {
                        Some(ml_present) => Some(ml_present),
                        None => None,
                    },
                    None => None,
                }
            };
            if match (operation_2.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                None::<self::JavaScriptConstructionState>
            } else {
                Some(self::read_java_script_active_construction(
                    operation_2.clone().expect("a proven present value"),
                ))
            }
        }
        self::JavaScriptCallPending::WaitArrayTransform(_, _, _, construction_3) => {
            let operation_3 = {
                let source_ml_source2_optional_chain_receiver32289 = (((construction_3
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptSpeciesConstructionState>,)))
                .0)
                    .clone(),);
                match if match (source_ml_source2_optional_chain_receiver32289.clone()).0 {
                    Some(ml_present) => {
                        let _ = ml_present.clone();
                        true
                    }
                    None => false,
                } {
                    Some({
                        let source_ml_source1_temporary_optional_chain1339_3 =
                            ((source_ml_source2_optional_chain_receiver32289.clone()).0)
                                .clone()
                                .expect("a proven present value");
                        match source_ml_source1_temporary_optional_chain1339_3.clone() {
                            self::JavaScriptSpeciesConstructionState::SpeciesConstruction(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value32290,
                            ) => ((source_ml_source2_data_property_value32290
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
                            .0)
                                .clone(),
                        }
                    })
                } else {
                    None::<self::JavaScriptConstructionState>
                } {
                    Some(ml_nested) => match ml_nested {
                        Some(ml_present) => Some(ml_present),
                        None => None,
                    },
                    None => None,
                }
            };
            if match (operation_3.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                None::<self::JavaScriptConstructionState>
            } else {
                Some(self::read_java_script_active_construction(
                    operation_3.clone().expect("a proven present value"),
                ))
            }
        }
        self::JavaScriptCallPending::WaitArrayCreation(_, _, _, pending_construction, _, _) => {
            let construction_4 = ((pending_construction
                .clone()
                .unwrap_or_else(|| (None::<self::JavaScriptConstructionState>,)))
            .0)
                .clone();
            if match (construction_4.clone(), None::<()>) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                None::<self::JavaScriptConstructionState>
            } else {
                Some(self::read_java_script_active_construction(
                    construction_4.clone().expect("a proven present value"),
                ))
            }
        }
        self::JavaScriptCallPending::WaitConstruction(_, _, state, _) => {
            let state = *state;
            Some(self::read_java_script_active_construction(state.clone()))
        }
        _ => None::<self::JavaScriptConstructionState>,
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_construction_receiver(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    frame: self::JavaScriptCallFrame,
    instruction: self::JavaScriptCallInstruction,
) -> std::rc::Rc<
    std::cell::RefCell<(
        (self::ml_string::Text, Option<self::JavaScriptSemanticValue>),
        Vec<f64>,
    )>,
> {
    match frame.clone() {
        self::JavaScriptCallFrame::CallFrame(
            shared_constructor_field3,
            _,
            shared_constructor_field4,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
        ) => match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(
                shared_constructor_field1,
                shared_constructor_field2,
            ) => {
                let shared_constructor_field1 = *shared_constructor_field1;
                {
                    let receiver = ({
                        let source_ml_source2_array_read_value32292 =
                            shared_constructor_field4.clone();
                        {
                            let source_ml_source2_array_read_index32293 = 0f64;
                            {
                                let ml_reference_array1996 =
                                    source_ml_source2_array_read_value32292.clone();
                                let ml_reference_position1996 =
                                    source_ml_source2_array_read_index32293;
                                let ml_reference_projection1996 = self::ml_array::optional_at(
                                    &*ml_reference_array1996.borrow(),
                                    self::ml_array::number_index(ml_reference_position1996),
                                );
                                ml_reference_projection1996
                            }
                        }
                    })
                    .clone()
                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                    if ((match instruction.clone() {
                        self::JavaScriptCallInstruction::CollectionAllocate(_, _, _) => false,
                        _ => true,
                    } || !(self::test_java_script_constructed_collection_kind(
                        &((((match instruction.clone() {
                            self::JavaScriptCallInstruction::CollectionAllocate(
                                _,
                                _,
                                source_ml_source2_data_property_value32297,
                            ) => Some((source_ml_source2_data_property_value32297.clone(),)),
                            _ => None::<(self::ml_string::Text,)>,
                        })
                        .clone()
                        .expect("a proven present value"))
                        .0)
                            .clone()),
                    ))) || match receiver.clone() {
                        self::JavaScriptSemanticValue::Object(_, _) => false,
                        _ => true,
                    }) {
                        std::rc::Rc::new(std::cell::RefCell::new({
                            let source_ml_source2_field32300 =
                                None::<self::JavaScriptSemanticValue>;
                            {
                                let source_ml_source3_record_field455 =
                                    self::ml_string::Text::from_units(vec![]);
                                {
                                    let source_ml_source3_record_field456 =
                                        source_ml_source2_field32300.clone();
                                    ((source_ml_source3_record_field455.clone(), source_ml_source3_record_field456.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64]], vec![1f64, 0f64]))
                                }
                            }
                        }))
                    } else {
                        {
                            let parent = {
                                let source_ml_source2_array_read_value32304 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_array_read_index32305 = ({
                                        let ml_reference_array1997 =
                                            shared_constructor_field2.clone();
                                        let ml_reference_projection1997 =
                                            ml_reference_array1997.borrow().len() as f64;
                                        ml_reference_projection1997
                                    } - 2f64);
                                    {
                                        let ml_reference_array1998 =
                                            source_ml_source2_array_read_value32304.clone();
                                        let ml_reference_position1998 =
                                            source_ml_source2_array_read_index32305;
                                        let ml_reference_projection1998 =
                                            self::ml_array::optional_at(
                                                &*ml_reference_array1998.borrow(),
                                                self::ml_array::number_index(
                                                    ml_reference_position1998,
                                                ),
                                            );
                                        ml_reference_projection1998
                                    }
                                }
                            };
                            {
                                let state = if match (parent.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    None::<self::JavaScriptConstructionState>
                                } else {
                                    self::read_java_script_pending_construction(
                                        match parent.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source2_data_property_value32306,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => {
                                                let source_ml_source2_data_property_value32306 =
                                                    *source_ml_source2_data_property_value32306;
                                                source_ml_source2_data_property_value32306.clone()
                                            }
                                        },
                                    )
                                };
                                {
                                    let fn_ = self::read_java_script_executable_call_function(
                                        program.clone(),
                                        shared_constructor_field1.clone(),
                                        shared_constructor_field3.clone(),
                                    );
                                    if (((((((((match (state.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match state.clone().expect("a proven present value") {
                                            self::JavaScriptConstructionState::ConstructionState(_, _, _, source_ml_source2_data_property_value32310, _, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value32310.clone()
                                            }
                                        }) != (self::ml_string::Text::from_units(vec![98, 111, 100, 121])))) || ((match state.clone().expect("a proven present value") {
                                            self::JavaScriptConstructionState::ConstructionState(_, _, _, _, source_ml_source2_data_property_value32313, _, _, _, _, _, _, _, _) => {
                                                source_ml_source2_data_property_value32313.clone()
                                            }
                                        }) != ((((match instruction.clone() {
                                            self::JavaScriptCallInstruction::CollectionAllocate(_, _, source_ml_source2_data_property_value32314) => {
                                                Some((source_ml_source2_data_property_value32314.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml_string::Text,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone()))) || ((match state.clone().expect("a proven present value") {
                                            self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value32317, _, _, _) => {
                                                source_ml_source2_data_property_value32317.clone()
                                            }
                                        }) != (self::read_java_script_constructed_collection_allocation_kind(&((((match instruction.clone() {
                                            self::JavaScriptCallInstruction::CollectionAllocate(_, _, source_ml_source2_data_property_value32318) => {
                                                Some((source_ml_source2_data_property_value32318.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml_string::Text,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone()))))) || {
                                            let source_ml_source2_s32320 = match state.clone().expect("a proven present value") {
                                                self::JavaScriptConstructionState::ConstructionState(source_ml_source2_data_property_value32323, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                    let source_ml_source2_data_property_value32323 = *source_ml_source2_data_property_value32323;
                                                    source_ml_source2_data_property_value32323.clone()
                                                }
                                            };
                                            match source_ml_source2_s32320.clone() {
                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }
                                        }) || (((((match match state.clone().expect("a proven present value") {
                                            self::JavaScriptConstructionState::ConstructionState(source_ml_source2_data_property_value32327, _, _, _, _, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value32327 = *source_ml_source2_data_property_value32327;
                                                source_ml_source2_data_property_value32327.clone()
                                            }
                                        } {
                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value32328, _) => {
                                                Some((source_ml_source2_data_property_value32328.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml::Big,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone()) != shared_constructor_field3)) || !(self::test_java_script_semantic_same_value(match state.clone().expect("a proven present value") {
                                            self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, _, source_ml_source2_data_property_value32330, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value32330 = *source_ml_source2_data_property_value32330;
                                                source_ml_source2_data_property_value32330.clone()
                                            }
                                        }, receiver.clone()))) || !(self::test_java_script_semantic_same_value(match state.clone().expect("a proven present value") {
                                            self::JavaScriptConstructionState::ConstructionState(_, source_ml_source2_data_property_value32331, _, _, _, _, _, _, _, _, _, _, _) => {
                                                let source_ml_source2_data_property_value32331 = *source_ml_source2_data_property_value32331;
                                                source_ml_source2_data_property_value32331.clone()
                                            }
                                        }, (match frame.clone() {
                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value32332) => {
                                                ((source_ml_source2_data_property_value32332.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                            }
                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || match (fn_.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || ((self::read_java_script_collection_constructor_kind(fn_.clone().expect("a proven present value"))) != ((((match instruction.clone() {
                                            self::JavaScriptCallInstruction::CollectionAllocate(_, _, source_ml_source2_data_property_value32334) => {
                                                Some((source_ml_source2_data_property_value32334.clone(),))
                                            }
                                            _ => {
                                                None::<(self::ml_string::Text,)>
                                            }
                                        }).clone().expect("a proven present value")).0).clone()))) {
                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                let source_ml_source2_field32335 = None::<self::JavaScriptSemanticValue>;
                                                {
                                                    let source_ml_source3_record_field457 = self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 114, 101, 117, 115, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 110, 101, 114, 105, 99, 32, 67, 111, 110, 115, 116, 114, 117, 99, 116, 32, 112, 97, 114, 101, 110, 116, 44, 32, 116, 97, 114, 103, 101, 116, 44, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116]);
                                                    {
                                                        let source_ml_source3_record_field458 = source_ml_source2_field32335.clone();
                                                        ((source_ml_source3_record_field457.clone(), source_ml_source3_record_field458.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64]], vec![1f64, 0f64]))
                                                    }
                                                }
                                            }))
                                        } else {
                                            {
                                                let directory = match program.clone() {
                                                    self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value32337, _) => {
                                                        ((source_ml_source2_data_property_value32337.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                                    }
                                                };
                                                if match (directory.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                        let source_ml_source2_field32338 = None::<self::JavaScriptSemanticValue>;
                                                        {
                                                            let source_ml_source3_record_field459 = self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 114, 32, 114, 101, 117, 115, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121]);
                                                            {
                                                                let source_ml_source3_record_field460 = source_ml_source2_field32338.clone();
                                                                ((source_ml_source3_record_field459.clone(), source_ml_source3_record_field460.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64]], vec![1f64, 0f64]))
                                                            }
                                                        }
                                                    }))
                                                } else {
                                                    {
                                                        let invalid = self::read_java_script_construction_state_error(program.clone(), shared_constructor_field1.clone(), directory.clone().expect("a proven present value"), state.clone().expect("a proven present value"));
                                                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                            std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source2_field32341 = None::<self::JavaScriptSemanticValue>;
                                                                {
                                                                    let source_ml_source3_record_field461 = invalid.clone();
                                                                    {
                                                                        let source_ml_source3_record_field462 = source_ml_source2_field32341.clone();
                                                                        ((source_ml_source3_record_field461.clone(), source_ml_source3_record_field462.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64]], vec![1f64, 0f64]))
                                                                    }
                                                                }
                                                            }))
                                                        } else {
                                                            {
                                                                let arguments_error = self::read_java_script_collection_construction_arguments_error(frame.clone(), fn_.clone().expect("a proven present value"), instruction.clone(), state.clone().expect("a proven present value"));
                                                                if (arguments_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                    std::rc::Rc::new(std::cell::RefCell::new({
                                                                        let source_ml_source2_field32343 = None::<self::JavaScriptSemanticValue>;
                                                                        {
                                                                            let source_ml_source3_record_field463 = arguments_error.clone();
                                                                            {
                                                                                let source_ml_source3_record_field464 = source_ml_source2_field32343.clone();
                                                                                ((source_ml_source3_record_field463.clone(), source_ml_source3_record_field464.clone()), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![114f64, 101f64, 99f64, 101f64, 105f64, 118f64, 101f64, 114f64]], vec![1f64, 0f64]))
                                                                            }
                                                                        }
                                                                    }))
                                                                } else {
                                                                    self::read_java_script_fresh_collection_construction_receiver(shared_constructor_field1.clone(), &((((match instruction.clone() {
                                                                        self::JavaScriptCallInstruction::CollectionAllocate(_, _, source_ml_source2_data_property_value32346) => {
                                                                            Some((source_ml_source2_data_property_value32346.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::ml_string::Text,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone()), (((match receiver.clone() {
                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value32347, _) => {
                                                                            Some((source_ml_source2_data_property_value32347.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::ml::Big,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone(), receiver.clone(), match state.clone().expect("a proven present value") {
                                                                        self::JavaScriptConstructionState::ConstructionState(_, _, _, _, _, source_ml_source2_data_property_value32348, _, _, _, _, _, _, _) => {
                                                                            let source_ml_source2_data_property_value32348 = *source_ml_source2_data_property_value32348;
                                                                            source_ml_source2_data_property_value32348.clone()
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
                }
            }
        },
    }
}
