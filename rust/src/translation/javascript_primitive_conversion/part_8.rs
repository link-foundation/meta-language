// Generated from complete js/src/translation/javascript-primitive-conversion.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptPrimitiveConversionTransition;
use super::JavaScriptSemanticValue;
use super::continue_java_script_primitive_conversion;
use super::ml_array;
use super::ml_math;
use super::plan_java_script_ordinary_get;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_conversion_spelling;
use super::read_java_script_primitive_conversion_state_error;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn apply_java_script_primitive_conversion_method(
    state: self::JavaScriptPrimitiveConversionState,
    method: self::JavaScriptSemanticValue,
) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let exotic = ((match state.clone() {
            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                _,
                _,
                _,
                source_ml_source1_data_property_value815,
            ) => source_ml_source1_data_property_value815.clone(),
        }) == "exoticGet")
            || ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value817,
                ) => source_ml_source1_data_property_value817.clone(),
            }) == "exoticGetter");
        {
            let first = ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value819,
                ) => source_ml_source1_data_property_value819.clone(),
            }) == "firstGet")
                || ((match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value821,
                    ) => source_ml_source1_data_property_value821.clone(),
                }) == "firstGetter");
            if exotic
                && (match method.clone() {
                    self::JavaScriptSemanticValue::Undefined => true,
                    self::JavaScriptSemanticValue::Null => false,
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f822) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f823) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f824) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f825) => false,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f826) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f827,
                        source_ml_source1_f828,
                    ) => false,
                } || match method.clone() {
                    self::JavaScriptSemanticValue::Undefined => false,
                    self::JavaScriptSemanticValue::Null => true,
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f829) => false,
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f830) => false,
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f831) => false,
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f832) => false,
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f833) => false,
                    self::JavaScriptSemanticValue::Object(
                        source_ml_source1_f834,
                        source_ml_source1_f835,
                    ) => false,
                })
            {
                self::continue_java_script_primitive_conversion(state.clone(), "firstGet")
            } else {
                match method.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "an exotic primitive conversion method must be callable or absent",
                                ),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                "secondGet",
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "ordinary conversion exhausted its callable primitive-producing methods",
                                ),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Null => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "an exotic primitive conversion method must be callable or absent",
                                ),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                "secondGet",
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "ordinary conversion exhausted its callable primitive-producing methods",
                                ),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f836) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "an exotic primitive conversion method must be callable or absent",
                                ),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                "secondGet",
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "ordinary conversion exhausted its callable primitive-producing methods",
                                ),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f837) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "an exotic primitive conversion method must be callable or absent",
                                ),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                "secondGet",
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "ordinary conversion exhausted its callable primitive-producing methods",
                                ),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f838) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "an exotic primitive conversion method must be callable or absent",
                                ),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                "secondGet",
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "ordinary conversion exhausted its callable primitive-producing methods",
                                ),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f839) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "an exotic primitive conversion method must be callable or absent",
                                ),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                "secondGet",
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "ordinary conversion exhausted its callable primitive-producing methods",
                                ),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f840) => {
                        if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "an exotic primitive conversion method must be callable or absent",
                                ),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                "secondGet",
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "ordinary conversion exhausted its callable primitive-producing methods",
                                ),
                            )
                        }
                    }
                    self::JavaScriptSemanticValue::Object(source_ml_source1_f841, callable) => {
                        if callable {
                            {
                                let phase = if exotic {
                                    String::from("exoticCall")
                                } else if first {
                                    String::from("firstCall")
                                } else {
                                    String::from("secondCall")
                                };
                                {
                                    let arguments_ = if exotic {
                                        vec![self::read_java_script_primitive_conversion_spelling(&(match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value842, _) => {
                                                source_ml_source1_data_property_value842.clone()
                                            }
                                        }))]
                                    } else {
                                        Vec::<self::JavaScriptSemanticValue>::new()
                                    };
                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value843, _, _, _) => {
                                            let source_ml_source1_data_property_value843 = *source_ml_source1_data_property_value843;
                                            source_ml_source1_data_property_value843.clone()
                                        }
                                    }), Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value844, _, _) => {
                                            let source_ml_source1_data_property_value844 = *source_ml_source1_data_property_value844;
                                            source_ml_source1_data_property_value844.clone()
                                        }
                                    }), match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value845, _) => {
                                            source_ml_source1_data_property_value845.clone()
                                        }
                                    }, phase)), Box::new(method.clone()), Box::new(match state.clone() {
                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source1_data_property_value846, _, _, _) => {
                                            let source_ml_source1_data_property_value846 = *source_ml_source1_data_property_value846;
                                            source_ml_source1_data_property_value846.clone()
                                        }
                                    }), arguments_)
                                }
                            }
                        } else if exotic {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "an exotic primitive conversion method must be callable or absent",
                                ),
                            )
                        } else if first {
                            self::continue_java_script_primitive_conversion(
                                state.clone(),
                                "secondGet",
                            )
                        } else {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(
                                String::from("TypeError"),
                                String::from(
                                    "ordinary conversion exhausted its callable primitive-producing methods",
                                ),
                            )
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
    {
        let error =
            self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
        if !(error).is_empty() {
            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
        } else if (((match state.clone() {
            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                _,
                _,
                _,
                source_ml_source1_data_property_value848,
            ) => source_ml_source1_data_property_value848.clone(),
        }) != "exoticGet")
            && ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value850,
                ) => source_ml_source1_data_property_value850.clone(),
            }) != "firstGet"))
            && ((match state.clone() {
                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                    _,
                    _,
                    _,
                    source_ml_source1_data_property_value852,
                ) => source_ml_source1_data_property_value852.clone(),
            }) != "secondGet")
        {
            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(String::from(
                "a pending primitive conversion call needs its actual completion",
            ))
        } else {
            {
                let receiver = match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                        source_ml_source1_data_property_value853,
                        _,
                        _,
                        _,
                    ) => {
                        let source_ml_source1_data_property_value853 =
                            *source_ml_source1_data_property_value853;
                        source_ml_source1_data_property_value853.clone()
                    }
                };
                match receiver.clone() {
                    self::JavaScriptSemanticValue::Undefined => {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                            String::from(
                                "a primitive conversion continuation retains its actual Object receiver",
                            ),
                        )
                    }
                    self::JavaScriptSemanticValue::Null => {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                            String::from(
                                "a primitive conversion continuation retains its actual Object receiver",
                            ),
                        )
                    }
                    self::JavaScriptSemanticValue::Boolean(source_ml_source1_f854) => {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                            String::from(
                                "a primitive conversion continuation retains its actual Object receiver",
                            ),
                        )
                    }
                    self::JavaScriptSemanticValue::Number(source_ml_source1_f855) => {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                            String::from(
                                "a primitive conversion continuation retains its actual Object receiver",
                            ),
                        )
                    }
                    self::JavaScriptSemanticValue::Integer(source_ml_source1_f856) => {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                            String::from(
                                "a primitive conversion continuation retains its actual Object receiver",
                            ),
                        )
                    }
                    self::JavaScriptSemanticValue::Text(source_ml_source1_f857) => {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                            String::from(
                                "a primitive conversion continuation retains its actual Object receiver",
                            ),
                        )
                    }
                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_f858) => {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                            String::from(
                                "a primitive conversion continuation retains its actual Object receiver",
                            ),
                        )
                    }
                    self::JavaScriptSemanticValue::Object(identity, source_ml_source1_f859) => {
                        let first = (match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value861,
                            ) => source_ml_source1_data_property_value861.clone(),
                        }) == "firstGet";
                        {
                            let method = if (match state.clone() {
                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                    _,
                                    _,
                                    source_ml_source1_data_property_value863,
                                    _,
                                ) => source_ml_source1_data_property_value863.clone(),
                            }) == "string"
                            {
                                if first {
                                    String::from("toString")
                                } else {
                                    String::from("valueOf")
                                }
                            } else if first {
                                String::from("valueOf")
                            } else {
                                String::from("toString")
                            };
                            {
                                let key = if (match state.clone() {
                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value865) => {
                                                source_ml_source1_data_property_value865.clone()
                                            }
                                        }) == "exoticGet" {
                                            match state.clone() {
                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value866, _, _) => {
                                                    let source_ml_source1_data_property_value866 = *source_ml_source1_data_property_value866;
                                                    source_ml_source1_data_property_value866.clone()
                                                }
                                            }
                                        } else {
                                            self::read_java_script_primitive_conversion_spelling(&(method))
                                        };
                                {
                                    let plan = self::plan_java_script_ordinary_get(
                                        heap.clone(),
                                        identity.clone(),
                                        key,
                                        receiver.clone(),
                                    );
                                    {
                                        let ml_library_inequality_2 = ((plan.clone()).3) != "";
                                        if ml_library_inequality_2 {
                                            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid((plan.clone()).3)
                                        } else if ((plan.clone()).0) == "call" {
                                            {
                                                let phase = if (match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value868) => {
                                                                source_ml_source1_data_property_value868.clone()
                                                            }
                                                        }) == "exoticGet" {
                                                            String::from("exoticGetter")
                                                        } else if first {
                                                                String::from("firstGetter")
                                                            } else {
                                                                String::from("secondGetter")
                                                            };
                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCall(Box::new(self::JavaScriptPrimitiveConversionState::PrimitiveConversion(Box::new(receiver.clone()), Box::new(match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source1_data_property_value869, _, _) => {
                                                                let source_ml_source1_data_property_value869 = *source_ml_source1_data_property_value869;
                                                                source_ml_source1_data_property_value869.clone()
                                                            }
                                                        }), match state.clone() {
                                                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source1_data_property_value870, _) => {
                                                                source_ml_source1_data_property_value870.clone()
                                                            }
                                                        }, phase)), Box::new((plan.clone()).2), Box::new((plan.clone()).4), (plan.clone()).1)
                                            }
                                        } else {
                                            let ml_library_inequality_1 =
                                                ((plan.clone()).0) != "value";
                                            if ml_library_inequality_1 {
                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(String::from("ordinary Get needs its actual value or pending getter"))
                                            } else {
                                                self::apply_java_script_primitive_conversion_method(
                                                    state.clone(),
                                                    (plan.clone()).5,
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_primitive_conversion(
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
    exotic_key: self::JavaScriptSemanticValue,
    hint: &str,
) -> self::JavaScriptPrimitiveConversionTransition {
    {
        let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
        if !(heap_error).is_empty() {
            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(heap_error.clone())
        } else {
            {
                let error =
                    self::read_java_script_ordinary_value_error(heap.clone(), input.clone());
                if !(error).is_empty() {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
                } else if ((hint != "default") && (hint != "number")) && (hint != "string") {
                    self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(String::from(
                        "a primitive conversion requires an exact preferred-type hint",
                    ))
                } else {
                    match input.clone() {
                        self::JavaScriptSemanticValue::Undefined => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(
                                Box::new(input.clone()),
                            )
                        }
                        self::JavaScriptSemanticValue::Null => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(
                                Box::new(input.clone()),
                            )
                        }
                        self::JavaScriptSemanticValue::Boolean(source_ml_source1_f871) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(
                                Box::new(input.clone()),
                            )
                        }
                        self::JavaScriptSemanticValue::Number(source_ml_source1_f872) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(
                                Box::new(input.clone()),
                            )
                        }
                        self::JavaScriptSemanticValue::Integer(source_ml_source1_f873) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(
                                Box::new(input.clone()),
                            )
                        }
                        self::JavaScriptSemanticValue::Text(source_ml_source1_f874) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(
                                Box::new(input.clone()),
                            )
                        }
                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_f875) => {
                            self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(
                                Box::new(input.clone()),
                            )
                        }
                        self::JavaScriptSemanticValue::Object(
                            source_ml_source1_f876,
                            source_ml_source1_f877,
                        ) => self::step_java_script_primitive_conversion(
                            heap.clone(),
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                Box::new(input.clone()),
                                Box::new(exotic_key.clone()),
                                hint.to_owned(),
                                String::from("exoticGet"),
                            ),
                        ),
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
    {
        let error =
            self::read_java_script_primitive_conversion_state_error(heap.clone(), state.clone());
        if !(error).is_empty() {
            self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(error.clone())
        } else {
            {
                let getter = (((match state.clone() {
                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                        _,
                        _,
                        _,
                        source_ml_source1_data_property_value879,
                    ) => source_ml_source1_data_property_value879.clone(),
                }) == "exoticGetter")
                    || ((match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value881,
                        ) => source_ml_source1_data_property_value881.clone(),
                    }) == "firstGetter"))
                    || ((match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value883,
                        ) => source_ml_source1_data_property_value883.clone(),
                    }) == "secondGetter");
                {
                    let call = (((match state.clone() {
                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                            _,
                            _,
                            _,
                            source_ml_source1_data_property_value885,
                        ) => source_ml_source1_data_property_value885.clone(),
                    }) == "exoticCall")
                        || ((match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value887,
                            ) => source_ml_source1_data_property_value887.clone(),
                        }) == "firstCall"))
                        || ((match state.clone() {
                            self::JavaScriptPrimitiveConversionState::PrimitiveConversion(
                                _,
                                _,
                                _,
                                source_ml_source1_data_property_value889,
                            ) => source_ml_source1_data_property_value889.clone(),
                        }) == "secondCall");
                    if ((getter) || (call)) {
                        {
                            let value_error = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source1_data_property_value890) => {
                                    let source_ml_source1_data_property_value890 = *source_ml_source1_data_property_value890;
                                    source_ml_source1_data_property_value890.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source1_data_property_value891) => {
                                    let source_ml_source1_data_property_value891 = *source_ml_source1_data_property_value891;
                                    source_ml_source1_data_property_value891.clone()
                                }
                            });
                            if !(value_error).is_empty() {
                                self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                                    value_error.clone(),
                                )
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        if getter {
                                            self::apply_java_script_primitive_conversion_method(state.clone(), value.clone())
                                        } else if match value.clone() {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Boolean(source_ml_source1_f892) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Number(source_ml_source1_f893) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Integer(source_ml_source1_f894) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Text(source_ml_source1_f895) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_f896) => {
                                                    true
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source1_f897, source_ml_source1_f898) => {
                                                    false
                                                }
                                            } {
                                                self::JavaScriptPrimitiveConversionTransition::PrimitiveCompleted(Box::new(value.clone()))
                                            } else if (match state.clone() {
                                                    self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value900) => {
                                                        source_ml_source1_data_property_value900.clone()
                                                    }
                                                }) == "firstCall" {
                                                    self::continue_java_script_primitive_conversion(state.clone(), "secondGet")
                                                } else {
                                                    self::JavaScriptPrimitiveConversionTransition::PrimitiveIntrinsicError(String::from("TypeError"), if (match state.clone() {
                                                        self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, _, source_ml_source1_data_property_value902) => {
                                                            source_ml_source1_data_property_value902.clone()
                                                        }
                                                    }) == "exoticCall" {
                                                        String::from("an exotic primitive conversion must return a primitive")
                                                    } else {
                                                        String::from("ordinary conversion exhausted its callable primitive-producing methods")
                                                    })
                                                }
                                    }
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptPrimitiveConversionTransition::PrimitiveThrow(Box::new(value_2.clone()))
                                    }
                                }
                            }
                        }
                    } else {
                        self::JavaScriptPrimitiveConversionTransition::PrimitiveInvalid(
                            String::from(
                                "primitive resumption needs its actual pending getter or method",
                            ),
                        )
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done,
    MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_semantic_value_error_loop1(
    units: &[f64],
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result {
    loop {
        return if at < (units.len() as f64) {
            {
                let unit = (self::ml_array::optional_at(units, self::ml_array::number_index(at)))
                    .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Return(String::from("semantic text requires complete ordered UTF16 code units"))
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Result::MlTemporaryReadJavaScriptSemanticValueErrorLoop1Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Done,
    MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_test_java_script_semantic_same_value_loop2(
    left_units: &[f64],
    right_units: &[f64],
    mut at: f64,
) -> self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result {
    loop {
        return if at < (left_units.len() as f64) {
            {
                let ml_library_inequality_1 =
                    ((self::ml_array::optional_at(left_units, self::ml_array::number_index(at)))
                        .unwrap_or_else(|| -1f64))
                        != ((self::ml_array::optional_at(
                            right_units,
                            self::ml_array::number_index(at),
                        ))
                        .unwrap_or_else(|| -1f64));
                if ml_library_inequality_1 {
                    self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Return(false)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryTestJavaScriptSemanticSameValueLoop2Result::MlTemporaryTestJavaScriptSemanticSameValueLoop2Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryPlanJavaScriptPropertyDefinitionJoin3 {
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Next,
    MlTemporaryPlanJavaScriptPropertyDefinitionJoin3Return(
        (
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
            String,
            String,
            Vec<String>,
        ),
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result {
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Done(f64),
    MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_literal_property_index_loop4(
    key: &[f64],
    mut value: f64,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result {
    loop {
        return if at < (key.len() as f64) {
            {
                let digit = (self::ml_array::optional_at(key, self::ml_array::number_index(at)))
                    .unwrap_or(f64::NAN)
                    - 48f64;
                if (!(self::ml_math::is_integer(digit)) || (digit < 0f64)) || (digit > 9f64) {
                    self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Return(-1f64)
                } else {
                    {
                        let value_2 = (value * 10f64) + digit;
                        {
                            let at_2 = at + 1f64;
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Result::MlTemporaryReadJavaScriptLiteralPropertyIndexLoop4Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryFindJavaScriptOrdinaryObjectLoop5Result {
    MlTemporaryFindJavaScriptOrdinaryObjectLoop5Done,
    MlTemporaryFindJavaScriptOrdinaryObjectLoop5Return(f64),
}
