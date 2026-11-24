// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySpeciesDirectory;
use super::JavaScriptCallExecution;
use super::JavaScriptCallFunction;
use super::JavaScriptCallInstruction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallProgram;
use super::JavaScriptCallTemplate;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Result;
use super::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Result;
use super::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Result;
use super::MlSource1TemporaryRunJavaScriptCallMachineLoop1439Result;
use super::find_java_script_call_code_function;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_species_directory_error;
use super::read_java_script_call_code_registers;
use super::read_java_script_call_machine_error;
use super::read_java_script_call_template_directory_error;
use super::read_java_script_collection_constructor_kind;
use super::read_java_script_function_source_directory_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_read_java_script_call_function_body_error_loop1453;
use super::source_ml_source1_temporary_read_java_script_call_function_body_error_loop1454;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop1446;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop1447;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop1448;
use super::source_ml_source1_temporary_read_java_script_call_program_error_loop1449;
use super::source_ml_source1_temporary_run_java_script_call_machine_loop1439;
use super::test_java_script_call_code_coordinate;
use super::test_java_script_collection_constructor_mode;
use super::test_java_script_collection_intrinsic_mode;
use super::test_java_script_collection_iterator_intrinsic_mode;
use super::test_java_script_error_constructor_mode;
use super::test_java_script_error_intrinsic_mode;
use super::test_java_script_host_capability_mode;
use super::test_java_script_ordinary_class_constructor_mode;
use super::test_java_script_primitive_intrinsic_mode;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn run_java_script_call_machine(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    limit: f64,
    object_identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
) -> self::JavaScriptCallExecution {
    if (!(self::test_java_script_call_coordinate(limit, 1048577f64))
        || (({
            let ml_reference_array2197 = object_identities.clone();
            let ml_reference_projection2197 = ml_reference_array2197.borrow().len() as f64;
            ml_reference_projection2197
        }) > (4096f64)))
    {
        self::JavaScriptCallExecution::CallExecution(
            Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(
                Box::new(match machine.clone() {
                    self::JavaScriptCallMachine::CallMachine(
                        source_ml_source2_data_property_value34007,
                        _,
                    ) => {
                        let source_ml_source2_data_property_value34007 =
                            *source_ml_source2_data_property_value34007;
                        source_ml_source2_data_property_value34007.clone()
                    }
                }),
                self::ml_string::Text::from_units(vec![
                    101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114,
                    101, 115, 32, 98, 111, 117, 110, 100, 101, 100, 32, 101, 120, 97, 99, 116, 32,
                    115, 116, 101, 112, 32, 97, 110, 100, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                    111, 110, 32, 98, 117, 100, 103, 101, 116, 115,
                ]),
            )),
            0f64,
            0f64,
        )
    } else {
        {
            let error = self::read_java_script_call_machine_error(program.clone(), machine.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptCallExecution::CallExecution(
                    Box::new(self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(match machine.clone() {
                            self::JavaScriptCallMachine::CallMachine(
                                source_ml_source2_data_property_value34008,
                                _,
                            ) => {
                                let source_ml_source2_data_property_value34008 =
                                    *source_ml_source2_data_property_value34008;
                                source_ml_source2_data_property_value34008.clone()
                            }
                        }),
                        error.clone(),
                    )),
                    0f64,
                    0f64,
                )
            } else {
                {
                    let current = self::read_java_script_call_execution_transition(
                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(
                            machine.clone(),
                        )),
                    );
                    {
                        let steps = 0f64;
                        {
                            let used = 0f64;
                            {
                                let source_ml_source2_s34009 = self::source_ml_source1_temporary_run_java_script_call_machine_loop1439(program.clone(), limit, object_identities.clone(), current.clone(), steps, used);
                                match source_ml_source2_s34009.clone() {
                                    self::MlSource1TemporaryRunJavaScriptCallMachineLoop1439Result::MlSource1TemporaryRunJavaScriptCallMachineLoop1439Done(current_2, steps_2, used_2) => {
                                        let current_2 = *current_2;
                                        {
                                            let final_ = self::read_java_script_call_execution_transition(current_2.clone());
                                            match final_.clone() {
                                                self::JavaScriptCallMachineTransition::MachineContinue(machine_2) => {
                                                    let machine_2 = *machine_2;
                                                    self::JavaScriptCallExecution::CallExecution(Box::new(self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine_2.clone()), self::ml_string::Text::from_units(vec![101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 116, 114, 97, 110, 115, 105, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116]))), steps_2, used_2)
                                                }
                                                _ => {
                                                    self::JavaScriptCallExecution::CallExecution(Box::new(current_2.clone()), steps_2, used_2)
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryRunJavaScriptCallMachineLoop1439Result::MlSource1TemporaryRunJavaScriptCallMachineLoop1439Return(source_ml_source1_temporary_result1439) => {
                                        let source_ml_source1_temporary_result1439 = *source_ml_source1_temporary_result1439;
                                        source_ml_source1_temporary_result1439.clone()
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
pub fn read_java_script_call_execution_transition(
    transition: self::JavaScriptCallMachineTransition,
) -> self::JavaScriptCallMachineTransition {
    transition.clone()
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_coordinate(value: f64, limit: f64) -> bool {
    self::test_java_script_call_code_coordinate(value, limit)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_function(
    program: self::JavaScriptCallProgram,
    identity: self::ml::Big,
) -> f64 {
    self::find_java_script_call_code_function(program.clone(), identity.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_registers(
    instruction: self::JavaScriptCallInstruction,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    self::read_java_script_call_code_registers(instruction.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_call_program_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
) -> self::ml_string::Text {
    match program.clone() {
        self::JavaScriptCallProgram::CallProgram(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            {
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    {
                        let source_error =
                            self::read_java_script_function_source_directory_error(program.clone());
                        if (source_error != (self::ml_string::Text::from_units(vec![]))) {
                            source_error.clone()
                        } else {
                            {
                                let source_ml_source2_s34012 = if !(match (
                                    match program.clone() {
                                        self::JavaScriptCallProgram::CallProgram(
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value34013,
                                            _,
                                        ) => ((source_ml_source2_data_property_value34013
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptArraySpeciesDirectory>,)
                                            }))
                                        .0)
                                            .clone(),
                                    },
                                    None::<()>,
                                ) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                }) {
                                    {
                                        let invalid = self::read_java_script_array_species_directory_error(heap.clone(), (match program.clone() {
                                            self::JavaScriptCallProgram::CallProgram(_, _, _, _, source_ml_source2_data_property_value34014, _) => {
                                                ((source_ml_source2_data_property_value34014.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySpeciesDirectory>,))).0).clone()
                                            }
                                        }).clone().expect("a proven present value"));
                                        if (invalid != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445::MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445Return(invalid.clone())
                                        } else {
                                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445::MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445Next
                                        }
                                    }
                                } else {
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445::MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445Next
                                };
                                match source_ml_source2_s34012.clone() {
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445::MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445Next => {
                                        if ((({ let ml_reference_array2198 = shared_constructor_field1.clone(); let ml_reference_projection2198 = ml_reference_array2198.borrow().len() as f64; ml_reference_projection2198 }) > (4096f64)) || (({ let ml_reference_array2199 = shared_constructor_field2.clone(); let ml_reference_projection2199 = ml_reference_array2199.borrow().len() as f64; ml_reference_projection2199 }) > (4096f64))) {
                                            self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 100, 105, 114, 101, 99, 116, 111, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 114, 101, 115, 111, 117, 114, 99, 101, 32, 98, 117, 100, 103, 101, 116])
                                        } else {
                                            {
                                                let previous = self::ml::Big::from_i128(0);
                                                {
                                                    let source_ml_source1_temporary_values1441 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source1_temporary_index1441 = self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s34020 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop1446(previous.clone(), source_ml_source1_temporary_values1441.clone(), source_ml_source1_temporary_index1441.clone());
                                                            match source_ml_source2_s34020.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Done(previous_2) => {
                                                                    if {
                                                                        let source_ml_source2_s34022 = shared_constructor_field3.clone();
                                                                        match source_ml_source2_s34022.clone() {
                                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                                false
                                                                            }
                                                                            _ => {
                                                                                true
                                                                            }
                                                                        }
                                                                    } {
                                                                        self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 116, 104, 101, 32, 97, 117, 116, 104, 111, 114, 105, 116, 97, 116, 105, 118, 101, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 83, 121, 109, 98, 111, 108])
                                                                    } else {
                                                                        {
                                                                            let key_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field3.clone());
                                                                            if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                key_error.clone()
                                                                            } else {
                                                                                {
                                                                                    let previous_3 = self::ml::Big::from_i128(0);
                                                                                    {
                                                                                        let count = 0f64;
                                                                                        {
                                                                                            let source_ml_source1_temporary_values1442 = shared_constructor_field1.clone();
                                                                                            {
                                                                                                let source_ml_source1_temporary_index1442 = self::ml::Big::from_i128(0);
                                                                                                {
                                                                                                    let source_ml_source2_s34027 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop1447(program.clone(), heap.clone(), previous_3.clone(), count, source_ml_source1_temporary_values1442.clone(), source_ml_source1_temporary_index1442.clone());
                                                                                                    match source_ml_source2_s34027.clone() {
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Done(count_2) => {
                                                                                                            {
                                                                                                                let template_error = self::read_java_script_call_template_directory_error(program.clone(), heap.clone());
                                                                                                                if (template_error != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                                    template_error.clone()
                                                                                                                } else {
                                                                                                                    {
                                                                                                                        let source_ml_source1_temporary_values1443 = (match program.clone() {
                                                                                                                            self::JavaScriptCallProgram::CallProgram(_, _, _, source_ml_source2_data_property_value34028, _, _) => {
                                                                                                                                ((source_ml_source2_data_property_value34028.clone().unwrap_or_else(|| (None::<std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>>>,))).0).clone()
                                                                                                                            }
                                                                                                                        }).clone().unwrap_or_else(|| std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptCallTemplate>::new())));
                                                                                                                        {
                                                                                                                            let source_ml_source1_temporary_index1443 = self::ml::Big::from_i128(0);
                                                                                                                            {
                                                                                                                                let source_ml_source2_s34029 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop1448(program.clone(), heap.clone(), count_2, source_ml_source1_temporary_values1443.clone(), source_ml_source1_temporary_index1443.clone());
                                                                                                                                match source_ml_source2_s34029.clone() {
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Done => {
                                                                                                                                        {
                                                                                                                                            let source_ml_source1_temporary_values1444 = match heap.clone() {
                                                                                                                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_data_property_value34030, _, _, _, _, _) => {
                                                                                                                                                    source_ml_source2_data_property_value34030.clone()
                                                                                                                                                }
                                                                                                                                            };
                                                                                                                                            {
                                                                                                                                                let source_ml_source1_temporary_index1444 = self::ml::Big::from_i128(0);
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_s34031 = self::source_ml_source1_temporary_read_java_script_call_program_error_loop1449(program.clone(), heap.clone(), source_ml_source1_temporary_values1444.clone(), source_ml_source1_temporary_index1444.clone());
                                                                                                                                                    match source_ml_source2_s34031.clone() {
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Done => {
                                                                                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                                                                                        }
                                                                                                                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Return(source_ml_source1_temporary_result1449) => {
                                                                                                                                                            source_ml_source1_temporary_result1449.clone()
                                                                                                                                                        }
                                                                                                                                                    }
                                                                                                                                                }
                                                                                                                                            }
                                                                                                                                        }
                                                                                                                                    }
                                                                                                                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Return(source_ml_source1_temporary_result1449_2) => {
                                                                                                                                        source_ml_source1_temporary_result1449_2.clone()
                                                                                                                                    }
                                                                                                                                }
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Return(source_ml_source1_temporary_result1449_3) => {
                                                                                                            source_ml_source1_temporary_result1449_3.clone()
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
                                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Return(source_ml_source1_temporary_result1449_4) => {
                                                                    source_ml_source1_temporary_result1449_4.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445::MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445Return(source_ml_source1_temporary_result1449_5) => {
                                        source_ml_source1_temporary_result1449_5.clone()
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
pub fn read_java_script_call_function_body_error(
    program: self::JavaScriptCallProgram,
    heap: self::JavaScriptOrdinaryHeap,
    fn_: self::JavaScriptCallFunction,
) -> self::ml_string::Text {
    match fn_.clone() {
        self::JavaScriptCallFunction::CallFunction(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            _,
            _,
        ) => {
            let shared_constructor_field4 = *shared_constructor_field4;
            if !(self::ml_array::includes(
                &{
                    let ml_reference_value2200 = (match program.clone() {
                        self::JavaScriptCallProgram::CallProgram(
                            _,
                            source_ml_source2_data_property_value34033,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value34033.clone(),
                    })
                    .borrow()
                    .clone();
                    ml_reference_value2200
                },
                &shared_constructor_field2.clone(),
                0f64,
                false,
                |a, b| a == b,
            )) {
                self::ml_string::Text::from_units(vec![
                    99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101,
                    113, 117, 105, 114, 101, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97,
                    108, 32, 100, 101, 99, 108, 97, 114, 101, 100, 32, 115, 111, 117, 114, 99, 101,
                    32, 114, 101, 97, 108, 109,
                ])
            } else {
                {
                    let callable = self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        self::JavaScriptSemanticValue::Object(
                            shared_constructor_field1.clone(),
                            true,
                        ),
                    );
                    if (callable != (self::ml_string::Text::from_units(vec![]))) {
                        callable.clone()
                    } else {
                        if ((((((((((shared_constructor_field3
                            != (self::ml_string::Text::from_units(vec![
                                115, 116, 114, 105, 99, 116,
                            ])))
                            && (shared_constructor_field3
                                != (self::ml_string::Text::from_units(vec![
                                    108, 101, 120, 105, 99, 97, 108,
                                ]))))
                            && !(self::test_java_script_error_constructor_mode(
                                &(shared_constructor_field3),
                            )))
                            && !(self::test_java_script_error_intrinsic_mode(
                                &(shared_constructor_field3),
                            )))
                            && !(self::test_java_script_host_capability_mode(
                                &(shared_constructor_field3),
                            )))
                            && !(self::test_java_script_ordinary_class_constructor_mode(
                                &(shared_constructor_field3),
                            )))
                            && !(self::test_java_script_collection_constructor_mode(
                                &(shared_constructor_field3),
                            )))
                            && !(self::test_java_script_collection_intrinsic_mode(
                                &(shared_constructor_field3),
                            )))
                            && !(self::test_java_script_primitive_intrinsic_mode(
                                &(shared_constructor_field3),
                            )))
                            && !(self::test_java_script_collection_iterator_intrinsic_mode(
                                &(shared_constructor_field3),
                            )))
                        {
                            self::ml_string::Text::from_units(vec![
                                99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 114, 101, 113, 117,
                                105, 114, 101, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32,
                                115, 116, 114, 105, 99, 116, 44, 32, 108, 101, 120, 105, 99, 97,
                                108, 32, 111, 114, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101,
                                100, 32, 69, 114, 114, 111, 114, 32, 114, 101, 99, 101, 105, 118,
                                101, 114, 32, 98, 105, 110, 100, 105, 110, 103,
                            ])
                        } else {
                            if (!(match (
                                match fn_.clone() {
                                    self::JavaScriptCallFunction::CallFunction(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value34048,
                                        _,
                                    ) => ((source_ml_source2_data_property_value34048
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptSemanticValue>,)
                                        }))
                                    .0)
                                        .clone(),
                                },
                                None::<()>,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            }) && ((((shared_constructor_field3
                                != (self::ml_string::Text::from_units(vec![
                                    108, 101, 120, 105, 99, 97, 108,
                                ])))
                                || {
                                    let source_ml_source2_s34052 = (match fn_.clone() {
                                        self::JavaScriptCallFunction::CallFunction(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value34055,
                                            _,
                                        ) => ((source_ml_source2_data_property_value34055
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptSemanticValue>,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .expect("a proven present value");
                                    match source_ml_source2_s34052.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => false,
                                        _ => true,
                                    }
                                })
                                || !(((match (match fn_.clone() {
                                    self::JavaScriptCallFunction::CallFunction(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value34056,
                                        _,
                                    ) => ((source_ml_source2_data_property_value34056
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptSemanticValue>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptSemanticValue::Object(
                                        _,
                                        source_ml_source2_data_property_value34057,
                                    ) => Some((source_ml_source2_data_property_value34057,)),
                                    _ => None::<(bool,)>,
                                })
                                .clone()
                                .expect("a proven present value"))
                                .0))
                                || ((self::read_java_script_ordinary_value_error(
                                    heap.clone(),
                                    (match fn_.clone() {
                                        self::JavaScriptCallFunction::CallFunction(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value34059,
                                            _,
                                        ) => ((source_ml_source2_data_property_value34059
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptSemanticValue>,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .expect("a proven present value"),
                                )) != (self::ml_string::Text::from_units(vec![])))))
                            {
                                self::ml_string::Text::from_units(vec![
                                    108, 101, 120, 105, 99, 97, 108, 32, 102, 117, 110, 99, 116,
                                    105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116,
                                    104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97,
                                    112, 116, 117, 114, 101, 100, 32, 110, 101, 119, 84, 97, 114,
                                    103, 101, 116,
                                ])
                            } else {
                                if ((self::read_java_script_ordinary_value_error(
                                    heap.clone(),
                                    shared_constructor_field4.clone(),
                                )) != (self::ml_string::Text::from_units(vec![])))
                                {
                                    self::ml_string::Text::from_units(vec![
                                        108, 101, 120, 105, 99, 97, 108, 32, 114, 101, 99, 101,
                                        105, 118, 101, 114, 115, 32, 98, 101, 108, 111, 110, 103,
                                        32, 116, 111, 32, 116, 104, 101, 32, 108, 97, 116, 101,
                                        115, 116, 32, 103, 108, 111, 98, 97, 108, 32, 99, 111, 110,
                                        116, 101, 120, 116,
                                    ])
                                } else {
                                    if (self::test_java_script_collection_constructor_mode(
                                        &(shared_constructor_field3),
                                    ) && ((self::read_java_script_collection_constructor_kind(
                                        fn_.clone(),
                                    )) == (self::ml_string::Text::from_units(vec![]))))
                                    {
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 99,
                                            111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 115,
                                            32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101,
                                            105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 105,
                                            110, 103, 108, 101, 32, 115, 116, 97, 110, 100, 97,
                                            114, 100, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111,
                                            110, 32, 107, 105, 110, 100,
                                        ])
                                    } else {
                                        if (((!(self::test_java_script_call_coordinate(
                                            shared_constructor_field6,
                                            4097f64,
                                        )) || (shared_constructor_field6 == (0f64)))
                                            || !(self::test_java_script_call_coordinate(
                                                shared_constructor_field5,
                                                shared_constructor_field6,
                                            )))
                                            || (((shared_constructor_field5 + {
                                                let ml_reference_array2201 =
                                                    shared_constructor_field7.clone();
                                                let ml_reference_projection2201 =
                                                    ml_reference_array2201.borrow().len() as f64;
                                                ml_reference_projection2201
                                            }) + 1f64)
                                                > shared_constructor_field6))
                                        {
                                            self::ml_string::Text::from_units(vec![
                                                99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105,
                                                111, 110, 32, 114, 101, 103, 105, 115, 116, 101,
                                                114, 32, 97, 110, 100, 32, 112, 97, 114, 97, 109,
                                                101, 116, 101, 114, 32, 99, 111, 111, 114, 100,
                                                105, 110, 97, 116, 101, 115, 32, 109, 117, 115,
                                                116, 32, 98, 101, 32, 99, 111, 109, 112, 108, 101,
                                                116, 101,
                                            ])
                                        } else {
                                            {
                                                let source_ml_source1_temporary_values1450 =
                                                    shared_constructor_field7.clone();
                                                {
                                                    let source_ml_source1_temporary_index1450 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let source_ml_source2_s34075 = self::source_ml_source1_temporary_read_java_script_call_function_body_error_loop1453(heap.clone(), source_ml_source1_temporary_values1450.clone(), source_ml_source1_temporary_index1450.clone());
                                                        match source_ml_source2_s34075.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Done => {
                                                                if ((({ let ml_reference_array2202 = shared_constructor_field8.clone(); let ml_reference_projection2202 = ml_reference_array2202.borrow().len() as f64; ml_reference_projection2202 }) > (65536f64)) || (({ let ml_reference_array2203 = shared_constructor_field8.clone(); let ml_reference_projection2203 = ml_reference_array2203.borrow().len() as f64; ml_reference_projection2203 }) == (0f64))) {
                                                                    self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116, 32, 111, 114, 32, 104, 97, 115, 32, 110, 111, 32, 101, 110, 116, 114, 121])
                                                                } else {
                                                                    {
                                                                        let source_ml_source1_temporary_values1451 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source1_temporary_index1451 = self::ml::Big::from_i128(0);
                                                                            {
                                                                                let source_ml_source2_s34081 = self::source_ml_source1_temporary_read_java_script_call_function_body_error_loop1454(program.clone(), heap.clone(), fn_.clone(), source_ml_source1_temporary_values1451.clone(), source_ml_source1_temporary_index1451.clone());
                                                                                match source_ml_source2_s34081.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Done => {
                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Return(source_ml_source1_temporary_result1454) => {
                                                                                        source_ml_source1_temporary_result1454.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1453Return(source_ml_source1_temporary_result1455) => {
                                                                source_ml_source1_temporary_result1455.clone()
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
