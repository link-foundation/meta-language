// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArgumentRestState;
use super::JavaScriptArgumentRestTransition;
use super::JavaScriptArrayCompletion;
use super::JavaScriptIndexedPropertyCreation;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Result;
use super::allocate_java_script_array;
use super::create_java_script_indexed_data_property;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_read_java_script_argument_rest_state_error_loop1063;
use super::source_ml_source1_temporary_step_java_script_argument_rest_loop1065;
use super::source_ml_source3_record_order;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_argument_rest_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    state: self::JavaScriptArgumentRestState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArgumentRestState::ArgumentRestState(
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field6 = *shared_constructor_field6;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = self::read_java_script_ordinary_heap_error(heap.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        error.clone()
                    } else {
                        if ((((((((((match state.clone() {
                            self::JavaScriptArgumentRestState::ArgumentRestState(_, _, _, _, _) => {
                                false
                            }
                        } || (({
                            let ml_reference_array1185 = arguments_.clone();
                            let ml_reference_projection1185 =
                                ml_reference_array1185.borrow().len() as f64;
                            ml_reference_projection1185
                        }) > (1048576f64)))
                            || !(self::ml_math::is_safe_integer(shared_constructor_field3)))
                            || (shared_constructor_field3 < (0f64)))
                            || {
                                let source_ml_source2_same_value_operand23365 =
                                    shared_constructor_field3;
                                {
                                    let source_ml_source2_same_value_operand23366 = (-0f64);
                                    (((source_ml_source2_same_value_operand23365
                                        == source_ml_source2_same_value_operand23366)
                                        && ((source_ml_source2_same_value_operand23365 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand23365)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand23366))))
                                        || ((source_ml_source2_same_value_operand23365
                                            != source_ml_source2_same_value_operand23365)
                                            && (source_ml_source2_same_value_operand23366
                                                != source_ml_source2_same_value_operand23366)))
                                }
                            })
                            || (shared_constructor_field3 > (1048576f64)))
                            || (shared_constructor_field4
                                != (self::ml_math::max(
                                    0f64,
                                    ({
                                        let ml_reference_array1186 = arguments_.clone();
                                        let ml_reference_projection1186 =
                                            ml_reference_array1186.borrow().len() as f64;
                                        ml_reference_projection1186
                                    } - shared_constructor_field3),
                                ))))
                            || !(self::ml_math::is_safe_integer(shared_constructor_field5)))
                            || (shared_constructor_field5 < (0f64)))
                            || {
                                let source_ml_source2_same_value_operand23377 =
                                    shared_constructor_field5;
                                {
                                    let source_ml_source2_same_value_operand23378 = (-0f64);
                                    (((source_ml_source2_same_value_operand23377
                                        == source_ml_source2_same_value_operand23378)
                                        && ((source_ml_source2_same_value_operand23377 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand23377)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand23378))))
                                        || ((source_ml_source2_same_value_operand23377
                                            != source_ml_source2_same_value_operand23377)
                                            && (source_ml_source2_same_value_operand23378
                                                != source_ml_source2_same_value_operand23378)))
                                }
                            })
                            || (shared_constructor_field5 > shared_constructor_field4))
                        {
                            self::ml_string::Text::from_units(vec![
                                97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 115, 116, 32,
                                114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99,
                                116, 117, 97, 108, 32, 102, 114, 97, 109, 101, 32, 97, 114, 103,
                                117, 109, 101, 110, 116, 32, 114, 97, 110, 103, 101, 32, 97, 110,
                                100, 32, 99, 111, 112, 121, 32, 99, 111, 111, 114, 100, 105, 110,
                                97, 116, 101,
                            ])
                        } else {
                            {
                                let source_ml_source1_temporary_values1061 = std::rc::Rc::new(
                                    std::cell::RefCell::new(self::ml_array::append(
                                        vec![
                                            shared_constructor_field2.clone(),
                                            shared_constructor_field6.clone(),
                                        ],
                                        {
                                            let ml_reference_value1187 =
                                                arguments_.borrow().clone();
                                            ml_reference_value1187
                                        },
                                    )),
                                );
                                {
                                    let source_ml_source1_temporary_index1061 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s23385 = self::source_ml_source1_temporary_read_java_script_argument_rest_state_error_loop1063(heap.clone(), source_ml_source1_temporary_values1061.clone(), source_ml_source1_temporary_index1061.clone());
                                        match source_ml_source2_s23385.clone() {
                                                self::MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Result::MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Done => {
                                                    if ({
                                                        let source_ml_source2_s23387 = shared_constructor_field2.clone();
                                                        match source_ml_source2_s23387.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    } || ((match shared_constructor_field2.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value23392) => {
                                                            Some((source_ml_source2_data_property_value23392,))
                                                        }
                                                        _ => {
                                                            None::<(bool,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0) {
                                                        self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 115, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                    } else {
                                                        if {
                                                            let source_ml_source2_s23394 = shared_constructor_field6.clone();
                                                            match source_ml_source2_s23394.clone() {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    true
                                                                }
                                                                _ => {
                                                                    false
                                                                }
                                                            }
                                                        } {
                                                            if (shared_constructor_field5 == (0f64)) {
                                                                self::ml_string::Text::from_units(vec![])
                                                            } else {
                                                                self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 115, 116, 32, 99, 97, 110, 110, 111, 116, 32, 97, 100, 118, 97, 110, 99, 101, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110])
                                                            }
                                                        } else {
                                                            {
                                                                let target = if {
                                                                    let source_ml_source2_s23399 = shared_constructor_field6.clone();
                                                                    match source_ml_source2_s23399.clone() {
                                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                                            true
                                                                        }
                                                                        _ => {
                                                                            false
                                                                        }
                                                                    }
                                                                } {
                                                                    {
                                                                        let source_ml_source2_array_read_value23407 = shared_constructor_field1.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index23408 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match shared_constructor_field6.clone() {
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value23406, _) => {
                                                                                    Some((source_ml_source2_data_property_value23406.clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(self::ml::Big,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone());
                                                                            { let ml_reference_array1188 = source_ml_source2_array_read_value23407.clone(); let ml_reference_position1188 = source_ml_source2_array_read_index23408; let ml_reference_projection1188 = self::ml_array::optional_at(&*ml_reference_array1188.borrow(), self::ml_array::number_index(ml_reference_position1188)); ml_reference_projection1188 }
                                                                        }
                                                                    }
                                                                } else {
                                                                    None::<self::JavaScriptOrdinaryObject>
                                                                };
                                                                if {
                                                                    let source_ml_source2_comparison_operand23410 = {
                                                                        let source_ml_source2_optional_chain_receiver23409 = (target.clone(),);
                                                                        if match (source_ml_source2_optional_chain_receiver23409.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                                            Some({
                                                                                let source_ml_source1_temporary_optional_chain1062 = ((source_ml_source2_optional_chain_receiver23409.clone()).0).clone().expect("a proven present value");
                                                                                match source_ml_source1_temporary_optional_chain1062.clone() {
                                                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![99, 108, 111, 115, 117, 114, 101, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 70, 117, 110, 99, 116, 105, 111, 110, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 73, 116, 101, 114, 97, 116, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![109, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![115, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![119, 101, 97, 107, 77, 97, 112, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![119, 101, 97, 107, 83, 101, 116, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![104, 111, 115, 116, 69, 114, 114, 111, 114, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![104, 111, 115, 116, 85, 82, 76, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![104, 111, 115, 116, 65, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, _, _, _) => {
                                                                                        self::ml_string::Text::from_units(vec![104, 111, 115, 116, 66, 121, 116, 101, 66, 117, 102, 102, 101, 114, 79, 98, 106, 101, 99, 116])
                                                                                    }
                                                                                }
                                                                            })
                                                                        } else {
                                                                            None::<self::ml_string::Text>
                                                                        }
                                                                    };
                                                                    match (source_ml_source2_comparison_operand23410.clone(), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 79, 98, 106, 101, 99, 116])) { (Some(ml_left), ml_right) => ml_left == ml_right, _ => false }
                                                                } {
                                                                    self::ml_string::Text::from_units(vec![])
                                                                } else {
                                                                    self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 115, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 101, 119, 108, 121, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 65, 114, 114, 97, 121])
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Result::MlSource1TemporaryReadJavaScriptArgumentRestStateErrorLoop1063Return(source_ml_source1_temporary_result1063) => {
                                                    source_ml_source1_temporary_result1063.clone()
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
pub fn begin_java_script_argument_rest(
    heap: self::JavaScriptOrdinaryHeap,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    from: f64,
    prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptArgumentRestTransition {
    {
        let state = self::JavaScriptArgumentRestState::ArgumentRestState(
            Box::new(prototype.clone()),
            from,
            self::ml_math::max(
                0f64,
                ({
                    let ml_reference_array1189 = arguments_.clone();
                    let ml_reference_projection1189 = ml_reference_array1189.borrow().len() as f64;
                    ml_reference_projection1189
                } - from),
            ),
            0f64,
            Box::new(self::JavaScriptSemanticValue::Undefined),
        );
        {
            let error = self::read_java_script_argument_rest_state_error(
                heap.clone(),
                arguments_.clone(),
                state.clone(),
            );
            if (error == (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptArgumentRestTransition::ArgumentRestContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            } else {
                self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_argument_rest(
    heap: self::JavaScriptOrdinaryHeap,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    state: self::JavaScriptArgumentRestState,
) -> self::JavaScriptArgumentRestTransition {
    match state.clone() {
        self::JavaScriptArgumentRestState::ArgumentRestState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field5 = *shared_constructor_field5;
            {
                let error = self::read_java_script_argument_rest_state_error(
                    heap.clone(),
                    arguments_.clone(),
                    state.clone(),
                );
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    if {
                        let source_ml_source2_s23413 = shared_constructor_field5.clone();
                        match source_ml_source2_s23413.clone() {
                            self::JavaScriptSemanticValue::Undefined => true,
                            _ => false,
                        }
                    } {
                        {
                            let properties = 0f64;
                            {
                                let source_ml_source1_temporary_values1064 = match heap.clone() {
                                    self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                        source_ml_source2_data_property_value23415,
                                        _,
                                        _,
                                        _,
                                        _,
                                        _,
                                    ) => source_ml_source2_data_property_value23415.clone(),
                                };
                                {
                                    let source_ml_source1_temporary_index1064 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let properties_2 = self::source_ml_source1_temporary_step_java_script_argument_rest_loop1065(properties, source_ml_source1_temporary_values1064.clone(), source_ml_source1_temporary_index1064.clone());
                                        if ((({
                                            let ml_reference_array1190 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                    source_ml_source2_data_property_value23417,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => source_ml_source2_data_property_value23417
                                                    .clone(),
                                            };
                                            let ml_reference_projection1190 =
                                                ml_reference_array1190.borrow().len() as f64;
                                            ml_reference_projection1190
                                        }) >= (4096f64))
                                            || (properties_2 >= (16384f64)))
                                        {
                                            self::JavaScriptArgumentRestTransition::ArgumentRestResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![97, 114, 103, 117, 109, 101, 110, 116, 32, 114, 101, 115, 116, 32, 65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 104, 97, 114, 101, 100, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                        } else {
                                            self::JavaScriptArgumentRestTransition::ArgumentRestAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new(shared_constructor_field1.clone()))
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if (shared_constructor_field4 == shared_constructor_field3) {
                            self::JavaScriptArgumentRestTransition::ArgumentRestCompleted(
                                Box::new(heap.clone()),
                                Box::new(shared_constructor_field5.clone()),
                            )
                        } else {
                            {
                                let copied = self::create_java_script_indexed_data_property(
                                    heap.clone(),
                                    shared_constructor_field5.clone(),
                                    shared_constructor_field4,
                                    ({
                                        let source_ml_source2_array_read_index23429 =
                                            (shared_constructor_field2 + shared_constructor_field4);
                                        {
                                            let ml_reference_array1191 = arguments_.clone();
                                            let ml_reference_position1191 =
                                                source_ml_source2_array_read_index23429;
                                            let ml_reference_projection1191 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array1191.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position1191,
                                                    ),
                                                );
                                            ml_reference_projection1191
                                        }
                                    })
                                    .clone()
                                    .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                                );
                                match copied.clone() {
                                    self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(heap_2, reason) => {
                                        let heap_2 = *heap_2;
                                        self::JavaScriptArgumentRestTransition::ArgumentRestResourceLimit(Box::new(heap_2.clone()), Box::new(state.clone()), reason.clone())
                                    }
                                    _ => {
                                        match copied.clone() {
                                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyCreated(heap_3) => {
                                                let heap_3 = *heap_3;
                                                self::JavaScriptArgumentRestTransition::ArgumentRestContinue(Box::new(heap_3.clone()), Box::new({
                                                    let source_ml_source2_nominal_field23432 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field23434 = shared_constructor_field2;
                                                        {
                                                            let source_ml_source2_nominal_field23436 = shared_constructor_field3;
                                                            {
                                                                let source_ml_source2_nominal_field23438 = shared_constructor_field4;
                                                                {
                                                                    let source_ml_source2_nominal_field23440 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_override23442 = (shared_constructor_field4 + 1f64);
                                                                        self::JavaScriptArgumentRestState::ArgumentRestState(Box::new(source_ml_source2_nominal_field23432.clone()), source_ml_source2_nominal_field23434, source_ml_source2_nominal_field23436, source_ml_source2_nominal_override23442, Box::new(source_ml_source2_nominal_field23440.clone()))
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }))
                                            }
                                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(heap_4, reason_2) => {
                                                let heap_4 = *heap_4;
                                                self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(Box::new(heap_4.clone()), reason_2.clone())
                                            }
                                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(heap_5, reason_3) => {
                                                let heap_5 = *heap_5;
                                                self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(Box::new(heap_5.clone()), reason_3.clone())
                                            }
                                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(heap_6, reason_4) => {
                                                let heap_6 = *heap_6;
                                                self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(Box::new(heap_6.clone()), reason_4.clone())
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
pub fn resume_java_script_argument_rest_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    state: self::JavaScriptArgumentRestState,
    identity: self::ml::Big,
) -> self::JavaScriptArgumentRestTransition {
    match state.clone() {
        self::JavaScriptArgumentRestState::ArgumentRestState(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field5 = *shared_constructor_field5;
            {
                let request = self::step_java_script_argument_rest(
                    heap.clone(),
                    arguments_.clone(),
                    state.clone(),
                );
                match request.clone() {
                    self::JavaScriptArgumentRestTransition::ArgumentRestAllocate(_, _, _) => {
                        let created = self::allocate_java_script_array(
                            heap.clone(),
                            identity.clone(),
                            shared_constructor_field1.clone(),
                            shared_constructor_field3,
                        );
                        match created.clone() {
                            self::JavaScriptArrayCompletion::ArrayNormal(heap_2, value) => {
                                let heap_2 = *heap_2;
                                let value = *value;
                                self::JavaScriptArgumentRestTransition::ArgumentRestContinue(
                                    Box::new(heap_2.clone()),
                                    Box::new({
                                        let source_ml_source2_nominal_field23458 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_nominal_field23460 =
                                                shared_constructor_field2;
                                            {
                                                let source_ml_source2_nominal_field23462 =
                                                    shared_constructor_field3;
                                                {
                                                    let source_ml_source2_nominal_field23464 =
                                                        shared_constructor_field4;
                                                    {
                                                        let source_ml_source2_nominal_field23466 =
                                                            shared_constructor_field5.clone();
                                                        self::JavaScriptArgumentRestState::ArgumentRestState(Box::new(source_ml_source2_nominal_field23458.clone()), source_ml_source2_nominal_field23460, source_ml_source2_nominal_field23462, source_ml_source2_nominal_field23464, Box::new(value.clone()))
                                                    }
                                                }
                                            }
                                        }
                                    }),
                                )
                            }
                            self::JavaScriptArrayCompletion::ArrayPropertyComplete(
                                heap_3,
                                _,
                                _,
                            ) => {
                                let heap_3 = *heap_3;
                                self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(
                                    Box::new(heap_3.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        114, 101, 115, 116, 32, 97, 114, 103, 117, 109, 101, 110,
                                        116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116,
                                        104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102,
                                        114, 101, 115, 104, 32, 65, 114, 114, 97, 121, 32, 99, 114,
                                        101, 97, 116, 105, 111, 110,
                                    ]),
                                )
                            }
                            self::JavaScriptArrayCompletion::ArrayNumberRequest(heap_4, _, _) => {
                                let heap_4 = *heap_4;
                                self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(
                                    Box::new(heap_4.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        114, 101, 115, 116, 32, 97, 114, 103, 117, 109, 101, 110,
                                        116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116,
                                        104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102,
                                        114, 101, 115, 104, 32, 65, 114, 114, 97, 121, 32, 99, 114,
                                        101, 97, 116, 105, 111, 110,
                                    ]),
                                )
                            }
                            self::JavaScriptArrayCompletion::ArrayDefinitionContinue(heap_5, _) => {
                                let heap_5 = *heap_5;
                                self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(
                                    Box::new(heap_5.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        114, 101, 115, 116, 32, 97, 114, 103, 117, 109, 101, 110,
                                        116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116,
                                        104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102,
                                        114, 101, 115, 104, 32, 65, 114, 114, 97, 121, 32, 99, 114,
                                        101, 97, 116, 105, 111, 110,
                                    ]),
                                )
                            }
                            self::JavaScriptArrayCompletion::ArrayInvalid(
                                heap_6,
                                source_ml_source2_f23453,
                            ) => {
                                let heap_6 = *heap_6;
                                self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(
                                    Box::new(heap_6.clone()),
                                    source_ml_source2_f23453.clone(),
                                )
                            }
                            self::JavaScriptArrayCompletion::ArrayError(heap_7, _, _) => {
                                let heap_7 = *heap_7;
                                self::JavaScriptArgumentRestTransition::ArgumentRestInvalid(
                                    Box::new(heap_7.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        114, 101, 115, 116, 32, 97, 114, 103, 117, 109, 101, 110,
                                        116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 116,
                                        104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 102,
                                        114, 101, 115, 104, 32, 65, 114, 114, 97, 121, 32, 99, 114,
                                        101, 97, 116, 105, 111, 110,
                                    ]),
                                )
                            }
                            self::JavaScriptArrayCompletion::ArrayResourceLimit(
                                heap_8,
                                reason,
                                _,
                            ) => {
                                let heap_8 = *heap_8;
                                self::JavaScriptArgumentRestTransition::ArgumentRestResourceLimit(
                                    Box::new(heap_8.clone()),
                                    Box::new(state.clone()),
                                    reason.clone(),
                                )
                            }
                        }
                    }
                    _ => request.clone(),
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn move_java_script_collection_iterator_cursor(
    heap: self::JavaScriptOrdinaryHeap,
    iterator: Option<self::JavaScriptOrdinaryObject>,
    collection: self::ml::Big,
    cursor: f64,
) -> self::JavaScriptOrdinaryHeap {
    if match (iterator.clone(), None::<()>) {
        (Some(_ml_left), Some(_ml_right)) => false,
        (None, None) => true,
        _ => false,
    } {
        heap.clone()
    } else {
        match iterator.clone().expect("a proven present value") {
            self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                identity,
                prototype,
                extensible,
                callable,
                properties,
                _,
                _,
                kind,
                collection_kind,
                _,
                _,
            ) => {
                let prototype = *prototype;
                self::replace_java_script_ordinary_object(
                    heap.clone(),
                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                        identity.clone(),
                        Box::new(prototype.clone()),
                        extensible,
                        callable,
                        properties.clone(),
                        collection.clone(),
                        cursor,
                        kind.clone(),
                        collection_kind.clone(),
                        None::<(Option<bool>,)>,
                        None::<(Option<self::JavaScriptSemanticValue>,)>,
                    ),
                )
            }
            _ => heap.clone(),
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_collection_iterator_receiver_error(
    heap: self::JavaScriptOrdinaryHeap,
    receiver: self::JavaScriptSemanticValue,
    collection_kind: &self::ml_string::Text,
) -> std::rc::Rc<std::cell::RefCell<((self::ml_string::Text, bool), Vec<f64>)>> {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                std::rc::Rc::new(std::cell::RefCell::new({
                    let source_ml_source3_record_field439 = error.clone();
                    {
                        let source_ml_source3_record_field440 = false;
                        ((source_ml_source3_record_field439.clone(), source_ml_source3_record_field440), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![116f64, 121f64, 112f64, 101f64, 69f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                    }
                }))
            } else {
                if !(self::ml_array::includes(
                    &{
                        let ml_reference_value1192 =
                            (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                self::ml_string::Text::from_units(vec![77, 97, 112]),
                                self::ml_string::Text::from_units(vec![83, 101, 116]),
                            ])))
                            .borrow()
                            .clone();
                        ml_reference_value1192
                    },
                    collection_kind,
                    0f64,
                    false,
                    |a, b| a == b,
                )) {
                    std::rc::Rc::new(std::cell::RefCell::new({
                        let source_ml_source3_record_field441 =
                            self::ml_string::Text::from_units(vec![
                                105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114, 101, 97, 116,
                                105, 111, 110, 32, 110, 101, 101, 100, 115, 32, 105, 116, 115, 32,
                                97, 99, 116, 117, 97, 108, 32, 114, 101, 103, 105, 115, 116, 101,
                                114, 101, 100, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110,
                                32, 107, 105, 110, 100,
                            ]);
                        {
                            let source_ml_source3_record_field442 = false;
                            ((source_ml_source3_record_field441.clone(), source_ml_source3_record_field442), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![116f64, 121f64, 112f64, 101f64, 69f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                        }
                    }))
                } else {
                    {
                        let invalid = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            receiver.clone(),
                        );
                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                            std::rc::Rc::new(std::cell::RefCell::new({
                                let source_ml_source3_record_field443 = invalid.clone();
                                {
                                    let source_ml_source3_record_field444 = false;
                                    ((source_ml_source3_record_field443.clone(), source_ml_source3_record_field444), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![116f64, 121f64, 112f64, 101f64, 69f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                                }
                            }))
                        } else {
                            match receiver.clone() {
                                self::JavaScriptSemanticValue::Object(identity, _) => {
                                    let collection = {
                                        let source_ml_source2_array_read_value23496 =
                                            shared_constructor_field1.clone();
                                        {
                                            let source_ml_source2_array_read_index23497 =
                                                self::find_java_script_ordinary_object(
                                                    shared_constructor_field1.clone(),
                                                    identity.clone(),
                                                );
                                            {
                                                let ml_reference_array1193 =
                                                    source_ml_source2_array_read_value23496.clone();
                                                let ml_reference_position1193 =
                                                    source_ml_source2_array_read_index23497;
                                                let ml_reference_projection1193 =
                                                    self::ml_array::optional_at(
                                                        &*ml_reference_array1193.borrow(),
                                                        self::ml_array::number_index(
                                                            ml_reference_position1193,
                                                        ),
                                                    );
                                                ml_reference_projection1193
                                            }
                                        }
                                    };
                                    if (match (collection.clone(), None::<()>) {
                                        (Some(_ml_left), Some(_ml_right)) => false,
                                        (None, None) => true,
                                        _ => false,
                                    } || !((((*collection_kind)
                                        == (self::ml_string::Text::from_units(vec![
                                            77, 97, 112,
                                        ])))
                                        && match collection.clone().expect("a proven present value")
                                        {
                                            self::JavaScriptOrdinaryObject::MapObject(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => true,
                                            _ => false,
                                        })
                                        || (((*collection_kind)
                                            == (self::ml_string::Text::from_units(vec![
                                                83, 101, 116,
                                            ])))
                                            && match collection
                                                .clone()
                                                .expect("a proven present value")
                                            {
                                                self::JavaScriptOrdinaryObject::SetObject(
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => true,
                                                _ => false,
                                            })))
                                    {
                                        std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source2_field23510 =
                                                ((self::ml_string::Text::from_units(vec![
                                                    105, 116, 101, 114, 97, 116, 111, 114, 32, 99,
                                                    114, 101, 97, 116, 105, 111, 110, 32, 110, 101,
                                                    101, 100, 115, 32, 105, 116, 115, 32, 97, 99,
                                                    116, 117, 97, 108, 32,
                                                ]))
                                                .concat(collection_kind))
                                                .concat(&self::ml_string::Text::from_units(vec![
                                                    32, 105, 110, 116, 101, 114, 110, 97, 108, 45,
                                                    115, 108, 111, 116, 32, 114, 101, 99, 101, 105,
                                                    118, 101, 114,
                                                ]));
                                            {
                                                let source_ml_source3_record_field445 =
                                                    source_ml_source2_field23510.clone();
                                                {
                                                    let source_ml_source3_record_field446 = true;
                                                    ((source_ml_source3_record_field445.clone(), source_ml_source3_record_field446), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![116f64, 121f64, 112f64, 101f64, 69f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                                                }
                                            }
                                        }))
                                    } else {
                                        std::rc::Rc::new(std::cell::RefCell::new({
                                            let source_ml_source3_record_field447 =
                                                self::ml_string::Text::from_units(vec![]);
                                            {
                                                let source_ml_source3_record_field448 = false;
                                                ((source_ml_source3_record_field447.clone(), source_ml_source3_record_field448), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![116f64, 121f64, 112f64, 101f64, 69f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                                            }
                                        }))
                                    }
                                }
                                _ => std::rc::Rc::new(std::cell::RefCell::new({
                                    let source_ml_source2_field23514 =
                                        ((self::ml_string::Text::from_units(vec![
                                            105, 116, 101, 114, 97, 116, 111, 114, 32, 99, 114,
                                            101, 97, 116, 105, 111, 110, 32, 110, 101, 101, 100,
                                            115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108,
                                            32,
                                        ]))
                                        .concat(collection_kind))
                                        .concat(&self::ml_string::Text::from_units(vec![
                                            32, 105, 110, 116, 101, 114, 110, 97, 108, 45, 115,
                                            108, 111, 116, 32, 114, 101, 99, 101, 105, 118, 101,
                                            114,
                                        ]));
                                    {
                                        let source_ml_source3_record_field449 =
                                            source_ml_source2_field23514.clone();
                                        {
                                            let source_ml_source3_record_field450 = true;
                                            ((source_ml_source3_record_field449.clone(), source_ml_source3_record_field450), self::source_ml_source3_record_order::create_java_script_record_property_order(vec![vec![101f64, 114f64, 114f64, 111f64, 114f64], vec![116f64, 121f64, 112f64, 101f64, 69f64, 114f64, 114f64, 111f64, 114f64]], vec![0f64, 1f64]))
                                        }
                                    }
                                })),
                            }
                        }
                    }
                }
            }
        }
    }
}
