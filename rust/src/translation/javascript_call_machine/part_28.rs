// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayConstructorFact;
use super::JavaScriptCallPrimitiveResult;
use super::JavaScriptCallProgram;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Result;
use super::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Result;
use super::continue_java_script_primitive_conversion;
use super::execute_java_script_call_scalar_conversion;
use super::ml;
use super::ml_math;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_call_scalar_conversion_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_spelling;
use super::read_java_script_primitive_conversion_state_error;
use super::read_java_script_semantic_value_error;
use super::source_ml_source1_temporary_find_java_script_call_code_function_loop198;
use super::source_ml_source1_temporary_find_java_script_call_constructor_fact_loop197;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_primitive_conversion_method(
    state: self::JavaScriptPrimitiveConversionState,
    method: self::JavaScriptSemanticValue,
) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let exotic = ((shared_constructor_field4
                    == (self::ml_string::Text::from_units(vec![
                        101, 120, 111, 116, 105, 99, 71, 101, 116,
                    ])))
                    || (shared_constructor_field4
                        == (self::ml_string::Text::from_units(vec![
                            101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114,
                        ]))));
                {
                    let first = ((shared_constructor_field4
                        == (self::ml_string::Text::from_units(vec![
                            102, 105, 114, 115, 116, 71, 101, 116,
                        ])))
                        || (shared_constructor_field4
                            == (self::ml_string::Text::from_units(vec![
                                102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114,
                            ]))));
                    if (exotic
                        && (match method.clone() {
                            self::JavaScriptSemanticValue::Undefined => true,
                            _ => false,
                        } || match method.clone() {
                            self::JavaScriptSemanticValue::Null => true,
                            _ => false,
                        }))
                    {
                        self::continue_java_script_primitive_conversion(
                            state.clone(),
                            &(self::ml_string::Text::from_units(vec![
                                102, 105, 114, 115, 116, 71, 101, 116,
                            ])),
                        )
                    } else {
                        match method.clone() {
                            self::JavaScriptSemanticValue::Object(_, callable) => {
                                if !callable {
                                    if exotic {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                    } else {
                                        if first {
                                            self::continue_java_script_primitive_conversion(
                                                state.clone(),
                                                &(self::ml_string::Text::from_units(vec![
                                                    115, 101, 99, 111, 110, 100, 71, 101, 116,
                                                ])),
                                            )
                                        } else {
                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
                                        }
                                    }
                                } else {
                                    {
                                        let phase = if exotic {
                                            self::ml_string::Text::from_units(vec![
                                                101, 120, 111, 116, 105, 99, 67, 97, 108, 108,
                                            ])
                                        } else {
                                            if first {
                                                self::ml_string::Text::from_units(vec![
                                                    102, 105, 114, 115, 116, 67, 97, 108, 108,
                                                ])
                                            } else {
                                                self::ml_string::Text::from_units(vec![
                                                    115, 101, 99, 111, 110, 100, 67, 97, 108, 108,
                                                ])
                                            }
                                        };
                                        {
                                            let arguments_ = if exotic {
                                                std::rc::Rc::new(std::cell::RefCell::new(vec![self::read_java_script_primitive_conversion_spelling(&(shared_constructor_field3))]))
                                            } else {
                                                std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                                    self::JavaScriptSemanticValue,
                                                >::new(
                                                )))
                                            };
                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), phase.clone())), Box::new(method.clone()), Box::new(shared_constructor_field1.clone()), arguments_.clone())
                                        }
                                    }
                                }
                            }
                            _ => {
                                if exotic {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 101, 116, 104, 111, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 97, 98, 115, 101, 110, 116]))
                                } else {
                                    if first {
                                        self::continue_java_script_primitive_conversion(
                                            state.clone(),
                                            &(self::ml_string::Text::from_units(vec![
                                                115, 101, 99, 111, 110, 100, 71, 101, 116,
                                            ])),
                                        )
                                    } else {
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115]))
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
pub fn step_java_script_primitive_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let error = self::read_java_script_primitive_conversion_state_error(
                    heap.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
                } else {
                    if (((shared_constructor_field4
                        != (self::ml_string::Text::from_units(vec![
                            101, 120, 111, 116, 105, 99, 71, 101, 116,
                        ])))
                        && (shared_constructor_field4
                            != (self::ml_string::Text::from_units(vec![
                                102, 105, 114, 115, 116, 71, 101, 116,
                            ]))))
                        && (shared_constructor_field4
                            != (self::ml_string::Text::from_units(vec![
                                115, 101, 99, 111, 110, 100, 71, 101, 116,
                            ]))))
                    {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                            self::ml_string::Text::from_units(vec![
                                97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 112, 114, 105, 109,
                                105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105,
                                111, 110, 32, 99, 97, 108, 108, 32, 110, 101, 101, 100, 115, 32,
                                105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109,
                                112, 108, 101, 116, 105, 111, 110,
                            ]),
                        )
                    } else {
                        {
                            let receiver = shared_constructor_field1.clone();
                            match receiver.clone() {
                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                    let first = (shared_constructor_field4
                                        == (self::ml_string::Text::from_units(vec![
                                            102, 105, 114, 115, 116, 71, 101, 116,
                                        ])));
                                    {
                                        let method = if (shared_constructor_field3
                                            == (self::ml_string::Text::from_units(vec![
                                                115, 116, 114, 105, 110, 103,
                                            ]))) {
                                            if first {
                                                self::ml_string::Text::from_units(vec![
                                                    116, 111, 83, 116, 114, 105, 110, 103,
                                                ])
                                            } else {
                                                self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101, 79, 102,
                                                ])
                                            }
                                        } else {
                                            if first {
                                                self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101, 79, 102,
                                                ])
                                            } else {
                                                self::ml_string::Text::from_units(vec![
                                                    116, 111, 83, 116, 114, 105, 110, 103,
                                                ])
                                            }
                                        };
                                        {
                                            let key = if (shared_constructor_field4
                                                == (self::ml_string::Text::from_units(vec![
                                                    101, 120, 111, 116, 105, 99, 71, 101, 116,
                                                ]))) {
                                                shared_constructor_field2.clone()
                                            } else {
                                                self::read_java_script_primitive_conversion_spelling(
                                                    &(method),
                                                )
                                            };
                                            {
                                                let plan = self::plan_java_script_ordinary_get(
                                                    heap.clone(),
                                                    identity.clone(),
                                                    key.clone(),
                                                    receiver.clone(),
                                                );
                                                if (((({
                                                    let ml_reference_value330 =
                                                        plan.borrow().clone();
                                                    ml_reference_value330
                                                })
                                                .0)
                                                    .3)
                                                    != (self::ml_string::Text::from_units(vec![])))
                                                {
                                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid((({ let ml_reference_value331 = plan.borrow().clone(); ml_reference_value331 }).0).3)
                                                } else {
                                                    if (((({
                                                        let ml_reference_value332 =
                                                            plan.borrow().clone();
                                                        ml_reference_value332
                                                    })
                                                    .0)
                                                        .0)
                                                        == (self::ml_string::Text::from_units(
                                                            vec![99, 97, 108, 108],
                                                        )))
                                                    {
                                                        {
                                                            let phase = if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116]))) {
                                                                    self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114])
                                                                } else {
                                                                    if first {
                                                                        self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114])
                                                                    } else {
                                                                        self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114])
                                                                    }
                                                                };
                                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(receiver.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone(), phase.clone())), Box::new((({ let ml_reference_value333 = plan.borrow().clone(); ml_reference_value333 }).0).2), Box::new((({ let ml_reference_value334 = plan.borrow().clone(); ml_reference_value334 }).0).4), (({ let ml_reference_value335 = plan.borrow().clone(); ml_reference_value335 }).0).1)
                                                        }
                                                    } else {
                                                        if (((({
                                                            let ml_reference_value336 =
                                                                plan.borrow().clone();
                                                            ml_reference_value336
                                                        })
                                                        .0)
                                                            .0)
                                                            != (self::ml_string::Text::from_units(
                                                                vec![118, 97, 108, 117, 101],
                                                            )))
                                                        {
                                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 71, 101, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 118, 97, 108, 117, 101, 32, 111, 114, 32, 112, 101, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114]))
                                                        } else {
                                                            self::apply_java_script_primitive_conversion_method(state.clone(), (({ let ml_reference_value337 = plan.borrow().clone(); ml_reference_value337 }).0).5)
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                _ => {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                        self::ml_string::Text::from_units(vec![
                                            97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101,
                                            32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                                            32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105,
                                            111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                                            105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 79,
                                            98, 106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118,
                                            101, 114,
                                        ]),
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_primitive_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
    hint: &self::ml_string::Text,
) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(heap_error.clone())
        } else {
            {
                let error =
                    self::read_java_script_ordinary_value_error(heap.clone(), input.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
                } else {
                    if ((((*hint)
                        != (self::ml_string::Text::from_units(vec![
                            100, 101, 102, 97, 117, 108, 116,
                        ])))
                        && ((*hint)
                            != (self::ml_string::Text::from_units(vec![
                                110, 117, 109, 98, 101, 114,
                            ]))))
                        && ((*hint)
                            != (self::ml_string::Text::from_units(vec![
                                115, 116, 114, 105, 110, 103,
                            ]))))
                    {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                            self::ml_string::Text::from_units(vec![
                                97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111,
                                110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117,
                                105, 114, 101, 115, 32, 97, 110, 32, 101, 120, 97, 99, 116, 32,
                                112, 114, 101, 102, 101, 114, 114, 101, 100, 45, 116, 121, 112,
                                101, 32, 104, 105, 110, 116,
                            ]),
                        )
                    } else {
                        match input.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => {
                                self::step_java_script_primitive_conversion(
                                    heap.clone(),
                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                        Box::new(input.clone()),
                                        Box::new(exotic_key.clone()),
                                        hint.clone(),
                                        self::ml_string::Text::from_units(vec![
                                            101, 120, 111, 116, 105, 99, 71, 101, 116,
                                        ]),
                                    ),
                                )
                            }
                            _ => self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(
                                Box::new(input.clone()),
                            ),
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_primitive_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptPrimitiveConversionTransition {
    match state.clone() {
        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
            _,
            _,
            _,
            shared_constructor_field1,
        ) => {
            let error = self::read_java_script_primitive_conversion_state_error(
                heap.clone(),
                state.clone(),
            );
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
            } else {
                {
                    let getter = (((shared_constructor_field1
                        == (self::ml_string::Text::from_units(vec![
                            101, 120, 111, 116, 105, 99, 71, 101, 116, 116, 101, 114,
                        ])))
                        || (shared_constructor_field1
                            == (self::ml_string::Text::from_units(vec![
                                102, 105, 114, 115, 116, 71, 101, 116, 116, 101, 114,
                            ]))))
                        || (shared_constructor_field1
                            == (self::ml_string::Text::from_units(vec![
                                115, 101, 99, 111, 110, 100, 71, 101, 116, 116, 101, 114,
                            ]))));
                    {
                        let call = (((shared_constructor_field1
                            == (self::ml_string::Text::from_units(vec![
                                101, 120, 111, 116, 105, 99, 67, 97, 108, 108,
                            ])))
                            || (shared_constructor_field1
                                == (self::ml_string::Text::from_units(vec![
                                    102, 105, 114, 115, 116, 67, 97, 108, 108,
                                ]))))
                            || (shared_constructor_field1
                                == (self::ml_string::Text::from_units(vec![
                                    115, 101, 99, 111, 110, 100, 67, 97, 108, 108,
                                ]))));
                        if (!getter && !call) {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                self::ml_string::Text::from_units(vec![
                                    112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 114, 101, 115,
                                    117, 109, 112, 116, 105, 111, 110, 32, 110, 101, 101, 100, 115,
                                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 101,
                                    110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114, 32,
                                    111, 114, 32, 109, 101, 116, 104, 111, 100,
                                ]),
                            )
                        } else {
                            {
                                let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                        self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value3648) => {
                                            let source_ml_source2_data_property_value3648 = *source_ml_source2_data_property_value3648;
                                            source_ml_source2_data_property_value3648.clone()
                                        }
                                        self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value3649) => {
                                            let source_ml_source2_data_property_value3649 = *source_ml_source2_data_property_value3649;
                                            source_ml_source2_data_property_value3649.clone()
                                        }
                                    });
                                if (value_error != (self::ml_string::Text::from_units(vec![]))) {
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                        value_error.clone(),
                                    )
                                } else {
                                    match completion.clone() {
                                            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                                let value = *value;
                                                if getter {
                                                    self::apply_java_script_primitive_conversion_method(state.clone(), value.clone())
                                                } else {
                                                    if match value.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    } {
                                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(value.clone()))
                                                    } else {
                                                        if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![102, 105, 114, 115, 116, 67, 97, 108, 108]))) {
                                                            self::continue_java_script_primitive_conversion(state.clone(), &(self::ml_string::Text::from_units(vec![115, 101, 99, 111, 110, 100, 71, 101, 116])))
                                                        } else {
                                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![101, 120, 111, 116, 105, 99, 67, 97, 108, 108]))) {
                                                                self::ml_string::Text::from_units(vec![97, 110, 32, 101, 120, 111, 116, 105, 99, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 109, 117, 115, 116, 32, 114, 101, 116, 117, 114, 110, 32, 97, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 115, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 45, 112, 114, 111, 100, 117, 99, 105, 110, 103, 32, 109, 101, 116, 104, 111, 100, 115])
                                                            })
                                                        }
                                                    }
                                                }
                                            }
                                            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                                let value_2 = *value_2;
                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(Box::new(value_2.clone()))
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
pub fn read_java_script_scalar_protocol_operation_error(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    self::read_java_script_call_scalar_conversion_error(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_scalar_protocol_hint(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if ((*operation)
        == (self::ml_string::Text::from_units(vec![
            99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32,
            116, 101, 120, 116,
        ])))
    {
        self::ml_string::Text::from_units(vec![115, 116, 114, 105, 110, 103])
    } else {
        if ((*operation)
            == (self::ml_string::Text::from_units(vec![
                99, 111, 110, 118, 101, 114, 116, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101,
                32, 118, 97, 108, 117, 101,
            ])))
        {
            self::ml_string::Text::from_units(vec![100, 101, 102, 97, 117, 108, 116])
        } else {
            self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_scalar_protocol(
    operation: &self::ml_string::Text,
    transition: self::JavaScriptPrimitiveConversionTransition,
) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            match transition.clone() {
                self::JavaScriptPrimitiveConversionTransition::PrimitiveContinuation(state) => {
                    let state = *state;
                    self::JavaScriptScalarConversionTransition::ScalarContinuation(Box::new(
                        state.clone(),
                    ))
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(
                    state_2,
                    callee,
                    receiver,
                    arguments,
                ) => {
                    let state_2 = *state_2;
                    let callee = *callee;
                    let receiver = *receiver;
                    self::JavaScriptScalarConversionTransition::ScalarCall(
                        Box::new(state_2.clone()),
                        Box::new(callee.clone()),
                        Box::new(receiver.clone()),
                        arguments.clone(),
                    )
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(value) => {
                    let value = *value;
                    self::finish_java_script_scalar_primitive(operation, value.clone())
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(value_2) => {
                    let value_2 = *value_2;
                    self::JavaScriptScalarConversionTransition::ScalarThrow(Box::new(
                        value_2.clone(),
                    ))
                }
                self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                    error_class,
                    reason,
                ) => self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(
                    error_class.clone(),
                    reason.clone(),
                ),
                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(reason_2) => {
                    self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2.clone())
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_scalar_primitive(
    operation: &self::ml_string::Text,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            {
                let invalid = self::read_java_script_semantic_value_error(value.clone());
                if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptScalarConversionTransition::ScalarInvalid(invalid.clone())
                } else {
                    {
                        let result = self::execute_java_script_call_scalar_conversion(
                            operation,
                            value.clone(),
                        );
                        match result.clone() {
                            self::JavaScriptCallPrimitiveResult::PrimitiveValue(value_2) => {
                                let value_2 = *value_2;
                                self::JavaScriptScalarConversionTransition::ScalarCompleted(
                                    Box::new(value_2.clone()),
                                )
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveInvalid(reason) => {
                                self::JavaScriptScalarConversionTransition::ScalarInvalid(
                                    reason.clone(),
                                )
                            }
                            self::JavaScriptCallPrimitiveResult::PrimitiveError(
                                error_class,
                                reason_2,
                            ) => self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(
                                error_class.clone(),
                                reason_2.clone(),
                            ),
                            self::JavaScriptCallPrimitiveResult::PrimitiveResourceLimit(
                                reason_3,
                            ) => self::JavaScriptScalarConversionTransition::ScalarResourceLimit(
                                Box::new(value.clone()),
                                reason_3.clone(),
                            ),
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_scalar_protocol(
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
    operation: &self::ml_string::Text,
) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            self::finish_java_script_scalar_protocol(
                operation,
                self::begin_java_script_primitive_conversion(
                    heap.clone(),
                    input.clone(),
                    exotic_key.clone(),
                    &(self::read_java_script_scalar_protocol_hint(operation)),
                ),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_scalar_protocol(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
    operation: &self::ml_string::Text,
) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            if ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    source_ml_source2_data_property_value3657,
                    _,
                ) => source_ml_source2_data_property_value3657.clone(),
            }) != (self::read_java_script_scalar_protocol_hint(operation)))
            {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(
                    self::ml_string::Text::from_units(vec![
                        115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                        110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 104, 105,
                        110, 116, 32, 111, 102, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                        32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111,
                        110,
                    ]),
                )
            } else {
                self::finish_java_script_scalar_protocol(
                    operation,
                    self::step_java_script_primitive_conversion(heap.clone(), state.clone()),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_scalar_protocol(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPrimitiveConversionState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
    operation: &self::ml_string::Text,
) -> self::JavaScriptScalarConversionTransition {
    {
        let error = self::read_java_script_scalar_protocol_operation_error(operation);
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptScalarConversionTransition::ScalarInvalid(error.clone())
        } else {
            if ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    source_ml_source2_data_property_value3659,
                    _,
                ) => source_ml_source2_data_property_value3659.clone(),
            }) != (self::read_java_script_scalar_protocol_hint(operation)))
            {
                self::JavaScriptScalarConversionTransition::ScalarInvalid(
                    self::ml_string::Text::from_units(vec![
                        115, 99, 97, 108, 97, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                        110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 104, 105,
                        110, 116, 32, 111, 102, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                        32, 115, 111, 117, 114, 99, 101, 32, 111, 112, 101, 114, 97, 116, 105, 111,
                        110,
                    ]),
                )
            } else {
                self::finish_java_script_scalar_protocol(
                    operation,
                    self::resume_java_script_primitive_conversion(
                        heap.clone(),
                        state.clone(),
                        completion.clone(),
                    ),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_source_coordinate(value: f64, limit: f64) -> bool {
    (((self::ml_math::is_safe_integer(value) && (value >= (0f64))) && (value < limit))
        && ((value != (0f64)) || ((1f64 / value) > (0f64))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_constructor_fact(
    facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptArrayConstructorFact>>>,
    identity: self::ml::Big,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array338 = facts.clone();
                let ml_reference_projection338 = ml_reference_array338.borrow().len() as f64;
                ml_reference_projection338
            };
            {
                let source_ml_source2_s3660 = self::source_ml_source1_temporary_find_java_script_call_constructor_fact_loop197(facts.clone(), identity.clone(), low, high);
                match source_ml_source2_s3660.clone() {
                    self::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Result::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Result::MlSource1TemporaryFindJavaScriptCallConstructorFactLoop197Return(source_ml_source1_temporary_result197) => {
                        source_ml_source1_temporary_result197
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_call_code_coordinate(value: f64, limit: f64) -> bool {
    self::test_java_script_source_coordinate(value, limit)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_call_code_function(
    program: self::JavaScriptCallProgram,
    identity: self::ml::Big,
) -> f64 {
    {
        let low = 0f64;
        {
            let high = {
                let ml_reference_array339 = match program.clone() {
                    self::JavaScriptCallProgram::CallProgram(
                        source_ml_source2_data_property_value3661,
                        _,
                        _,
                        _,
                        _,
                        _,
                    ) => source_ml_source2_data_property_value3661.clone(),
                };
                let ml_reference_projection339 = ml_reference_array339.borrow().len() as f64;
                ml_reference_projection339
            };
            {
                let source_ml_source2_s3662 =
                    self::source_ml_source1_temporary_find_java_script_call_code_function_loop198(
                        program.clone(),
                        identity.clone(),
                        low,
                        high,
                    );
                match source_ml_source2_s3662.clone() {
                    self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Done => {
                        (-1f64)
                    }
                    self::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Result::MlSource1TemporaryFindJavaScriptCallCodeFunctionLoop198Return(source_ml_source1_temporary_result198) => {
                        source_ml_source1_temporary_result198
                    }
                }
            }
        }
    }
}
