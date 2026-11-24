// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCallbackCall;
use super::JavaScriptArrayCallbackState;
use super::JavaScriptArrayCallbackTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::ml;
use super::ml_string;
use super::read_java_script_array_callback_state_error;
use super::read_java_script_indexed_property_presence;
use super::request_java_script_array_callback_call;
use super::test_java_script_array_callback_reduction;
use super::test_java_script_array_callback_reverse;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_array_callback(
    heap: self::JavaScriptOrdinaryHeap,
    operation: &self::ml_string::Text,
    receiver: self::JavaScriptSemanticValue,
    callback: self::JavaScriptSemanticValue,
    this_argument: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
    initial_present: bool,
) -> self::JavaScriptArrayCallbackTransition {
    if (match receiver.clone() {
        self::JavaScriptSemanticValue::Null => true,
        _ => false,
    } || match receiver.clone() {
        self::JavaScriptSemanticValue::Undefined => true,
        _ => false,
    }) {
        self::JavaScriptArrayCallbackTransition::ArrayCallbackError(
            Box::new(heap.clone()),
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 97,
                108, 108, 98, 97, 99, 107, 32, 99, 97, 108, 108, 101, 100, 32, 111, 110, 32, 110,
                117, 108, 108, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100,
            ]),
        )
    } else {
        {
            let state = {
                let source_ml_source2_constructor_field28509 = 0f64;
                {
                    let source_ml_source2_constructor_field28510 = 0f64;
                    {
                        let source_ml_source2_constructor_field28513 =
                            (self::test_java_script_array_callback_reduction(operation)
                                && initial_present);
                        {
                            let source_ml_source2_constructor_field28514 =
                                if (self::test_java_script_array_callback_reduction(operation)
                                    && initial_present)
                                {
                                    this_argument.clone()
                                } else {
                                    self::JavaScriptSemanticValue::Undefined
                                };
                            {
                                let source_ml_source2_constructor_field28515 =
                                    self::JavaScriptSemanticValue::Undefined;
                                self::JavaScriptArrayCallbackState::ArrayCallbackState(
                                    operation.clone(),
                                    Box::new(receiver.clone()),
                                    Box::new(callback.clone()),
                                    Box::new(this_argument.clone()),
                                    Box::new(exotic_key.clone()),
                                    source_ml_source2_constructor_field28509,
                                    source_ml_source2_constructor_field28510,
                                    self::ml_string::Text::from_units(vec![
                                        108, 101, 110, 103, 116, 104,
                                    ]),
                                    initial_present,
                                    source_ml_source2_constructor_field28513,
                                    Box::new(source_ml_source2_constructor_field28514.clone()),
                                    Box::new(source_ml_source2_constructor_field28515.clone()),
                                    None::<(Option<self::JavaScriptPrimitiveConversionState>,)>,
                                    None::<(Option<self::JavaScriptArrayCallbackCall>,)>,
                                )
                            }
                        }
                    }
                }
            };
            {
                let error =
                    self::read_java_script_array_callback_state_error(heap.clone(), state.clone());
                if (error == (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayCallbackTransition::ArrayCallbackContinue(
                        Box::new(heap.clone()),
                        Box::new(state.clone()),
                    )
                } else {
                    self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_callback_property_presence(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
) -> std::rc::Rc<std::cell::RefCell<((self::ml_string::Text, bool), Vec<f64>)>> {
    self::read_java_script_indexed_property_presence(heap.clone(), identity.clone(), key.clone())
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_callback_empty_result(
    state: self::JavaScriptArrayCallbackState,
) -> self::JavaScriptSemanticValue {
    match state.clone() {
        self::JavaScriptArrayCallbackState::ArrayCallbackState(
            shared_constructor_field1,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field2,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            if self::test_java_script_array_callback_reduction(&(shared_constructor_field1)) {
                shared_constructor_field2.clone()
            } else {
                if (shared_constructor_field1
                    == (self::ml_string::Text::from_units(vec![
                        65, 114, 114, 97, 121, 46, 115, 111, 109, 101,
                    ])))
                {
                    self::JavaScriptSemanticValue::Boolean(false)
                } else {
                    if (shared_constructor_field1
                        == (self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 46, 101, 118, 101, 114, 121,
                        ])))
                    {
                        self::JavaScriptSemanticValue::Boolean(true)
                    } else {
                        if ((shared_constructor_field1
                            == (self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 73, 110, 100, 101,
                                120,
                            ])))
                            || (shared_constructor_field1
                                == (self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 76, 97, 115,
                                    116, 73, 110, 100, 101, 120,
                                ]))))
                        {
                            self::JavaScriptSemanticValue::Number((-1f64))
                        } else {
                            self::JavaScriptSemanticValue::Undefined
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_array_callback_element(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCallbackState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayCallbackTransition {
    match state.clone() {
        self::JavaScriptArrayCallbackState::ArrayCallbackState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
            shared_constructor_field14,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            if self::test_java_script_array_callback_reduction(&(shared_constructor_field1)) {
                if !shared_constructor_field10 {
                    self::JavaScriptArrayCallbackTransition::ArrayCallbackContinue(
                        Box::new(heap.clone()),
                        Box::new({
                            let source_ml_source2_nominal_field28531 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_nominal_field28533 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field28535 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field28537 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field28539 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field28541 =
                                                    shared_constructor_field6;
                                                {
                                                    let source_ml_source2_nominal_field28543 =
                                                        shared_constructor_field7;
                                                    {
                                                        let source_ml_source2_nominal_field28545 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field28547 =
                                                                shared_constructor_field9;
                                                            {
                                                                let source_ml_source2_nominal_field28549 =
                                                                    shared_constructor_field10;
                                                                {
                                                                    let source_ml_source2_nominal_field28551 =
                                                                        shared_constructor_field11
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28553 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field28555 = shared_constructor_field13.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field28557 = shared_constructor_field14.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_override28562 = (shared_constructor_field7 + if self::test_java_script_array_callback_reverse(&(shared_constructor_field1)) {
                                                                                    (-1f64)
                                                                                } else {
                                                                                    1f64
                                                                                });
                                                                                    {
                                                                                        let source_ml_source2_nominal_override28563 = self::JavaScriptSemanticValue::Undefined;
                                                                                        self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28531.clone(), Box::new(source_ml_source2_nominal_field28533.clone()), Box::new(source_ml_source2_nominal_field28535.clone()), Box::new(source_ml_source2_nominal_field28537.clone()), Box::new(source_ml_source2_nominal_field28539.clone()), source_ml_source2_nominal_field28541, source_ml_source2_nominal_override28562, self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), source_ml_source2_nominal_field28547, true, Box::new(value.clone()), Box::new(source_ml_source2_nominal_override28563.clone()), source_ml_source2_nominal_field28555.clone(), source_ml_source2_nominal_field28557.clone())
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
                        }),
                    )
                } else {
                    self::request_java_script_array_callback_call(
                        heap.clone(),
                        {
                            let source_ml_source2_nominal_field28565 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_nominal_field28567 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field28569 =
                                        shared_constructor_field3.clone();
                                    {
                                        let source_ml_source2_nominal_field28571 =
                                            shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field28573 =
                                                shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field28575 =
                                                    shared_constructor_field6;
                                                {
                                                    let source_ml_source2_nominal_field28577 =
                                                        shared_constructor_field7;
                                                    {
                                                        let source_ml_source2_nominal_field28579 =
                                                            shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field28581 =
                                                                shared_constructor_field9;
                                                            {
                                                                let source_ml_source2_nominal_field28583 =
                                                                    shared_constructor_field10;
                                                                {
                                                                    let source_ml_source2_nominal_field28585 =
                                                                        shared_constructor_field11
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28587 = shared_constructor_field12.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field28589 = shared_constructor_field13.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field28591 = shared_constructor_field14.clone();
                                                                                self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28565.clone(), Box::new(source_ml_source2_nominal_field28567.clone()), Box::new(source_ml_source2_nominal_field28569.clone()), Box::new(source_ml_source2_nominal_field28571.clone()), Box::new(source_ml_source2_nominal_field28573.clone()), source_ml_source2_nominal_field28575, source_ml_source2_nominal_field28577, self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107]), source_ml_source2_nominal_field28581, source_ml_source2_nominal_field28583, Box::new(source_ml_source2_nominal_field28585.clone()), Box::new(value.clone()), source_ml_source2_nominal_field28589.clone(), source_ml_source2_nominal_field28591.clone())
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
                        shared_constructor_field3.clone(),
                        self::JavaScriptSemanticValue::Undefined,
                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                            shared_constructor_field11.clone(),
                            value.clone(),
                            self::JavaScriptSemanticValue::Number(shared_constructor_field7),
                            shared_constructor_field2.clone(),
                        ])),
                    )
                }
            } else {
                self::request_java_script_array_callback_call(
                    heap.clone(),
                    {
                        let source_ml_source2_nominal_field28600 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_nominal_field28602 =
                                shared_constructor_field2.clone();
                            {
                                let source_ml_source2_nominal_field28604 =
                                    shared_constructor_field3.clone();
                                {
                                    let source_ml_source2_nominal_field28606 =
                                        shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_nominal_field28608 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_nominal_field28610 =
                                                shared_constructor_field6;
                                            {
                                                let source_ml_source2_nominal_field28612 =
                                                    shared_constructor_field7;
                                                {
                                                    let source_ml_source2_nominal_field28614 =
                                                        shared_constructor_field8.clone();
                                                    {
                                                        let source_ml_source2_nominal_field28616 =
                                                            shared_constructor_field9;
                                                        {
                                                            let source_ml_source2_nominal_field28618 =
                                                                shared_constructor_field10;
                                                            {
                                                                let source_ml_source2_nominal_field28620 =
                                                                    shared_constructor_field11
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field28622 =
                                                                        shared_constructor_field12
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28624 = shared_constructor_field13.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field28626 = shared_constructor_field14.clone();
                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28600.clone(), Box::new(source_ml_source2_nominal_field28602.clone()), Box::new(source_ml_source2_nominal_field28604.clone()), Box::new(source_ml_source2_nominal_field28606.clone()), Box::new(source_ml_source2_nominal_field28608.clone()), source_ml_source2_nominal_field28610, source_ml_source2_nominal_field28612, self::ml_string::Text::from_units(vec![99, 97, 108, 108, 98, 97, 99, 107]), source_ml_source2_nominal_field28616, source_ml_source2_nominal_field28618, Box::new(source_ml_source2_nominal_field28620.clone()), Box::new(value.clone()), source_ml_source2_nominal_field28624.clone(), source_ml_source2_nominal_field28626.clone())
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
                    shared_constructor_field3.clone(),
                    shared_constructor_field4.clone(),
                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                        value.clone(),
                        self::JavaScriptSemanticValue::Number(shared_constructor_field7),
                        shared_constructor_field2.clone(),
                    ])),
                )
            }
        }
    }
}
