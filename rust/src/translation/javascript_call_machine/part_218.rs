// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallFrame;
use super::JavaScriptCallFunction;
use super::JavaScriptCallMachine;
use super::JavaScriptCallMachineTransition;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::MlSource1TemporaryPushJavaScriptConstructionBodyLoop1537Result;
use super::create_java_script_call_frame;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_string;
use super::ml_utf16_text;
use super::read_java_script_collection_constructor_kind;
use super::read_java_script_executable_call_function;
use super::read_java_script_semantic_text_unit;
use super::source_ml_source1_temporary_push_java_script_construction_body_loop1537;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn push_java_script_construction_body(
    program: self::JavaScriptCallProgram,
    machine: self::JavaScriptCallMachine,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    new_target: self::JavaScriptSemanticValue,
) -> self::JavaScriptCallMachineTransition {
    match machine.clone() {
        self::JavaScriptCallMachine::CallMachine(
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field11 = *shared_constructor_field11;
            if (((((match callee.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => false,
                _ => true,
            } || !(((match callee.clone() {
                self::JavaScriptSemanticValue::Object(
                    _,
                    source_ml_source2_data_property_value38922,
                ) => Some((source_ml_source2_data_property_value38922,)),
                _ => None::<(bool,)>,
            })
            .clone()
            .expect("a proven present value"))
            .0)) || match receiver.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => false,
                _ => true,
            }) || ((match receiver.clone() {
                self::JavaScriptSemanticValue::Object(
                    _,
                    source_ml_source2_data_property_value38925,
                ) => Some((source_ml_source2_data_property_value38925,)),
                _ => None::<(bool,)>,
            })
            .clone()
            .expect("a proven present value"))
            .0) || match new_target.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => false,
                _ => true,
            }) || !(((match new_target.clone() {
                self::JavaScriptSemanticValue::Object(
                    _,
                    source_ml_source2_data_property_value38928,
                ) => Some((source_ml_source2_data_property_value38928,)),
                _ => None::<(bool,)>,
            })
            .clone()
            .expect("a proven present value"))
            .0))
            {
                self::JavaScriptCallMachineTransition::MachineInvalid(
                    Box::new(shared_constructor_field11.clone()),
                    self::ml_string::Text::from_units(vec![
                        99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 111, 100,
                        105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97,
                        108, 32, 116, 97, 114, 103, 101, 116, 32, 114, 101, 99, 101, 105, 118, 101,
                        114, 32, 97, 110, 100, 32, 110, 101, 119, 84, 97, 114, 103, 101, 116, 32,
                        105, 100, 101, 110, 116, 105, 116, 105, 101, 115,
                    ]),
                )
            } else {
                {
                    let fn_ = self::read_java_script_executable_call_function(
                        program.clone(),
                        shared_constructor_field11.clone(),
                        (((match callee.clone() {
                            self::JavaScriptSemanticValue::Object(
                                source_ml_source2_data_property_value38931,
                                _,
                            ) => Some((source_ml_source2_data_property_value38931.clone(),)),
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
                    } || ((((match fn_.clone().expect("a proven present value") {
                        self::JavaScriptCallFunction::CallFunction(
                            _,
                            _,
                            source_ml_source2_data_property_value38933,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value38933.clone(),
                    }) != (self::ml_string::Text::from_units(vec![
                        115, 116, 114, 105, 99, 116,
                    ]))) && ((match fn_.clone().expect("a proven present value") {
                        self::JavaScriptCallFunction::CallFunction(
                            _,
                            _,
                            source_ml_source2_data_property_value38935,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                            _,
                        ) => source_ml_source2_data_property_value38935.clone(),
                    }) != (self::ml_string::Text::from_units(vec![
                        111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 108, 97, 115, 115,
                    ]))))
                        && !(self::ml_array::includes(
                            &{
                                let ml_reference_value2512 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![77, 97, 112]),
                                        self::ml_string::Text::from_units(vec![83, 101, 116]),
                                        self::ml_string::Text::from_units(vec![
                                            87, 101, 97, 107, 77, 97, 112,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            87, 101, 97, 107, 83, 101, 116,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value2512
                            },
                            &self::read_java_script_collection_constructor_kind(
                                fn_.clone().expect("a proven present value"),
                            ),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))))
                    {
                        self::JavaScriptCallMachineTransition::MachineInvalid(
                            Box::new(shared_constructor_field11.clone()),
                            self::ml_string::Text::from_units(vec![
                                99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98,
                                111, 100, 105, 101, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32,
                                97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101,
                                114, 101, 100, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 102,
                                117, 110, 99, 116, 105, 111, 110, 44, 32, 98, 97, 115, 101, 32, 99,
                                108, 97, 115, 115, 32, 111, 114, 32, 99, 111, 108, 108, 101, 99,
                                116, 105, 111, 110, 32, 99, 111, 100, 101,
                            ]),
                        )
                    } else {
                        {
                            let slots = match fn_.clone().expect("a proven present value") {
                                self::JavaScriptCallFunction::CallFunction(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value38937,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value38937,
                            };
                            {
                                let count = {
                                    let ml_reference_array2513 = arguments_.clone();
                                    let ml_reference_projection2513 =
                                        ml_reference_array2513.borrow().len() as f64;
                                    ml_reference_projection2513
                                };
                                {
                                    let source_ml_source1_temporary_values1535 =
                                        shared_constructor_field12.clone();
                                    {
                                        let source_ml_source1_temporary_index1535 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s38939 = self::source_ml_source1_temporary_push_java_script_construction_body_loop1537(slots, count, source_ml_source1_temporary_values1535.clone(), source_ml_source1_temporary_index1535.clone());
                                            match source_ml_source2_s38939.clone() {
                                                self::MlSource1TemporaryPushJavaScriptConstructionBodyLoop1537Result::MlSource1TemporaryPushJavaScriptConstructionBodyLoop1537Done(slots_2, count_2) => {
                                                    if (((({ let ml_reference_array2514 = shared_constructor_field12.clone(); let ml_reference_projection2514 = ml_reference_array2514.borrow().len() as f64; ml_reference_projection2514 }) >= (1024f64)) || (slots_2 > (1048576f64))) || (count_2 > (1048576f64))) {
                                                        self::JavaScriptCallMachineTransition::MachineResourceLimit(Box::new(machine.clone()), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 98, 111, 100, 121, 32, 101, 120, 99, 101, 101, 100, 115, 32, 115, 104, 97, 114, 101, 100, 32, 115, 116, 97, 99, 107, 32, 97, 110, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 111, 117, 110, 100, 115]))
                                                    } else {
                                                        {
                                                            let child = {
                                                                let source_ml_source2_nominal_spread38942 = self::create_java_script_call_frame(fn_.clone().expect("a proven present value"), receiver.clone(), arguments_.clone());
                                                                match source_ml_source2_nominal_spread38942.clone() {
                                                                    self::JavaScriptCallFrame::CallFrame(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10) => {
                                                                        let shared_constructor_field6 = *shared_constructor_field6;
                                                                        {
                                                                            let source_ml_source2_nominal_field38943 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field38945 = shared_constructor_field2;
                                                                                {
                                                                                    let source_ml_source2_nominal_field38947 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field38949 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field38951 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field38953 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field38955 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field38957 = shared_constructor_field8.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field38959 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field38961 = shared_constructor_field10.clone();
                                                                                                                self::JavaScriptCallFrame::CallFrame(source_ml_source2_nominal_field38943.clone(), source_ml_source2_nominal_field38945, source_ml_source2_nominal_field38947.clone(), source_ml_source2_nominal_field38949.clone(), source_ml_source2_nominal_field38951.clone(), Box::new(source_ml_source2_nominal_field38953.clone()), source_ml_source2_nominal_field38955.clone(), source_ml_source2_nominal_field38957.clone(), source_ml_source2_nominal_field38959.clone(), Some((Some(new_target.clone()),)))
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
                                                            };
                                                            self::JavaScriptCallMachineTransition::MachineContinue(Box::new(self::JavaScriptCallMachine::CallMachine(Box::new(shared_constructor_field11.clone()), std::rc::Rc::new(std::cell::RefCell::new({
                                                                let source_ml_source2_concat_value38966 = shared_constructor_field12.clone();
                                                                {
                                                                    let source_ml_source2_concat_value38967 = std::rc::Rc::new(std::cell::RefCell::new(vec![child.clone()]));
                                                                    self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptCallFrame>::new(), { let ml_reference_value2515 = source_ml_source2_concat_value38966.borrow().clone(); ml_reference_value2515 }), { let ml_reference_value2516 = source_ml_source2_concat_value38967.borrow().clone(); ml_reference_value2516 })
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_weak_key_eligibility_loop2(
    mut value: self::JavaScriptSemanticValue,
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    mut low: f64,
    mut high: f64,
) -> f64 {
    loop {
        return if (low < high) {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let row = {
                        let ml_reference_array2517 = facts.clone();
                        let ml_reference_position2517 = middle;
                        let ml_reference_projection2517 = self::ml_array::optional_at(
                            &*ml_reference_array2517.borrow(),
                            self::ml_array::number_index(ml_reference_position2517),
                        );
                        ml_reference_projection2517
                    };
                    if (!(match (row.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && {
                        let source_ml_source2_comparison_operand38973 =
                            match row.clone().expect("a proven present value") {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source2_data_property_value38970,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value38970.clone(),
                            };
                        {
                            let source_ml_source2_comparison_operand38974 = match value.clone() {
                                self::JavaScriptSemanticValue::Symbol(
                                    source_ml_source2_data_property_value38971,
                                ) => Some(source_ml_source2_data_property_value38971.clone()),
                                self::JavaScriptSemanticValue::Object(
                                    source_ml_source2_data_property_value38972,
                                    _,
                                ) => Some(source_ml_source2_data_property_value38972.clone()),
                                _ => None::<self::ml::Big>,
                            };
                            if match (
                                source_ml_source2_comparison_operand38974.clone(),
                                None::<self::ml::Big>,
                            ) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                {
                                    let source_ml_source2_comparison_order38975 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source2_comparison_operand38973.clone());
                                    (source_ml_source2_comparison_order38975 == (1f64))
                                }
                            } else {
                                (source_ml_source2_comparison_operand38973
                                    < (source_ml_source2_comparison_operand38974
                                        .clone()
                                        .expect("a proven present value")))
                            }
                        }
                    }) {
                        {
                            let low_2 = (middle + 1f64);
                            {
                                low = low_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            high = middle;
                            continue;
                        }
                    }
                }
            }
        } else {
            low
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop3(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Result {
    loop {
        return if (at
            < ({
                let ml_reference_array2518 = units.clone();
                let ml_reference_projection2518 = ml_reference_array2518.borrow().len() as f64;
                ml_reference_projection2518
            })) {
            {
                let unit = ({
                    let ml_reference_array2519 = units.clone();
                    let ml_reference_position2519 = at;
                    let ml_reference_projection2519 = self::ml_array::optional_at(
                        &*ml_reference_array2519.borrow(),
                        self::ml_array::number_index(ml_reference_position2519),
                    );
                    ml_reference_projection2519
                })
                .clone()
                .unwrap_or_else(|| (-1f64));
                if ((!(self::ml_math::is_integer(unit)) || (unit < (0f64))) || (unit > (65535f64)))
                {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Result {
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Done,
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_semantic_text_equal_loop4(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut at: f64,
) -> self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Result {
    loop {
        return if (at < length) {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Text(units) => ({
                        let ml_reference_array2520 = units.clone();
                        let ml_reference_position2520 = at;
                        let ml_reference_projection2520 = self::ml_array::optional_at(
                            &*ml_reference_array2520.borrow(),
                            self::ml_array::number_index(ml_reference_position2520),
                        );
                        ml_reference_projection2520
                    })
                    .clone()
                    .unwrap_or_else(|| (-1f64)),
                    _ => match left.clone() {
                        self::JavaScriptSemanticValue::SourceText(_, text) => {
                            let source_ml_source2_string_unit_operand38981 = text.clone();
                            {
                                let source_ml_source2_string_unit_operand38982 = at;
                                (std::rc::Rc::new(
                                    move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                        {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if ((index < (0f64)) || (index >= (text_2.length()))) {
                                                f64::NAN
                                            } else {
                                                ({
                                                    let source_ml_source2_string_unit_operand38988 = {
                                                        let source_ml_source2_string_range_operand38983 = text_2.clone();
                                                        {
                                                            let source_ml_source2_string_range_operand38984 = index;
                                                            {
                                                                let source_ml_source2_string_range_operand38985 = (index + 1f64);
                                                                {
                                                                    let source_ml_source2_string_range_position38986 = source_ml_source2_string_range_operand38984;
                                                                    {
                                                                        let source_ml_source2_string_range_position38987 = source_ml_source2_string_range_operand38985;
                                                                        source_ml_source2_string_range_operand38983.slice(source_ml_source2_string_range_position38986, source_ml_source2_string_range_position38987)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_string_unit_operand38989 = 0f64;
                                                        source_ml_source2_string_unit_operand38988.code_point_at(source_ml_source2_string_unit_operand38989)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source2_string_unit_operand38981.clone(),
                                    source_ml_source2_string_unit_operand38982,
                                )
                            }
                        }
                        _ => (-1f64),
                    },
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Text(units_2) => ({
                            let ml_reference_array2521 = units_2.clone();
                            let ml_reference_position2521 = at;
                            let ml_reference_projection2521 = self::ml_array::optional_at(
                                &*ml_reference_array2521.borrow(),
                                self::ml_array::number_index(ml_reference_position2521),
                            );
                            ml_reference_projection2521
                        })
                        .clone()
                        .unwrap_or_else(|| (-1f64)),
                        _ => {
                            match right.clone() {
                                self::JavaScriptSemanticValue::SourceText(_, text_2) => {
                                    let source_ml_source2_string_unit_operand38993 = text_2.clone();
                                    {
                                        let source_ml_source2_string_unit_operand38994 = at;
                                        (std::rc::Rc::new(move |text_3: self::ml_string::Text, position: f64| -> f64 { {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if ((index < (0f64)) || (index >= (text_3.length()))) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                        let source_ml_source2_string_unit_operand39000 = {
                                                            let source_ml_source2_string_range_operand38995 = text_3.clone();
                                                            {
                                                                let source_ml_source2_string_range_operand38996 = index;
                                                                {
                                                                    let source_ml_source2_string_range_operand38997 = (index + 1f64);
                                                                    {
                                                                        let source_ml_source2_string_range_position38998 = source_ml_source2_string_range_operand38996;
                                                                        {
                                                                            let source_ml_source2_string_range_position38999 = source_ml_source2_string_range_operand38997;
                                                                            source_ml_source2_string_range_operand38995.slice(source_ml_source2_string_range_position38998, source_ml_source2_string_range_position38999)
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        };
                                                        {
                                                            let source_ml_source2_string_unit_operand39001 = 0f64;
                                                            source_ml_source2_string_unit_operand39000.code_point_at(source_ml_source2_string_unit_operand39001)
                                                        }
                                                    }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            } }))(source_ml_source2_string_unit_operand38993.clone(), source_ml_source2_string_unit_operand38994)
                                    }
                                }
                                _ => (-1f64),
                            }
                        }
                    };
                    if (a != b) {
                        self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Return(false)
                    } else {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Done
        };
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_text_string_loop6(
    mut text: self::ml_string::Text,
    mut source_ml_source1_temporary_values5: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_source1_temporary_index5: self::ml::Big,
) -> self::ml_string::Text {
    loop {
        return if (source_ml_source1_temporary_index5
            < ({
                let ml_reference_array2522 = source_ml_source1_temporary_values5.clone();
                let ml_reference_projection2522 =
                    self::ml::Big::from_u128(ml_reference_array2522.borrow().len() as u128);
                ml_reference_projection2522
            })) {
            {
                let unit = {
                    let ml_reference_array2523 = source_ml_source1_temporary_values5.clone();
                    let ml_reference_position2523 = source_ml_source1_temporary_index5.clone();
                    let ml_reference_projection2523 = self::ml_array::at(
                        &*ml_reference_array2523.borrow(),
                        ml_reference_position2523.to_index(),
                    );
                    ml_reference_projection2523
                };
                {
                    let text_2 = text.concat(&self::ml_string::Text::from_units(
                        self::ml_utf16_text::character_units_utf16_text(&vec![unit])
                            .into_iter()
                            .map(|ml_unit| ml_unit as u16)
                            .collect(),
                    ));
                    {
                        let source_ml_source1_temporary_index5_2 =
                            source_ml_source1_temporary_index5.add(&self::ml::Big::from_i128(1));
                        {
                            (text, source_ml_source1_temporary_index5) =
                                (text_2.clone(), source_ml_source1_temporary_index5_2.clone());
                            continue;
                        }
                    }
                }
            }
        } else {
            text.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Result {
    MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_semantic_text_units_loop7(
    mut value: self::JavaScriptSemanticValue,
    mut length: f64,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Result {
    loop {
        return if (at < length) {
            {
                let source_ml_source1_temporary_construct_java_script_semantic_text_units_ignored8 = {
                    let ml_method_owner2524 = units.clone();
                    let ml_method_argument2524_0 =
                        self::read_java_script_semantic_text_unit(value.clone(), at);
                    let ml_inserted = vec![ml_method_argument2524_0];
                    let mut ml_method_values2524 = ml_method_owner2524.borrow_mut();
                    let ml_length = ml_method_values2524
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2524.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result {
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done,
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop9(
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut key: self::JavaScriptSemanticValue,
    mut at: f64,
) -> self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result {
    loop {
        return if (at
            < ({
                let ml_reference_array2525 = properties.clone();
                let ml_reference_projection2525 = ml_reference_array2525.borrow().len() as f64;
                ml_reference_projection2525
            })) {
            {
                let property = {
                    let ml_reference_array2526 = properties.clone();
                    let ml_reference_position2526 = at;
                    let ml_reference_projection2526 = self::ml_array::optional_at(
                        &*ml_reference_array2526.borrow(),
                        self::ml_array::number_index(ml_reference_position2526),
                    );
                    ml_reference_projection2526
                };
                if (!(match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source2_data_property_value39007,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value39007 =
                                *source_ml_source2_data_property_value39007;
                            source_ml_source2_data_property_value39007.clone()
                        }
                    },
                    key.clone(),
                )) {
                    self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(at)
                } else {
                    {
                        let at_2 = (at + 1f64);
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10 {
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Next,
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Return(
        std::rc::Rc<
            std::cell::RefCell<(
                (
                    std::rc::Rc<
                        std::cell::RefCell<(
                            (
                                bool,
                                bool,
                                bool,
                                bool,
                                self::JavaScriptSemanticValue,
                                bool,
                                self::JavaScriptSemanticValue,
                                bool,
                                self::JavaScriptSemanticValue,
                                bool,
                                bool,
                                bool,
                            ),
                            Vec<f64>,
                        )>,
                    >,
                    self::ml_string::Text,
                    self::ml_string::Text,
                    std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
                ),
                Vec<f64>,
            )>,
        >,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Result {
    MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(f64),
    MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_literal_property_index_loop11(
    mut key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut value: f64,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Result {
    loop {
        return if (at
            < ({
                let ml_reference_array2527 = key.clone();
                let ml_reference_projection2527 = ml_reference_array2527.borrow().len() as f64;
                ml_reference_projection2527
            })) {
            {
                let digit = (match {
                    let ml_reference_array2528 = key.clone();
                    let ml_reference_position2528 = at;
                    let ml_reference_projection2528 = self::ml_array::optional_at(
                        &*ml_reference_array2528.borrow(),
                        self::ml_array::number_index(ml_reference_position2528),
                    );
                    ml_reference_projection2528
                } {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                } - 48f64);
                if ((!(self::ml_math::is_integer(digit)) || (digit < (0f64))) || (digit > (9f64))) {
                    self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Return((-1f64))
                } else {
                    {
                        let value_2 = ((value * 10f64) + digit);
                        {
                            let at_2 = (at + 1f64);
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result {
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Done,
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(f64),
}
