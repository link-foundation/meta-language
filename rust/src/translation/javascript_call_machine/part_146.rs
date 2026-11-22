// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArraySearchCall;
use super::JavaScriptArraySearchState;
use super::JavaScriptArraySearchTransition;
use super::JavaScriptArraySpreadState;
use super::JavaScriptArraySpreadTransition;
use super::JavaScriptIndexedPropertyCreation;
use super::JavaScriptIteratorProtocolState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234;
use super::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235;
use super::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Result;
use super::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Result;
use super::begin_java_script_synchronous_iterator_step;
use super::continue_java_script_array_spread_iterator;
use super::create_java_script_indexed_data_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::provide_java_script_iterator_primitive_prototype;
use super::read_java_script_array_spread_state_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_primitive_conversion_state_error;
use super::resume_java_script_iterator_primitive_get;
use super::resume_java_script_iterator_protocol_call;
use super::source_ml_source1_temporary_read_java_script_array_search_state_error_loop1233;
use super::source_ml_source1_temporary_read_java_script_array_search_state_error_loop1236;
use super::step_java_script_iterator_protocol;
use super::test_java_script_array_search_operation;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_spread(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySpreadState,
) -> self::JavaScriptArraySpreadTransition {
    match state.clone() {
        self::JavaScriptArraySpreadState::ArraySpreadState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field6 = *shared_constructor_field6;
            {
                let error =
                    self::read_java_script_array_spread_state_error(heap.clone(), state.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if (shared_constructor_field5
                        == (self::ml_string::Text::from_units(vec![
                            105, 116, 101, 114, 97, 116, 111, 114,
                        ])))
                    {
                        if match (
                            match state.clone() {
                                self::JavaScriptArraySpreadState::ArraySpreadState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value27451,
                                    _,
                                ) => ((source_ml_source2_data_property_value27451
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
                        } {
                            self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105, 110,
                                    115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105,
                                    116, 101, 114, 97, 116, 111, 114, 32, 99, 111, 110, 116, 105,
                                    110, 117, 97, 116, 105, 111, 110,
                                ]),
                            )
                        } else {
                            self::continue_java_script_array_spread_iterator(
                                heap.clone(),
                                state.clone(),
                                self::step_java_script_iterator_protocol(
                                    heap.clone(),
                                    (match state.clone() {
                                        self::JavaScriptArraySpreadState::ArraySpreadState(
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            _,
                                            source_ml_source2_data_property_value27452,
                                            _,
                                        ) => ((source_ml_source2_data_property_value27452
                                            .clone()
                                            .unwrap_or_else(|| {
                                                (None::<self::JavaScriptIteratorProtocolState>,)
                                            }))
                                        .0)
                                            .clone(),
                                    })
                                    .clone()
                                    .expect("a proven present value"),
                                ),
                            )
                        }
                    } else {
                        if (shared_constructor_field5
                            == (self::ml_string::Text::from_units(vec![115, 116, 101, 112])))
                        {
                            if match (
                                match state.clone() {
                                    self::JavaScriptArraySpreadState::ArraySpreadState(
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                        source_ml_source2_data_property_value27455,
                                    ) => ((source_ml_source2_data_property_value27455
                                        .clone()
                                        .unwrap_or_else(|| {
                                            (None::<self::JavaScriptSynchronousIteratorRecord>,)
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
                                self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        115, 112, 114, 101, 97, 100, 32, 114, 101, 116, 97, 105,
                                        110, 115, 32, 105, 116, 115, 32, 99, 97, 99, 104, 101, 100,
                                        32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101,
                                        99, 111, 114, 100,
                                    ]),
                                )
                            } else {
                                self::continue_java_script_array_spread_iterator(
                                    heap.clone(),
                                    state.clone(),
                                    self::begin_java_script_synchronous_iterator_step(
                                        heap.clone(),
                                        (match state.clone() {
                                            self::JavaScriptArraySpreadState::ArraySpreadState(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                source_ml_source2_data_property_value27456,
                                            ) => ((source_ml_source2_data_property_value27456
                                                .clone()
                                                .unwrap_or_else(|| {
                                                    (None::<
                                                        self::JavaScriptSynchronousIteratorRecord,
                                                    >,)
                                                }))
                                            .0)
                                                .clone(),
                                        })
                                        .clone()
                                        .expect("a proven present value"),
                                        true,
                                    ),
                                )
                            }
                        } else {
                            if (shared_constructor_field4 == (9007199254740991f64)) {
                                self::JavaScriptArraySpreadTransition::ArraySpreadResourceLimit(
                                    Box::new(heap.clone()),
                                    Box::new(state.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 97, 99, 99, 117, 109, 117, 108,
                                        97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100,
                                        115, 32, 105, 116, 115, 32, 101, 120, 97, 99, 116, 32, 115,
                                        97, 102, 101, 32, 105, 110, 100, 101, 120, 32, 99, 111,
                                        111, 114, 100, 105, 110, 97, 116, 101, 32, 98, 117, 100,
                                        103, 101, 116,
                                    ]),
                                )
                            } else {
                                {
                                    let created = self::create_java_script_indexed_data_property(
                                        heap.clone(),
                                        shared_constructor_field1.clone(),
                                        shared_constructor_field4,
                                        shared_constructor_field6.clone(),
                                    );
                                    match created.clone() {
                                        self::JavaScriptIndexedPropertyCreation::IndexedPropertyCreated(heap_2) => {
                                            let heap_2 = *heap_2;
                                            self::JavaScriptArraySpreadTransition::ArraySpreadContinue(Box::new(heap_2.clone()), Box::new({
                                                let source_ml_source2_nominal_field27464 = shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source2_nominal_field27466 = shared_constructor_field2.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27468 = shared_constructor_field3.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27470 = shared_constructor_field4;
                                                            {
                                                                let source_ml_source2_nominal_field27472 = shared_constructor_field5.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field27474 = shared_constructor_field6.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field27476 = shared_constructor_field7.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field27478 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_override27481 = (shared_constructor_field4 + 1f64);
                                                                                {
                                                                                    let source_ml_source2_nominal_override27482 = self::JavaScriptSemanticValue::Undefined;
                                                                                    self::JavaScriptArraySpreadState::ArraySpreadState(Box::new(source_ml_source2_nominal_field27464.clone()), Box::new(source_ml_source2_nominal_field27466.clone()), Box::new(source_ml_source2_nominal_field27468.clone()), source_ml_source2_nominal_override27481, self::ml_string::Text::from_units(vec![115, 116, 101, 112]), Box::new(source_ml_source2_nominal_override27482.clone()), source_ml_source2_nominal_field27476.clone(), source_ml_source2_nominal_field27478.clone())
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
                                        }
                                        self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(heap_3, reason) => {
                                            let heap_3 = *heap_3;
                                            self::JavaScriptArraySpreadTransition::ArraySpreadError(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]), reason.clone())
                                        }
                                        self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(heap_4, reason_2) => {
                                            let heap_4 = *heap_4;
                                            self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(Box::new(heap_4.clone()), reason_2.clone())
                                        }
                                        self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(heap_5, reason_3) => {
                                            let heap_5 = *heap_5;
                                            self::JavaScriptArraySpreadTransition::ArraySpreadResourceLimit(Box::new(heap_5.clone()), Box::new(state.clone()), reason_3.clone())
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
pub fn resume_java_script_array_spread(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySpreadState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
    primitive_get: bool,
) -> self::JavaScriptArraySpreadTransition {
    {
        let error = self::read_java_script_array_spread_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            if (((match state.clone() {
                self::JavaScriptArraySpreadState::ArraySpreadState(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value27484,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value27484.clone(),
            }) != (self::ml_string::Text::from_units(vec![
                105, 116, 101, 114, 97, 116, 111, 114,
            ]))) || match (
                match state.clone() {
                    self::JavaScriptArraySpreadState::ArraySpreadState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value27485,
                        _,
                    ) => ((source_ml_source2_data_property_value27485
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) {
                self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        115, 112, 114, 101, 97, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105,
                        111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115,
                        32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100,
                        32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 111, 112, 101, 114, 97, 116,
                        105, 111, 110,
                    ]),
                )
            } else {
                self::continue_java_script_array_spread_iterator(
                    heap.clone(),
                    state.clone(),
                    if primitive_get {
                        self::resume_java_script_iterator_primitive_get(
                            heap.clone(),
                            (match state.clone() {
                                self::JavaScriptArraySpreadState::ArraySpreadState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value27486,
                                    _,
                                ) => ((source_ml_source2_data_property_value27486
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
                        )
                    } else {
                        self::resume_java_script_iterator_protocol_call(
                            heap.clone(),
                            (match state.clone() {
                                self::JavaScriptArraySpreadState::ArraySpreadState(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value27487,
                                    _,
                                ) => ((source_ml_source2_data_property_value27487
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
                        )
                    },
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn provide_java_script_array_spread_primitive_prototype(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySpreadState,
    prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptArraySpreadTransition {
    {
        let error = self::read_java_script_array_spread_state_error(heap.clone(), state.clone());
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(
                Box::new(heap.clone()),
                error.clone(),
            )
        } else {
            if (((match state.clone() {
                self::JavaScriptArraySpreadState::ArraySpreadState(
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value27489,
                    _,
                    _,
                    _,
                ) => source_ml_source2_data_property_value27489.clone(),
            }) != (self::ml_string::Text::from_units(vec![
                105, 116, 101, 114, 97, 116, 111, 114,
            ]))) || match (
                match state.clone() {
                    self::JavaScriptArraySpreadState::ArraySpreadState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value27490,
                        _,
                    ) => ((source_ml_source2_data_property_value27490
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptIteratorProtocolState>,)))
                    .0)
                        .clone(),
                },
                None::<()>,
            ) {
                (Some(_ml_left), Some(_ml_right)) => false,
                (None, None) => true,
                _ => false,
            }) {
                self::JavaScriptArraySpreadTransition::ArraySpreadInvalid(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        115, 112, 114, 101, 97, 100, 32, 112, 114, 105, 109, 105, 116, 105, 118,
                        101, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 114, 101, 113,
                        117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                        32, 112, 101, 110, 100, 105, 110, 103, 32, 97, 99, 113, 117, 105, 115, 105,
                        116, 105, 111, 110,
                    ]),
                )
            } else {
                self::continue_java_script_array_spread_iterator(
                    heap.clone(),
                    state.clone(),
                    self::provide_java_script_iterator_primitive_prototype(
                        heap.clone(),
                        (match state.clone() {
                            self::JavaScriptArraySpreadState::ArraySpreadState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value27491,
                                _,
                            ) => ((source_ml_source2_data_property_value27491
                                .clone()
                                .unwrap_or_else(|| {
                                    (None::<self::JavaScriptIteratorProtocolState>,)
                                }))
                            .0)
                                .clone(),
                        })
                        .clone()
                        .expect("a proven present value"),
                        prototype.clone(),
                    ),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_search_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySearchState,
) -> self::ml_string::Text {
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
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field10 = *shared_constructor_field10;
            {
                let heap_error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (heap_error != (self::ml_string::Text::from_units(vec![]))) {
                    heap_error.clone()
                } else {
                    if (((!(self::test_java_script_array_search_operation(
                        &(shared_constructor_field1),
                    )) || {
                        let source_ml_source2_s27494 = shared_constructor_field2.clone();
                        match source_ml_source2_s27494.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => false,
                            _ => true,
                        }
                    }) || {
                        let source_ml_source2_s27499 = shared_constructor_field6.clone();
                        match source_ml_source2_s27499.clone() {
                            self::JavaScriptSemanticValue::Symbol(_) => false,
                            _ => true,
                        }
                    }) || (({
                        let source_ml_source2_type_query_operand27505 = shared_constructor_field5;
                        self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                    }) != (self::ml_string::Text::from_units(vec![
                        98, 111, 111, 108, 101, 97, 110,
                    ])))) {
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101,
                            116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97,
                            108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 44, 32, 79, 98,
                            106, 101, 99, 116, 32, 114, 101, 99, 101, 105, 118, 101, 114, 44, 32,
                            97, 114, 103, 117, 109, 101, 110, 116, 32, 112, 114, 101, 115, 101,
                            110, 99, 101, 32, 97, 110, 100, 32, 99, 111, 110, 118, 101, 114, 115,
                            105, 111, 110, 32, 83, 121, 109, 98, 111, 108,
                        ])
                    } else {
                        {
                            let source_ml_source1_temporary_values1231 =
                                std::rc::Rc::new(std::cell::RefCell::new(vec![
                                    shared_constructor_field2.clone(),
                                    shared_constructor_field3.clone(),
                                    shared_constructor_field4.clone(),
                                    shared_constructor_field6.clone(),
                                    shared_constructor_field10.clone(),
                                ]));
                            {
                                let source_ml_source1_temporary_index1231 =
                                    self::ml::Big::from_i128(0);
                                {
                                    let source_ml_source2_s27511 = self::source_ml_source1_temporary_read_java_script_array_search_state_error_loop1233(heap.clone(), source_ml_source1_temporary_values1231.clone(), source_ml_source1_temporary_index1231.clone());
                                    match source_ml_source2_s27511.clone() {
                                        self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Result::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Done => {
                                            if ((((((!(self::ml_math::is_safe_integer(shared_constructor_field7)) || (shared_constructor_field7 < (0f64))) || {
                                                let source_ml_source2_same_value_operand27516 = shared_constructor_field7;
                                                {
                                                    let source_ml_source2_same_value_operand27517 = (-0f64);
                                                    (((source_ml_source2_same_value_operand27516 == source_ml_source2_same_value_operand27517) && ((source_ml_source2_same_value_operand27516 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand27516)) == ((1f64 / source_ml_source2_same_value_operand27517))))) || ((source_ml_source2_same_value_operand27516 != source_ml_source2_same_value_operand27516) && (source_ml_source2_same_value_operand27517 != source_ml_source2_same_value_operand27517)))
                                                }
                                            }) || !(self::ml_math::is_safe_integer(shared_constructor_field8))) || (shared_constructor_field8 < ((-1f64)))) || (shared_constructor_field8 > shared_constructor_field7)) || {
                                                let source_ml_source2_same_value_operand27526 = shared_constructor_field8;
                                                {
                                                    let source_ml_source2_same_value_operand27527 = (-0f64);
                                                    (((source_ml_source2_same_value_operand27526 == source_ml_source2_same_value_operand27527) && ((source_ml_source2_same_value_operand27526 != (0f64)) || (((1f64 / source_ml_source2_same_value_operand27526)) == ((1f64 / source_ml_source2_same_value_operand27527))))) || ((source_ml_source2_same_value_operand27526 != source_ml_source2_same_value_operand27526) && (source_ml_source2_same_value_operand27527 != source_ml_source2_same_value_operand27527)))
                                                }
                                            }) {
                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 99, 97, 112, 116, 117, 114, 101, 100, 32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 105, 110, 100, 101, 120])
                                            } else {
                                                if !(self::ml_array::includes(&{ let ml_reference_value1534 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101]), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101]), self::ml_string::Text::from_units(vec![114, 101, 97, 100, 121]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value1534 }, &shared_constructor_field9.clone(), 0f64, false, |a, b| a == b)) {
                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 112, 104, 97, 115, 101])
                                                } else {
                                                    {
                                                        let source_ml_source2_s27546 = if ((shared_constructor_field9 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]))) || (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])))) {
                                                            if (((match (match state.clone() {
                                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27551, _) => {
                                                                    ((source_ml_source2_data_property_value27551.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || ((match (match state.clone() {
                                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27554, _) => {
                                                                    ((source_ml_source2_data_property_value27554.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value") {
                                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, _, source_ml_source2_data_property_value27555, _) => {
                                                                    source_ml_source2_data_property_value27555.clone()
                                                                }
                                                            }) != (self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27556, _) => {
                                                                    ((source_ml_source2_data_property_value27556.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value") {
                                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(source_ml_source2_data_property_value27557, _, _, _) => {
                                                                    let source_ml_source2_data_property_value27557 = *source_ml_source2_data_property_value27557;
                                                                    source_ml_source2_data_property_value27557.clone()
                                                                }
                                                            }, shared_constructor_field10.clone()))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27559, _) => {
                                                                    ((source_ml_source2_data_property_value27559.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }).clone().expect("a proven present value") {
                                                                self::JavaScriptPrimitiveConversionState::PrimitiveConversion(_, source_ml_source2_data_property_value27560, _, _) => {
                                                                    let source_ml_source2_data_property_value27560 = *source_ml_source2_data_property_value27560;
                                                                    source_ml_source2_data_property_value27560.clone()
                                                                }
                                                            }, shared_constructor_field6.clone()))) {
                                                                self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97, 110, 100, 44, 32, 104, 105, 110, 116, 32, 97, 110, 100, 32, 83, 121, 109, 98, 111, 108]))
                                                            } else {
                                                                {
                                                                    let error = self::read_java_script_primitive_conversion_state_error(heap.clone(), (match state.clone() {
                                                                        self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27562, _) => {
                                                                            ((source_ml_source2_data_property_value27562.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                        }
                                                                    }).clone().expect("a proven present value"));
                                                                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                                                                        self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234Return(error.clone())
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234Next
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            if !(match (match state.clone() {
                                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27563, _) => {
                                                                    ((source_ml_source2_data_property_value27563.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,))).0).clone()
                                                                }
                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234Return(self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234Next
                                                            }
                                                        };
                                                        match source_ml_source2_s27546.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234Next => {
                                                                if ((shared_constructor_field9 == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]))) && ((shared_constructor_field8 < (0f64)) || (shared_constructor_field8 >= shared_constructor_field7))) {
                                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 103, 101, 116, 116, 101, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 105, 110, 45, 114, 97, 110, 103, 101, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101])
                                                                } else {
                                                                    {
                                                                        let source_ml_source2_s27599 = if !(match (match state.clone() {
                                                                            self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27600) => {
                                                                                ((source_ml_source2_data_property_value27600.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,))).0).clone()
                                                                            }
                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                            if (((!(self::ml_array::includes(&{ let ml_reference_value1535 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![105, 110, 100, 101, 120, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value1535 }, &shared_constructor_field9.clone(), 0f64, false, |a, b| a == b)) || {
                                                                                let source_ml_source2_s27604 = match (match state.clone() {
                                                                                    self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27607) => {
                                                                                        ((source_ml_source2_data_property_value27607.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value") {
                                                                                    self::JavaScriptArraySearchCall::ArraySearchSelectedCall(source_ml_source2_data_property_value27608, _, _) => {
                                                                                        let source_ml_source2_data_property_value27608 = *source_ml_source2_data_property_value27608;
                                                                                        source_ml_source2_data_property_value27608.clone()
                                                                                    }
                                                                                };
                                                                                match source_ml_source2_s27604.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }
                                                                            }) || !(((match match (match state.clone() {
                                                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27609) => {
                                                                                    ((source_ml_source2_data_property_value27609.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value") {
                                                                                self::JavaScriptArraySearchCall::ArraySearchSelectedCall(source_ml_source2_data_property_value27610, _, _) => {
                                                                                    let source_ml_source2_data_property_value27610 = *source_ml_source2_data_property_value27610;
                                                                                    source_ml_source2_data_property_value27610.clone()
                                                                                }
                                                                            } {
                                                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value27611) => {
                                                                                    Some((source_ml_source2_data_property_value27611,))
                                                                                }
                                                                                _ => {
                                                                                    None::<(bool,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0)) || (({ let ml_reference_array1536 = match (match state.clone() {
                                                                                self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27614) => {
                                                                                    ((source_ml_source2_data_property_value27614.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value") {
                                                                                self::JavaScriptArraySearchCall::ArraySearchSelectedCall(_, _, source_ml_source2_data_property_value27615) => {
                                                                                    source_ml_source2_data_property_value27615.clone()
                                                                                }
                                                                            }; let ml_reference_projection1536 = ml_reference_array1536.borrow().len() as f64; ml_reference_projection1536 }) > (3f64))) {
                                                                                self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 103, 101, 116, 116, 101, 114, 32, 111, 114, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 99, 97, 108, 108]))
                                                                            } else {
                                                                                {
                                                                                    let source_ml_source1_temporary_values1232 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append(vec![match (match state.clone() {
                                                                                        self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27616) => {
                                                                                            ((source_ml_source2_data_property_value27616.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value") {
                                                                                        self::JavaScriptArraySearchCall::ArraySearchSelectedCall(source_ml_source2_data_property_value27617, _, _) => {
                                                                                            let source_ml_source2_data_property_value27617 = *source_ml_source2_data_property_value27617;
                                                                                            source_ml_source2_data_property_value27617.clone()
                                                                                        }
                                                                                    }, match (match state.clone() {
                                                                                        self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27618) => {
                                                                                            ((source_ml_source2_data_property_value27618.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value") {
                                                                                        self::JavaScriptArraySearchCall::ArraySearchSelectedCall(_, source_ml_source2_data_property_value27619, _) => {
                                                                                            let source_ml_source2_data_property_value27619 = *source_ml_source2_data_property_value27619;
                                                                                            source_ml_source2_data_property_value27619.clone()
                                                                                        }
                                                                                    }], { let ml_reference_value1537 = (match (match state.clone() {
                                                                                        self::JavaScriptArraySearchState::ArraySearchState(_, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value27620) => {
                                                                                            ((source_ml_source2_data_property_value27620.clone().unwrap_or_else(|| (None::<self::JavaScriptArraySearchCall>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value") {
                                                                                        self::JavaScriptArraySearchCall::ArraySearchSelectedCall(_, _, source_ml_source2_data_property_value27621) => {
                                                                                            source_ml_source2_data_property_value27621.clone()
                                                                                        }
                                                                                    }).borrow().clone(); ml_reference_value1537 })));
                                                                                    {
                                                                                        let source_ml_source1_temporary_index1232 = self::ml::Big::from_i128(0);
                                                                                        {
                                                                                            let source_ml_source2_s27622 = self::source_ml_source1_temporary_read_java_script_array_search_state_error_loop1236(heap.clone(), source_ml_source1_temporary_values1232.clone(), source_ml_source1_temporary_index1232.clone());
                                                                                            match source_ml_source2_s27622.clone() {
                                                                                                self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Result::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Done => {
                                                                                                    self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235Next
                                                                                                }
                                                                                                self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Result::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1236Return(source_ml_source1_temporary_result1236) => {
                                                                                                    self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235Return(source_ml_source1_temporary_result1236.clone())
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        } else {
                                                                            if ((shared_constructor_field9 == (self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]))) || (shared_constructor_field9 == (self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114])))) {
                                                                                self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 115, 101, 97, 114, 99, 104, 32, 103, 101, 116, 116, 101, 114, 32, 112, 104, 97, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 99, 97, 108, 108]))
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235Next
                                                                            }
                                                                        };
                                                                        match source_ml_source2_s27599.clone() {
                                                                            self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235Next => {
                                                                                self::ml_string::Text::from_units(vec![])
                                                                            }
                                                                            self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1235Return(source_ml_source1_temporary_result1235) => {
                                                                                source_ml_source1_temporary_result1235.clone()
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234::MlSource1TemporaryReadJavaScriptArraySearchStateErrorJoin1234Return(source_ml_source1_temporary_result1236_2) => {
                                                                source_ml_source1_temporary_result1236_2.clone()
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        self::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Result::MlSource1TemporaryReadJavaScriptArraySearchStateErrorLoop1233Return(source_ml_source1_temporary_result1236_3) => {
                                            source_ml_source1_temporary_result1236_3.clone()
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
pub fn request_java_script_array_search_call(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArraySearchState,
    callee: self::JavaScriptSemanticValue,
    receiver: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
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
            self::JavaScriptArraySearchTransition::ArraySearchCall(
                Box::new(heap.clone()),
                Box::new({
                    let source_ml_source2_nominal_field27628 = shared_constructor_field1.clone();
                    {
                        let source_ml_source2_nominal_field27630 =
                            shared_constructor_field2.clone();
                        {
                            let source_ml_source2_nominal_field27632 =
                                shared_constructor_field3.clone();
                            {
                                let source_ml_source2_nominal_field27634 =
                                    shared_constructor_field4.clone();
                                {
                                    let source_ml_source2_nominal_field27636 =
                                        shared_constructor_field5;
                                    {
                                        let source_ml_source2_nominal_field27638 =
                                            shared_constructor_field6.clone();
                                        {
                                            let source_ml_source2_nominal_field27640 =
                                                shared_constructor_field7;
                                            {
                                                let source_ml_source2_nominal_field27642 =
                                                    shared_constructor_field8;
                                                {
                                                    let source_ml_source2_nominal_field27644 =
                                                        shared_constructor_field9.clone();
                                                    {
                                                        let source_ml_source2_nominal_field27646 =
                                                            shared_constructor_field10.clone();
                                                        {
                                                            let source_ml_source2_nominal_field27648 =
                                                                shared_constructor_field11.clone();
                                                            {
                                                                let source_ml_source2_nominal_field27650 =
                                                                    shared_constructor_field12
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_override27652 = self::JavaScriptArraySearchCall::ArraySearchSelectedCall(Box::new(callee.clone()), Box::new(receiver.clone()), arguments_.clone());
                                                                    self::JavaScriptArraySearchState::ArraySearchState(source_ml_source2_nominal_field27628.clone(), Box::new(source_ml_source2_nominal_field27630.clone()), Box::new(source_ml_source2_nominal_field27632.clone()), Box::new(source_ml_source2_nominal_field27634.clone()), source_ml_source2_nominal_field27636, Box::new(source_ml_source2_nominal_field27638.clone()), source_ml_source2_nominal_field27640, source_ml_source2_nominal_field27642, source_ml_source2_nominal_field27644.clone(), Box::new(source_ml_source2_nominal_field27646.clone()), source_ml_source2_nominal_field27648.clone(), Some((Some(source_ml_source2_nominal_override27652.clone()),)))
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
                Box::new(callee.clone()),
                Box::new(receiver.clone()),
                arguments_.clone(),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_search_missing_value(
    state: self::JavaScriptArraySearchState,
) -> self::JavaScriptSemanticValue {
    match state.clone() {
        self::JavaScriptArraySearchState::ArraySearchState(
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
            _,
            _,
        ) => {
            if (shared_constructor_field1
                == (self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 105, 110, 99, 108, 117, 100, 101, 115,
                ])))
            {
                self::JavaScriptSemanticValue::Boolean(false)
            } else {
                if (shared_constructor_field1
                    == (self::ml_string::Text::from_units(vec![
                        65, 114, 114, 97, 121, 46, 97, 116,
                    ])))
                {
                    self::JavaScriptSemanticValue::Undefined
                } else {
                    self::JavaScriptSemanticValue::Number((-1f64))
                }
            }
        }
    }
}
