// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallExecution;
use super::JavaScriptCallFunction;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallProgram;
use super::JavaScriptCallTemplate;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::project_java_script_call_template_function;
use super::read_java_script_call_execution_transition;
use super::read_java_script_call_function_body_error;
use super::read_java_script_executable_call_function;
use super::resume_java_script_call_object_allocation;
use super::step_java_script_call_machine;
use super::test_java_script_call_coordinate;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_request_java_script_call_array_fallback_loop1437(
    mut properties: f64,
    mut source_ml_source1_temporary_values1436: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index1436: self::ml::Big,
) -> f64 {
    loop {
        return if (source_ml_source1_temporary_index1436
            < ({
                let ml_reference_array3200 = source_ml_source1_temporary_values1436.clone();
                let ml_reference_projection3200 =
                    self::ml::Big::from_u128(ml_reference_array3200.borrow().len() as u128);
                ml_reference_projection3200
            })) {
            {
                let object = {
                    let ml_reference_array3201 = source_ml_source1_temporary_values1436.clone();
                    let ml_reference_position3201 = source_ml_source1_temporary_index1436.clone();
                    let ml_reference_projection3201 = self::ml_array::at(
                        &*ml_reference_array3201.borrow(),
                        ml_reference_position3201.to_index(),
                    );
                    ml_reference_projection3201
                };
                {
                    let properties_2 = (properties + {
                        let ml_reference_array3202 = match object.clone() {
                            self::JavaScriptOrdinaryObject::ClosureObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42210,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value42210.clone(),
                            self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42211,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value42211.clone(),
                            self::JavaScriptOrdinaryObject::ArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42212,
                            ) => source_ml_source2_data_property_value42212.clone(),
                            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42213,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value42213.clone(),
                            self::JavaScriptOrdinaryObject::MapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42214,
                                _,
                            ) => source_ml_source2_data_property_value42214.clone(),
                            self::JavaScriptOrdinaryObject::SetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42215,
                                _,
                            ) => source_ml_source2_data_property_value42215.clone(),
                            self::JavaScriptOrdinaryObject::WeakMapObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42216,
                                _,
                            ) => source_ml_source2_data_property_value42216.clone(),
                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42217,
                                _,
                            ) => source_ml_source2_data_property_value42217.clone(),
                            self::JavaScriptOrdinaryObject::OrdinaryObject(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42218,
                            ) => source_ml_source2_data_property_value42218.clone(),
                            self::JavaScriptOrdinaryObject::HostErrorObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42219,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value42219.clone(),
                            self::JavaScriptOrdinaryObject::HostURLObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42220,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value42220.clone(),
                            self::JavaScriptOrdinaryObject::HostArrayObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42221,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value42221.clone(),
                            self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value42222,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value42222.clone(),
                        };
                        let ml_reference_projection3202 =
                            ml_reference_array3202.borrow().len() as f64;
                        ml_reference_projection3202
                    });
                    {
                        let source_ml_source1_temporary_index1436_2 =
                            source_ml_source1_temporary_index1436.add(&self::ml::Big::from_i128(1));
                        {
                            (properties, source_ml_source1_temporary_index1436) = (
                                properties_2,
                                source_ml_source1_temporary_index1436_2.clone(),
                            );
                            continue;
                        }
                    }
                }
            }
        } else {
            properties
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryRunJavaScriptCallMachineLoop1439Result {
    MlSource1TemporaryRunJavaScriptCallMachineLoop1439Done(
        Box<self::JavaScriptCallMachineTransition>,
        f64,
        f64,
    ),
    MlSource1TemporaryRunJavaScriptCallMachineLoop1439Return(Box<self::JavaScriptCallExecution>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryRunJavaScriptCallMachineJoin1440 {
    MlSource1TemporaryRunJavaScriptCallMachineJoin1440Next(
        Box<self::JavaScriptCallMachineTransition>,
        f64,
    ),
    MlSource1TemporaryRunJavaScriptCallMachineJoin1440Return(Box<self::JavaScriptCallExecution>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_run_java_script_call_machine_loop1439(
    mut program: self::JavaScriptCallProgram,
    mut limit: f64,
    mut object_identities: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut current: self::JavaScriptCallMachineTransition,
    mut steps: f64,
    mut used: f64,
) -> self::MlSource1TemporaryRunJavaScriptCallMachineLoop1439Result {
    loop {
        return if (steps < limit) {
            {
                let active = self::read_java_script_call_execution_transition(current.clone());
                {
                    let source_ml_source2_s42229 = match active.clone() {
                        self::JavaScriptCallMachineTransition::MachineContinue(machine) => {
                            let machine = *machine;
                            {
                                let current_2 = self::step_java_script_call_machine(program.clone(), machine.clone());
                                self::MlSource1TemporaryRunJavaScriptCallMachineJoin1440::MlSource1TemporaryRunJavaScriptCallMachineJoin1440Next(Box::new(current_2.clone()), used)
                            }
                        }
                        _ => {
                            match active.clone() {
                                self::JavaScriptCallMachineTransition::MachineAllocateObject(machine_2, _) => {
                                    let machine_2 = *machine_2;
                                    {
                                        let identity = { let ml_reference_array3203 = object_identities.clone(); let ml_reference_position3203 = used; let ml_reference_projection3203 = self::ml_array::optional_at(&*ml_reference_array3203.borrow(), self::ml_array::number_index(ml_reference_position3203)); ml_reference_projection3203 };
                                        if {
                                            let source_ml_source2_comparison_operand42234 = None::<()>;
                                            match (identity.clone(), source_ml_source2_comparison_operand42234.clone()) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }
                                        } {
                                            self::MlSource1TemporaryRunJavaScriptCallMachineJoin1440::MlSource1TemporaryRunJavaScriptCallMachineJoin1440Return(Box::new(self::JavaScriptCallExecution::CallExecution(Box::new(current.clone()), steps, used)))
                                        } else {
                                            {
                                                let next = self::resume_java_script_call_object_allocation(program.clone(), machine_2.clone(), identity.clone().expect("a proven present value"));
                                                {
                                                    let used_3 = match next.clone() {
                                                        self::JavaScriptCallMachineTransition::MachineContinue(_) => {
                                                            {
                                                                let used_2 = (used + 1f64);
                                                                used_2
                                                            }
                                                        }
                                                        _ => {
                                                            used
                                                        }
                                                    };
                                                    self::MlSource1TemporaryRunJavaScriptCallMachineJoin1440::MlSource1TemporaryRunJavaScriptCallMachineJoin1440Next(Box::new(next.clone()), used_3)
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    self::MlSource1TemporaryRunJavaScriptCallMachineJoin1440::MlSource1TemporaryRunJavaScriptCallMachineJoin1440Return(Box::new(self::JavaScriptCallExecution::CallExecution(Box::new(current.clone()), steps, used)))
                                }
                            }
                        }
                    };
                    match source_ml_source2_s42229.clone() {
                        self::MlSource1TemporaryRunJavaScriptCallMachineJoin1440::MlSource1TemporaryRunJavaScriptCallMachineJoin1440Next(current_3, used_4) => {
                            let current_3 = *current_3;
                            {
                                let steps_2 = (steps + 1f64);
                                {
                                    (current, steps, used) = (current_3.clone(), steps_2, used_4);
                                    continue;
                                }
                            }
                        }
                        self::MlSource1TemporaryRunJavaScriptCallMachineJoin1440::MlSource1TemporaryRunJavaScriptCallMachineJoin1440Return(source_ml_source1_temporary_result1440) => {
                            let source_ml_source1_temporary_result1440 = *source_ml_source1_temporary_result1440;
                            self::MlSource1TemporaryRunJavaScriptCallMachineLoop1439Result::MlSource1TemporaryRunJavaScriptCallMachineLoop1439Return(Box::new(source_ml_source1_temporary_result1440.clone()))
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryRunJavaScriptCallMachineLoop1439Result::MlSource1TemporaryRunJavaScriptCallMachineLoop1439Done(Box::new(current.clone()), steps, used)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop1449(
    mut program: self::JavaScriptCallProgram,
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1444: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryObject>>,
    >,
    mut source_ml_source1_temporary_index1444: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Result {
    loop {
        return if (source_ml_source1_temporary_index1444
            < ({
                let ml_reference_array3204 = source_ml_source1_temporary_values1444.clone();
                let ml_reference_projection3204 =
                    self::ml::Big::from_u128(ml_reference_array3204.borrow().len() as u128);
                ml_reference_projection3204
            })) {
            {
                let object = {
                    let ml_reference_array3205 = source_ml_source1_temporary_values1444.clone();
                    let ml_reference_position3205 = source_ml_source1_temporary_index1444.clone();
                    let ml_reference_projection3205 = self::ml_array::at(
                        &*ml_reference_array3205.borrow(),
                        ml_reference_position3205.to_index(),
                    );
                    ml_reference_projection3205
                };
                match object.clone() {
                    self::JavaScriptOrdinaryObject::BoundFunctionObject(
                        identity,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let fn_ = self::read_java_script_executable_call_function(
                            program.clone(),
                            heap.clone(),
                            identity.clone(),
                        );
                        if !(match (fn_.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        }) {
                            {
                                let error = self::read_java_script_call_function_body_error(
                                    program.clone(),
                                    heap.clone(),
                                    fn_.clone().expect("a proven present value"),
                                );
                                if (error != (self::ml_string::Text::from_units(vec![]))) {
                                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Return(error.clone())
                                } else {
                                    {
                                        let source_ml_source1_temporary_index1444_2 =
                                            source_ml_source1_temporary_index1444
                                                .add(&self::ml::Big::from_i128(1));
                                        {
                                            source_ml_source1_temporary_index1444 =
                                                source_ml_source1_temporary_index1444_2.clone();
                                            continue;
                                        }
                                    }
                                }
                            }
                        } else {
                            {
                                let source_ml_source1_temporary_index1444_3 =
                                    source_ml_source1_temporary_index1444
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index1444 =
                                        source_ml_source1_temporary_index1444_3.clone();
                                    continue;
                                }
                            }
                        }
                    }
                    _ => {
                        let source_ml_source1_temporary_index1444_4 =
                            source_ml_source1_temporary_index1444.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index1444 =
                                source_ml_source1_temporary_index1444_4.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1449Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Done,
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop1448(
    mut program: self::JavaScriptCallProgram,
    mut heap: self::JavaScriptOrdinaryHeap,
    mut count: f64,
    mut source_ml_source1_temporary_values1443: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallTemplate>>,
    >,
    mut source_ml_source1_temporary_index1443: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Result {
    loop {
        return if (source_ml_source1_temporary_index1443
            < ({
                let ml_reference_array3206 = source_ml_source1_temporary_values1443.clone();
                let ml_reference_projection3206 =
                    self::ml::Big::from_u128(ml_reference_array3206.borrow().len() as u128);
                ml_reference_projection3206
            })) {
            {
                let template = {
                    let ml_reference_array3207 = source_ml_source1_temporary_values1443.clone();
                    let ml_reference_position3207 = source_ml_source1_temporary_index1443.clone();
                    let ml_reference_projection3207 = self::ml_array::at(
                        &*ml_reference_array3207.borrow(),
                        ml_reference_position3207.to_index(),
                    );
                    ml_reference_projection3207
                };
                match template.clone() {
                    self::JavaScriptCallTemplate::CallTemplate(
                        shared_constructor_field1,
                        _,
                        _,
                        _,
                        _,
                        _,
                        shared_constructor_field2,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => {
                        let count_2 = (count + {
                            let ml_reference_array3208 = shared_constructor_field2.clone();
                            let ml_reference_projection3208 =
                                ml_reference_array3208.borrow().len() as f64;
                            ml_reference_projection3208
                        });
                        if (count_2 > (65536f64)) {
                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Return(self::ml_string::Text::from_units(vec![115, 116, 97, 116, 105, 99, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 97, 110, 100, 32, 116, 101, 109, 112, 108, 97, 116, 101, 115, 32, 115, 104, 97, 114, 101, 32, 111, 110, 101, 32, 116, 111, 116, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116]))
                        } else {
                            {
                                let body = self::project_java_script_call_template_function(
                                    template.clone(),
                                    shared_constructor_field1.clone(),
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::JavaScriptSemanticValue,
                                    >::new(
                                    ))),
                                    self::JavaScriptSemanticValue::Undefined,
                                    None::<self::JavaScriptSemanticValue>,
                                );
                                {
                                    let body_error =
                                        self::read_java_script_call_function_body_error(
                                            program.clone(),
                                            heap.clone(),
                                            body.clone(),
                                        );
                                    if (body_error != (self::ml_string::Text::from_units(vec![]))) {
                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Return(body_error.clone())
                                    } else {
                                        {
                                            let source_ml_source1_temporary_index1443_2 =
                                                source_ml_source1_temporary_index1443
                                                    .add(&self::ml::Big::from_i128(1));
                                            {
                                                (count, source_ml_source1_temporary_index1443) = (
                                                    count_2,
                                                    source_ml_source1_temporary_index1443_2.clone(),
                                                );
                                                continue;
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
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1448Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Done(f64),
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop1447(
    mut program: self::JavaScriptCallProgram,
    mut heap: self::JavaScriptOrdinaryHeap,
    mut previous: self::ml::Big,
    mut count: f64,
    mut source_ml_source1_temporary_values1442: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCallFunction>>,
    >,
    mut source_ml_source1_temporary_index1442: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Result {
    loop {
        return if (source_ml_source1_temporary_index1442
            < ({
                let ml_reference_array3209 = source_ml_source1_temporary_values1442.clone();
                let ml_reference_projection3209 =
                    self::ml::Big::from_u128(ml_reference_array3209.borrow().len() as u128);
                ml_reference_projection3209
            })) {
            {
                let fn_ = {
                    let ml_reference_array3210 = source_ml_source1_temporary_values1442.clone();
                    let ml_reference_position3210 = source_ml_source1_temporary_index1442.clone();
                    let ml_reference_projection3210 = self::ml_array::at(
                        &*ml_reference_array3210.borrow(),
                        ml_reference_position3210.to_index(),
                    );
                    ml_reference_projection3210
                };
                match fn_.clone() {
                    self::JavaScriptCallFunction::CallFunction(
                        shared_constructor_field1,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        shared_constructor_field2,
                        _,
                        _,
                    ) => {
                        if ((shared_constructor_field1 <= previous)
                            || (shared_constructor_field1
                                > (self::ml::Big::from_i128(18446744073709551615))))
                        {
                            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 102, 117, 110, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                        } else {
                            {
                                let previous_2 = shared_constructor_field1.clone();
                                {
                                    let count_2 = (count + {
                                        let ml_reference_array3211 =
                                            shared_constructor_field2.clone();
                                        let ml_reference_projection3211 =
                                            ml_reference_array3211.borrow().len() as f64;
                                        ml_reference_projection3211
                                    });
                                    if (count_2 > (65536f64)) {
                                        self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 99, 111, 100, 101, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 116, 111, 116, 97, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 117, 100, 103, 101, 116]))
                                    } else {
                                        {
                                            let body_error =
                                                self::read_java_script_call_function_body_error(
                                                    program.clone(),
                                                    heap.clone(),
                                                    fn_.clone(),
                                                );
                                            if (body_error
                                                != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Return(body_error.clone())
                                            } else {
                                                {
                                                    let source_ml_source1_temporary_index1442_2 =
                                                        source_ml_source1_temporary_index1442
                                                            .add(&self::ml::Big::from_i128(1));
                                                    {
                                                        (
                                                            previous,
                                                            count,
                                                            source_ml_source1_temporary_index1442,
                                                        ) = (
                                                            previous_2.clone(),
                                                            count_2,
                                                            source_ml_source1_temporary_index1442_2
                                                                .clone(),
                                                        );
                                                        continue;
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
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1447Done(count)
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Result {
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Done(self::ml::Big),
    MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_program_error_loop1446(
    mut previous: self::ml::Big,
    mut source_ml_source1_temporary_values1441: std::rc::Rc<std::cell::RefCell<Vec<self::ml::Big>>>,
    mut source_ml_source1_temporary_index1441: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Result {
    loop {
        return if (source_ml_source1_temporary_index1441
            < ({
                let ml_reference_array3212 = source_ml_source1_temporary_values1441.clone();
                let ml_reference_projection3212 =
                    self::ml::Big::from_u128(ml_reference_array3212.borrow().len() as u128);
                ml_reference_projection3212
            })) {
            {
                let realm = {
                    let ml_reference_array3213 = source_ml_source1_temporary_values1441.clone();
                    let ml_reference_position3213 = source_ml_source1_temporary_index1441.clone();
                    let ml_reference_projection3213 = self::ml_array::at(
                        &*ml_reference_array3213.borrow(),
                        ml_reference_position3213.to_index(),
                    );
                    ml_reference_projection3213
                };
                if ((realm <= previous)
                    || (realm > (self::ml::Big::from_i128(18446744073709551615))))
                {
                    self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 114, 101, 97, 108, 109, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 117, 110, 105, 113, 117, 101, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115, 105, 103, 110, 101, 100, 54, 52, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index1441_2 =
                            source_ml_source1_temporary_index1441.add(&self::ml::Big::from_i128(1));
                        {
                            (previous, source_ml_source1_temporary_index1441) = (
                                realm.clone(),
                                source_ml_source1_temporary_index1441_2.clone(),
                            );
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Result::MlSource1TemporaryReadJavaScriptCallProgramErrorLoop1446Done(previous.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445 {
    MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445Next,
    MlSource1TemporaryReadJavaScriptCallProgramErrorJoin1445Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Result {
    MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Done,
    MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1454Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Result {
    MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Done,
    MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_call_function_body_error_loop1455(
    mut fn_: self::JavaScriptCallFunction,
    mut source_ml_source1_temporary_values1452: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_source1_temporary_index1452: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Result {
    loop {
        return if (source_ml_source1_temporary_index1452
            < ({
                let ml_reference_array3214 = source_ml_source1_temporary_values1452.clone();
                let ml_reference_projection3214 =
                    self::ml::Big::from_u128(ml_reference_array3214.borrow().len() as u128);
                ml_reference_projection3214
            })) {
            {
                let register = {
                    let ml_reference_array3215 = source_ml_source1_temporary_values1452.clone();
                    let ml_reference_position3215 = source_ml_source1_temporary_index1452.clone();
                    let ml_reference_projection3215 = self::ml_array::at(
                        &*ml_reference_array3215.borrow(),
                        ml_reference_position3215.to_index(),
                    );
                    ml_reference_projection3215
                };
                if !(self::test_java_script_call_coordinate(
                    register,
                    match fn_.clone() {
                        self::JavaScriptCallFunction::CallFunction(
                            _,
                            _,
                            _,
                            _,
                            _,
                            source_ml_source2_data_property_value42261,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value42261,
                    },
                )) {
                    self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Return(self::ml_string::Text::from_units(vec![99, 97, 108, 108, 32, 105, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 114, 101, 103, 105, 115, 116, 101, 114, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101, 115]))
                } else {
                    {
                        let source_ml_source1_temporary_index1452_2 =
                            source_ml_source1_temporary_index1452.add(&self::ml::Big::from_i128(1));
                        {
                            source_ml_source1_temporary_index1452 =
                                source_ml_source1_temporary_index1452_2.clone();
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Result::MlSource1TemporaryReadJavaScriptCallFunctionBodyErrorLoop1455Done
        };
    }
}
