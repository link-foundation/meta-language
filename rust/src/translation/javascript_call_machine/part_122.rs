// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayIterationTransition;
use super::JavaScriptArrayIteratorCall;
use super::JavaScriptArrayIteratorState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::continue_java_script_array_iterator_length;
use super::continue_java_script_array_iterator_value;
use super::finish_java_script_array_iterator_cursor;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_number;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_array_iterator_state_error;
use super::read_java_script_ordinary_value_error;
use super::resume_java_script_scalar_protocol;
use super::step_java_script_scalar_protocol;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_iterator_next(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayIteratorState,
) -> self::JavaScriptArrayIterationTransition {
    match state.clone() {
        self::JavaScriptArrayIteratorState::ArrayIteratorState(
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field10 = *shared_constructor_field10;
            {
                let error =
                    self::read_java_script_array_iterator_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptArrayIteratorState::ArrayIteratorState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value24267,
                            ) => ((source_ml_source2_data_property_value24267
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::JavaScriptArrayIterationTransition::ArrayIteratorCall(
                            Box::new(heap.clone()),
                            Box::new(state.clone()),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptArrayIteratorState::ArrayIteratorState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value24268,
                                    ) => ((source_ml_source2_data_property_value24268
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArrayIteratorCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptArrayIteratorCall::ArrayIteratorCall(
                                        source_ml_source2_data_property_value24269,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value24269 =
                                            *source_ml_source2_data_property_value24269;
                                        source_ml_source2_data_property_value24269.clone()
                                    }
                                },
                            ),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptArrayIteratorState::ArrayIteratorState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value24270,
                                    ) => ((source_ml_source2_data_property_value24270
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArrayIteratorCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptArrayIteratorCall::ArrayIteratorCall(
                                        _,
                                        source_ml_source2_data_property_value24271,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value24271 =
                                            *source_ml_source2_data_property_value24271;
                                        source_ml_source2_data_property_value24271.clone()
                                    }
                                },
                            ),
                            match (match state.clone() {
                                self::JavaScriptArrayIteratorState::ArrayIteratorState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value24272,
                                ) => ((source_ml_source2_data_property_value24272
                                    .clone()
                                    .unwrap_or_else(|| {
                                        (None::<self::JavaScriptArrayIteratorCall>,)
                                    }))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value")
                            {
                                self::JavaScriptArrayIteratorCall::ArrayIteratorCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value24273,
                                ) => source_ml_source2_data_property_value24273.clone(),
                            },
                        )
                    } else {
                        if (shared_constructor_field9
                            == (self::ml_string::Text::from_units(vec![
                                99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                            ])))
                        {
                            if match (
                                match state.clone() {
                                    self::JavaScriptArrayIteratorState::ArrayIteratorState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value24276,
                                        _,
                                    ) => ((source_ml_source2_data_property_value24276
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptPrimitiveConversionState>,)
                                        }))
                                    .0)
                                        .clone(),
                                },
                                None::<()>,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116,
                                        111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                                        110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                        115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 116,
                                        105, 110, 117, 97, 116, 105, 111, 110,
                                    ]),
                                )
                            } else {
                                self::continue_java_script_array_iterator_length(heap.clone(), state.clone(), self::step_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                    self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, source_ml_source2_data_property_value24277, _) => {
                                        ((source_ml_source2_data_property_value24277.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value"), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                            }
                        } else {
                            if (shared_constructor_field9
                                == (self::ml_string::Text::from_units(vec![
                                    112, 114, 105, 109, 105, 116, 105, 118, 101,
                                ])))
                            {
                                self::continue_java_script_array_iterator_length(
                                    heap.clone(),
                                    state.clone(),
                                    self::finish_java_script_scalar_primitive(
                                        &(self::ml_string::Text::from_units(vec![
                                            99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109,
                                            105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101,
                                            114,
                                        ])),
                                        shared_constructor_field10.clone(),
                                    ),
                                )
                            } else {
                                {
                                    let key = if (shared_constructor_field9
                                        == (self::ml_string::Text::from_units(vec![
                                            108, 101, 110, 103, 116, 104,
                                        ]))) {
                                        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                            std::cell::RefCell::new(vec![
                                                108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
                                            ]),
                                        ))
                                    } else {
                                        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                            std::cell::RefCell::new({
                                                let ml_values1257 = ({
                                                    let source_ml_source2_string_argument24284 =
                                                        shared_constructor_field7;
                                                    self::ml_string::Text::from_text(
                                                        &self::ml_number::js_number(
                                                            source_ml_source2_string_argument24284,
                                                        ),
                                                    )
                                                })
                                                .code_points();
                                                let ml_callback1257 = std::rc::Rc::new(
                                                    move |ch: self::ml_string::Text| -> f64 {
                                                        {
                                                            let source_ml_source2_string_unit_operand24285 =
                                                                ch.clone();
                                                            {
                                                                let source_ml_source2_string_unit_operand24286 =
                                                                    0f64;
                                                                (std::rc::Rc::new(move |text: self::ml_string::Text, position: f64| -> f64 { {
                                                    let index = if position.is_nan() {
                                                        0f64
                                                    } else {
                                                        position.trunc()
                                                    };
                                                    if ((index < (0f64)) || (index >= (text.length()))) {
                                                        f64::NAN
                                                    } else {
                                                        ({
                                                            let source_ml_source2_string_unit_operand24292 = {
                                                                let source_ml_source2_string_range_operand24287 = text.clone();
                                                                {
                                                                    let source_ml_source2_string_range_operand24288 = index;
                                                                    {
                                                                        let source_ml_source2_string_range_operand24289 = (index + 1f64);
                                                                        {
                                                                            let source_ml_source2_string_range_position24290 = source_ml_source2_string_range_operand24288;
                                                                            {
                                                                                let source_ml_source2_string_range_position24291 = source_ml_source2_string_range_operand24289;
                                                                                source_ml_source2_string_range_operand24287.slice(source_ml_source2_string_range_position24290, source_ml_source2_string_range_position24291)
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            };
                                                            {
                                                                let source_ml_source2_string_unit_operand24293 = 0f64;
                                                                source_ml_source2_string_unit_operand24292.code_point_at(source_ml_source2_string_unit_operand24293)
                                                            }
                                                        }).clone().unwrap_or_else(|| f64::NAN)
                                                    }
                                                } }))(source_ml_source2_string_unit_operand24285.clone(), source_ml_source2_string_unit_operand24286)
                                                            }
                                                        }
                                                    },
                                                );
                                                ml_values1257
                                                    .iter()
                                                    .cloned()
                                                    .enumerate()
                                                    .map(|(ml_index1257, ml_value1257)| {
                                                        ml_callback1257(ml_value1257)
                                                    })
                                                    .collect::<Vec<_>>()
                                            }),
                                        ))
                                    };
                                    if {
                                        let source_ml_source2_s24295 =
                                            shared_constructor_field5.clone();
                                        match source_ml_source2_s24295.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => false,
                                            _ => true,
                                        }
                                    } {
                                        self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 111, 117, 114, 99, 101, 32, 79, 98, 106, 101, 99, 116]))
                                    } else {
                                        {
                                            let get = self::plan_java_script_ordinary_get(
                                                heap.clone(),
                                                (((match shared_constructor_field5.clone() {
                                                    self::JavaScriptSemanticValue::Object(
                                                        source_ml_source2_data_property_value24300,
                                                        _,
                                                    ) => Some((
                                                        source_ml_source2_data_property_value24300
                                                            .clone(),
                                                    )),
                                                    _ => None::<(self::ml::Big,)>,
                                                })
                                                .clone()
                                                .expect("a proven present value"))
                                                .0)
                                                    .clone(),
                                                key.clone(),
                                                shared_constructor_field5.clone(),
                                            );
                                            if (((({
                                                let ml_reference_value1258 = get.borrow().clone();
                                                ml_reference_value1258
                                            })
                                            .0)
                                                .3)
                                                != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(Box::new(heap.clone()), (({ let ml_reference_value1259 = get.borrow().clone(); ml_reference_value1259 }).0).3)
                                            } else {
                                                if (((({
                                                    let ml_reference_value1260 =
                                                        get.borrow().clone();
                                                    ml_reference_value1260
                                                })
                                                .0)
                                                    .0)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        99, 97, 108, 108,
                                                    ])))
                                                {
                                                    {
                                                        let call = self::JavaScriptArrayIteratorCall::ArrayIteratorCall(Box::new((({ let ml_reference_value1261 = get.borrow().clone(); ml_reference_value1261 }).0).2), Box::new((({ let ml_reference_value1262 = get.borrow().clone(); ml_reference_value1262 }).0).4), (({ let ml_reference_value1263 = get.borrow().clone(); ml_reference_value1263 }).0).1);
                                                        match call.clone() {
                                                            self::JavaScriptArrayIteratorCall::ArrayIteratorCall(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                                self::JavaScriptArrayIterationTransition::ArrayIteratorCall(Box::new(heap.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field24304 = shared_constructor_field4.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field24306 = shared_constructor_field5.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field24308 = shared_constructor_field6.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field24310 = shared_constructor_field7;
                                                                                {
                                                                                    let source_ml_source2_nominal_field24312 = shared_constructor_field8.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field24314 = shared_constructor_field9.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field24316 = shared_constructor_field10.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field24318 = shared_constructor_field11.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field24320 = shared_constructor_field12.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override24322 = shared_constructor_field9.concat(&self::ml_string::Text::from_units(vec![32, 103, 101, 116, 116, 101, 114]));
                                                                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(Box::new(source_ml_source2_nominal_field24304.clone()), Box::new(source_ml_source2_nominal_field24306.clone()), Box::new(source_ml_source2_nominal_field24308.clone()), source_ml_source2_nominal_field24310, source_ml_source2_nominal_field24312.clone(), source_ml_source2_nominal_override24322.clone(), Box::new(source_ml_source2_nominal_field24316.clone()), source_ml_source2_nominal_field24318.clone(), Some((Some(call.clone()),)))
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }), Box::new(shared_constructor_field1.clone()), Box::new(shared_constructor_field2.clone()), shared_constructor_field3.clone())
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    if (((({
                                                        let ml_reference_value1264 =
                                                            get.borrow().clone();
                                                        ml_reference_value1264
                                                    })
                                                    .0)
                                                        .0)
                                                        == (self::ml_string::Text::from_units(
                                                            vec![118, 97, 108, 117, 101],
                                                        )))
                                                    {
                                                        self::continue_java_script_array_iterator_value(heap.clone(), state.clone(), (({ let ml_reference_value1265 = get.borrow().clone(); ml_reference_value1265 }).0).5)
                                                    } else {
                                                        self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 116, 116, 101, 114, 47, 118, 97, 108, 117, 101, 32, 112, 114, 111, 116, 111, 99, 111, 108]))
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
pub fn resume_java_script_array_iterator_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayIteratorState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArrayIterationTransition {
    {
        let error = self::read_java_script_array_iterator_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            if match (
                match state.clone() {
                    self::JavaScriptArrayIteratorState::ArrayIteratorState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value24327,
                    ) => ((source_ml_source2_data_property_value24327
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            } {
                self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114,
                        101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105,
                        110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101,
                        108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108, 100,
                    ]),
                )
            } else {
                {
                    let invalid = self::read_java_script_ordinary_value_error(
                        heap.clone(),
                        match completion.clone() {
                            self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                source_ml_source2_data_property_value24328,
                            ) => {
                                let source_ml_source2_data_property_value24328 =
                                    *source_ml_source2_data_property_value24328;
                                source_ml_source2_data_property_value24328.clone()
                            }
                            self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                source_ml_source2_data_property_value24329,
                            ) => {
                                let source_ml_source2_data_property_value24329 =
                                    *source_ml_source2_data_property_value24329;
                                source_ml_source2_data_property_value24329.clone()
                            }
                        },
                    );
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(
                            Box::new(heap.clone()),
                            invalid.clone(),
                        )
                    } else {
                        if ((match state.clone() {
                            self::JavaScriptArrayIteratorState::ArrayIteratorState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value24331,
                                _,
                                _,
                                _,
                            ) => source_ml_source2_data_property_value24331.clone(),
                        }) == (self::ml_string::Text::from_units(vec![
                            99, 111, 110, 118, 101, 114, 115, 105, 111, 110,
                        ]))) {
                            if match (
                                match state.clone() {
                                    self::JavaScriptArrayIteratorState::ArrayIteratorState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value24332,
                                        _,
                                    ) => ((source_ml_source2_data_property_value24332
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptPrimitiveConversionState>,)
                                        }))
                                    .0)
                                        .clone(),
                                },
                                None::<()>,
                            ) {
                                (Some(_ml_left), Some(_ml_right)) => false,
                                (None, None) => true,
                                _ => false,
                            } {
                                self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116,
                                        111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111,
                                        110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116,
                                        115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101,
                                        99, 116, 101, 100, 32, 99, 111, 110, 116, 105, 110, 117,
                                        97, 116, 105, 111, 110,
                                    ]),
                                )
                            } else {
                                self::continue_java_script_array_iterator_length(heap.clone(), state.clone(), self::resume_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                    self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, _, _, _, source_ml_source2_data_property_value24333, _) => {
                                        ((source_ml_source2_data_property_value24333.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value"), completion.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                            }
                        } else {
                            match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                    value,
                                ) => {
                                    let value = *value;
                                    self::continue_java_script_array_iterator_value(
                                        heap.clone(),
                                        state.clone(),
                                        value.clone(),
                                    )
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                    value_2,
                                ) => {
                                    let value_2 = *value_2;
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorThrow(
                                        Box::new(self::finish_java_script_array_iterator_cursor(
                                            heap.clone(),
                                            state.clone(),
                                            true,
                                        )),
                                        Box::new(value_2.clone()),
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
