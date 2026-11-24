// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayCreationCall;
use super::JavaScriptArrayCreationState;
use super::JavaScriptArrayCreationTransition;
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::accept_java_script_array_creation_get;
use super::accept_java_script_array_creation_iterator;
use super::accept_java_script_array_creation_length;
use super::accept_java_script_array_creation_length_set;
use super::clear_java_script_array_creation_child;
use super::create_java_script_iterator_protocol_state;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_creation_state_error;
use super::read_java_script_ordinary_value_error;
use super::request_java_script_iterator_protocol_call;
use super::resume_java_script_array_assignment;
use super::resume_java_script_iterator_protocol_call;
use super::resume_java_script_scalar_protocol;
use super::throw_java_script_array_creation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_array_creation_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArrayCreationTransition {
    match state.clone() {
        self::JavaScriptArrayCreationState::ArrayCreationState(
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
            shared_constructor_field31,
            _,
            _,
            _,
            _,
            shared_constructor_field32,
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
        ) => {
            let shared_constructor_field32 = *shared_constructor_field32;
            {
                let error = {
                    let source_ml_source2_logical16878 =
                        self::read_java_script_array_creation_state_error(
                            heap.clone(),
                            state.clone(),
                        );
                    if !source_ml_source2_logical16878.0.is_empty() {
                        source_ml_source2_logical16878.clone()
                    } else {
                        self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                    source_ml_source2_data_property_value16876,
                                ) => {
                                    let source_ml_source2_data_property_value16876 =
                                        *source_ml_source2_data_property_value16876;
                                    source_ml_source2_data_property_value16876.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                    source_ml_source2_data_property_value16877,
                                ) => {
                                    let source_ml_source2_data_property_value16877 =
                                        *source_ml_source2_data_property_value16877;
                                    source_ml_source2_data_property_value16877.clone()
                                }
                            },
                        )
                    }
                };
                if ((error != (self::ml_string::Text::from_units(vec![])))
                    || match (
                        match state.clone() {
                            self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                source_ml_source2_data_property_value16879,
                                _,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value16879
                                .clone()
                                .unwrap_or_else(|| (None::<self::JavaScriptArrayCreationCall>,)))
                            .0)
                                .clone(),
                        },
                        None::<()>,
                    ) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    })
                {
                    self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                        Box::new(heap.clone()),
                        if !error.0.is_empty() {
                            error.clone()
                        } else {
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110,
                                32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                                99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100,
                                32, 99, 97, 108, 108, 98, 97, 99, 107,
                            ])
                        },
                    )
                } else {
                    if !(match (
                        match state.clone() {
                            self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value16881,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value16881
                                .clone()
                                .unwrap_or_else(|| {
                                    (None::<self::JavaScriptIteratorProtocolState>,)
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
                        self::accept_java_script_array_creation_iterator(
                            state.clone(),
                            self::resume_java_script_iterator_protocol_call(
                                heap.clone(),
                                (match state.clone() {
                                    self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value16882,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value16882
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptIteratorProtocolState>,)
                                        }))
                                    .0)
                                        .clone(),
                                })
                                .clone()
                                .expect("a proven present value"),
                                completion.clone(),
                            ),
                        )
                    } else {
                        if !(match (
                            match state.clone() {
                                self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value16883,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => ((source_ml_source2_data_property_value16883
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
                        }) {
                            self::accept_java_script_array_creation_length(
                                heap.clone(),
                                state.clone(),
                                self::resume_java_script_scalar_protocol(
                                    heap.clone(),
                                    (match state.clone() {
                                        self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value16884,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                        ) => ((source_ml_source2_data_property_value16884
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptPrimitiveConversionState>,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .expect("a proven present value"),
                                    completion.clone(),
                                    &(self::ml_string::Text::from_units(vec![
                                        99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105,
                                        116, 105, 118, 101, 32, 110, 117, 109, 98, 101, 114,
                                    ])),
                                ),
                            )
                        } else {
                            if !(match (
                                match state.clone() {
                                    self::JavaScriptArrayCreationState::ArrayCreationState(
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
                                        source_ml_source2_data_property_value16885,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => ((source_ml_source2_data_property_value16885
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptArrayAssignmentState>,)
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
                                self::accept_java_script_array_creation_length_set(state.clone(), self::resume_java_script_array_assignment(heap.clone(), (match state.clone() {
                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value16886, _, _, _, _) => {
                                        ((source_ml_source2_data_property_value16886.clone().unwrap_or_else(|| (None::<self::JavaScriptArrayAssignmentState>,))).0).clone()
                                    }
                                }).clone().expect("a proven present value"), completion.clone()))
                            } else {
                                match completion.clone() {
                                    self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                                        let value = *value;
                                        if (shared_constructor_field31 == (self::ml_string::Text::from_units(vec![109, 97, 112]))) {
                                            self::JavaScriptArrayCreationTransition::ArrayCreationContinue(Box::new(heap.clone()), Box::new({
                                                let source_ml_source2_nominal_spread16889 = self::clear_java_script_array_creation_child(state.clone());
                                                match source_ml_source2_nominal_spread16889.clone() {
                                                    self::JavaScriptArrayCreationState::ArrayCreationState(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5, shared_constructor_field6, shared_constructor_field7, shared_constructor_field8, shared_constructor_field9, shared_constructor_field10, shared_constructor_field11, shared_constructor_field12, shared_constructor_field13, shared_constructor_field14, shared_constructor_field15, shared_constructor_field16, shared_constructor_field17, shared_constructor_field18, shared_constructor_field19, shared_constructor_field20, shared_constructor_field21, shared_constructor_field22, shared_constructor_field23, shared_constructor_field24, shared_constructor_field25, shared_constructor_field26, shared_constructor_field27, shared_constructor_field28, shared_constructor_field29, shared_constructor_field30) => {
                                                        let shared_constructor_field2 = *shared_constructor_field2;
                                                        let shared_constructor_field4 = *shared_constructor_field4;
                                                        let shared_constructor_field5 = *shared_constructor_field5;
                                                        let shared_constructor_field6 = *shared_constructor_field6;
                                                        let shared_constructor_field7 = *shared_constructor_field7;
                                                        let shared_constructor_field8 = *shared_constructor_field8;
                                                        let shared_constructor_field9 = *shared_constructor_field9;
                                                        let shared_constructor_field15 = *shared_constructor_field15;
                                                        let shared_constructor_field16 = *shared_constructor_field16;
                                                        let shared_constructor_field17 = *shared_constructor_field17;
                                                        let shared_constructor_field18 = *shared_constructor_field18;
                                                        let shared_constructor_field19 = *shared_constructor_field19;
                                                        {
                                                            let source_ml_source2_nominal_field16890 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16892 = shared_constructor_field2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16894 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field16896 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field16898 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field16900 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field16902 = shared_constructor_field7.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16904 = shared_constructor_field8.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16906 = shared_constructor_field9.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16908 = shared_constructor_field10;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16910 = shared_constructor_field11;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16912 = shared_constructor_field12.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field16914 = shared_constructor_field13;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field16916 = shared_constructor_field14;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field16918 = shared_constructor_field15.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field16920 = shared_constructor_field16.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field16922 = shared_constructor_field17.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field16924 = shared_constructor_field18.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field16926 = shared_constructor_field19.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field16928 = shared_constructor_field20.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field16930 = shared_constructor_field21.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field16932 = shared_constructor_field22.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field16934 = shared_constructor_field23.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field16936 = shared_constructor_field24.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field16938 = shared_constructor_field25.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_field16940 = shared_constructor_field26.clone();
                                                                                                                                                                {
                                                                                                                                                                    let source_ml_source2_nominal_field16942 = shared_constructor_field27.clone();
                                                                                                                                                                    {
                                                                                                                                                                        let source_ml_source2_nominal_field16944 = shared_constructor_field28.clone();
                                                                                                                                                                        {
                                                                                                                                                                            let source_ml_source2_nominal_field16946 = shared_constructor_field29.clone();
                                                                                                                                                                            {
                                                                                                                                                                                let source_ml_source2_nominal_field16948 = shared_constructor_field30.clone();
                                                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16890.clone(), Box::new(source_ml_source2_nominal_field16892.clone()), source_ml_source2_nominal_field16894.clone(), Box::new(source_ml_source2_nominal_field16896.clone()), Box::new(source_ml_source2_nominal_field16898.clone()), Box::new(source_ml_source2_nominal_field16900.clone()), Box::new(source_ml_source2_nominal_field16902.clone()), Box::new(source_ml_source2_nominal_field16904.clone()), Box::new(source_ml_source2_nominal_field16906.clone()), source_ml_source2_nominal_field16908, source_ml_source2_nominal_field16910, self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]), source_ml_source2_nominal_field16914, source_ml_source2_nominal_field16916, Box::new(source_ml_source2_nominal_field16918.clone()), Box::new(source_ml_source2_nominal_field16920.clone()), Box::new(source_ml_source2_nominal_field16922.clone()), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field16926.clone()), source_ml_source2_nominal_field16928.clone(), source_ml_source2_nominal_field16930.clone(), source_ml_source2_nominal_field16932.clone(), source_ml_source2_nominal_field16934.clone(), source_ml_source2_nominal_field16936.clone(), source_ml_source2_nominal_field16938.clone(), source_ml_source2_nominal_field16940.clone(), source_ml_source2_nominal_field16942.clone(), source_ml_source2_nominal_field16944.clone(), source_ml_source2_nominal_field16946.clone(), source_ml_source2_nominal_field16948.clone())
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
                                            }))
                                        } else {
                                            if (shared_constructor_field31 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 115, 101, 116, 116, 101, 114]))) {
                                                self::JavaScriptArrayCreationTransition::ArrayCreationCompleted(Box::new(heap.clone()), Box::new(shared_constructor_field32.clone()))
                                            } else {
                                                if self::ml_array::includes(&{ let ml_reference_value882 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value882 }, &shared_constructor_field31.clone(), 0f64, false, |a, b| a == b) {
                                                    self::accept_java_script_array_creation_get(heap.clone(), state.clone(), value.clone())
                                                } else {
                                                    self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114, 32, 109, 97, 112, 112, 101, 114, 32, 112, 104, 97, 115, 101]))
                                                }
                                            }
                                        }
                                    }
                                    self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                                        let value_2 = *value_2;
                                        self::throw_java_script_array_creation(heap.clone(), state.clone(), value_2.clone())
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
pub fn resume_java_script_array_creation_construction(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArrayCreationTransition {
    match state.clone() {
        self::JavaScriptArrayCreationState::ArrayCreationState(
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
            shared_constructor_field15,
            shared_constructor_field16,
            shared_constructor_field17,
            shared_constructor_field18,
            shared_constructor_field19,
            shared_constructor_field20,
            shared_constructor_field21,
            shared_constructor_field22,
            shared_constructor_field23,
            shared_constructor_field24,
            shared_constructor_field25,
            shared_constructor_field26,
            shared_constructor_field27,
            shared_constructor_field28,
            shared_constructor_field29,
            shared_constructor_field30,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field9 = *shared_constructor_field9;
            let shared_constructor_field15 = *shared_constructor_field15;
            let shared_constructor_field16 = *shared_constructor_field16;
            let shared_constructor_field17 = *shared_constructor_field17;
            let shared_constructor_field18 = *shared_constructor_field18;
            let shared_constructor_field19 = *shared_constructor_field19;
            {
                let error = {
                    let source_ml_source2_logical16958 =
                        self::read_java_script_array_creation_state_error(
                            heap.clone(),
                            state.clone(),
                        );
                    if !source_ml_source2_logical16958.0.is_empty() {
                        source_ml_source2_logical16958.clone()
                    } else {
                        self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match completion.clone() {
                                self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(
                                    source_ml_source2_data_property_value16956,
                                ) => {
                                    let source_ml_source2_data_property_value16956 =
                                        *source_ml_source2_data_property_value16956;
                                    source_ml_source2_data_property_value16956.clone()
                                }
                                self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(
                                    source_ml_source2_data_property_value16957,
                                ) => {
                                    let source_ml_source2_data_property_value16957 =
                                        *source_ml_source2_data_property_value16957;
                                    source_ml_source2_data_property_value16957.clone()
                                }
                            },
                        )
                    }
                };
                if ((error != (self::ml_string::Text::from_units(vec![])))
                    || (shared_constructor_field12
                        != (self::ml_string::Text::from_units(vec![
                            99, 111, 110, 115, 116, 114, 117, 99, 116,
                        ]))))
                {
                    self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                        Box::new(heap.clone()),
                        if !error.0.is_empty() {
                            error.clone()
                        } else {
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110,
                                32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97,
                                99, 116, 117, 97, 108, 32, 67, 111, 110, 115, 116, 114, 117, 99,
                                116, 32, 114, 101, 113, 117, 101, 115, 116,
                            ])
                        },
                    )
                } else {
                    match completion.clone() {
                        self::JavaScriptPrimitiveConversionCompletion::NormalCompletion(value) => {
                            let value = *value;
                            if match value.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => false,
                                _ => true,
                            } {
                                self::JavaScriptArrayCreationTransition::ArrayCreationInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        97, 99, 116, 117, 97, 108, 32, 67, 111, 110, 115, 116, 114,
                                        117, 99, 116, 47, 65, 114, 114, 97, 121, 67, 114, 101, 97,
                                        116, 101, 32, 114, 101, 116, 117, 114, 110, 115, 32, 105,
                                        116, 115, 32, 79, 98, 106, 101, 99, 116, 32, 114, 101, 115,
                                        117, 108, 116,
                                    ]),
                                )
                            } else {
                                {
                                    let next = {
                                        let source_ml_source2_nominal_field16966 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field16968 =
                                                shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field16970 =
                                                    shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field16972 =
                                                        shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field16974 =
                                                            shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field16976 =
                                                                shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field16978 =
                                                                    shared_constructor_field7
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field16980 =
                                                                        shared_constructor_field8
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field16982 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field16984 = shared_constructor_field10;
                                                                            {
                                                                                let source_ml_source2_nominal_field16986 = shared_constructor_field11;
                                                                                {
                                                                                    let source_ml_source2_nominal_field16988 = shared_constructor_field12.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field16990 = shared_constructor_field13;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field16992 = shared_constructor_field14;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field16994 = shared_constructor_field15.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field16996 = shared_constructor_field16.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field16998 = shared_constructor_field17.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field17000 = shared_constructor_field18.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field17002 = shared_constructor_field19.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field17004 = shared_constructor_field20.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field17006 = shared_constructor_field21.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field17008 = shared_constructor_field22.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_field17010 = shared_constructor_field23.clone();
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_field17012 = shared_constructor_field24.clone();
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_field17014 = shared_constructor_field25.clone();
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_field17016 = shared_constructor_field26.clone();
                                                                                                                                            {
                                                                                                                                                let source_ml_source2_nominal_field17018 = shared_constructor_field27.clone();
                                                                                                                                                {
                                                                                                                                                    let source_ml_source2_nominal_field17020 = shared_constructor_field28.clone();
                                                                                                                                                    {
                                                                                                                                                        let source_ml_source2_nominal_field17022 = shared_constructor_field29.clone();
                                                                                                                                                        {
                                                                                                                                                            let source_ml_source2_nominal_field17024 = shared_constructor_field30.clone();
                                                                                                                                                            {
                                                                                                                                                                let source_ml_source2_nominal_override17027 = if shared_constructor_field11 {
                                                                                                                                                                    self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110])
                                                                                                                                                                } else {
                                                                                                                                                                    self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116])
                                                                                                                                                                };
                                                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(source_ml_source2_nominal_field16966.clone(), Box::new(source_ml_source2_nominal_field16968.clone()), source_ml_source2_nominal_field16970.clone(), Box::new(source_ml_source2_nominal_field16972.clone()), Box::new(source_ml_source2_nominal_field16974.clone()), Box::new(source_ml_source2_nominal_field16976.clone()), Box::new(source_ml_source2_nominal_field16978.clone()), Box::new(source_ml_source2_nominal_field16980.clone()), Box::new(source_ml_source2_nominal_field16982.clone()), source_ml_source2_nominal_field16984, source_ml_source2_nominal_field16986, source_ml_source2_nominal_override17027.clone(), source_ml_source2_nominal_field16990, source_ml_source2_nominal_field16992, Box::new(source_ml_source2_nominal_field16994.clone()), Box::new(source_ml_source2_nominal_field16996.clone()), Box::new(value.clone()), Box::new(source_ml_source2_nominal_field17000.clone()), Box::new(source_ml_source2_nominal_field17002.clone()), source_ml_source2_nominal_field17004.clone(), source_ml_source2_nominal_field17006.clone(), source_ml_source2_nominal_field17008.clone(), source_ml_source2_nominal_field17010.clone(), source_ml_source2_nominal_field17012.clone(), source_ml_source2_nominal_field17014.clone(), source_ml_source2_nominal_field17016.clone(), source_ml_source2_nominal_field17018.clone(), source_ml_source2_nominal_field17020.clone(), source_ml_source2_nominal_field17022.clone(), source_ml_source2_nominal_field17024.clone())
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
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    if !shared_constructor_field11 {
                                        self::JavaScriptArrayCreationTransition::ArrayCreationContinue(Box::new(heap.clone()), Box::new(next.clone()))
                                    } else {
                                        {
                                            let protocol = self::create_java_script_iterator_protocol_state(&(self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101])), &(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108])), shared_constructor_field7.clone(), shared_constructor_field5.clone(), self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(Box::new(self::JavaScriptSemanticValue::Undefined), Box::new(self::JavaScriptSemanticValue::Undefined), false), self::JavaScriptIteratorCompletion::IteratorCompletion(self::ml_string::Text::from_units(vec![110, 111, 114, 109, 97, 108]), Box::new(self::JavaScriptSemanticValue::Undefined), (-1f64)));
                                            self::accept_java_script_array_creation_iterator(
                                                next.clone(),
                                                self::request_java_script_iterator_protocol_call(
                                                    heap.clone(),
                                                    protocol.clone(),
                                                    shared_constructor_field15.clone(),
                                                    shared_constructor_field7.clone(),
                                                    &(self::ml_string::Text::from_units(vec![
                                                        105, 116, 101, 114, 97, 116, 111, 114, 32,
                                                        99, 97, 108, 108,
                                                    ])),
                                                ),
                                            )
                                        }
                                    }
                                }
                            }
                        }
                        self::JavaScriptPrimitiveConversionCompletion::ThrowCompletion(value_2) => {
                            let value_2 = *value_2;
                            self::JavaScriptArrayCreationTransition::ArrayCreationThrow(
                                Box::new(heap.clone()),
                                Box::new(value_2.clone()),
                            )
                        }
                    }
                }
            }
        }
    }
}
