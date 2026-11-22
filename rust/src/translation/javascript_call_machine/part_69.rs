// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIteratorCompletion;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptIteratorProtocolTransition;
use super::JavaScriptIteratorSelectedCall;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380;
use super::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381;
use super::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Result;
use super::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Result;
use super::MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Result;
use super::fail_java_script_iterator_protocol;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::source_ml_source1_temporary_read_java_script_iterator_protocol_state_error_loop379;
use super::source_ml_source1_temporary_read_java_script_iterator_protocol_state_error_loop382;
use super::source_ml_source1_temporary_read_java_script_synchronous_iterator_record_error_loop376;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_synchronous_iterator_record_error(
    heap: self::JavaScriptOrdinaryHeap,
    record: self::JavaScriptSynchronousIteratorRecord,
) -> self::ml_string::Text {
    match record.clone() {
        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    heap_error.clone()
                } else {
                    if ((match record.clone() {
                        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
                            _,
                            _,
                            _,
                        ) => false,
                    } || {
                        let source_ml_source2_s13096 = shared_constructor_field1.clone();
                        match source_ml_source2_s13096.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }
                    }) || (({
                        let source_ml_source2_type_query_operand13103 = shared_constructor_field3;
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                    }) != (self::ml_string::Text::from_units(vec![
                        98, 111, 111, 108, 101, 97, 110,
                    ])))) {
                        self::ml_string::Text::from_units(vec![
                            97, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 99,
                            111, 114, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115,
                            32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 97,
                            110, 100, 32, 100, 111, 110, 101, 32, 115, 108, 111, 116,
                        ])
                    } else {
                        {
                            let source_ml_source1_temporary_values375 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    shared_constructor_field1.clone(),
                                    shared_constructor_field2.clone(),
                                ]));
                            {
                                let source_ml_source1_temporary_index375 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s13106 = self::source_ml_source1_temporary_read_java_script_synchronous_iterator_record_error_loop376(heap.clone(), source_ml_source1_temporary_values375.clone(), source_ml_source1_temporary_index375.clone());
                                    match source_ml_source2_s13106.clone() {
                                        self::MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Result::MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Done => {
                                            self::ml_string::Text::from_units(vec![])
                                        }
                                        self::MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Result::MlSource1TemporaryReadJavaScriptSynchronousIteratorRecordErrorLoop376Return(source_ml_source1_temporary_result376) => {
                                            source_ml_source1_temporary_result376.clone()
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
pub fn read_java_script_iterator_protocol_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptIteratorProtocolState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptIteratorProtocolState::IteratorProtocol(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            _,
            _,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            {
                let invalid = self::read_java_script_ordinary_heap_error(heap.clone());
                if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                    invalid.clone()
                } else {
                    {
                        let phases = if (shared_constructor_field1
                            == (self::ml_string::Text::from_units(vec![
                                97, 99, 113, 117, 105, 114, 101,
                            ]))) {
                            std::rc::Rc::new(std::cell::RefCell::new(vec![
                                self::ml_string::Text::from_units(vec![
                                    109, 101, 116, 104, 111, 100,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116, 101, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108,
                                ]),
                                self::ml_string::Text::from_units(vec![110, 101, 120, 116]),
                                self::ml_string::Text::from_units(vec![
                                    110, 101, 120, 116, 32, 103, 101, 116, 116, 101, 114,
                                ]),
                            ]))
                        } else {
                            if (shared_constructor_field1
                                == (self::ml_string::Text::from_units(vec![115, 116, 101, 112])))
                            {
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    self::ml_string::Text::from_units(vec![
                                        110, 101, 120, 116, 32, 99, 97, 108, 108,
                                    ]),
                                    self::ml_string::Text::from_units(vec![100, 111, 110, 101]),
                                    self::ml_string::Text::from_units(vec![
                                        100, 111, 110, 101, 32, 103, 101, 116, 116, 101, 114,
                                    ]),
                                    self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]),
                                    self::ml_string::Text::from_units(vec![
                                        118, 97, 108, 117, 101, 32, 103, 101, 116, 116, 101, 114,
                                    ]),
                                ]))
                            } else {
                                if (shared_constructor_field1
                                    == (self::ml_string::Text::from_units(vec![
                                        99, 108, 111, 115, 101,
                                    ])))
                                {
                                    std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            114, 101, 116, 117, 114, 110,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            114, 101, 116, 117, 114, 110, 32, 103, 101, 116, 116,
                                            101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            114, 101, 116, 117, 114, 110, 32, 99, 97, 108, 108,
                                        ]),
                                    ]))
                                } else {
                                    std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::ml_string::Text,
                                    >::new(
                                    )))
                                }
                            }
                        };
                        if ({
                            let source_ml_source2_comparison_operand13114 = match state.clone() {
                                self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value13113,
                                    _,
                                ) => ((source_ml_source2_data_property_value13113
                                    .clone()
                                    .unwrap_or_else(|| (None::<bool>,)))
                                .0)
                                    .clone(),
                            };
                            {
                                let source_ml_source2_comparison_operand13115 = None::<()>;
                                !(match (
                                    source_ml_source2_comparison_operand13114.clone(),
                                    source_ml_source2_comparison_operand13115.clone(),
                                ) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                })
                            }
                        } && (((shared_constructor_field1
                            != (self::ml_string::Text::from_units(vec![115, 116, 101, 112])))
                            || (({
                                let source_ml_source2_type_query_operand13121 =
                                    (match state.clone() {
                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value13120,
                                            _,
                                        ) => ((source_ml_source2_data_property_value13120
                                            .clone()
                                            .unwrap_or_else(|| (None::<bool>,)))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .expect("a proven present value");
                                self::ml_string::Text::from_units(vec![
                                    98, 111, 111, 108, 101, 97, 110,
                                ])
                            }) != (self::ml_string::Text::from_units(vec![
                                98, 111, 111, 108, 101, 97, 110,
                            ]))))
                            || ((((match state.clone() {
                                self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value13123,
                                    _,
                                ) => ((source_ml_source2_data_property_value13123
                                    .clone()
                                    .unwrap_or_else(|| (None::<bool>,)))
                                .0)
                                    .clone(),
                            })
                            .clone()
                            .expect("a proven present value"))
                                == (false))
                                && self::ml_array::includes(
                                    &{
                                        let ml_reference_value782 =
                                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                                self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101,
                                                ]),
                                                self::ml_string::Text::from_units(vec![
                                                    118, 97, 108, 117, 101, 32, 103, 101, 116, 116,
                                                    101, 114,
                                                ]),
                                            ])))
                                            .borrow()
                                            .clone();
                                        ml_reference_value782
                                    },
                                    &shared_constructor_field2.clone(),
                                    0f64,
                                    false,
                                    |a, b| a == b,
                                ))))
                        {
                            self::ml_string::Text::from_units(vec![
                                105, 116, 101, 114, 97, 116, 111, 114, 32, 101, 108, 105, 115, 105,
                                111, 110, 32, 115, 107, 105, 112, 115, 32, 118, 97, 108, 117, 101,
                                32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110, 32, 111,
                                110, 108, 121, 32, 105, 110, 32, 105, 116, 115, 32, 97, 99, 116,
                                117, 97, 108, 32, 97, 100, 118, 97, 110, 99, 101, 109, 101, 110,
                                116, 32, 112, 114, 111, 116, 111, 99, 111, 108,
                            ])
                        } else {
                            if ((match state.clone() {
                                self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => false,
                            } || !(self::ml_array::includes(
                                &{
                                    let ml_reference_value783 = phases.borrow().clone();
                                    ml_reference_value783
                                },
                                &shared_constructor_field2.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            ))) || (({
                                let source_ml_source2_type_query_operand13140 = match shared_constructor_field5.clone() {
                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value13139) => {
                                        source_ml_source2_data_property_value13139
                                    }
                                };
                                self::ml_string::Text::from_units(vec![
                                    98, 111, 111, 108, 101, 97, 110,
                                ])
                            }) != (self::ml_string::Text::from_units(vec![
                                98, 111, 111, 108, 101, 97, 110,
                            ])))) {
                                self::ml_string::Text::from_units(vec![
                                    105, 116, 101, 114, 97, 116, 111, 114, 32, 119, 111, 114, 107,
                                    32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32,
                                    97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 116,
                                    105, 111, 110, 32, 97, 110, 100, 32, 99, 111, 110, 116, 105,
                                    110, 117, 97, 116, 105, 111, 110, 32, 112, 104, 97, 115, 101,
                                ])
                            } else {
                                {
                                    let source_ml_source1_temporary_values377 = std::rc::Rc::new(std::cell::RefCell::new(vec![shared_constructor_field3.clone(), shared_constructor_field4.clone(), match shared_constructor_field5.clone() {
                                        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value13144, _, _) => {
                                            let source_ml_source2_data_property_value13144 = *source_ml_source2_data_property_value13144;
                                            source_ml_source2_data_property_value13144.clone()
                                        }
                                    }, match shared_constructor_field5.clone() {
                                        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value13146, _) => {
                                            let source_ml_source2_data_property_value13146 = *source_ml_source2_data_property_value13146;
                                            source_ml_source2_data_property_value13146.clone()
                                        }
                                    }, shared_constructor_field6.clone(), match shared_constructor_field7.clone() {
                                        self::JavaScriptIteratorCompletion::IteratorCompletion(_, source_ml_source2_data_property_value13149, _) => {
                                            let source_ml_source2_data_property_value13149 = *source_ml_source2_data_property_value13149;
                                            source_ml_source2_data_property_value13149.clone()
                                        }
                                    }]));
                                    {
                                        let source_ml_source1_temporary_index377 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s13150 = self::source_ml_source1_temporary_read_java_script_iterator_protocol_state_error_loop379(heap.clone(), source_ml_source1_temporary_values377.clone(), source_ml_source1_temporary_index377.clone());
                                            match source_ml_source2_s13150.clone() {
                                                self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Result::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Done => {
                                                    if ((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]))) && {
                                                        let source_ml_source2_s13154 = shared_constructor_field4.clone();
                                                        match source_ml_source2_s13154.clone() {
                                                            self::JavaScriptSemanticValue::Symbol(_) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) {
                                                        self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 83, 121, 109, 98, 111, 108])
                                                    } else {
                                                        {
                                                            let source_ml_source2_s13161 = if ((shared_constructor_field1 != (self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101]))) || self::ml_array::includes(&{ let ml_reference_value784 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![110, 101, 120, 116]), self::ml_string::Text::from_units(vec![110, 101, 120, 116, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value784 }, &shared_constructor_field2.clone(), 0f64, false, |a, b| a == b)) {
                                                                {
                                                                    let error = self::read_java_script_synchronous_iterator_record_error(heap.clone(), shared_constructor_field5.clone());
                                                                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                        self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380Return(error.clone())
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380Next
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380Next
                                                            };
                                                            match source_ml_source2_s13161.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380Next => {
                                                                    if (((((!(self::ml_array::includes(&{ let ml_reference_value785 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![110, 111, 114, 109, 97, 108]), self::ml_string::Text::from_units(vec![116, 104, 114, 111, 119]), self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110]), self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107]), self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101])]))).borrow().clone(); ml_reference_value785 }, &match shared_constructor_field7.clone() {
                                                                        self::JavaScriptIteratorCompletion::IteratorCompletion(source_ml_source2_data_property_value13167, _, _) => {
                                                                            source_ml_source2_data_property_value13167.clone()
                                                                        }
                                                                    }, 0f64, false, |a, b| a == b)) || !(self::ml_math::is_safe_integer(match shared_constructor_field7.clone() {
                                                                        self::JavaScriptIteratorCompletion::IteratorCompletion(_, _, source_ml_source2_data_property_value13169) => {
                                                                            source_ml_source2_data_property_value13169
                                                                        }
                                                                    }))) || {
                                                                        let source_ml_source2_same_value_operand13172 = match shared_constructor_field7.clone() {
                                                                            self::JavaScriptIteratorCompletion::IteratorCompletion(_, _, source_ml_source2_data_property_value13171) => {
                                                                                source_ml_source2_data_property_value13171
                                                                            }
                                                                        };
                                                                        {
                                                                            let source_ml_source2_same_value_operand13173 = (-0f64);
                                                                            (((source_ml_source2_same_value_operand13172 == source_ml_source2_same_value_operand13173) && ((source_ml_source2_same_value_operand13172 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand13172)) == ((1f64 / source_ml_source2_same_value_operand13173))))) || ((source_ml_source2_same_value_operand13172 != source_ml_source2_same_value_operand13172) && (source_ml_source2_same_value_operand13173 != source_ml_source2_same_value_operand13173)))
                                                                        }
                                                                    }) || ((match shared_constructor_field7.clone() {
                                                                        self::JavaScriptIteratorCompletion::IteratorCompletion(_, _, source_ml_source2_data_property_value13177) => {
                                                                            source_ml_source2_data_property_value13177
                                                                        }
                                                                    }) < ((-1f64)))) || ((match shared_constructor_field7.clone() {
                                                                        self::JavaScriptIteratorCompletion::IteratorCompletion(_, _, source_ml_source2_data_property_value13181) => {
                                                                            source_ml_source2_data_property_value13181
                                                                        }
                                                                    }) >= (65536f64))) || (!(self::ml_array::includes(&{ let ml_reference_value786 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![98, 114, 101, 97, 107]), self::ml_string::Text::from_units(vec![99, 111, 110, 116, 105, 110, 117, 101])]))).borrow().clone(); ml_reference_value786 }, &match shared_constructor_field7.clone() {
                                                                        self::JavaScriptIteratorCompletion::IteratorCompletion(source_ml_source2_data_property_value13183, _, _) => {
                                                                            source_ml_source2_data_property_value13183.clone()
                                                                        }
                                                                    }, 0f64, false, |a, b| a == b)) && ((match shared_constructor_field7.clone() {
                                                                        self::JavaScriptIteratorCompletion::IteratorCompletion(_, _, source_ml_source2_data_property_value13187) => {
                                                                            source_ml_source2_data_property_value13187
                                                                        }
                                                                    }) != ((-1f64))))) {
                                                                        self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 108, 101, 97, 110, 117, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 97, 99, 116, 117, 97, 108, 32, 97, 98, 114, 117, 112, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110, 32, 97, 110, 100, 32, 99, 111, 110, 116, 114, 111, 108, 32, 116, 97, 114, 103, 101, 116])
                                                                    } else {
                                                                        {
                                                                            let source_ml_source2_s13283 = if !(match (match state.clone() {
                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13284) => {
                                                                                    ((source_ml_source2_data_property_value13284.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                }
                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                if ((((!(shared_constructor_field2.ends_with(&self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114]))) && !(shared_constructor_field2.ends_with(&self::ml_string::Text::from_units(vec![99, 97, 108, 108])))) || {
                                                                                    let source_ml_source2_s13289 = match (match state.clone() {
                                                                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13292) => {
                                                                                            ((source_ml_source2_data_property_value13292.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value") {
                                                                                        self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(source_ml_source2_data_property_value13293, _, _) => {
                                                                                            let source_ml_source2_data_property_value13293 = *source_ml_source2_data_property_value13293;
                                                                                            source_ml_source2_data_property_value13293.clone()
                                                                                        }
                                                                                    };
                                                                                    match source_ml_source2_s13289.clone() {
                                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                            false
                                                                                        }
                                                                                        _ => {
                                                                                            true
                                                                                        }
                                                                                    }
                                                                                }) || !(((match match (match state.clone() {
                                                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13294) => {
                                                                                        ((source_ml_source2_data_property_value13294.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value") {
                                                                                    self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(source_ml_source2_data_property_value13295, _, _) => {
                                                                                        let source_ml_source2_data_property_value13295 = *source_ml_source2_data_property_value13295;
                                                                                        source_ml_source2_data_property_value13295.clone()
                                                                                    }
                                                                                } {
                                                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value13296) => {
                                                                                        Some((source_ml_source2_data_property_value13296,))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(bool,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0)) || (({ let ml_reference_array787 = match (match state.clone() {
                                                                                    self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13299) => {
                                                                                        ((source_ml_source2_data_property_value13299.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value") {
                                                                                    self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(_, _, source_ml_source2_data_property_value13300) => {
                                                                                        source_ml_source2_data_property_value13300.clone()
                                                                                    }
                                                                                }; let ml_reference_projection787 = ml_reference_array787.borrow().len() as f64; ml_reference_projection787 }) != (0f64))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 122, 101, 114, 111, 45, 97, 114, 103, 117, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 109, 101, 116, 104, 111, 100]))
                                                                                } else {
                                                                                    if ((shared_constructor_field2 == (self::ml_string::Text::from_units(vec![110, 101, 120, 116, 32, 99, 97, 108, 108]))) && ((((({
                                                                                        let source_ml_source2_s13305 = match (match state.clone() {
                                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13308) => {
                                                                                                ((source_ml_source2_data_property_value13308.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(source_ml_source2_data_property_value13309, _, _) => {
                                                                                                let source_ml_source2_data_property_value13309 = *source_ml_source2_data_property_value13309;
                                                                                                source_ml_source2_data_property_value13309.clone()
                                                                                            }
                                                                                        };
                                                                                        match source_ml_source2_s13305.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    } || {
                                                                                        let source_ml_source2_s13312 = match shared_constructor_field5.clone() {
                                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value13316, _) => {
                                                                                                let source_ml_source2_data_property_value13316 = *source_ml_source2_data_property_value13316;
                                                                                                source_ml_source2_data_property_value13316.clone()
                                                                                            }
                                                                                        };
                                                                                        match source_ml_source2_s13312.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    }) || (((((match match (match state.clone() {
                                                                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13323) => {
                                                                                            ((source_ml_source2_data_property_value13323.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value") {
                                                                                        self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(source_ml_source2_data_property_value13324, _, _) => {
                                                                                            let source_ml_source2_data_property_value13324 = *source_ml_source2_data_property_value13324;
                                                                                            source_ml_source2_data_property_value13324.clone()
                                                                                        }
                                                                                    } {
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value13325, _) => {
                                                                                            Some((source_ml_source2_data_property_value13325.clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(self::ml::Big,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone()) != ((((match match shared_constructor_field5.clone() {
                                                                                        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, source_ml_source2_data_property_value13327, _) => {
                                                                                            let source_ml_source2_data_property_value13327 = *source_ml_source2_data_property_value13327;
                                                                                            source_ml_source2_data_property_value13327.clone()
                                                                                        }
                                                                                    } {
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value13328, _) => {
                                                                                            Some((source_ml_source2_data_property_value13328.clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(self::ml::Big,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone()))) || {
                                                                                        let source_ml_source2_s13331 = match (match state.clone() {
                                                                                            self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13334) => {
                                                                                                ((source_ml_source2_data_property_value13334.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(_, source_ml_source2_data_property_value13335, _) => {
                                                                                                let source_ml_source2_data_property_value13335 = *source_ml_source2_data_property_value13335;
                                                                                                source_ml_source2_data_property_value13335.clone()
                                                                                            }
                                                                                        };
                                                                                        match source_ml_source2_s13331.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    }) || {
                                                                                        let source_ml_source2_s13338 = match shared_constructor_field5.clone() {
                                                                                            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value13342, _, _) => {
                                                                                                let source_ml_source2_data_property_value13342 = *source_ml_source2_data_property_value13342;
                                                                                                source_ml_source2_data_property_value13342.clone()
                                                                                            }
                                                                                        };
                                                                                        match source_ml_source2_s13338.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                false
                                                                                            }
                                                                                            _ => {
                                                                                                true
                                                                                            }
                                                                                        }
                                                                                    }) || (((((match match (match state.clone() {
                                                                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13349) => {
                                                                                            ((source_ml_source2_data_property_value13349.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value") {
                                                                                        self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(_, source_ml_source2_data_property_value13350, _) => {
                                                                                            let source_ml_source2_data_property_value13350 = *source_ml_source2_data_property_value13350;
                                                                                            source_ml_source2_data_property_value13350.clone()
                                                                                        }
                                                                                    } {
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value13351, _) => {
                                                                                            Some((source_ml_source2_data_property_value13351.clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(self::ml::Big,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone()) != ((((match match shared_constructor_field5.clone() {
                                                                                        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value13353, _, _) => {
                                                                                            let source_ml_source2_data_property_value13353 = *source_ml_source2_data_property_value13353;
                                                                                            source_ml_source2_data_property_value13353.clone()
                                                                                        }
                                                                                    } {
                                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value13354, _) => {
                                                                                            Some((source_ml_source2_data_property_value13354.clone(),))
                                                                                        }
                                                                                        _ => {
                                                                                            None::<(self::ml::Big,)>
                                                                                        }
                                                                                    }).clone().expect("a proven present value")).0).clone())))) {
                                                                                        self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 100, 118, 97, 110, 99, 101, 109, 101, 110, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 99, 97, 99, 104, 101, 100, 32, 110, 101, 120, 116, 32, 109, 101, 116, 104, 111, 100, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                                                    } else {
                                                                                        {
                                                                                            let receiver = if ((shared_constructor_field2 == (self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116, 101, 114]))) || (shared_constructor_field2 == (self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108])))) {
                                                                                                shared_constructor_field3.clone()
                                                                                            } else {
                                                                                                if ((shared_constructor_field2 == (self::ml_string::Text::from_units(vec![100, 111, 110, 101, 32, 103, 101, 116, 116, 101, 114]))) || (shared_constructor_field2 == (self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101, 32, 103, 101, 116, 116, 101, 114])))) {
                                                                                                    shared_constructor_field6.clone()
                                                                                                } else {
                                                                                                    match shared_constructor_field5.clone() {
                                                                                                        self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(source_ml_source2_data_property_value13366, _, _) => {
                                                                                                            let source_ml_source2_data_property_value13366 = *source_ml_source2_data_property_value13366;
                                                                                                            source_ml_source2_data_property_value13366.clone()
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            };
                                                                                            if !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                                self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13367) => {
                                                                                                    ((source_ml_source2_data_property_value13367.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(_, source_ml_source2_data_property_value13368, _) => {
                                                                                                    let source_ml_source2_data_property_value13368 = *source_ml_source2_data_property_value13368;
                                                                                                    source_ml_source2_data_property_value13368.clone()
                                                                                                }
                                                                                            }, receiver.clone())) {
                                                                                                self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381Return(self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108, 98, 97, 99, 107, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 111, 114, 32, 109, 101, 116, 104, 111, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                                                            } else {
                                                                                                {
                                                                                                    let source_ml_source1_temporary_values378 = std::rc::Rc::new(std::cell::RefCell::new(vec![match (match state.clone() {
                                                                                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13369) => {
                                                                                                            ((source_ml_source2_data_property_value13369.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                                        }
                                                                                                    }).clone().expect("a proven present value") {
                                                                                                        self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(source_ml_source2_data_property_value13370, _, _) => {
                                                                                                            let source_ml_source2_data_property_value13370 = *source_ml_source2_data_property_value13370;
                                                                                                            source_ml_source2_data_property_value13370.clone()
                                                                                                        }
                                                                                                    }, match (match state.clone() {
                                                                                                        self::JavaScriptIteratorProtocolState::IteratorProtocol(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value13371) => {
                                                                                                            ((source_ml_source2_data_property_value13371.clone().unwrap_or_else(|| (None::<self::JavaScriptIteratorSelectedCall>,))).0).clone()
                                                                                                        }
                                                                                                    }).clone().expect("a proven present value") {
                                                                                                        self::JavaScriptIteratorSelectedCall::IteratorSelectedCall(_, source_ml_source2_data_property_value13372, _) => {
                                                                                                            let source_ml_source2_data_property_value13372 = *source_ml_source2_data_property_value13372;
                                                                                                            source_ml_source2_data_property_value13372.clone()
                                                                                                        }
                                                                                                    }]));
                                                                                                    {
                                                                                                        let source_ml_source1_temporary_index378 = self::ml::Big::from_i128(0);
                                                                                                        {
                                                                                                            let source_ml_source2_s13373 = self::source_ml_source1_temporary_read_java_script_iterator_protocol_state_error_loop382(heap.clone(), source_ml_source1_temporary_values378.clone(), source_ml_source1_temporary_index378.clone());
                                                                                                            match source_ml_source2_s13373.clone() {
                                                                                                                self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Result::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Done => {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381Next
                                                                                                                }
                                                                                                                self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Result::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop382Return(source_ml_source1_temporary_result382) => {
                                                                                                                    self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381Return(source_ml_source1_temporary_result382.clone())
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
                                                                                if ((shared_constructor_field2.ends_with(&self::ml_string::Text::from_units(vec![103, 101, 116, 116, 101, 114])) || (shared_constructor_field2 == (self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 97, 108, 108])))) || (shared_constructor_field2 == (self::ml_string::Text::from_units(vec![114, 101, 116, 117, 114, 110, 32, 99, 97, 108, 108])))) {
                                                                                    self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381Return(self::ml_string::Text::from_units(vec![97, 32, 112, 101, 110, 100, 105, 110, 103, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 109, 101, 116, 104, 111, 100, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108]))
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381Next
                                                                                }
                                                                            };
                                                                            match source_ml_source2_s13283.clone() {
                                                                                self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381Next => {
                                                                                    self::ml_string::Text::from_units(vec![])
                                                                                }
                                                                                self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin381Return(source_ml_source1_temporary_result381) => {
                                                                                    source_ml_source1_temporary_result381.clone()
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorJoin380Return(source_ml_source1_temporary_result382_2) => {
                                                                    source_ml_source1_temporary_result382_2.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Result::MlSource1TemporaryReadJavaScriptIteratorProtocolStateErrorLoop379Return(source_ml_source1_temporary_result382_3) => {
                                                    source_ml_source1_temporary_result382_3.clone()
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
pub fn create_java_script_iterator_protocol_state(
    operation: &self::ml_string::Text,
    phase: &self::ml_string::Text,
    input: self::JavaScriptSemanticValue,
    key: self::JavaScriptSemanticValue,
    record: self::JavaScriptSynchronousIteratorRecord,
    completion: self::JavaScriptIteratorCompletion,
) -> self::JavaScriptIteratorProtocolState {
    match completion.clone() {
        self::JavaScriptIteratorCompletion::IteratorCompletion(
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let shared_constructor_field5 = *shared_constructor_field5;
            match record.clone() {
                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
                    shared_constructor_field1,
                    shared_constructor_field2,
                    shared_constructor_field3,
                ) => {
                    let shared_constructor_field1 = *shared_constructor_field1;
                    let shared_constructor_field2 = *shared_constructor_field2;
                    {
                        let source_ml_source2_constructor_field13390 = {
                            let source_ml_source2_nominal_field13384 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_nominal_field13386 =
                                    shared_constructor_field2.clone();
                                {
                                    let source_ml_source2_nominal_field13388 =
                                        shared_constructor_field3;
                                    self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(Box::new(source_ml_source2_nominal_field13384.clone()), Box::new(source_ml_source2_nominal_field13386.clone()), source_ml_source2_nominal_field13388)
                                }
                            }
                        };
                        {
                            let source_ml_source2_constructor_field13391 =
                                self::JavaScriptSemanticValue::Undefined;
                            {
                                let source_ml_source2_constructor_field13399 = {
                                    let source_ml_source2_nominal_field13393 =
                                        shared_constructor_field4.clone();
                                    {
                                        let source_ml_source2_nominal_field13395 =
                                            shared_constructor_field5.clone();
                                        {
                                            let source_ml_source2_nominal_field13397 =
                                                shared_constructor_field6;
                                            self::JavaScriptIteratorCompletion::IteratorCompletion(
                                                source_ml_source2_nominal_field13393.clone(),
                                                Box::new(
                                                    source_ml_source2_nominal_field13395.clone(),
                                                ),
                                                source_ml_source2_nominal_field13397,
                                            )
                                        }
                                    }
                                };
                                self::JavaScriptIteratorProtocolState::IteratorProtocol(
                                    operation.clone(),
                                    phase.clone(),
                                    Box::new(input.clone()),
                                    Box::new(key.clone()),
                                    Box::new(source_ml_source2_constructor_field13390.clone()),
                                    Box::new(source_ml_source2_constructor_field13391.clone()),
                                    Box::new(source_ml_source2_constructor_field13399.clone()),
                                    None::<(Option<bool>,)>,
                                    None::<(Option<self::JavaScriptIteratorSelectedCall>,)>,
                                )
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
pub fn begin_java_script_synchronous_iterator(
    heap: self::JavaScriptOrdinaryHeap,
    input: self::JavaScriptSemanticValue,
    iterator_key: self::JavaScriptSemanticValue,
) -> self::JavaScriptIteratorProtocolTransition {
    {
        let state = self::create_java_script_iterator_protocol_state(
            &(self::ml_string::Text::from_units(vec![97, 99, 113, 117, 105, 114, 101])),
            &(self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100])),
            input.clone(),
            iterator_key.clone(),
            self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(
                Box::new(self::JavaScriptSemanticValue::Undefined),
                Box::new(self::JavaScriptSemanticValue::Undefined),
                false,
            ),
            self::JavaScriptIteratorCompletion::IteratorCompletion(
                self::ml_string::Text::from_units(vec![110, 111, 114, 109, 97, 108]),
                Box::new(self::JavaScriptSemanticValue::Undefined),
                (-1f64),
            ),
        );
        {
            let error =
                self::read_java_script_iterator_protocol_state_error(heap.clone(), state.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptIteratorProtocolTransition::IteratorProtocolInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                if (match input.clone() {
                    self::JavaScriptSemanticValue::Null => true,
                    _ => false,
                } || match input.clone() {
                    self::JavaScriptSemanticValue::Undefined => true,
                    _ => false,
                }) {
                    self::fail_java_script_iterator_protocol(
                        heap.clone(),
                        state.clone(),
                        &(self::ml_string::Text::from_units(vec![
                            118, 97, 108, 117, 101, 32, 105, 115, 32, 110, 111, 116, 32, 105, 116,
                            101, 114, 97, 98, 108, 101,
                        ])),
                    )
                } else {
                    self::JavaScriptIteratorProtocolTransition::IteratorProtocolContinue(
                        Box::new(heap.clone()),
                        Box::new(state.clone()),
                    )
                }
            }
        }
    }
}
