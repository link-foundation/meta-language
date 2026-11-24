// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayIterationTransition;
use super::JavaScriptArrayIteratorCall;
use super::JavaScriptArrayIteratorState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPrimitiveConversionState;
use super::JavaScriptScalarConversionTransition;
use super::JavaScriptSemanticValue;
use super::begin_java_script_scalar_protocol;
use super::find_java_script_ordinary_object;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_array_like_length;
use super::replace_java_script_array_iterator_slots;
use super::replace_java_script_ordinary_object;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn finish_java_script_array_iterator_cursor(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayIteratorState,
    done: bool,
) -> self::JavaScriptOrdinaryHeap {
    match state.clone() {
        self::JavaScriptArrayIteratorState::ArrayIteratorState(
            shared_constructor_field2,
            _,
            _,
            shared_constructor_field3,
            _,
            _,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let iterator = if {
                        let source_ml_source2_s24099 = shared_constructor_field2.clone();
                        match source_ml_source2_s24099.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => true,
                            _ => false,
                        }
                    } {
                        {
                            let source_ml_source2_array_read_value24107 =
                                shared_constructor_field1.clone();
                            {
                                let source_ml_source2_array_read_index24108 =
                                    self::find_java_script_ordinary_object(
                                        shared_constructor_field1.clone(),
                                        (((match shared_constructor_field2.clone() {
                                            self::JavaScriptSemanticValue::Object(
                                                source_ml_source2_data_property_value24106,
                                                _,
                                            ) => {
                                                Some((source_ml_source2_data_property_value24106
                                                    .clone(),))
                                            }
                                            _ => None::<(self::ml::Big,)>,
                                        })
                                        .clone()
                                        .expect("a proven present value"))
                                        .0)
                                            .clone(),
                                    );
                                {
                                    let ml_reference_array1256 =
                                        source_ml_source2_array_read_value24107.clone();
                                    let ml_reference_position1256 =
                                        source_ml_source2_array_read_index24108;
                                    let ml_reference_projection1256 = self::ml_array::optional_at(
                                        &*ml_reference_array1256.borrow(),
                                        self::ml_array::number_index(ml_reference_position1256),
                                    );
                                    ml_reference_projection1256
                                }
                            }
                        }
                    } else {
                        None::<self::JavaScriptOrdinaryObject>
                    };
                    if (match (iterator.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } || match iterator.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(
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
                        ) => false,
                        _ => true,
                    }) {
                        heap.clone()
                    } else {
                        self::replace_java_script_ordinary_object(
                            heap.clone(),
                            self::replace_java_script_array_iterator_slots(
                                iterator.clone().expect("a proven present value"),
                                if done {
                                    self::ml::Big::from_i128(0)
                                } else {
                                    (((match iterator.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, source_ml_source2_data_property_value24120, _, _, _, _, _) => {
                                        Some((source_ml_source2_data_property_value24120.clone(),))
                                    }
                                    _ => {
                                        None::<(self::ml::Big,)>
                                    }
                                }).clone().expect("a proven present value")).0).clone()
                                },
                                if done {
                                    0f64
                                } else {
                                    (shared_constructor_field3 + 1f64)
                                },
                                false,
                            ),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn continue_java_script_array_iterator_length(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayIteratorState,
    transition: self::JavaScriptScalarConversionTransition,
) -> self::JavaScriptArrayIterationTransition {
    {
        let source_ml_source1_temporary_binding1101 = state.clone();
        {
            let previous_call = match source_ml_source1_temporary_binding1101.clone() {
                self::JavaScriptArrayIteratorState::ArrayIteratorState(
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    _,
                    source_ml_source2_data_property_value24122,
                ) => ((source_ml_source2_data_property_value24122
                    .clone()
                    .unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,)))
                .0)
                    .clone(),
            };
            {
                let previous_conversion = match source_ml_source1_temporary_binding1101.clone() {
                    self::JavaScriptArrayIteratorState::ArrayIteratorState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value24123,
                        _,
                    ) => ((source_ml_source2_data_property_value24123
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptPrimitiveConversionState>,)))
                    .0)
                        .clone(),
                };
                {
                    let base = match source_ml_source1_temporary_binding1101.clone() {
                        self::JavaScriptArrayIteratorState::ArrayIteratorState(
                            source_ml_source2_nominal_rest_field24124,
                            source_ml_source2_nominal_rest_field24125,
                            source_ml_source2_nominal_rest_field24126,
                            source_ml_source2_nominal_rest_field24127,
                            source_ml_source2_nominal_rest_field24128,
                            source_ml_source2_nominal_rest_field24129,
                            source_ml_source2_nominal_rest_field24130,
                            _,
                            _,
                        ) => {
                            let source_ml_source2_nominal_rest_field24124 =
                                *source_ml_source2_nominal_rest_field24124;
                            let source_ml_source2_nominal_rest_field24125 =
                                *source_ml_source2_nominal_rest_field24125;
                            let source_ml_source2_nominal_rest_field24126 =
                                *source_ml_source2_nominal_rest_field24126;
                            let source_ml_source2_nominal_rest_field24130 =
                                *source_ml_source2_nominal_rest_field24130;
                            self::JavaScriptArrayIteratorState::ArrayIteratorState(
                                Box::new(source_ml_source2_nominal_rest_field24124.clone()),
                                Box::new(source_ml_source2_nominal_rest_field24125.clone()),
                                Box::new(source_ml_source2_nominal_rest_field24126.clone()),
                                source_ml_source2_nominal_rest_field24127,
                                source_ml_source2_nominal_rest_field24128.clone(),
                                source_ml_source2_nominal_rest_field24129.clone(),
                                Box::new(source_ml_source2_nominal_rest_field24130.clone()),
                                None::<(Option<self::JavaScriptPrimitiveConversionState>,)>,
                                None::<(Option<self::JavaScriptArrayIteratorCall>,)>,
                            )
                        }
                    };
                    match base.clone() {
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
                            match transition.clone() {
                                self::JavaScriptScalarConversionTransition::ScalarContinuation(state_2) => {
                                    let state_2 = *state_2;
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorContinue(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field24132 = shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field24134 = shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field24136 = shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field24138 = shared_constructor_field7;
                                                    {
                                                        let source_ml_source2_nominal_field24140 = shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field24142 = shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field24144 = shared_constructor_field10.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field24146 = shared_constructor_field11.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field24148 = shared_constructor_field12.clone();
                                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(Box::new(source_ml_source2_nominal_field24132.clone()), Box::new(source_ml_source2_nominal_field24134.clone()), Box::new(source_ml_source2_nominal_field24136.clone()), source_ml_source2_nominal_field24138, source_ml_source2_nominal_field24140.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), Box::new(source_ml_source2_nominal_field24144.clone()), Some((Some(state_2.clone()),)), source_ml_source2_nominal_field24148.clone())
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
                                self::JavaScriptScalarConversionTransition::ScalarCall(state_3, callee, receiver, arguments) => {
                                    let state_3 = *state_3;
                                    let callee = *callee;
                                    let receiver = *receiver;
                                    {
                                        let call = self::JavaScriptArrayIteratorCall::ArrayIteratorCall(Box::new(callee.clone()), Box::new(receiver.clone()), arguments.clone());
                                        match call.clone() {
                                            self::JavaScriptArrayIteratorCall::ArrayIteratorCall(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                let shared_constructor_field2 = *shared_constructor_field2;
                                                self::JavaScriptArrayIterationTransition::ArrayIteratorCall(Box::new(heap.clone()), Box::new({
                                                    let source_ml_source2_nominal_field24153 = shared_constructor_field4.clone();
                                                    {
                                                        let source_ml_source2_nominal_field24155 = shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field24157 = shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field24159 = shared_constructor_field7;
                                                                {
                                                                    let source_ml_source2_nominal_field24161 = shared_constructor_field8.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field24163 = shared_constructor_field9.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field24165 = shared_constructor_field10.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field24167 = shared_constructor_field11.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field24169 = shared_constructor_field12.clone();
                                                                                    self::JavaScriptArrayIteratorState::ArrayIteratorState(Box::new(source_ml_source2_nominal_field24153.clone()), Box::new(source_ml_source2_nominal_field24155.clone()), Box::new(source_ml_source2_nominal_field24157.clone()), source_ml_source2_nominal_field24159, source_ml_source2_nominal_field24161.clone(), self::ml_string::Text::from_units(vec![99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), Box::new(source_ml_source2_nominal_field24165.clone()), Some((Some(state_3.clone()),)), Some((Some(call.clone()),)))
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
                                }
                                self::JavaScriptScalarConversionTransition::ScalarCompleted(primitive) => {
                                    let primitive = *primitive;
                                    match primitive.clone() {
                                        self::JavaScriptSemanticValue::Number(value) => {
                                            if ((match state.clone() {
                                                self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, source_ml_source2_data_property_value24178, _, _, _, _, _) => {
                                                    source_ml_source2_data_property_value24178
                                                }
                                            }) >= (self::read_java_script_array_like_length(value))) {
                                                self::JavaScriptArrayIterationTransition::ArrayIteratorComponents(Box::new(self::finish_java_script_array_iterator_cursor(heap.clone(), state.clone(), true)), Box::new(match state.clone() {
                                                    self::JavaScriptArrayIteratorState::ArrayIteratorState(source_ml_source2_data_property_value24179, _, _, _, _, _, _, _, _) => {
                                                        let source_ml_source2_data_property_value24179 = *source_ml_source2_data_property_value24179;
                                                        source_ml_source2_data_property_value24179.clone()
                                                    }
                                                }), true, Box::new(self::JavaScriptSemanticValue::Undefined), Box::new(self::JavaScriptSemanticValue::Undefined), false)
                                            } else {
                                                if ((match state.clone() {
                                                    self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, _, source_ml_source2_data_property_value24181, _, _, _, _) => {
                                                        source_ml_source2_data_property_value24181.clone()
                                                    }
                                                }) == (self::ml_string::Text::from_units(vec![107, 101, 121, 115]))) {
                                                    self::JavaScriptArrayIterationTransition::ArrayIteratorComponents(Box::new(self::finish_java_script_array_iterator_cursor(heap.clone(), state.clone(), false)), Box::new(match state.clone() {
                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(source_ml_source2_data_property_value24182, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value24182 = *source_ml_source2_data_property_value24182;
                                                            source_ml_source2_data_property_value24182.clone()
                                                        }
                                                    }), false, Box::new(self::JavaScriptSemanticValue::Number(match state.clone() {
                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, source_ml_source2_data_property_value24183, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value24183
                                                        }
                                                    })), Box::new(self::JavaScriptSemanticValue::Number(match state.clone() {
                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(_, _, _, source_ml_source2_data_property_value24184, _, _, _, _, _) => {
                                                            source_ml_source2_data_property_value24184
                                                        }
                                                    })), false)
                                                } else {
                                                    self::JavaScriptArrayIterationTransition::ArrayIteratorContinue(Box::new(heap.clone()), Box::new({
                                                        let source_ml_source2_nominal_field24186 = shared_constructor_field4.clone();
                                                        {
                                                            let source_ml_source2_nominal_field24188 = shared_constructor_field5.clone();
                                                            {
                                                                let source_ml_source2_nominal_field24190 = shared_constructor_field6.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field24192 = shared_constructor_field7;
                                                                    {
                                                                        let source_ml_source2_nominal_field24194 = shared_constructor_field8.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field24196 = shared_constructor_field9.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field24198 = shared_constructor_field10.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field24200 = shared_constructor_field11.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field24202 = shared_constructor_field12.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_override24205 = self::JavaScriptSemanticValue::Undefined;
                                                                                            self::JavaScriptArrayIteratorState::ArrayIteratorState(Box::new(source_ml_source2_nominal_field24186.clone()), Box::new(source_ml_source2_nominal_field24188.clone()), Box::new(source_ml_source2_nominal_field24190.clone()), source_ml_source2_nominal_field24192, source_ml_source2_nominal_field24194.clone(), self::ml_string::Text::from_units(vec![118, 97, 108, 117, 101]), Box::new(source_ml_source2_nominal_override24205.clone()), source_ml_source2_nominal_field24200.clone(), source_ml_source2_nominal_field24202.clone())
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
                                            }
                                        }
                                        _ => {
                                            self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 108, 101, 110, 103, 116, 104, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 84, 111, 78, 117, 109, 98, 101, 114, 32, 114, 101, 115, 117, 108, 116]))
                                        }
                                    }
                                }
                                self::JavaScriptScalarConversionTransition::ScalarThrow(value_2) => {
                                    let value_2 = *value_2;
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorThrow(Box::new(self::finish_java_script_array_iterator_cursor(heap.clone(), state.clone(), true)), Box::new(value_2.clone()))
                                }
                                self::JavaScriptScalarConversionTransition::ScalarIntrinsicError(error_class, reason) => {
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorError(Box::new(self::finish_java_script_array_iterator_cursor(heap.clone(), state.clone(), true)), error_class.clone(), reason.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarInvalid(reason_2) => {
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorInvalid(Box::new(heap.clone()), reason_2.clone())
                                }
                                self::JavaScriptScalarConversionTransition::ScalarResourceLimit(value_3, reason_3) => {
                                    let value_3 = *value_3;
                                    self::JavaScriptArrayIterationTransition::ArrayIteratorResourceLimit(Box::new(heap.clone()), Box::new({
                                        let source_ml_source2_nominal_field24207 = shared_constructor_field4.clone();
                                        {
                                            let source_ml_source2_nominal_field24209 = shared_constructor_field5.clone();
                                            {
                                                let source_ml_source2_nominal_field24211 = shared_constructor_field6.clone();
                                                {
                                                    let source_ml_source2_nominal_field24213 = shared_constructor_field7;
                                                    {
                                                        let source_ml_source2_nominal_field24215 = shared_constructor_field8.clone();
                                                        {
                                                            let source_ml_source2_nominal_field24217 = shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_nominal_field24219 = shared_constructor_field10.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field24221 = shared_constructor_field11.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field24223 = shared_constructor_field12.clone();
                                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(Box::new(source_ml_source2_nominal_field24207.clone()), Box::new(source_ml_source2_nominal_field24209.clone()), Box::new(source_ml_source2_nominal_field24211.clone()), source_ml_source2_nominal_field24213, source_ml_source2_nominal_field24215.clone(), self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101]), Box::new(value_3.clone()), source_ml_source2_nominal_field24221.clone(), source_ml_source2_nominal_field24223.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }), reason_3.clone())
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
pub fn continue_java_script_array_iterator_value(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayIteratorState,
    value: self::JavaScriptSemanticValue,
) -> self::JavaScriptArrayIterationTransition {
    match state.clone() {
        self::JavaScriptArrayIteratorState::ArrayIteratorState(
            shared_constructor_field10,
            _,
            shared_constructor_field11,
            shared_constructor_field12,
            shared_constructor_field13,
            shared_constructor_field14,
            _,
            _,
            _,
        ) => {
            let shared_constructor_field10 = *shared_constructor_field10;
            let shared_constructor_field11 = *shared_constructor_field11;
            {
                let previous = match state.clone() {
                    self::JavaScriptArrayIteratorState::ArrayIteratorState(
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        _,
                        source_ml_source2_data_property_value24227,
                    ) => ((source_ml_source2_data_property_value24227
                        .clone()
                        .unwrap_or_else(|| (None::<self::JavaScriptArrayIteratorCall>,)))
                    .0)
                        .clone(),
                };
                {
                    let base = match state.clone() {
                        self::JavaScriptArrayIteratorState::ArrayIteratorState(
                            source_ml_source2_nominal_rest_field24228,
                            source_ml_source2_nominal_rest_field24229,
                            source_ml_source2_nominal_rest_field24230,
                            source_ml_source2_nominal_rest_field24231,
                            source_ml_source2_nominal_rest_field24232,
                            source_ml_source2_nominal_rest_field24233,
                            source_ml_source2_nominal_rest_field24234,
                            source_ml_source2_nominal_rest_field24235,
                            _,
                        ) => {
                            let source_ml_source2_nominal_rest_field24228 =
                                *source_ml_source2_nominal_rest_field24228;
                            let source_ml_source2_nominal_rest_field24229 =
                                *source_ml_source2_nominal_rest_field24229;
                            let source_ml_source2_nominal_rest_field24230 =
                                *source_ml_source2_nominal_rest_field24230;
                            let source_ml_source2_nominal_rest_field24234 =
                                *source_ml_source2_nominal_rest_field24234;
                            self::JavaScriptArrayIteratorState::ArrayIteratorState(
                                Box::new(source_ml_source2_nominal_rest_field24228.clone()),
                                Box::new(source_ml_source2_nominal_rest_field24229.clone()),
                                Box::new(source_ml_source2_nominal_rest_field24230.clone()),
                                source_ml_source2_nominal_rest_field24231,
                                source_ml_source2_nominal_rest_field24232.clone(),
                                source_ml_source2_nominal_rest_field24233.clone(),
                                Box::new(source_ml_source2_nominal_rest_field24234.clone()),
                                source_ml_source2_nominal_rest_field24235.clone(),
                                None::<(Option<self::JavaScriptArrayIteratorCall>,)>,
                            )
                        }
                    };
                    match base.clone() {
                        self::JavaScriptArrayIteratorState::ArrayIteratorState(
                            shared_constructor_field1,
                            shared_constructor_field2,
                            shared_constructor_field3,
                            shared_constructor_field4,
                            shared_constructor_field5,
                            shared_constructor_field6,
                            shared_constructor_field7,
                            shared_constructor_field8,
                            shared_constructor_field9,
                        ) => {
                            let shared_constructor_field1 = *shared_constructor_field1;
                            let shared_constructor_field2 = *shared_constructor_field2;
                            let shared_constructor_field3 = *shared_constructor_field3;
                            let shared_constructor_field7 = *shared_constructor_field7;
                            if ((shared_constructor_field14
                                == (self::ml_string::Text::from_units(vec![
                                    108, 101, 110, 103, 116, 104,
                                ])))
                                || (shared_constructor_field14
                                    == (self::ml_string::Text::from_units(vec![
                                        108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101,
                                        114,
                                    ]))))
                            {
                                {
                                    let next = {
                                        let source_ml_source2_nominal_field24241 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field24243 =
                                                shared_constructor_field2.clone();
                                            {
                                                let source_ml_source2_nominal_field24245 =
                                                    shared_constructor_field3.clone();
                                                {
                                                    let source_ml_source2_nominal_field24247 =
                                                        shared_constructor_field4;
                                                    {
                                                        let source_ml_source2_nominal_field24249 =
                                                            shared_constructor_field5.clone();
                                                        {
                                                            let source_ml_source2_nominal_field24251 =
                                                                shared_constructor_field6.clone();
                                                            {
                                                                let source_ml_source2_nominal_field24253 =
                                                                    shared_constructor_field7
                                                                        .clone();
                                                                {
                                                                    let source_ml_source2_nominal_field24255 =
                                                                        shared_constructor_field8
                                                                            .clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field24257 = shared_constructor_field9.clone();
                                                                        self::JavaScriptArrayIteratorState::ArrayIteratorState(Box::new(source_ml_source2_nominal_field24241.clone()), Box::new(source_ml_source2_nominal_field24243.clone()), Box::new(source_ml_source2_nominal_field24245.clone()), source_ml_source2_nominal_field24247, source_ml_source2_nominal_field24249.clone(), source_ml_source2_nominal_field24251.clone(), Box::new(value.clone()), source_ml_source2_nominal_field24255.clone(), source_ml_source2_nominal_field24257.clone())
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    self::continue_java_script_array_iterator_length(
                                        heap.clone(),
                                        next.clone(),
                                        if match value.clone() {
                                            self::JavaScriptSemanticValue::Object(_, _) => true,
                                            _ => false,
                                        } {
                                            self::begin_java_script_scalar_protocol(
                                                heap.clone(),
                                                value.clone(),
                                                shared_constructor_field11.clone(),
                                                &(self::ml_string::Text::from_units(vec![
                                                    99, 111, 101, 114, 99, 101, 32, 112, 114, 105,
                                                    109, 105, 116, 105, 118, 101, 32, 110, 117,
                                                    109, 98, 101, 114,
                                                ])),
                                            )
                                        } else {
                                            self::finish_java_script_scalar_primitive(
                                                &(self::ml_string::Text::from_units(vec![
                                                    99, 111, 101, 114, 99, 101, 32, 112, 114, 105,
                                                    109, 105, 116, 105, 118, 101, 32, 110, 117,
                                                    109, 98, 101, 114,
                                                ])),
                                                value.clone(),
                                            )
                                        },
                                    )
                                }
                            } else {
                                self::JavaScriptArrayIterationTransition::ArrayIteratorComponents(
                                    Box::new(self::finish_java_script_array_iterator_cursor(
                                        heap.clone(),
                                        state.clone(),
                                        false,
                                    )),
                                    Box::new(shared_constructor_field10.clone()),
                                    false,
                                    Box::new(self::JavaScriptSemanticValue::Number(
                                        shared_constructor_field12,
                                    )),
                                    Box::new(value.clone()),
                                    (shared_constructor_field13
                                        == (self::ml_string::Text::from_units(vec![
                                            101, 110, 116, 114, 105, 101, 115,
                                        ]))),
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}
