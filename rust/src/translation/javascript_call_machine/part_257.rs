// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallArgumentList;
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result;
use super::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Result;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_active_construction_frame_error;
use super::read_java_script_call_code_destination;
use super::read_java_script_call_frame_error;
use super::read_java_script_call_instruction_error;
use super::read_java_script_call_registers;
use super::read_java_script_executable_call_function;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_selected_parent_call_error;
use super::source_ml_source1_temporary_read_java_script_call_function_body_error_loop1455;
use super::test_java_script_call_coordinate;
use super::test_java_script_collection_constructor_mode;
use super::test_java_script_collection_intrinsic_mode;
use super::test_java_script_collection_iterator_intrinsic_mode;
use super::test_java_script_error_constructor_mode;
use super::test_java_script_ordinary_class_constructor_mode;
use super::test_java_script_primitive_intrinsic_mode;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_function_body_error_loop1454(
    mut program: self::JavaScriptCallProgram,
    mut heap: self::JavaScriptOrdinaryHeap,
    mut fn_: self::JavaScriptCallFunction,
    mut source_ml_source1_temporary_values1451: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallInstruction>>,
    >,
    mut source_ml_source1_temporary_index1451: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result {
    loop {
        return match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(
                _,
                _,
                shared_constructor_field1,
                _,
                _,
                _,
                _,
                _,
                _,
                _,
            ) => {
                if (source_ml_source1_temporary_index1451
                    < ({
                        let ml_reference_array3216 = source_ml_source1_temporary_values1451.clone();
                        let ml_reference_projection3216 =
                            self::ml::Big::from_u128(ml_reference_array3216.borrow().len() as u128);
                        ml_reference_projection3216
                    }))
                {
                    {
                        let instruction = {
                            let ml_reference_array3217 =
                                source_ml_source1_temporary_values1451.clone();
                            let ml_reference_position3217 =
                                source_ml_source1_temporary_index1451.clone();
                            let ml_reference_projection3217 = self::ml_array::at(
                                &*ml_reference_array3217.borrow(),
                                ml_reference_position3217.to_index(),
                            );
                            ml_reference_projection3217
                        };
                        if (self::test_java_script_error_constructor_mode(
                            &(shared_constructor_field1),
                        ) && ((self::read_java_script_call_code_destination(
                            instruction.clone(),
                        )) == (0f64)))
                        {
                            self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Return(self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 116, 104, 105, 115, 32, 105, 115, 32, 105, 110, 105, 116, 105, 97, 108, 105, 122, 101, 100, 32, 111, 110, 108, 121, 32, 98, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 112, 101, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110]))
                        } else {
                            if (self::test_java_script_ordinary_class_constructor_mode(
                                &(shared_constructor_field1),
                            ) && ((self::read_java_script_call_code_destination(
                                instruction.clone(),
                            )) == (0f64)))
                            {
                                self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 116, 104, 105, 115, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                            } else {
                                if ((((self::test_java_script_collection_constructor_mode(
                                    &(shared_constructor_field1),
                                ) || self::test_java_script_collection_intrinsic_mode(
                                    &(shared_constructor_field1),
                                )) || self::test_java_script_primitive_intrinsic_mode(
                                    &(shared_constructor_field1),
                                ))
                                    || self::test_java_script_collection_iterator_intrinsic_mode(
                                        &(shared_constructor_field1),
                                    ))
                                    && ((self::read_java_script_call_code_destination(
                                        instruction.clone(),
                                    )) == (0f64)))
                                {
                                    self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Return(self::ml_string::Text::from_units(vec![115, 116, 97, 110, 100, 97, 114, 100, 32, 105, 110, 116, 114, 105, 110, 115, 105, 99, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 114, 101, 103, 105, 115, 116, 101, 114]))
                                } else {
                                    if (match instruction.clone() {
                                        self::JavaScriptCallInstruction::ObjectSpread(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => true,
                                        _ => false,
                                    } && (({
                                        let ml_reference_array3218 =
                                            ((((match instruction.clone() {
                                                self::JavaScriptCallInstruction::ObjectSpread(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source2_data_property_value42277,
                                                    _,
                                                ) => Some((
                                                    ((source_ml_source2_data_property_value42277
                                                        .clone()
                                                        .unwrap_or_else(|| {
                                                            (None::<
                                                                std::rc::Rc<
                                                                    std::cell::RefCell<Vec<f64>>,
                                                                >,
                                                            >,)
                                                        }))
                                                    .0)
                                                        .clone(),
                                                )),
                                                _ => {
                                                    None::<(
                                                        Option<
                                                            std::rc::Rc<
                                                                std::cell::RefCell<Vec<f64>>,
                                                            >,
                                                        >,
                                                    )>
                                                }
                                            })
                                            .clone()
                                            .expect("a proven present value"))
                                            .0)
                                                .clone())
                                            .clone()
                                            .unwrap_or_else(|| {
                                                std::rc::Rc::new(std::cell::RefCell::new(
                                                    Vec::<f64>::new(),
                                                ))
                                            });
                                        let ml_reference_projection3218 =
                                            ml_reference_array3218.borrow().len() as f64;
                                        ml_reference_projection3218
                                    }) > (4096f64)))
                                    {
                                        self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Return(self::ml_string::Text::from_units(vec![111, 98, 106, 101, 99, 116, 32, 99, 111, 112, 121, 32, 101, 120, 99, 108, 117, 115, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 98, 111, 117, 110, 100, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 111, 112, 101, 114, 97, 110, 100, 115]))
                                    } else {
                                        {
                                            let source_ml_source1_temporary_values1452 =
                                                self::read_java_script_call_registers(
                                                    instruction.clone(),
                                                );
                                            {
                                                let source_ml_source1_temporary_index1452 =
                                                    self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source2_s42278 = self::source_ml_source1_temporary_read_java_script_call_function_body_error_loop1455(fn_.clone(), source_ml_source1_temporary_values1452.clone(), source_ml_source1_temporary_index1452.clone());
                                                    match source_ml_source2_s42278.clone() {
                                                        self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Done => {
                                                            {
                                                                let error = self::read_java_script_call_instruction_error(program.clone(), heap.clone(), fn_.clone(), instruction.clone());
                                                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                    self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Return(error.clone())
                                                                } else {
                                                                    {
                                                                        let source_ml_source1_temporary_index1451_2 = source_ml_source1_temporary_index1451.add(&self::ml::Big::from_i128(1));
                                                                        {
                                                                            source_ml_source1_temporary_index1451 = source_ml_source1_temporary_index1451_2.clone();
                                                                            continue;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Return(source_ml_source1_temporary_result1455) => {
                                                            self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Return(source_ml_source1_temporary_result1455.clone())
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
                } else {
                    self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Result {
    MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Done,
    MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_function_body_error_loop1453(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1450: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1450: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Result {
    loop {
        return if (source_ml_source1_temporary_index1450
            < ({
                let ml_reference_array3219 = source_ml_source1_temporary_values1450.clone();
                let ml_reference_projection3219 =
                    self::ml::Big::from_u128(ml_reference_array3219.borrow().len() as u128);
                ml_reference_projection3219
            })) {
            {
                let capture = {
                    let ml_reference_array3220 = source_ml_source1_temporary_values1450.clone();
                    let ml_reference_position3220 = source_ml_source1_temporary_index1450.clone();
                    let ml_reference_projection3220 = self::ml_array::at(
                        &*ml_reference_array3220.borrow(),
                        ml_reference_position3220.to_index(),
                    );
                    ml_reference_projection3220
                };
                if ((self::read_java_script_ordinary_value_error(heap.clone(), capture.clone()))
                    != (self::ml_string::Text::from_units(vec![])))
                {
                    self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 97, 112, 116, 117, 114, 101, 115, 32, 98, 101, 108, 111, 110, 103, 32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101, 115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110, 116, 101, 120, 116]))
                } else {
                    {
                        let source_ml_source1_temporary_index1450_2 =
                            source_ml_source1_temporary_index1450.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index1450 =
                                source_ml_source1_temporary_index1450_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallInstructionErrorJoin1459 {
    MlSource1TemporaryReadJavaScriptCallInstructionErrorJoin1459Next,
    MlSource1TemporaryReadJavaScriptCallInstructionErrorJoin1459Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460 {
    MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Next,
    MlSource1TemporaryReadJavaScriptCallOperationInstructionErrorJoin1460Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop1468(
    mut fn_: self::JavaScriptCallFunction,
    mut arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut registers: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut index: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return if (index
            < (match fn_.clone() {
                self::JavaScriptCallFunction::CallFunction(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value42282,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value42282,
            })) {
            {
                let value = ({
                    let ml_reference_array3221 = arguments_.clone();
                    let ml_reference_position3221 = index;
                    let ml_reference_projection3221 = self::ml_array::optional_at(
                        &*ml_reference_array3221.borrow(),
                        self::ml_array::number_index(ml_reference_position3221),
                    );
                    ml_reference_projection3221
                })
                .clone()
                .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                {
                    let slot = (index + 1f64);
                    {
                        let registers_2 = std::rc::Rc::new(std::cell::RefCell::new({
                            let ml_values3222 = {
                                let ml_reference_value3223 = registers.borrow().clone();
                                ml_reference_value3223
                            };
                            let ml_callback3222 = {
                                let slot = slot;
                                let value = value.clone();
                                std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, position: f64| -> self::JavaScriptSemanticValue { if (position == slot) {
                            value.clone()
                        } else {
                            entry.clone()
                        } })
                            };
                            ml_values3222
                                .iter()
                                .cloned()
                                .enumerate()
                                .map(|(ml_index3222, ml_value3222)| {
                                    ml_callback3222(ml_value3222, (ml_index3222 as f64))
                                })
                                .collect::<Vec<_>>()
                        }));
                        {
                            let index_2 = (index + 1f64);
                            {
                                (registers, index) = (registers_2.clone(), index_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            registers.clone()
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop1469(
    mut fn_: self::JavaScriptCallFunction,
    mut registers: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut index: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return match fn_.clone() {
            self::JavaScriptCallFunction::CallFunction(
                _,
                _,
                _,
                _,
                shared_constructor_field1,
                _,
                shared_constructor_field2,
                _,
                _,
                _,
            ) => {
                if (index
                    < ({
                        let ml_reference_array3224 = shared_constructor_field2.clone();
                        let ml_reference_projection3224 =
                            ml_reference_array3224.borrow().len() as f64;
                        ml_reference_projection3224
                    }))
                {
                    {
                        let value = ({
                            let source_ml_source2_array_read_value42288 =
                                shared_constructor_field2.clone();
                            {
                                let ml_reference_array3225 =
                                    source_ml_source2_array_read_value42288.clone();
                                let ml_reference_position3225 = index;
                                let ml_reference_projection3225 = self::ml_array::optional_at(
                                    &*ml_reference_array3225.borrow(),
                                    self::ml_array::number_index(ml_reference_position3225),
                                );
                                ml_reference_projection3225
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined);
                        {
                            let slot = ((shared_constructor_field1 + 1f64) + index);
                            {
                                let registers_2 = std::rc::Rc::new(std::cell::RefCell::new({
                                    let ml_values3226 = {
                                        let ml_reference_value3227 = registers.borrow().clone();
                                        ml_reference_value3227
                                    };
                                    let ml_callback3226 = {
                                        let slot = slot;
                                        let value = value.clone();
                                        std::rc::Rc::new(move |entry: self::JavaScriptSemanticValue, position: f64| -> self::JavaScriptSemanticValue { if (position == slot) {
                                    value.clone()
                                } else {
                                    entry.clone()
                                } })
                                    };
                                    ml_values3226
                                        .iter()
                                        .cloned()
                                        .enumerate()
                                        .map(|(ml_index3226, ml_value3226)| {
                                            ml_callback3226(ml_value3226, (ml_index3226 as f64))
                                        })
                                        .collect::<Vec<_>>()
                                }));
                                {
                                    let index_2 = (index + 1f64);
                                    {
                                        (registers, index) = (registers_2.clone(), index_2);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    registers.clone()
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryCreateJavaScriptCallFrameLoop1467Result {
    MlSource1TemporaryCreateJavaScriptCallFrameLoop1467Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_create_java_script_call_frame_loop1467(
    mut fn_: self::JavaScriptCallFunction,
    mut registers: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut index: f64,
) -> self::MlSource1TemporaryCreateJavaScriptCallFrameLoop1467Result {
    loop {
        return if (index
            < (match fn_.clone() {
                self::JavaScriptCallFunction::CallFunction(
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value42292,
                    _,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value42292,
            })) {
            {
                let source_ml_source1_temporary_create_java_script_call_frame_ignored1470 = {
                    let ml_method_owner3228 = registers.clone();
                    let ml_method_argument3228_0 = self::JavaScriptSemanticValue::Undefined;
                    let ml_inserted = vec![ml_method_argument3228_0];
                    let mut ml_method_values3228 = ml_method_owner3228.borrow_mut();
                    let ml_length = ml_method_values3228
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values3228.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let index_2 = (index + 1f64);
                    {
                        index = index_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryCreateJavaScriptCallFrameLoop1467Result::MlSource1TemporaryCreateJavaScriptCallFrameLoop1467Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471 {
    MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471Next,
    MlSource1TemporaryReadJavaScriptNumberCallSourceOperationJoin1471Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472 {
    MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472Next,
    MlSource1TemporaryReadJavaScriptTextCallSourceOperationJoin1472Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result {
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Done,
    MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop1476(
    mut argument_slots: f64,
    mut source_ml_source1_temporary_values1473: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>,
    >,
    mut source_ml_source1_temporary_index1473: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1473
            < ({
                let ml_reference_array3229 = source_ml_source1_temporary_values1473.clone();
                let ml_reference_projection3229 =
                    self::ml::Big::from_u128(ml_reference_array3229.borrow().len() as u128);
                ml_reference_projection3229
            })) {
            {
                let list = {
                    let ml_reference_array3230 = source_ml_source1_temporary_values1473.clone();
                    let ml_reference_position3230 = source_ml_source1_temporary_index1473.clone();
                    let ml_reference_projection3230 = self::ml_array::at(
                        &*ml_reference_array3230.borrow(),
                        ml_reference_position3230.to_index(),
                    );
                    ml_reference_projection3230
                };
                {
                    let argument_slots_2 = (argument_slots + {
                        let ml_reference_array3231 = match list.clone() {
                            self::JavaScriptCallArgumentList::CallArgumentList(
                                _,
                                _,
                                source_ml_source2_data_property_value42295,
                            ) => source_ml_source2_data_property_value42295.clone(),
                        };
                        let ml_reference_projection3231 =
                            ml_reference_array3231.borrow().len() as f64;
                        ml_reference_projection3231
                    });
                    {
                        let source_ml_source1_temporary_index1473_2 =
                            source_ml_source1_temporary_index1473.add(&self::ml::Big::from_i128(1));
                        {
                            (argument_slots, source_ml_source1_temporary_index1473) = (
                                argument_slots_2,
                                source_ml_source1_temporary_index1473_2.clone(),
                            );
                            continue;
                        }
                    }
                }
            }
        } else {
            argument_slots
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477 {
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477Next,
    MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_machine_error_loop1475(
    mut program: self::JavaScriptCallProgram,
    mut machine: self::JavaScriptCallMachine,
    mut slots: f64,
    mut argument_slots: f64,
    mut index: f64,
) -> self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result {
    loop {
        return match machine.clone() {
            self::JavaScriptCallMachine::CallMachine(
                shared_constructor_field1,
                shared_constructor_field2,
            ) => {
                let shared_constructor_field1 = *shared_constructor_field1;
                if (index
                    < ({
                        let ml_reference_array3232 = shared_constructor_field2.clone();
                        let ml_reference_projection3232 =
                            ml_reference_array3232.borrow().len() as f64;
                        ml_reference_projection3232
                    }))
                {
                    {
                        let frame = {
                            let source_ml_source2_array_read_value42299 =
                                shared_constructor_field2.clone();
                            {
                                let ml_reference_array3233 =
                                    source_ml_source2_array_read_value42299.clone();
                                let ml_reference_position3233 = index;
                                let ml_reference_projection3233 = self::ml_array::optional_at(
                                    &*ml_reference_array3233.borrow(),
                                    self::ml_array::number_index(ml_reference_position3233),
                                );
                                ml_reference_projection3233
                            }
                        };
                        if match (frame.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 115, 116, 97, 99, 107, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                        } else {
                            {
                                let fn_ = self::read_java_script_executable_call_function(
                                    program.clone(),
                                    shared_constructor_field1.clone(),
                                    match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(
                                            source_ml_source2_data_property_value42302,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value42302.clone(),
                                    },
                                );
                                if match (fn_.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } {
                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 102, 117, 110, 99, 116, 105, 111, 110]))
                                } else {
                                    if ((({
                                        let ml_reference_array3234 =
                                            match frame.clone().expect("a proven present value") {
                                                self::JavaScriptCallFrame::CallFrame(
                                                    _,
                                                    _,
                                                    source_ml_source2_data_property_value42305,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => source_ml_source2_data_property_value42305
                                                    .clone(),
                                            };
                                        let ml_reference_projection3234 =
                                            ml_reference_array3234.borrow().len() as f64;
                                        ml_reference_projection3234
                                    }) != (match fn_.clone().expect("a proven present value") {
                                        self::JavaScriptCallFunction::CallFunction(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value42306,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value42306,
                                    })) || !(self::test_java_script_call_coordinate(
                                        match frame.clone().expect("a proven present value") {
                                            self::JavaScriptCallFrame::CallFrame(
                                                _,
                                                source_ml_source2_data_property_value42307,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source2_data_property_value42307,
                                        },
                                        ({
                                            let ml_reference_array3235 = match fn_
                                                .clone()
                                                .expect("a proven present value")
                                            {
                                                self::JavaScriptCallFunction::CallFunction(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source2_data_property_value42308,
                                                    _,
                                                    _,
                                                ) => source_ml_source2_data_property_value42308
                                                    .clone(),
                                            };
                                            let ml_reference_projection3235 =
                                                ml_reference_array3235.borrow().len() as f64;
                                            ml_reference_projection3235
                                        } + 1f64),
                                    ))) {
                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 97, 110, 100, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                                    } else {
                                        {
                                            let slots_2 = (slots + {
                                                let ml_reference_array3236 = match frame
                                                    .clone()
                                                    .expect("a proven present value")
                                                {
                                                    self::JavaScriptCallFrame::CallFrame(
                                                        _,
                                                        _,
                                                        source_ml_source2_data_property_value42309,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source2_data_property_value42309
                                                        .clone(),
                                                };
                                                let ml_reference_projection3236 =
                                                    ml_reference_array3236.borrow().len() as f64;
                                                ml_reference_projection3236
                                            });
                                            {
                                                let argument_slots_2 = (argument_slots
                                                    + {
                                                        let ml_reference_array3237 = match frame.clone().expect("a proven present value") {
                                                    self::JavaScriptCallFrame::CallFrame(_, _, _, source_ml_source2_data_property_value42310, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value42310.clone()
                                                    }
                                                };
                                                        let ml_reference_projection3237 =
                                                            ml_reference_array3237.borrow().len()
                                                                as f64;
                                                        ml_reference_projection3237
                                                    });
                                                {
                                                    let source_ml_source1_temporary_values1473 = (match frame.clone().expect("a proven present value") {
                                                        self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, source_ml_source2_data_property_value42311, _, _) => {
                                                            ((source_ml_source2_data_property_value42311.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallArgumentList>>>>,))).0).clone()
                                                        }
                                                    }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallArgumentList>::new())));
                                                    {
                                                        let source_ml_source1_temporary_index1473 =
                                                            self::ml::Big::from_i128(0);
                                                        {
                                                            let argument_slots_3 = self::source_ml_source1_temporary_read_java_script_call_machine_error_loop1476(argument_slots_2, source_ml_source1_temporary_values1473.clone(), source_ml_source1_temporary_index1473.clone());
                                                            if (((slots_2 > (1048576f64))
                                                                || (argument_slots_3
                                                                    > (1048576f64)))
                                                                || (({
                                                                    let ml_reference_array3238 = match frame.clone().expect("a proven present value") {
                                                                self::JavaScriptCallFrame::CallFrame(_, _, _, _, source_ml_source2_data_property_value42313, _, _, _, _, _) => {
                                                                    source_ml_source2_data_property_value42313.clone()
                                                                }
                                                            };
                                                                    let ml_reference_projection3238 =
                                                                        ml_reference_array3238
                                                                            .borrow()
                                                                            .len()
                                                                            as f64;
                                                                    ml_reference_projection3238
                                                                }) > (1024f64)))
                                                            {
                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 114, 97, 109, 101, 32, 115, 116, 111, 114, 97, 103, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                            } else {
                                                                {
                                                                    let frame_error = self::read_java_script_call_frame_error(program.clone(), shared_constructor_field1.clone(), frame.clone().expect("a proven present value"), fn_.clone().expect("a proven present value"), (index < (({ let ml_reference_array3239 = shared_constructor_field2.clone(); let ml_reference_projection3239 = ml_reference_array3239.borrow().len() as f64; ml_reference_projection3239 } - 1f64))));
                                                                    if (frame_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Return(frame_error.clone())
                                                                    } else {
                                                                        if (!(match (match frame.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value42317, _) => {
                                                                                ((source_ml_source2_data_property_value42317.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && { let ml_values3240 = { let ml_reference_value3241 = (std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value3242 = shared_constructor_field2.borrow().clone(); ml_reference_value3242 }, 0f64, index)))).borrow().clone(); ml_reference_value3241 }; let ml_callback3240 = { let program = program.clone(); let machine = machine.clone(); let fn_ = fn_.clone(); let frame = frame.clone(); std::rc::Rc::new(move |parent: self::JavaScriptCallFrame| -> bool { ((!(match (match parent.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value42320, _) => {
                                                                                ((source_ml_source2_data_property_value42320.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && {
                                                                            let source_ml_source2_comparison_operand42326 = {
                                                                                let source_ml_source2_optional_chain_receiver42323 = (self::read_java_script_executable_call_function(program.clone(), match machine.clone() {
                                                                                    self::JavaScriptCallMachine::CallMachine(source_ml_source2_data_property_value42321, _) => {
                                                                                        let source_ml_source2_data_property_value42321 = *source_ml_source2_data_property_value42321;
                                                                                        source_ml_source2_data_property_value42321.clone()
                                                                                    }
                                                                                }, match parent.clone() {
                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_data_property_value42322, _, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value42322.clone()
                                                                                    }
                                                                                }),);
                                                                                if match (source_ml_source2_optional_chain_receiver42323.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                                    Some({
                                                                                        let source_ml_source1_temporary_optional_chain1474 = ((source_ml_source2_optional_chain_receiver42323.clone()).0).clone().expect("a proven present value");
                                                                                        match source_ml_source1_temporary_optional_chain1474.clone() {
                                                                                            self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value42324, _, _, _, _, _, _, _, _) => {
                                                                                                source_ml_source2_data_property_value42324.clone()
                                                                                            }
                                                                                        }
                                                                                    })
                                                                                } else {
                                                                                    None::<self::ml::Big>
                                                                                }
                                                                            };
                                                                            {
                                                                                let source_ml_source2_comparison_operand42327 = match fn_.clone().expect("a proven present value") {
                                                                                    self::JavaScriptCallFunction::CallFunction(_, source_ml_source2_data_property_value42325, _, _, _, _, _, _, _, _) => {
                                                                                        source_ml_source2_data_property_value42325.clone()
                                                                                    }
                                                                                };
                                                                                match (source_ml_source2_comparison_operand42326.clone(), source_ml_source2_comparison_operand42327.clone()) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                            }
                                                                        }) && self::test_java_script_semantic_same_value((match parent.clone() {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value42328, _) => {
                                                                                ((source_ml_source2_data_property_value42328.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value"), (match frame.clone().expect("a proven present value") {
                                                                            self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value42329, _) => {
                                                                                ((source_ml_source2_data_property_value42329.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                            }
                                                                        }).clone().expect("a proven present value"))) }) }; ml_values3240.iter().cloned().enumerate().any(|(ml_index3240, ml_value3240)| ml_callback3240(ml_value3240)) }) {
                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Return(self::ml_string::Text::from_units(vec![106, 111, 105, 110, 32, 97, 99, 116, 105, 118, 97, 116, 105, 111, 110, 115, 32, 105, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 114, 101, 97, 108, 109, 32, 99, 97, 110, 110, 111, 116, 32, 111, 119, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 103, 117, 97, 114, 100]))
                                                                        } else {
                                                                            {
                                                                                let selected_parent = {
                                                                                    let source_ml_source2_array_read_value42331 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_array_read_index42332 = (index - 1f64);
                                                                                        { let ml_reference_array3243 = source_ml_source2_array_read_value42331.clone(); let ml_reference_position3243 = source_ml_source2_array_read_index42332; let ml_reference_projection3243 = self::ml_array::optional_at(&*ml_reference_array3243.borrow(), self::ml_array::number_index(ml_reference_position3243)); ml_reference_projection3243 }
                                                                                    }
                                                                                };
                                                                                {
                                                                                    let source_ml_source2_s42334 = if !(match (selected_parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                        {
                                                                                            let selected_error = self::read_java_script_selected_parent_call_error(program.clone(), shared_constructor_field1.clone(), selected_parent.clone().expect("a proven present value"), fn_.clone().expect("a proven present value"), frame.clone().expect("a proven present value"));
                                                                                            if (selected_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477Return(selected_error.clone())
                                                                                            } else {
                                                                                                self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477Next
                                                                                            }
                                                                                        }
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477Next
                                                                                    };
                                                                                    match source_ml_source2_s42334.clone() {
                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477Next => {
                                                                                            {
                                                                                                let construction_error = self::read_java_script_active_construction_frame_error(program.clone(), machine.clone(), index, frame.clone().expect("a proven present value"), fn_.clone().expect("a proven present value"));
                                                                                                if (construction_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Return(construction_error.clone())
                                                                                                } else {
                                                                                                    {
                                                                                                        let index_2 = (index + 1f64);
                                                                                                        {
                                                                                                            (slots, argument_slots, index) = (slots_2, argument_slots_3, index_2);
                                                                                                            continue;
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477::MlSource1TemporaryReadJavaScriptCallMachineErrorJoin1477Return(source_ml_source1_temporary_result1477) => {
                                                                                            self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Return(source_ml_source1_temporary_result1477.clone())
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
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Result::MlSource1TemporaryReadJavaScriptCallMachineErrorLoop1475Done
                }
            }
        };
    }
}
