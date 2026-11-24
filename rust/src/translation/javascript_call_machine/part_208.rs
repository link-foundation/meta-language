// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallPending;
use super::JavaScriptCallProgram;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryPushJavaScriptErrorConstructorLoop1519Result;
use super::MlSource1TemporaryPushJavaScriptOrdinaryClassConstructorLoop1523Result;
use super::create_java_script_call_frame;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_call_machine_error;
use super::read_java_script_executable_call_function;
use super::read_java_script_ordinary_class_receiver_error;
use super::read_java_script_ordinary_value_error;
use super::request_java_script_call_error;
use super::source_ml_source1_temporary_push_java_script_error_constructor_loop1519;
use super::source_ml_source1_temporary_push_java_script_ordinary_class_constructor_loop1523;
use super::test_java_script_error_constructor_mode;
use super::test_java_script_ordinary_class_constructor_mode;
use super::throw_java_script_call_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_call_source_error_allocation(
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
                            let source_ml_source2_array_read_value36831 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_array_read_index36832 = ({
                                    let ml_reference_array2414 = shared_constructor_field2.clone();
                                    let ml_reference_projection2414 =
                                        ml_reference_array2414.borrow().len() as f64;
                                    ml_reference_projection2414
                                } - 1f64);
                                {
                                    let ml_reference_array2415 =
                                        source_ml_source2_array_read_value36831.clone();
                                    let ml_reference_position2415 =
                                        source_ml_source2_array_read_index36832;
                                    let ml_reference_projection2415 = self::ml_array::optional_at(
                                        &*ml_reference_array2415.borrow(),
                                        self::ml_array::number_index(ml_reference_position2415),
                                    );
                                    ml_reference_projection2415
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
                                    115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32,
                                    114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101,
                                    113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99,
                                    116, 117, 97, 108, 32, 97, 99, 116, 105, 118, 101, 32, 102,
                                    114, 97, 109, 101,
                                ]),
                            )
                        } else {
                            if {
                                let source_ml_source2_s36835 =
                                    match frame.clone().expect("a proven present value") {
                                        self::JavaScriptCallFrame::CallFrame(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value36840,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => {
                                            let source_ml_source2_data_property_value36840 =
                                                *source_ml_source2_data_property_value36840;
                                            source_ml_source2_data_property_value36840.clone()
                                        }
                                    };
                                match source_ml_source2_s36835.clone() {
                                    self::JavaScriptCallPending::WaitSourceError(_, _, _, _) => {
                                        false
                                    }
                                    _ => true,
                                }
                            } {
                                self::JavaScriptCallMachineTransition::MachineInvalid(
                                    Box::new(shared_constructor_field1.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114,
                                        32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32,
                                        114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115,
                                        32, 97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105,
                                        110, 103, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111,
                                        110,
                                    ]),
                                )
                            } else {
                                if (((((match match frame.clone().expect("a proven present value") {
                                    self::JavaScriptCallFrame::CallFrame(_, _, _, _, _, source_ml_source2_data_property_value36844, _, _, _, _) => {
                                        let source_ml_source2_data_property_value36844 = *source_ml_source2_data_property_value36844;
                                        source_ml_source2_data_property_value36844.clone()
                                    }
                                } {
                                    self::JavaScriptCallPending::WaitSourceError(source_ml_source2_data_property_value36845, _, _, _) => {
                                        Some((source_ml_source2_data_property_value36845.clone(),))
                                    }
                                    _ => {
                                        None::<(self::ml::Big,)>
                                    }
                                }).clone().expect("a proven present value")).0).clone()) != realm) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 113, 117, 101, 115, 116, 101, 100, 32, 114, 101, 97, 108, 109]))
                                } else {
                                    {
                                        let value_error = self::read_java_script_ordinary_value_error(shared_constructor_field1.clone(), value.clone());
                                        if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), value_error.clone())
                                        } else {
                                            match value.clone() {
                                                self::JavaScriptSemanticValue::Object(_, callable) => {
                                                    if callable {
                                                        self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
                                                    } else {
                                                        self::throw_java_script_call_value(machine.clone(), value.clone())
                                                    }
                                                }
                                                _ => {
                                                    self::JavaScriptCallMachineTransition::MachineInvalid(Box::new(shared_constructor_field1.clone()), self::ml_string::Text::from_units(vec![115, 111, 117, 114, 99, 101, 32, 69, 114, 114, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 115, 117, 112, 112, 108, 105, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 99, 97, 108, 108, 97, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116]))
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
pub fn push_java_script_error_constructor(
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
                    let source_ml_source2_array_read_value36854 =
                        shared_constructor_field12.clone();
                    {
                        let source_ml_source2_array_read_index36855 = ({
                            let ml_reference_array2416 = shared_constructor_field12.clone();
                            let ml_reference_projection2416 =
                                ml_reference_array2416.borrow().len() as f64;
                            ml_reference_projection2416
                        } - 1f64);
                        {
                            let ml_reference_array2417 =
                                source_ml_source2_array_read_value36854.clone();
                            let ml_reference_position2417 = source_ml_source2_array_read_index36855;
                            let ml_reference_projection2417 = self::ml_array::optional_at(
                                &*ml_reference_array2417.borrow(),
                                self::ml_array::number_index(ml_reference_position2417),
                            );
                            ml_reference_projection2417
                        }
                    }
                };
                if match (parent.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::JavaScriptCallMachineTransition::MachineInvalid(
                        Box::new(shared_constructor_field11.clone()),
                        self::ml_string::Text::from_units(vec![
                            69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116,
                            105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97,
                            99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100,
                            32, 99, 97, 108, 108, 101, 114,
                        ]),
                    )
                } else {
                    {
                        let owner = self::read_java_script_executable_call_function(
                            program.clone(),
                            shared_constructor_field11.clone(),
                            match parent.clone().expect("a proven present value") {
                                self::JavaScriptCallFrame::CallFrame(
                                    source_ml_source2_data_property_value36858,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value36858.clone(),
                            },
                        );
                        if match (owner.clone(), None::<()>) {
                            (Some(_ml_left), Some(_ml_right)) => false,
                            (None, None) => true,
                            _ => false,
                        } {
                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                Box::new(shared_constructor_field11.clone()),
                                self::ml_string::Text::from_units(vec![
                                    69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116, 114, 117,
                                    99, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105,
                                    116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108,
                                    101, 114, 32, 114, 101, 97, 108, 109,
                                ]),
                            )
                        } else {
                            if (match callee.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => false,
                                _ => true,
                            } || !(((match callee.clone() {
                                self::JavaScriptSemanticValue::Object(
                                    _,
                                    source_ml_source2_data_property_value36862,
                                ) => Some((source_ml_source2_data_property_value36862,)),
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
                                            source_ml_source2_data_property_value36863,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value36863.clone(),
                                    },
                                    &(self::ml_string::Text::from_units(vec![
                                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                                    ])),
                                    &(self::ml_string::Text::from_units(vec![
                                        116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100,
                                        32, 69, 114, 114, 111, 114, 32, 99, 108, 97, 115, 115, 32,
                                        105, 115, 32, 110, 111, 116, 32, 97, 32, 99, 111, 110, 115,
                                        116, 114, 117, 99, 116, 111, 114,
                                    ])),
                                )
                            } else {
                                {
                                    let fn_ = self::read_java_script_executable_call_function(
                                        program.clone(),
                                        shared_constructor_field11.clone(),
                                        (((match callee.clone() {
                                            self::JavaScriptSemanticValue::Object(
                                                source_ml_source2_data_property_value36865,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value36865
                                                    .clone(),))
                                            }
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
                                    } || !(self::test_java_script_error_constructor_mode(
                                        &(match fn_.clone().expect("a proven present value") {
                                            self::JavaScriptCallFunction::CallFunction(
                                                _,
                                                _,
                                                source_ml_source2_data_property_value36866,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => source_ml_source2_data_property_value36866.clone(),
                                        }),
                                    ))) {
                                        self::JavaScriptCallMachineTransition::MachineInvalid(
                                            Box::new(shared_constructor_field11.clone()),
                                            self::ml_string::Text::from_units(vec![
                                                69, 114, 114, 111, 114, 32, 99, 111, 110, 115, 116,
                                                114, 117, 99, 116, 105, 111, 110, 32, 114, 101,
                                                113, 117, 105, 114, 101, 115, 32, 116, 104, 101,
                                                32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 103,
                                                105, 115, 116, 101, 114, 101, 100, 32, 100, 101,
                                                114, 105, 118, 101, 100, 32, 99, 111, 110, 115,
                                                116, 114, 117, 99, 116, 111, 114, 32, 100, 101,
                                                102, 105, 110, 105, 116, 105, 111, 110,
                                            ]),
                                        )
                                    } else {
                                        {
                                            let slots = match fn_
                                                .clone()
                                                .expect("a proven present value")
                                            {
                                                self::JavaScriptCallFunction::CallFunction(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    source_ml_source2_data_property_value36868,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => source_ml_source2_data_property_value36868,
                                            };
                                            {
                                                let argument_slots = {
                                                    let ml_reference_array2418 = arguments_.clone();
                                                    let ml_reference_projection2418 =
                                                        ml_reference_array2418.borrow().len()
                                                            as f64;
                                                    ml_reference_projection2418
                                                };
                                                {
                                                    let source_ml_source1_temporary_values1517 =
                                                        shared_constructor_field12.clone();
                                                    {
                                                        let source_ml_source1_temporary_index1517 =
                                                            self::ml::Big::from_i128(0);
                                                        {
                                                            let source_ml_source2_s36870 = self::source_ml_source1_temporary_push_java_script_error_constructor_loop1519(slots, argument_slots, source_ml_source1_temporary_values1517.clone(), source_ml_source1_temporary_index1517.clone());
                                                            match source_ml_source2_s36870.clone() {
                                                                self::MlSource1TemporaryPushJavaScriptErrorConstructorLoop1519Result::MlSource1TemporaryPushJavaScriptErrorConstructorLoop1519Done(slots_2, argument_slots_2) => {
                                                                    if (((({ let ml_reference_array2419 = shared_constructor_field12.clone(); let ml_reference_projection2419 = ml_reference_array2419.borrow().len() as f64; ml_reference_projection2419 }) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![100, 101, 114, 105, 118, 101, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114, 32, 102, 114, 97, 109, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field11.clone()), std::rc::Rc::new(std::cell::RefCell::new({
                                                                            let source_ml_source2_concat_value36897 = shared_constructor_field12.clone();
                                                                            {
                                                                                let source_ml_source2_concat_value36898 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                                    let source_ml_source2_nominal_spread36875 = self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), self::JavaScriptSemanticValue::Undefined, arguments_.clone());
                                                                                    match source_ml_source2_nominal_spread36875.clone() {
                                                                                        self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                                            let shared_constructor_field6 = *shared_constructor_field6;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field36876 = shared_constructor_field1.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field36878 = shared_constructor_field2;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field36880 = shared_constructor_field3.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field36882 = shared_constructor_field4.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field36884 = shared_constructor_field5.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field36886 = shared_constructor_field6.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field36888 = shared_constructor_field7.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field36890 = shared_constructor_field8.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field36892 = shared_constructor_field9.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field36894 = shared_constructor_field10.clone();
                                                                                                                                    self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field36876.clone(), source_ml_source2_nominal_field36878, source_ml_source2_nominal_field36880.clone(), source_ml_source2_nominal_field36882.clone(), source_ml_source2_nominal_field36884.clone(), Box::new(source_ml_source2_nominal_field36886.clone()), source_ml_source2_nominal_field36888.clone(), source_ml_source2_nominal_field36890.clone(), source_ml_source2_nominal_field36892.clone(), Some((Some(callee.clone()),)))
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
                                                                                self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), { let ml_reference_value2420 = source_ml_source2_concat_value36897.borrow().clone(); ml_reference_value2420 }), { let ml_reference_value2421 = source_ml_source2_concat_value36898.borrow().clone(); ml_reference_value2421 })
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
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn push_java_script_ordinary_class_constructor(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
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
                    let source_ml_source2_array_read_value36901 =
                        shared_constructor_field12.clone();
                    {
                        let source_ml_source2_array_read_index36902 = ({
                            let ml_reference_array2422 = shared_constructor_field12.clone();
                            let ml_reference_projection2422 =
                                ml_reference_array2422.borrow().len() as f64;
                            ml_reference_projection2422
                        } - 1f64);
                        {
                            let ml_reference_array2423 =
                                source_ml_source2_array_read_value36901.clone();
                            let ml_reference_position2423 = source_ml_source2_array_read_index36902;
                            let ml_reference_projection2423 = self::ml_array::optional_at(
                                &*ml_reference_array2423.borrow(),
                                self::ml_array::number_index(ml_reference_position2423),
                            );
                            ml_reference_projection2423
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
                                    source_ml_source2_data_property_value36904,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value36904.clone(),
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
                                111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115,
                                32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32,
                                110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117,
                                97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99,
                                97, 108, 108, 101, 114,
                            ]),
                        )
                    } else {
                        if (match callee.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        } || !(((match callee.clone() {
                            self::JavaScriptSemanticValue::Object(
                                _,
                                source_ml_source2_data_property_value36908,
                            ) => Some((source_ml_source2_data_property_value36908,)),
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
                                        source_ml_source2_data_property_value36909,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value36909.clone(),
                                },
                                &(self::ml_string::Text::from_units(vec![
                                    84, 121, 112, 101, 69, 114, 114, 111, 114,
                                ])),
                                &(self::ml_string::Text::from_units(vec![
                                    116, 104, 101, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32,
                                    99, 108, 97, 115, 115, 32, 105, 115, 32, 110, 111, 116, 32, 97,
                                    32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 111, 114,
                                ])),
                            )
                        } else {
                            {
                                let fn_ = self::read_java_script_executable_call_function(
                                    program.clone(),
                                    shared_constructor_field11.clone(),
                                    (((match callee.clone() {
                                        self::JavaScriptSemanticValue::Object(
                                            source_ml_source2_data_property_value36911,
                                            _,
                                        ) => Some((
                                            source_ml_source2_data_property_value36911.clone(),
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
                                } || !(self::test_java_script_ordinary_class_constructor_mode(
                                    &(match fn_.clone().expect("a proven present value") {
                                        self::JavaScriptCallFunction::CallFunction(
                                            _,
                                            _,
                                            source_ml_source2_data_property_value36912,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => source_ml_source2_data_property_value36912.clone(),
                                    }),
                                ))) {
                                    self::JavaScriptCallMachineTransition::MachineInvalid(
                                        Box::new(shared_constructor_field11.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97,
                                            115, 115, 32, 99, 111, 110, 115, 116, 114, 117, 99,
                                            116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114,
                                            101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                                            108, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101,
                                            100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116,
                                            111, 114, 32, 98, 111, 100, 121,
                                        ]),
                                    )
                                } else {
                                    {
                                        let invalid =
                                            self::read_java_script_ordinary_class_receiver_error(
                                                shared_constructor_field11.clone(),
                                                callee.clone(),
                                                receiver.clone(),
                                            );
                                        if (invalid != (self::ml_string::Text::from_units(vec![])))
                                        {
                                            self::JavaScriptCallMachineTransition::MachineInvalid(
                                                Box::new(shared_constructor_field11.clone()),
                                                invalid.clone(),
                                            )
                                        } else {
                                            {
                                                let slots = match fn_
                                                    .clone()
                                                    .expect("a proven present value")
                                                {
                                                    self::JavaScriptCallFunction::CallFunction(
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                        source_ml_source2_data_property_value36916,
                                                        _,
                                                        _,
                                                        _,
                                                        _,
                                                    ) => source_ml_source2_data_property_value36916,
                                                };
                                                {
                                                    let argument_slots = {
                                                        let ml_reference_array2424 =
                                                            arguments_.clone();
                                                        let ml_reference_projection2424 =
                                                            ml_reference_array2424.borrow().len()
                                                                as f64;
                                                        ml_reference_projection2424
                                                    };
                                                    {
                                                        let source_ml_source1_temporary_values1521 =
                                                            shared_constructor_field12.clone();
                                                        {
                                                            let source_ml_source1_temporary_index1521 =
                                                                self::ml::Big::from_i128(0);
                                                            {
                                                                let source_ml_source2_s36918 = self::source_ml_source1_temporary_push_java_script_ordinary_class_constructor_loop1523(slots, argument_slots, source_ml_source1_temporary_values1521.clone(), source_ml_source1_temporary_index1521.clone());
                                                                match source_ml_source2_s36918.clone() {
                                                                    self::MlSource1TemporaryPushJavaScriptOrdinaryClassConstructorLoop1523Result::MlSource1TemporaryPushJavaScriptOrdinaryClassConstructorLoop1523Done(slots_2, argument_slots_2) => {
                                                                        if (((({ let ml_reference_array2425 = shared_constructor_field12.clone(); let ml_reference_projection2425 = ml_reference_array2425.borrow().len() as f64; ml_reference_projection2425 }) >= (1024f64)) || (slots_2 > (1048576f64))) || (argument_slots_2 > (1048576f64))) {
                                                                            self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115, 32, 102, 114, 97, 109, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 101, 120, 101, 99, 117, 116, 105, 111, 110, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                                        } else {
                                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field11.clone()), std::rc::Rc::new(std::cell::RefCell::new({
                                                                                let source_ml_source2_concat_value36945 = shared_constructor_field12.clone();
                                                                                {
                                                                                    let source_ml_source2_concat_value36946 = std::rc::Rc::new(std::cell::RefCell::new(vec![{
                                                                                        let source_ml_source2_nominal_spread36923 = self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_.clone());
                                                                                        match source_ml_source2_nominal_spread36923.clone() {
                                                                                            self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                                                let shared_constructor_field6 = *shared_constructor_field6;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field36924 = shared_constructor_field1.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field36926 = shared_constructor_field2;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field36928 = shared_constructor_field3.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field36930 = shared_constructor_field4.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field36932 = shared_constructor_field5.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field36934 = shared_constructor_field6.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field36936 = shared_constructor_field7.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field36938 = shared_constructor_field8.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field36940 = shared_constructor_field9.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field36942 = shared_constructor_field10.clone();
                                                                                                                                        self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field36924.clone(), source_ml_source2_nominal_field36926, source_ml_source2_nominal_field36928.clone(), source_ml_source2_nominal_field36930.clone(), source_ml_source2_nominal_field36932.clone(), Box::new(source_ml_source2_nominal_field36934.clone()), source_ml_source2_nominal_field36936.clone(), source_ml_source2_nominal_field36938.clone(), source_ml_source2_nominal_field36940.clone(), Some((Some(callee.clone()),)))
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
                                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), { let ml_reference_value2426 = source_ml_source2_concat_value36945.borrow().clone(); ml_reference_value2426 }), { let ml_reference_value2427 = source_ml_source2_concat_value36946.borrow().clone(); ml_reference_value2427 })
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
    }
}
