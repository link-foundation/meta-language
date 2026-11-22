// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCallbackCall;
use super::JavaScriptArrayCallbackState;
use super::JavaScriptArrayCallbackTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251;
use super::MlSource1TemporaryStepJavaScriptArrayCallbackLoop1250Result;
use super::begin_java_script_array_callback_element;
use super::begin_java_script_scalar_protocol;
use super::continue_java_script_array_callback_length;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_number;
use super::ml_string;
use super::plan_java_script_ordinary_get;
use super::read_java_script_array_callback_empty_result;
use super::read_java_script_array_callback_property_presence;
use super::read_java_script_array_callback_state_error;
use super::request_java_script_array_callback_call;
use super::source_ml_source1_temporary_step_java_script_array_callback_loop1250;
use super::step_java_script_scalar_protocol;
use super::test_java_script_array_callback_reduction;
use super::test_java_script_array_callback_reverse;
use super::test_java_script_array_callback_visit_holes;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_callback(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCallbackState,
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
            {
                let error =
                    self::read_java_script_array_callback_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptArrayCallbackState::ArrayCallbackState(
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
                                _,
                                _,
                                source_ml_source2_data_property_value28634,
                            ) => ((source_ml_source2_data_property_value28634
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        self::JavaScriptArrayCallbackTransition::ArrayCallbackCall(Box::new(heap.clone()), Box::new(state.clone()), Box::new(match (match state.clone() {
                            self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28635) => {
                                ((source_ml_source2_data_property_value28635.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(source_ml_source2_data_property_value28636, _, _) => {
                                let source_ml_source2_data_property_value28636 = *source_ml_source2_data_property_value28636;
                                source_ml_source2_data_property_value28636.clone()
                            }
                        }), Box::new(match (match state.clone() {
                            self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28637) => {
                                ((source_ml_source2_data_property_value28637.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(_, source_ml_source2_data_property_value28638, _) => {
                                let source_ml_source2_data_property_value28638 = *source_ml_source2_data_property_value28638;
                                source_ml_source2_data_property_value28638.clone()
                            }
                        }), match (match state.clone() {
                            self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28639) => {
                                ((source_ml_source2_data_property_value28639.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayCallbackCall>,))).0).clone()
                            }
                        }).clone().expect("a proven present value") {
                            self::JavaScriptArrayCallbackCall::ArrayCallbackSelectedCall(_, _, source_ml_source2_data_property_value28640) => {
                                source_ml_source2_data_property_value28640.clone()
                            }
                        })
                    } else {
                        if (shared_constructor_field8
                            == (self::ml_string::Text::from_units(vec![
                                108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115,
                                105, 111, 110,
                            ])))
                        {
                            if match (
                                match state.clone() {
                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(
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
                                        _,
                                        source_ml_source2_data_property_value28643,
                                        _,
                                    ) => ((source_ml_source2_data_property_value28643
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
                                self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104,
                                        32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                        114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                                        97, 99, 116, 117, 97, 108, 32, 112, 101, 110, 100, 105,
                                        110, 103, 32, 115, 116, 97, 116, 101,
                                    ]),
                                )
                            } else {
                                self::continue_java_script_array_callback_length(heap.clone(), state.clone(), self::step_java_script_scalar_protocol(heap.clone(), (match state.clone() {
                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(_, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value28644, _) => {
                                        ((source_ml_source2_data_property_value28644.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value"), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                            }
                        } else {
                            if (shared_constructor_field8
                                == (self::ml_string::Text::from_units(vec![
                                    108, 101, 110, 103, 116, 104, 32, 112, 114, 105, 109, 105, 116,
                                    105, 118, 101,
                                ])))
                            {
                                self::continue_java_script_array_callback_length(
                                    heap.clone(),
                                    state.clone(),
                                    self::finish_java_script_scalar_primitive(
                                        &(self::ml_string::Text::from_units(vec![
                                            99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109,
                                            105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101,
                                            114,
                                        ])),
                                        shared_constructor_field12.clone(),
                                    ),
                                )
                            } else {
                                if {
                                    let source_ml_source2_s28649 =
                                        shared_constructor_field2.clone();
                                    match source_ml_source2_s28649.clone() {
                                        self::JavaScriptSemanticValue::Object(_, _) => false,
                                        _ => true,
                                    }
                                } {
                                    self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97,
                                            99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116,
                                            104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32,
                                            98, 111, 120, 101, 100, 32, 79, 98, 106, 101, 99, 116,
                                            32, 114, 101, 99, 101, 105, 118, 101, 114,
                                        ]),
                                    )
                                } else {
                                    if (shared_constructor_field8
                                        == (self::ml_string::Text::from_units(vec![
                                            108, 101, 110, 103, 116, 104,
                                        ])))
                                    {
                                        {
                                            let get = self::plan_java_script_ordinary_get(
                                                heap.clone(),
                                                (((match shared_constructor_field2.clone() {
                                                    self::JavaScriptSemanticValue::Object(
                                                        source_ml_source2_data_property_value28656,
                                                        _,
                                                    ) => Some((
                                                        source_ml_source2_data_property_value28656
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
                                                let ml_reference_value1567 = get.borrow().clone();
                                                ml_reference_value1567
                                            })
                                            .0)
                                                .3)
                                                != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(Box::new(heap.clone()), (({ let ml_reference_value1568 = get.borrow().clone(); ml_reference_value1568 }).0).3)
                                            } else {
                                                if (((({
                                                    let ml_reference_value1569 =
                                                        get.borrow().clone();
                                                    ml_reference_value1569
                                                })
                                                .0)
                                                    .0)
                                                    == (self::ml_string::Text::from_units(vec![
                                                        99, 97, 108, 108,
                                                    ])))
                                                {
                                                    self::request_java_script_array_callback_call(
                                                        heap.clone(),
                                                        {
                                                            let source_ml_source2_nominal_field28659 =
                                                                shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field28661 =
                                                                    shared_constructor_field2
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field28663 =
                                                                        shared_constructor_field3
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28665 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field28667 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field28669 = shared_constructor_field6;
                                                                                {
                                                                                    let source_ml_source2_nominal_field28671 = shared_constructor_field7;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field28673 = shared_constructor_field8.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field28675 = shared_constructor_field9;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field28677 = shared_constructor_field10;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field28679 = shared_constructor_field11.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field28681 = shared_constructor_field12.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field28683 = shared_constructor_field13.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field28685 = shared_constructor_field14.clone();
                                                                                                                self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28659.clone(), Box::new(source_ml_source2_nominal_field28661.clone()), Box::new(source_ml_source2_nominal_field28663.clone()), Box::new(source_ml_source2_nominal_field28665.clone()), Box::new(source_ml_source2_nominal_field28667.clone()), source_ml_source2_nominal_field28669, source_ml_source2_nominal_field28671, self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), source_ml_source2_nominal_field28675, source_ml_source2_nominal_field28677, Box::new(source_ml_source2_nominal_field28679.clone()), Box::new(source_ml_source2_nominal_field28681.clone()), source_ml_source2_nominal_field28683.clone(), source_ml_source2_nominal_field28685.clone())
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
                                                        (({
                                                            let ml_reference_value1570 =
                                                                get.borrow().clone();
                                                            ml_reference_value1570
                                                        })
                                                        .0)
                                                            .2,
                                                        (({
                                                            let ml_reference_value1571 =
                                                                get.borrow().clone();
                                                            ml_reference_value1571
                                                        })
                                                        .0)
                                                            .4,
                                                        (({
                                                            let ml_reference_value1572 =
                                                                get.borrow().clone();
                                                            ml_reference_value1572
                                                        })
                                                        .0)
                                                            .1,
                                                    )
                                                } else {
                                                    if (((({
                                                        let ml_reference_value1573 =
                                                            get.borrow().clone();
                                                        ml_reference_value1573
                                                    })
                                                    .0)
                                                        .0)
                                                        != (self::ml_string::Text::from_units(
                                                            vec![118, 97, 108, 117, 101],
                                                        )))
                                                    {
                                                        self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 108, 101, 110, 103, 116, 104, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 71, 101, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                    } else {
                                                        self::continue_java_script_array_callback_length(heap.clone(), {
                                                            let source_ml_source2_nominal_field28689 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field28691 = shared_constructor_field2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field28693 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field28695 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field28697 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field28699 = shared_constructor_field6;
                                                                                {
                                                                                    let source_ml_source2_nominal_field28701 = shared_constructor_field7;
                                                                                    {
                                                                                        let source_ml_source2_nominal_field28703 = shared_constructor_field8.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field28705 = shared_constructor_field9;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field28707 = shared_constructor_field10;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field28709 = shared_constructor_field11.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field28711 = shared_constructor_field12.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field28713 = shared_constructor_field13.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field28715 = shared_constructor_field14.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override28717 = (({ let ml_reference_value1574 = get.borrow().clone(); ml_reference_value1574 }).0).5;
                                                                                                                    self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28689.clone(), Box::new(source_ml_source2_nominal_field28691.clone()), Box::new(source_ml_source2_nominal_field28693.clone()), Box::new(source_ml_source2_nominal_field28695.clone()), Box::new(source_ml_source2_nominal_field28697.clone()), source_ml_source2_nominal_field28699, source_ml_source2_nominal_field28701, source_ml_source2_nominal_field28703.clone(), source_ml_source2_nominal_field28705, source_ml_source2_nominal_field28707, Box::new(source_ml_source2_nominal_field28709.clone()), Box::new(source_ml_source2_nominal_override28717.clone()), source_ml_source2_nominal_field28713.clone(), source_ml_source2_nominal_field28715.clone())
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
                                                        }, self::begin_java_script_scalar_protocol(heap.clone(), (({ let ml_reference_value1575 = get.borrow().clone(); ml_reference_value1575 }).0).5, shared_constructor_field5.clone(), &(self::ml_string::Text::from_units(vec![99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114]))))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (shared_constructor_field8
                                            != (self::ml_string::Text::from_units(vec![
                                                114, 101, 97, 100, 121,
                                            ])))
                                        {
                                            self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 115, 117, 109, 101, 115, 32, 111, 110, 108, 121, 32, 97, 102, 116, 101, 114, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                        } else {
                                            if ((shared_constructor_field7 < (0f64))
                                                || (shared_constructor_field7
                                                    >= shared_constructor_field6))
                                            {
                                                if (self::test_java_script_array_callback_reduction(
                                                    &(shared_constructor_field1),
                                                ) && !shared_constructor_field10)
                                                {
                                                    self::JavaScriptArrayCallbackTransition::ArrayCallbackError(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), self::ml_string::Text::from_units(vec![82, 101, 100, 117, 99, 101, 32, 111, 102, 32, 101, 109, 112, 116, 121, 32, 97, 114, 114, 97, 121, 32, 119, 105, 116, 104, 32, 110, 111, 32, 105, 110, 105, 116, 105, 97, 108, 32, 118, 97, 108, 117, 101]))
                                                } else {
                                                    self::JavaScriptArrayCallbackTransition::ArrayCallbackCompleted(Box::new(heap.clone()), Box::new(self::read_java_script_array_callback_empty_result(state.clone())))
                                                }
                                            } else {
                                                {
                                                    let name = {
                                                        let source_ml_source2_string_argument28730 =
                                                            shared_constructor_field7;
                                                        self::ml_string::Text::from_text(&self::ml_number::js_number(source_ml_source2_string_argument28730))
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
                                                                let source_ml_source2_s28731 = self::source_ml_source1_temporary_step_java_script_array_callback_loop1250(&(name), units.clone(), at);
                                                                match source_ml_source2_s28731.clone() {
                                                                    self::MlSource1TemporaryStepJavaScriptArrayCallbackLoop1250Result::MlSource1TemporaryStepJavaScriptArrayCallbackLoop1250Done => {
                                                                        {
                                                                            let key = self::JavaScriptSemanticValue::Text(units.clone());
                                                                            {
                                                                                let source_ml_source2_s28767 = if !(self::test_java_script_array_callback_visit_holes(&(shared_constructor_field1))) {
                                                                                    {
                                                                                        let presence = self::read_java_script_array_callback_property_presence(heap.clone(), (((match shared_constructor_field2.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value28770, _) => {
                                                                                                Some((source_ml_source2_data_property_value28770.clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(self::ml::Big,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone(), key.clone());
                                                                                        if (((({ let ml_reference_value1576 = presence.borrow().clone(); ml_reference_value1576 }).0).0) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                            self::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251Return(Box::new(self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(Box::new(heap.clone()), (({ let ml_reference_value1577 = presence.borrow().clone(); ml_reference_value1577 }).0).0)))
                                                                                        } else {
                                                                                            if !((({ let ml_reference_value1578 = presence.borrow().clone(); ml_reference_value1578 }).0).1) {
                                                                                                self::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251Return(Box::new(self::JavaScriptArrayCallbackTransition::ArrayCallbackContinue(Box::new(heap.clone()), Box::new({
                                                                                                    let source_ml_source2_nominal_field28774 = shared_constructor_field1.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field28776 = shared_constructor_field2.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field28778 = shared_constructor_field3.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field28780 = shared_constructor_field4.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field28782 = shared_constructor_field5.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field28784 = shared_constructor_field6;
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field28786 = shared_constructor_field7;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field28788 = shared_constructor_field8.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field28790 = shared_constructor_field9;
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field28792 = shared_constructor_field10;
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field28794 = shared_constructor_field11.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field28796 = shared_constructor_field12.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field28798 = shared_constructor_field13.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field28800 = shared_constructor_field14.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_override28802 = (shared_constructor_field7 + if self::test_java_script_array_callback_reverse(&(shared_constructor_field1)) {
                                                                                                                                                                (-1f64)
                                                                                                                                                            } else {
                                                                                                                                                                1f64
                                                                                                                                                            });
                                                                                                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28774.clone(), Box::new(source_ml_source2_nominal_field28776.clone()), Box::new(source_ml_source2_nominal_field28778.clone()), Box::new(source_ml_source2_nominal_field28780.clone()), Box::new(source_ml_source2_nominal_field28782.clone()), source_ml_source2_nominal_field28784, source_ml_source2_nominal_override28802, source_ml_source2_nominal_field28788.clone(), source_ml_source2_nominal_field28790, source_ml_source2_nominal_field28792, Box::new(source_ml_source2_nominal_field28794.clone()), Box::new(source_ml_source2_nominal_field28796.clone()), source_ml_source2_nominal_field28798.clone(), source_ml_source2_nominal_field28800.clone())
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
                                                                                                }))))
                                                                                            } else {
                                                                                                self::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251Next
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    self::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251Next
                                                                                };
                                                                                match source_ml_source2_s28767.clone() {
                                                                                    self::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251Next => {
                                                                                        {
                                                                                            let get_2 = self::plan_java_script_ordinary_get(heap.clone(), (((match shared_constructor_field2.clone() {
                                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value28804, _) => {
                                                                                                    Some((source_ml_source2_data_property_value28804.clone(),))
                                                                                                }
                                                                                                _ => {
                                                                                                    None::<(self::ml::Big,)>
                                                                                                }
                                                                                            }).clone().expect("a proven present value")).0).clone(), key.clone(), shared_constructor_field2.clone());
                                                                                            if (((({ let ml_reference_value1579 = get_2.borrow().clone(); ml_reference_value1579 }).0).3) != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(Box::new(heap.clone()), (({ let ml_reference_value1580 = get_2.borrow().clone(); ml_reference_value1580 }).0).3)
                                                                                            } else {
                                                                                                if (((({ let ml_reference_value1581 = get_2.borrow().clone(); ml_reference_value1581 }).0).0) == (self::ml_string::Text::from_units(vec![99, 97, 108, 108]))) {
                                                                                                    self::request_java_script_array_callback_call(heap.clone(), {
                                                                                                        let source_ml_source2_nominal_field28807 = shared_constructor_field1.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field28809 = shared_constructor_field2.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field28811 = shared_constructor_field3.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field28813 = shared_constructor_field4.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field28815 = shared_constructor_field5.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field28817 = shared_constructor_field6;
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field28819 = shared_constructor_field7;
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field28821 = shared_constructor_field8.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field28823 = shared_constructor_field9;
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field28825 = shared_constructor_field10;
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field28827 = shared_constructor_field11.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field28829 = shared_constructor_field12.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field28831 = shared_constructor_field13.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field28833 = shared_constructor_field14.clone();
                                                                                                                                                            self::JavaScriptArrayCallbackState::ArrayCallbackState(source_ml_source2_nominal_field28807.clone(), Box::new(source_ml_source2_nominal_field28809.clone()), Box::new(source_ml_source2_nominal_field28811.clone()), Box::new(source_ml_source2_nominal_field28813.clone()), Box::new(source_ml_source2_nominal_field28815.clone()), source_ml_source2_nominal_field28817, source_ml_source2_nominal_field28819, self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), source_ml_source2_nominal_field28823, source_ml_source2_nominal_field28825, Box::new(source_ml_source2_nominal_field28827.clone()), Box::new(source_ml_source2_nominal_field28829.clone()), source_ml_source2_nominal_field28831.clone(), source_ml_source2_nominal_field28833.clone())
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
                                                                                                    }, (({ let ml_reference_value1582 = get_2.borrow().clone(); ml_reference_value1582 }).0).2, (({ let ml_reference_value1583 = get_2.borrow().clone(); ml_reference_value1583 }).0).4, (({ let ml_reference_value1584 = get_2.borrow().clone(); ml_reference_value1584 }).0).1)
                                                                                                } else {
                                                                                                    if (((({ let ml_reference_value1585 = get_2.borrow().clone(); ml_reference_value1585 }).0).0) != (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]))) {
                                                                                                        self::JavaScriptArrayCallbackTransition::ArrayCallbackInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 101, 108, 101, 109, 101, 110, 116, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 71, 101, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110]))
                                                                                                    } else {
                                                                                                        self::begin_java_script_array_callback_element(heap.clone(), state.clone(), (({ let ml_reference_value1586 = get_2.borrow().clone(); ml_reference_value1586 }).0).5)
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                    self::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251::MlSource1TemporaryStepJavaScriptArrayCallbackJoin1251Return(source_ml_source1_temporary_result1251) => {
                                                                                        let source_ml_source1_temporary_result1251 = *source_ml_source1_temporary_result1251;
                                                                                        source_ml_source1_temporary_result1251.clone()
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
