// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySearchCall;
use super::JavaScriptArraySearchState;
use super::JavaScriptArraySearchTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptArraySearchJoin1239;
use super::MlSource1TemporaryStepJavaScriptArraySearchLoop1238Result;
use super::begin_java_script_scalar_protocol;
use super::complete_java_script_array_search_element;
use super::continue_java_script_array_search_number;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_number;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_array_search_missing_value;
use super::read_java_script_array_search_state_error;
use super::read_java_script_indexed_property_presence;
use super::read_java_script_ordinary_value_error;
use super::request_java_script_array_search_call;
use super::resume_java_script_scalar_protocol;
use super::source_ml_source1_temporary_step_java_script_array_search_loop1238;
use super::step_java_script_scalar_protocol;
use super::test_java_script_standard_array_callback_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_search(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySearchState,
) -> self::JavaScriptArraySearchTransition {
    match state.clone() {
        self::JavaScriptArraySearchState::ArraySearchState(
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
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field10 = *shared_constructor_field10;
            {
                let error =
                    self::read_java_script_array_search_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArraySearchTransition::ArraySearchInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptArraySearchState::ArraySearchState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value27903,
                            ) => ((source_ml_source2_data_property_value27903
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::JavaScriptArraySearchTransition::ArraySearchCall(
                            Box::new(heap.clone()),
                            Box::new(state.clone()),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptArraySearchState::ArraySearchState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value27904,
                                    ) => ((source_ml_source2_data_property_value27904
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArraySearchCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptArraySearchCall::ArraySearchSelectedCall(
                                        source_ml_source2_data_property_value27905,
                                        _,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value27905 =
                                            *source_ml_source2_data_property_value27905;
                                        source_ml_source2_data_property_value27905.clone()
                                    }
                                },
                            ),
                            Box::new(
                                match (match state.clone() {
                                    self::JavaScriptArraySearchState::ArraySearchState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value27906,
                                    ) => ((source_ml_source2_data_property_value27906
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArraySearchCall>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value")
                                {
                                    self::JavaScriptArraySearchCall::ArraySearchSelectedCall(
                                        _,
                                        source_ml_source2_data_property_value27907,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value27907 =
                                            *source_ml_source2_data_property_value27907;
                                        source_ml_source2_data_property_value27907.clone()
                                    }
                                },
                            ),
                            match (match state.clone() {
                                self::JavaScriptArraySearchState::ArraySearchState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value27908,
                                ) => ((source_ml_source2_data_property_value27908
                                    .clone()
                                    .unwrap_or_else(
                                        || (None::<self::JavaScriptArraySearchCall>,),
                                    ))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value")
                            {
                                self::JavaScriptArraySearchCall::ArraySearchSelectedCall(
                                    _,
                                    _,
                                    source_ml_source2_data_property_value27909,
                                ) => source_ml_source2_data_property_value27909.clone(),
                            },
                        )
                    } else {
                        if ((shared_constructor_field9
                            == (self::ml_string::Text::from_units(vec![
                                108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115,
                                105, 111, 110,
                            ])))
                            || (shared_constructor_field9
                                == (self::ml_string::Text::from_units(vec![
                                    105, 110, 100, 101, 120, 32, 99, 111, 110, 118, 101, 114, 115,
                                    105, 111, 110,
                                ]))))
                        {
                            if match (
                                match state.clone() {
                                    self::JavaScriptArraySearchState::ArraySearchState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value27914,
                                        _,
                                    ) => ((source_ml_source2_data_property_value27914
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
                                self::JavaScriptArraySearchTransition::ArraySearchInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32,
                                        99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114,
                                        101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32,
                                        97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 116, 105, 110,
                                        117, 97, 116, 105, 111, 110,
                                    ]),
                                )
                            } else {
                                self::continue_java_script_array_search_number(heap.clone(), state.clone(), self::step_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                    self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27915, _) => {
                                        ((source_ml_source2_data_property_value27915.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value"), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                            }
                        } else {
                            if ((shared_constructor_field9
                                == (self::ml_string::Text::from_units(vec![
                                    108, 101, 110, 103, 116, 104, 32, 112, 114, 105, 109, 105, 116,
                                    105, 118, 101,
                                ])))
                                || (shared_constructor_field9
                                    == (self::ml_string::Text::from_units(vec![
                                        105, 110, 100, 101, 120, 32, 112, 114, 105, 109, 105, 116,
                                        105, 118, 101,
                                    ]))))
                            {
                                self::continue_java_script_array_search_number(
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
                                if {
                                    let source_ml_source2_s27922 =
                                        shared_constructor_field2.clone();
                                    match source_ml_source2_s27922.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => false,
                                        _ => true,
                                    }
                                } {
                                    self::JavaScriptArraySearchTransition::ArraySearchInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104,
                                            32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105,
                                            116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111,
                                            120, 101, 100, 32, 79, 98, 106, 101, 99, 116, 32, 114,
                                            101, 99, 101, 105, 118, 101, 114,
                                        ]),
                                    )
                                } else {
                                    if (shared_constructor_field9
                                        == (self::ml_string::Text::from_units(vec![
                                            108, 101, 110, 103, 116, 104,
                                        ])))
                                    {
                                        {
                                            let get = self::plan_java_script_ordinary_get(
                                                heap.clone(),
                                                (((match shared_constructor_field2.clone() {
                                                    self::JavaScriptSemanticValue::Object(
                                                        source_ml_source2_data_property_value27929,
                                                        _,
                                                    ) => Some((
                                                        source_ml_source2_data_property_value27929
                                                            .clone(),
                                                    )),
                                                    _ => None::<(self::ml::Big,)>,
                                                })
                                                .clone()
                                                .expect("a proven present value"))
                                                .0)
                                                    .clone(),
                                                self::JavaScriptSemanticValue::Text(
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        vec![
                                                            108f64, 101f64, 110f64, 103f64, 116f64,
                                                            104f64,
                                                        ],
                                                    )),
                                                ),
                                                shared_constructor_field2.clone(),
                                            );
                                            if (((({
                                                let ml_reference_value1538 = get.borrow().clone();
                                                ml_reference_value1538
                                            })
                                            .0)
                                                .3)
                                                != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                self::JavaScriptArraySearchTransition::ArraySearchInvalid(Box::new(heap.clone()), (({ let ml_reference_value1539 = get.borrow().clone(); ml_reference_value1539 }).0).3)
                                            } else {
                                                if (((({
                                                    let ml_reference_value1540 =
                                                        get.borrow().clone();
                                                    ml_reference_value1540
                                                })
                                                .0)
                                                    .0)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        99, 97, 108, 108,
                                                    ])))
                                                {
                                                    self::request_java_script_array_search_call(
                                                        heap.clone(),
                                                        {
                                                            let source_ml_source2_nominal_field27932 =
                                                                shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field27934 =
                                                                    shared_constructor_field2
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27936 =
                                                                        shared_constructor_field3
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field27938 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field27940 = shared_constructor_field5;
                                                                            {
                                                                                let source_ml_source2_nominal_field27942 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field27944 = shared_constructor_field7;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field27946 = shared_constructor_field8;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field27948 = shared_constructor_field9.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field27950 = shared_constructor_field10.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field27952 = shared_constructor_field11.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field27954 = shared_constructor_field12.clone();
                                                                                                        self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field27932.clone(), Box::new(source_ml_source2_nominal_field27934.clone()), Box::new(source_ml_source2_nominal_field27936.clone()), Box::new(source_ml_source2_nominal_field27938.clone()), source_ml_source2_nominal_field27940, Box::new(source_ml_source2_nominal_field27942.clone()), source_ml_source2_nominal_field27944, source_ml_source2_nominal_field27946, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), Box::new(source_ml_source2_nominal_field27950.clone()), source_ml_source2_nominal_field27952.clone(), source_ml_source2_nominal_field27954.clone())
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
                                                        (({
                                                            let ml_reference_value1541 =
                                                                get.borrow().clone();
                                                            ml_reference_value1541
                                                        })
                                                        .0)
                                                            .2,
                                                        (({
                                                            let ml_reference_value1542 =
                                                                get.borrow().clone();
                                                            ml_reference_value1542
                                                        })
                                                        .0)
                                                            .4,
                                                        (({
                                                            let ml_reference_value1543 =
                                                                get.borrow().clone();
                                                            ml_reference_value1543
                                                        })
                                                        .0)
                                                            .1,
                                                    )
                                                } else {
                                                    if (((({
                                                        let ml_reference_value1544 =
                                                            get.borrow().clone();
                                                        ml_reference_value1544
                                                    })
                                                    .0)
                                                        .0)
                                                        != (self::ml_string::Text::from_units(
                                                            vec![118, 97, 108, 117, 101],
                                                        )))
                                                    {
                                                        self::JavaScriptArraySearchTransition::ArraySearchInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 71, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                    } else {
                                                        self::continue_java_script_array_search_number(heap.clone(), {
                                                            let source_ml_source2_nominal_field27958 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field27960 = shared_constructor_field2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27962 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field27964 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field27966 = shared_constructor_field5;
                                                                            {
                                                                                let source_ml_source2_nominal_field27968 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field27970 = shared_constructor_field7;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field27972 = shared_constructor_field8;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field27974 = shared_constructor_field9.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field27976 = shared_constructor_field10.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field27978 = shared_constructor_field11.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field27980 = shared_constructor_field12.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_override27982 = (({ let ml_reference_value1545 = get.borrow().clone(); ml_reference_value1545 }).0).5;
                                                                                                            self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field27958.clone(), Box::new(source_ml_source2_nominal_field27960.clone()), Box::new(source_ml_source2_nominal_field27962.clone()), Box::new(source_ml_source2_nominal_field27964.clone()), source_ml_source2_nominal_field27966, Box::new(source_ml_source2_nominal_field27968.clone()), source_ml_source2_nominal_field27970, source_ml_source2_nominal_field27972, source_ml_source2_nominal_field27974.clone(), Box::new(source_ml_source2_nominal_override27982.clone()), source_ml_source2_nominal_field27978.clone(), source_ml_source2_nominal_field27980.clone())
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
                                                        }, self::begin_java_script_scalar_protocol(heap.clone(), (({ let ml_reference_value1546 = get.borrow().clone(); ml_reference_value1546 }).0).5, shared_constructor_field6.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (shared_constructor_field9
                                            != (self::ml_string::Text::from_units(vec![
                                                114, 101, 97, 100, 121,
                                            ])))
                                        {
                                            self::JavaScriptArraySearchTransition::ArraySearchInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 116, 116, 101, 114, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                        } else {
                                            if ((shared_constructor_field8 < (0f64))
                                                || (shared_constructor_field8
                                                    >= shared_constructor_field7))
                                            {
                                                self::JavaScriptArraySearchTransition::ArraySearchCompleted(Box::new(heap.clone()), Box::new(self::read_java_script_array_search_missing_value(state.clone())))
                                            } else {
                                                {
                                                    let name = {
                                                        let source_ml_source2_string_argument27993 =
                                                            shared_constructor_field8;
                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument27993))
                                                    };
                                                    {
                                                        let units = std::rc::Rc::new(
                                                            std::cell::RefCell::new(
                                                                Vec::<f64>::new(),
                                                            ),
                                                        );
                                                        {
                                                            let at = 0f64;
                                                            {
                                                                let source_ml_source2_s27994 = self::source_ml_source1_temporary_step_java_script_array_search_loop1238(&(name), units.clone(), at);
                                                                match source_ml_source2_s27994.clone() {
                                                                    self::MlSource1TemporaryStepJavaScriptArraySearchLoop1238Result::MlSource1TemporaryStepJavaScriptArraySearchLoop1238Done => {
                                                                        {
                                                                            let key = self::JavaScriptSemanticValue::Text(units.clone());
                                                                            {
                                                                                let source_ml_source2_s28030 = if ((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 105, 110, 100, 101, 120, 79, 102]))) || (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102])))) {
                                                                                    {
                                                                                        let presence = self::read_java_script_indexed_property_presence(heap.clone(), (((match shared_constructor_field2.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value28036, _) => {
                                                                                                Some((source_ml_source2_data_property_value28036.clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(self::ml::Big,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone(), key.clone());
                                                                                        if (((({ let ml_reference_value1547 = presence.borrow().clone(); ml_reference_value1547 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryStepJavaScriptArraySearchJoin1239::MlSource1TemporaryStepJavaScriptArraySearchJoin1239Return(Box::new(self::JavaScriptArraySearchTransition::ArraySearchInvalid(Box::new(heap.clone()), (({ let ml_reference_value1548 = presence.borrow().clone(); ml_reference_value1548 }).0).0)))
                                                                                        } else {
                                                                                            if !((({ let ml_reference_value1549 = presence.borrow().clone(); ml_reference_value1549 }).0).1) {
                                                                                                self::MlSource1TemporaryStepJavaScriptArraySearchJoin1239::MlSource1TemporaryStepJavaScriptArraySearchJoin1239Return(Box::new(self::JavaScriptArraySearchTransition::ArraySearchContinue(Box::new(heap.clone()), Box::new({
                                                                                                    let source_ml_source2_nominal_field28041 = shared_constructor_field1.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field28043 = shared_constructor_field2.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field28045 = shared_constructor_field3.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field28047 = shared_constructor_field4.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field28049 = shared_constructor_field5;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field28051 = shared_constructor_field6.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field28053 = shared_constructor_field7;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field28055 = shared_constructor_field8;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field28057 = shared_constructor_field9.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field28059 = shared_constructor_field10.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field28061 = shared_constructor_field11.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field28063 = shared_constructor_field12.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_override28065 = (shared_constructor_field8 + if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 108, 97, 115, 116, 73, 110, 100, 101, 120, 79, 102]))) {
                                                                                                                                                        (-1f64)
                                                                                                                                                    } else {
                                                                                                                                                        1f64
                                                                                                                                                    });
                                                                                                                                                    self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field28041.clone(), Box::new(source_ml_source2_nominal_field28043.clone()), Box::new(source_ml_source2_nominal_field28045.clone()), Box::new(source_ml_source2_nominal_field28047.clone()), source_ml_source2_nominal_field28049, Box::new(source_ml_source2_nominal_field28051.clone()), source_ml_source2_nominal_field28053, source_ml_source2_nominal_override28065, source_ml_source2_nominal_field28057.clone(), Box::new(source_ml_source2_nominal_field28059.clone()), source_ml_source2_nominal_field28061.clone(), source_ml_source2_nominal_field28063.clone())
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
                                                                                                }))))
                                                                                            } else {
                                                                                                self::MlSource1TemporaryStepJavaScriptArraySearchJoin1239::MlSource1TemporaryStepJavaScriptArraySearchJoin1239Next
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryStepJavaScriptArraySearchJoin1239::MlSource1TemporaryStepJavaScriptArraySearchJoin1239Next
                                                                                };
                                                                                match source_ml_source2_s28030.clone() {
                                                                                    self::MlSource1TemporaryStepJavaScriptArraySearchJoin1239::MlSource1TemporaryStepJavaScriptArraySearchJoin1239Next => {
                                                                                        {
                                                                                            let get_2 = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field2.clone() {
                                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value28067, _) => {
                                                                                                    Some((source_ml_source2_data_property_value28067.clone(),))
                                                                                                }
                                                                                                _ => {
                                                                                                    None::<(self::ml::Big,)>
                                                                                                }
                                                                                            }).clone().expect("a proven present value")).0).clone(), key.clone(), shared_constructor_field2.clone());
                                                                                            if (((({ let ml_reference_value1550 = get_2.borrow().clone(); ml_reference_value1550 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::JavaScriptArraySearchTransition::ArraySearchInvalid(Box::new(heap.clone()), (({ let ml_reference_value1551 = get_2.borrow().clone(); ml_reference_value1551 }).0).3)
                                                                                            } else {
                                                                                                if (((({ let ml_reference_value1552 = get_2.borrow().clone(); ml_reference_value1552 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                                                    self::request_java_script_array_search_call(heap.clone(), {
                                                                                                        let source_ml_source2_nominal_field28070 = shared_constructor_field1.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field28072 = shared_constructor_field2.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field28074 = shared_constructor_field3.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field28076 = shared_constructor_field4.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field28078 = shared_constructor_field5;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field28080 = shared_constructor_field6.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field28082 = shared_constructor_field7;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field28084 = shared_constructor_field8;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field28086 = shared_constructor_field9.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field28088 = shared_constructor_field10.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field28090 = shared_constructor_field11.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field28092 = shared_constructor_field12.clone();
                                                                                                                                                    self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field28070.clone(), Box::new(source_ml_source2_nominal_field28072.clone()), Box::new(source_ml_source2_nominal_field28074.clone()), Box::new(source_ml_source2_nominal_field28076.clone()), source_ml_source2_nominal_field28078, Box::new(source_ml_source2_nominal_field28080.clone()), source_ml_source2_nominal_field28082, source_ml_source2_nominal_field28084, self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), Box::new(source_ml_source2_nominal_field28088.clone()), source_ml_source2_nominal_field28090.clone(), source_ml_source2_nominal_field28092.clone())
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
                                                                                                    }, (({ let ml_reference_value1553 = get_2.borrow().clone(); ml_reference_value1553 }).0).2, (({ let ml_reference_value1554 = get_2.borrow().clone(); ml_reference_value1554 }).0).4, (({ let ml_reference_value1555 = get_2.borrow().clone(); ml_reference_value1555 }).0).1)
                                                                                                } else {
                                                                                                    if (((({ let ml_reference_value1556 = get_2.borrow().clone(); ml_reference_value1556 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                                                        self::JavaScriptArraySearchTransition::ArraySearchInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 101, 108, 101, 109, 101, 110, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 71, 101, 116, 32, 114, 101, 115, 117, 108, 116]))
                                                                                                    } else {
                                                                                                        self::complete_java_script_array_search_element(heap.clone(), state.clone(), (({ let ml_reference_value1557 = get_2.borrow().clone(); ml_reference_value1557 }).0).5)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryStepJavaScriptArraySearchJoin1239::MlSource1TemporaryStepJavaScriptArraySearchJoin1239Return(source_ml_source1_temporary_result1239) => {
                                                                                        let source_ml_source1_temporary_result1239 = *source_ml_source1_temporary_result1239;
                                                                                        source_ml_source1_temporary_result1239.clone()
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
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_array_search(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySearchState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArraySearchTransition {
    match state.clone() {
        self::JavaScriptArraySearchState::ArraySearchState(
            _,
            _,
            _,
            _,
            _,
            shared_constructor_field13,
            _,
            _,
            shared_constructor_field14,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field13 = *shared_constructor_field13;
            {
                let error =
                    self::read_java_script_array_search_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArraySearchTransition::ArraySearchInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if match (
                        match state.clone() {
                            self::JavaScriptArraySearchState::ArraySearchState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value28095,
                            ) => ((source_ml_source2_data_property_value28095
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        self::JavaScriptArraySearchTransition::ArraySearchInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 114,
                                101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97,
                                105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 104, 105, 108,
                                100,
                            ]),
                        )
                    } else {
                        {
                            let invalid = self::read_java_script_ordinary_value_error(heap.clone(), match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(source_ml_source2_data_property_value28096) => {
                                    let source_ml_source2_data_property_value28096 = *source_ml_source2_data_property_value28096;
                                    source_ml_source2_data_property_value28096.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(source_ml_source2_data_property_value28097) => {
                                    let source_ml_source2_data_property_value28097 = *source_ml_source2_data_property_value28097;
                                    source_ml_source2_data_property_value28097.clone()
                                }
                            });
                            if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                self::JavaScriptArraySearchTransition::ArraySearchInvalid(
                                    Box::new(heap.clone()),
                                    invalid.clone(),
                                )
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        {
                                            let prior_call = match state.clone() {
                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28098) => {
                                                    ((source_ml_source2_data_property_value28098.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,))).0).clone()
                                                }
                                            };
                                            {
                                                let base = match state.clone() {
                                                    self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_rest_field28099, source_ml_source2_nominal_rest_field28100, source_ml_source2_nominal_rest_field28101, source_ml_source2_nominal_rest_field28102, source_ml_source2_nominal_rest_field28103, source_ml_source2_nominal_rest_field28104, source_ml_source2_nominal_rest_field28105, source_ml_source2_nominal_rest_field28106, source_ml_source2_nominal_rest_field28107, source_ml_source2_nominal_rest_field28108, source_ml_source2_nominal_rest_field28109, _) => {
                                                        let source_ml_source2_nominal_rest_field28100 = *source_ml_source2_nominal_rest_field28100;
                                                        let source_ml_source2_nominal_rest_field28101 = *source_ml_source2_nominal_rest_field28101;
                                                        let source_ml_source2_nominal_rest_field28102 = *source_ml_source2_nominal_rest_field28102;
                                                        let source_ml_source2_nominal_rest_field28104 = *source_ml_source2_nominal_rest_field28104;
                                                        let source_ml_source2_nominal_rest_field28108 = *source_ml_source2_nominal_rest_field28108;
                                                        self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_rest_field28099.clone(), Box::new(source_ml_source2_nominal_rest_field28100.clone()), Box::new(source_ml_source2_nominal_rest_field28101.clone()), Box::new(source_ml_source2_nominal_rest_field28102.clone()), source_ml_source2_nominal_rest_field28103, Box::new(source_ml_source2_nominal_rest_field28104.clone()), source_ml_source2_nominal_rest_field28105, source_ml_source2_nominal_rest_field28106, source_ml_source2_nominal_rest_field28107.clone(), Box::new(source_ml_source2_nominal_rest_field28108.clone()), source_ml_source2_nominal_rest_field28109.clone(), None::<(Option<self::JavaScriptArraySearchCall>,)>)
                                                    }
                                                };
                                                match base.clone() {
                                                    self::JavaScriptArraySearchState::ArraySearchState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12) => {
                                                        let shared_constructor_field2 = *shared_constructor_field2;
                                                        let shared_constructor_field3 = *shared_constructor_field3;
                                                        let shared_constructor_field4 = *shared_constructor_field4;
                                                        let shared_constructor_field6 = *shared_constructor_field6;
                                                        let shared_constructor_field10 = *shared_constructor_field10;
                                                        if (shared_constructor_field14 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]))) {
                                                            self::continue_java_script_array_search_number(heap.clone(), {
                                                                let source_ml_source2_nominal_field28113 = shared_constructor_field1.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field28115 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28117 = shared_constructor_field3.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field28119 = shared_constructor_field4.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field28121 = shared_constructor_field5;
                                                                                {
                                                                                    let source_ml_source2_nominal_field28123 = shared_constructor_field6.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field28125 = shared_constructor_field7;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field28127 = shared_constructor_field8;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field28129 = shared_constructor_field9.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field28131 = shared_constructor_field10.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field28133 = shared_constructor_field11.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field28135 = shared_constructor_field12.clone();
                                                                                                            self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field28113.clone(), Box::new(source_ml_source2_nominal_field28115.clone()), Box::new(source_ml_source2_nominal_field28117.clone()), Box::new(source_ml_source2_nominal_field28119.clone()), source_ml_source2_nominal_field28121, Box::new(source_ml_source2_nominal_field28123.clone()), source_ml_source2_nominal_field28125, source_ml_source2_nominal_field28127, source_ml_source2_nominal_field28129.clone(), Box::new(value.clone()), source_ml_source2_nominal_field28133.clone(), source_ml_source2_nominal_field28135.clone())
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
                                                            }, self::begin_java_script_scalar_protocol(heap.clone(), value.clone(), shared_constructor_field13.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                        } else {
                                                            if ((shared_constructor_field14 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) || (shared_constructor_field14 == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])))) {
                                                                if match (match state.clone() {
                                                                    self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28143, _) => {
                                                                        ((source_ml_source2_data_property_value28143.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                    }
                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                                                    self::JavaScriptArraySearchTransition::ArraySearchInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 104, 105, 108, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 115, 116, 97, 116, 101]))
                                                                } else {
                                                                    self::continue_java_script_array_search_number(heap.clone(), base.clone(), self::resume_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                                                        self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28144, _) => {
                                                                            ((source_ml_source2_data_property_value28144.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value"), completion.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                                }
                                                            } else {
                                                                if (shared_constructor_field14 == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]))) {
                                                                    self::complete_java_script_array_search_element(heap.clone(), base.clone(), value.clone())
                                                                } else {
                                                                    self::JavaScriptArraySearchTransition::ArraySearchInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101, 115, 117, 109, 112, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 112, 104, 97, 115, 101]))
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::JavaScriptArraySearchTransition::ArraySearchThrow(Box::new(heap.clone()), Box::new(value_2.clone()))
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
pub fn test_java_script_array_callback_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_array_callback_operation(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_callback_reverse(operation: &self::ml_string::Text) -> bool {
    ((((*operation)
        == (self::ml_string::Text::from_units(vec![
            65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 76, 97, 115, 116,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 76, 97, 115, 116, 73, 110, 100, 101,
                120,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 46, 114, 101, 100, 117, 99, 101, 82, 105, 103, 104, 116,
            ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_callback_visit_holes(operation: &self::ml_string::Text) -> bool {
    (((((*operation)
        == (self::ml_string::Text::from_units(vec![
            65, 114, 114, 97, 121, 46, 102, 105, 110, 100,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 73, 110, 100, 101, 120,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 76, 97, 115, 116,
            ]))))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 76, 97, 115, 116, 73, 110, 100, 101,
                120,
            ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_callback_reduction(operation: &self::ml_string::Text) -> bool {
    (((*operation)
        == (self::ml_string::Text::from_units(vec![
            65, 114, 114, 97, 121, 46, 114, 101, 100, 117, 99, 101,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 46, 114, 101, 100, 117, 99, 101, 82, 105, 103, 104, 116,
            ]))))
}
