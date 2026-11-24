// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptFunctionBindCall;
use super::JavaScriptFunctionBindState;
use super::JavaScriptFunctionBindTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptPropertyReflectionState;
use super::JavaScriptPropertyReflectionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283;
use super::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284;
use super::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Result;
use super::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280;
use super::allocate_java_script_array;
use super::find_java_script_ordinary_object;
use super::insert_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_property_reflection_state_error;
use super::read_java_script_semantic_text_length;
use super::read_java_script_semantic_text_unit;
use super::source_ml_source1_temporary_read_java_script_function_bind_state_error_loop282;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn resume_java_script_property_reflection_allocation(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyReflectionState,
    identity: self::ml::Big,
) -> self::JavaScriptPropertyReflectionTransition {
    match state.clone() {
        self::JavaScriptPropertyReflectionState::PropertyReflection(
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
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field11 = *shared_constructor_field11;
            let shared_constructor_field12 = *shared_constructor_field12;
            {
                let error = self::read_java_script_property_reflection_state_error(
                    heap.clone(),
                    state.clone(),
                );
                if ((error != (self::ml_string::Text::from_units(vec![])))
                    || ((shared_constructor_field6
                        != (self::ml_string::Text::from_units(vec![
                            116, 97, 114, 103, 101, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105,
                            111, 110,
                        ])))
                        && (shared_constructor_field6
                            != (self::ml_string::Text::from_units(vec![
                                100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 108, 108,
                                111, 99, 97, 116, 105, 111, 110,
                            ])))))
                {
                    self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(
                        Box::new(heap.clone()),
                        if !error.0.is_empty() {
                            error.clone()
                        } else {
                            self::ml_string::Text::from_units(vec![
                                114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108, 108,
                                111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110,
                                115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112,
                                101, 110, 100, 105, 110, 103, 32, 112, 104, 97, 115, 101,
                            ])
                        },
                    )
                } else {
                    {
                        let ordinary = ((shared_constructor_field6
                            == (self::ml_string::Text::from_units(vec![
                                100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 108, 108,
                                111, 99, 97, 116, 105, 111, 110,
                            ])))
                            || (shared_constructor_field1
                                == (self::ml_string::Text::from_units(vec![
                                    79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80,
                                    114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105,
                                    112, 116, 111, 114, 115,
                                ]))));
                        {
                            let next_heap = heap.clone();
                            {
                                let value = self::JavaScriptSemanticValue::Undefined;
                                {
                                    let source_ml_source2_s6804 = if ordinary {
                                        if (({
                                            let ml_reference_array530 = match heap.clone() {
                                                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                                    source_ml_source2_data_property_value6806,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                    _,
                                                ) => source_ml_source2_data_property_value6806
                                                    .clone(),
                                            };
                                            let ml_reference_projection530 =
                                                ml_reference_array530.borrow().len() as f64;
                                            ml_reference_projection530
                                        }) >= (4096f64))
                                        {
                                            self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Return(Box::new(self::JavaScriptPropertyReflectionTransition::PropertyReflectionResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 121, 32, 98, 117, 100, 103, 101, 116]))))
                                        } else {
                                            {
                                                let allocated =
                                                    self::insert_java_script_ordinary_object(
                                                        heap.clone(),
                                                        identity.clone(),
                                                        shared_constructor_field4.clone(),
                                                    );
                                                if (((({
                                                    let ml_reference_value531 =
                                                        allocated.borrow().clone();
                                                    ml_reference_value531
                                                })
                                                .0)
                                                    .0)
                                                    != (self::ml_string::Text::from_units(vec![])))
                                                {
                                                    self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Return(Box::new(self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new((({ let ml_reference_value532 = allocated.borrow().clone(); ml_reference_value532 }).0).1), (({ let ml_reference_value533 = allocated.borrow().clone(); ml_reference_value533 }).0).0)))
                                                } else {
                                                    {
                                                        let next_heap_2 = (({
                                                            let ml_reference_value534 =
                                                                allocated.borrow().clone();
                                                            ml_reference_value534
                                                        })
                                                        .0)
                                                            .1;
                                                        {
                                                            let value_2 = self::JavaScriptSemanticValue::Object(identity.clone(), false);
                                                            self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Next(Box::new(next_heap_2.clone()), Box::new(value_2.clone()))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let allocated_2 = self::allocate_java_script_array(
                                                heap.clone(),
                                                identity.clone(),
                                                shared_constructor_field5.clone(),
                                                {
                                                    let ml_reference_array535 =
                                                        shared_constructor_field7.clone();
                                                    let ml_reference_projection535 =
                                                        ml_reference_array535.borrow().len() as f64;
                                                    ml_reference_projection535
                                                },
                                            );
                                            match allocated_2.clone() {
                                                self::JavaScriptArrayCompletion::ArrayResourceLimit(heap_2, reason, _) => {
                                                    let heap_2 = *heap_2;
                                                    self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Return(Box::new(self::JavaScriptPropertyReflectionTransition::PropertyReflectionResourceLimit(Box::new(heap_2.clone()), Box::new(state.clone()), reason.clone())))
                                                }
                                                _ => {
                                                    match allocated_2.clone() {
                                                        self::JavaScriptArrayCompletion::ArrayNormal(next_heap_3, value_3) => {
                                                            let next_heap_3 = *next_heap_3;
                                                            let value_3 = *value_3;
                                                            self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Next(Box::new(next_heap_3.clone()), Box::new(value_3.clone()))
                                                        }
                                                        self::JavaScriptArrayCompletion::ArrayPropertyComplete(heap_3, _, _) => {
                                                            let heap_3 = *heap_3;
                                                            self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Return(Box::new(self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap_3.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 65, 114, 114, 97, 121]))))
                                                        }
                                                        self::JavaScriptArrayCompletion::ArrayNumberRequest(heap_4, _, _) => {
                                                            let heap_4 = *heap_4;
                                                            self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Return(Box::new(self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap_4.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 65, 114, 114, 97, 121]))))
                                                        }
                                                        self::JavaScriptArrayCompletion::ArrayDefinitionContinue(heap_5, _) => {
                                                            let heap_5 = *heap_5;
                                                            self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Return(Box::new(self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap_5.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 65, 114, 114, 97, 121]))))
                                                        }
                                                        self::JavaScriptArrayCompletion::ArrayInvalid(heap_6, _) => {
                                                            let heap_6 = *heap_6;
                                                            self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Return(Box::new(self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap_6.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 65, 114, 114, 97, 121]))))
                                                        }
                                                        self::JavaScriptArrayCompletion::ArrayError(heap_7, _, _) => {
                                                            let heap_7 = *heap_7;
                                                            self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Return(Box::new(self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap_7.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 65, 114, 114, 97, 121]))))
                                                        }
                                                        self::JavaScriptArrayCompletion::ArrayResourceLimit(heap_8, _, _) => {
                                                            let heap_8 = *heap_8;
                                                            self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Return(Box::new(self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap_8.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 65, 114, 114, 97, 121]))))
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    match source_ml_source2_s6804.clone() {
                                        self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Next(next_heap_4, value_4) => {
                                            let next_heap_4 = *next_heap_4;
                                            let value_4 = *value_4;
                                            self::JavaScriptPropertyReflectionTransition::PropertyReflectionContinue(Box::new(next_heap_4.clone()), Box::new(if (shared_constructor_field6 == (self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))) {
                                                {
                                                    let source_ml_source2_nominal_field6824 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field6826 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field6828 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field6830 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field6832 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field6834 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field6836 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field6838 = shared_constructor_field8;
                                                                                {
                                                                                    let source_ml_source2_nominal_field6840 = shared_constructor_field9.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6842 = shared_constructor_field10;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6844 = shared_constructor_field11.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6846 = shared_constructor_field12.clone();
                                                                                                self::JavaScriptPropertyReflectionState::PropertyReflection(source_ml_source2_nominal_field6824.clone(), Box::new(source_ml_source2_nominal_field6826.clone()), Box::new(source_ml_source2_nominal_field6828.clone()), Box::new(source_ml_source2_nominal_field6830.clone()), Box::new(source_ml_source2_nominal_field6832.clone()), self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 119, 114, 105, 116, 101]), source_ml_source2_nominal_field6836.clone(), source_ml_source2_nominal_field6838, source_ml_source2_nominal_field6840.clone(), source_ml_source2_nominal_field6842, Box::new(source_ml_source2_nominal_field6844.clone()), Box::new(value_4.clone()))
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
                                            } else {
                                                {
                                                    let source_ml_source2_nominal_field6851 = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_nominal_field6853 = shared_constructor_field2.clone();
                                                        {
                                                            let source_ml_source2_nominal_field6855 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_nominal_field6857 = shared_constructor_field4.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field6859 = shared_constructor_field5.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field6861 = shared_constructor_field6.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field6863 = shared_constructor_field7.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field6865 = shared_constructor_field8;
                                                                                {
                                                                                    let source_ml_source2_nominal_field6867 = shared_constructor_field9.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6869 = shared_constructor_field10;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6871 = shared_constructor_field11.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6873 = shared_constructor_field12.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_override6875 = if ordinary {
                                                                                                        self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116])
                                                                                                    } else {
                                                                                                        self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 119, 114, 105, 116, 101])
                                                                                                    };
                                                                                                    self::JavaScriptPropertyReflectionState::PropertyReflection(source_ml_source2_nominal_field6851.clone(), Box::new(source_ml_source2_nominal_field6853.clone()), Box::new(source_ml_source2_nominal_field6855.clone()), Box::new(source_ml_source2_nominal_field6857.clone()), Box::new(source_ml_source2_nominal_field6859.clone()), source_ml_source2_nominal_override6875.clone(), source_ml_source2_nominal_field6863.clone(), source_ml_source2_nominal_field6865, source_ml_source2_nominal_field6867.clone(), source_ml_source2_nominal_field6869, Box::new(value_4.clone()), Box::new(source_ml_source2_nominal_field6873.clone()))
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
                                        }
                                        self::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280::MlSource1TemporaryResumeJavaScriptPropertyReflectionAllocationJoin280Return(source_ml_source1_temporary_result280) => {
                                            let source_ml_source1_temporary_result280 = *source_ml_source1_temporary_result280;
                                            source_ml_source1_temporary_result280.clone()
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
pub fn read_java_script_bound_function_metadata_key(length: bool) -> self::JavaScriptSemanticValue {
    if length {
        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![
            108f64, 101f64, 110f64, 103f64, 116f64, 104f64,
        ])))
    } else {
        self::JavaScriptSemanticValue::Text(std::rc::Rc::new(std::cell::RefCell::new(vec![
            110f64, 97f64, 109f64, 101f64,
        ])))
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_function_bind_metadata_call(
    call: self::JavaScriptFunctionBindCall,
    target: self::JavaScriptSemanticValue,
) -> Option<bool> {
    match call.clone() {
        self::JavaScriptFunctionBindCall::FunctionBindCall(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
        ) => {
            let shared_constructor_field1 = *shared_constructor_field1;
            let shared_constructor_field2 = *shared_constructor_field2;
            Some(
                ((((match call.clone() {
                    self::JavaScriptFunctionBindCall::FunctionBindCall(_, _, _) => true,
                } && (({
                    let ml_reference_array536 = shared_constructor_field3.clone();
                    let ml_reference_projection536 = ml_reference_array536.borrow().len() as f64;
                    ml_reference_projection536
                }) == (0f64)))
                    && self::test_java_script_semantic_same_value(
                        shared_constructor_field2.clone(),
                        target.clone(),
                    ))
                    && {
                        let source_ml_source2_s6884 = shared_constructor_field1.clone();
                        match source_ml_source2_s6884.clone() {
                            self::JavaScriptSemanticValue::Object(_, _) => true,
                            _ => false,
                        }
                    })
                    && ((match shared_constructor_field1.clone() {
                        self::JavaScriptSemanticValue::Object(
                            _,
                            source_ml_source2_data_property_value6889,
                        ) => Some((source_ml_source2_data_property_value6889,)),
                        _ => None::<(bool,)>,
                    })
                    .clone()
                    .expect("a proven present value"))
                    .0),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_function_bind_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptFunctionBindState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptFunctionBindState::FunctionBindState(
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            _,
        ) => {
            let shared_constructor_field2 = *shared_constructor_field2;
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field7 = *shared_constructor_field7;
            let shared_constructor_field9 = *shared_constructor_field9;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field1,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let invalid = self::read_java_script_ordinary_heap_error(heap.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        invalid.clone()
                    } else {
                        if (((((((match state.clone() {
                            self::JavaScriptFunctionBindState::FunctionBindState(
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
                                let ml_reference_value537 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            112, 114, 111, 116, 111, 116, 121, 112, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            97, 108, 108, 111, 99, 97, 116, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            108, 101, 110, 103, 116, 104, 32, 111, 119, 110,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            108, 101, 110, 103, 116, 104,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116,
                                            101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 102, 105, 110, 101, 32, 108, 101, 110, 103,
                                            116, 104,
                                        ]),
                                        self::ml_string::Text::from_units(vec![110, 97, 109, 101]),
                                        self::ml_string::Text::from_units(vec![
                                            110, 97, 109, 101, 32, 103, 101, 116, 116, 101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            110, 97, 109, 101, 32, 118, 97, 108, 117, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 102, 105, 110, 101, 32, 110, 97, 109, 101,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value537
                            },
                            &shared_constructor_field5.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))) || (({
                            let ml_reference_array538 = shared_constructor_field4.clone();
                            let ml_reference_projection538 =
                                ml_reference_array538.borrow().len() as f64;
                            ml_reference_projection538
                        }) > (1048576f64)))
                            || (({
                                let source_ml_source2_type_query_operand6905 =
                                    shared_constructor_field8;
                                self::ml_string::Text::from_units(vec![110, 117, 109, 98, 101, 114])
                            }) != (self::ml_string::Text::from_units(vec![
                                110, 117, 109, 98, 101, 114,
                            ]))))
                            || (shared_constructor_field8 < (0f64)))
                            || {
                                let source_ml_source2_same_value_operand6909 =
                                    shared_constructor_field8;
                                {
                                    let source_ml_source2_same_value_operand6910 = (-0f64);
                                    (((source_ml_source2_same_value_operand6909
                                        == source_ml_source2_same_value_operand6910)
                                        && ((source_ml_source2_same_value_operand6909 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand6909)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand6910))))
                                        || ((source_ml_source2_same_value_operand6909
                                            != source_ml_source2_same_value_operand6909)
                                            && (source_ml_source2_same_value_operand6910
                                                != source_ml_source2_same_value_operand6910)))
                                }
                            })
                            || shared_constructor_field8.is_nan())
                            || ((shared_constructor_field8 != (f64::INFINITY))
                                && !(self::ml_math::is_integer(shared_constructor_field8))))
                        {
                            self::ml_string::Text::from_units(vec![
                                98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110,
                                115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111,
                                117, 110, 100, 101, 100, 32, 97, 114, 103, 117, 109, 101, 110, 116,
                                115, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 97, 110, 100, 32,
                                112, 104, 97, 115, 101,
                            ])
                        } else {
                            {
                                let source_ml_source1_temporary_values281 = std::rc::Rc::new(
                                    std::cell::RefCell::new(self::ml_array::append(
                                        vec![
                                            shared_constructor_field2.clone(),
                                            shared_constructor_field3.clone(),
                                            shared_constructor_field6.clone(),
                                            shared_constructor_field7.clone(),
                                            shared_constructor_field9.clone(),
                                        ],
                                        {
                                            let ml_reference_value539 =
                                                shared_constructor_field4.borrow().clone();
                                            ml_reference_value539
                                        },
                                    )),
                                );
                                {
                                    let source_ml_source1_temporary_index281 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s6921 = self::source_ml_source1_temporary_read_java_script_function_bind_state_error_loop282(heap.clone(), source_ml_source1_temporary_values281.clone(), source_ml_source1_temporary_index281.clone());
                                        match source_ml_source2_s6921.clone() {
                                                self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Result::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Done => {
                                                    if ((shared_constructor_field5 == (self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101, 32, 110, 97, 109, 101]))) && (((self::read_java_script_semantic_text_length(shared_constructor_field9.clone())) < (6f64)) || { let ml_values540 = { let ml_reference_value541 = (std::rc::Rc::new(std::cell::RefCell::new(vec![98f64, 111f64, 117f64, 110f64, 100f64, 32f64]))).borrow().clone(); ml_reference_value541 }; let ml_callback540 = { let state = state.clone(); std::rc::Rc::new(move |unit: f64, index: f64| -> bool { ((self::read_java_script_semantic_text_unit(match state.clone() {
                                                        self::JavaScriptFunctionBindState::FunctionBindState(_, _, _, _, _, _, _, source_ml_source2_data_property_value6927, _) => {
                                                            let source_ml_source2_data_property_value6927 = *source_ml_source2_data_property_value6927;
                                                            source_ml_source2_data_property_value6927.clone()
                                                        }
                                                    }, index)) != unit) }) }; ml_values540.iter().cloned().enumerate().any(|(ml_index540, ml_value540)| ml_callback540(ml_value540, (ml_index540 as f64))) })) {
                                                        self::ml_string::Text::from_units(vec![98, 111, 117, 110, 100, 32, 70, 117, 110, 99, 116, 105, 111, 110, 32, 110, 97, 109, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 83, 116, 114, 105, 110, 103, 32, 112, 114, 101, 102, 105, 120])
                                                    } else {
                                                        if ({
                                                            let source_ml_source2_s6929 = shared_constructor_field2.clone();
                                                            match source_ml_source2_s6929.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }
                                                        } || !(((match shared_constructor_field2.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value6934) => {
                                                                Some((source_ml_source2_data_property_value6934,))
                                                            }
                                                            _ => {
                                                                None::<(bool,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0)) {
                                                            self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 116, 97, 114, 103, 101, 116])
                                                        } else {
                                                            if (((shared_constructor_field5 != (self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101]))) && {
                                                                let source_ml_source2_s6938 = shared_constructor_field6.clone();
                                                                match source_ml_source2_s6938.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        false
                                                                    }
                                                                    _ => {
                                                                        true
                                                                    }
                                                                }
                                                            }) && {
                                                                let source_ml_source2_s6943 = shared_constructor_field6.clone();
                                                                match source_ml_source2_s6943.clone() {
                                                                    self::JavaScriptSemanticValue::Null => {
                                                                        false
                                                                    }
                                                                    _ => {
                                                                        true
                                                                    }
                                                                }
                                                            }) {
                                                                self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 116, 97, 114, 103, 101, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                            } else {
                                                                {
                                                                    let source_ml_source2_s6979 = if !(self::ml_array::includes(&{ let ml_reference_value542 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![112, 114, 111, 116, 111, 116, 121, 112, 101]), self::ml_string::Text::from_units(vec![97, 108, 108, 111, 99, 97, 116, 101])]))).borrow().clone(); ml_reference_value542 }, &shared_constructor_field5.clone(), 0f64, false, |a, b| a == b)) {
                                                                        if {
                                                                            let source_ml_source2_s6982 = shared_constructor_field7.clone();
                                                                            match source_ml_source2_s6982.clone() {
                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                    false
                                                                                }
                                                                                _ => {
                                                                                    true
                                                                                }
                                                                            }
                                                                        } {
                                                                            self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283Return(self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                                                                        } else {
                                                                            {
                                                                                let object = {
                                                                                    let source_ml_source2_array_read_value6990 = shared_constructor_field1.clone();
                                                                                    {
                                                                                        let source_ml_source2_array_read_index6991 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match shared_constructor_field7.clone() {
                                                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value6989, _) => {
                                                                                                Some((source_ml_source2_data_property_value6989.clone(),))
                                                                                            }
                                                                                            _ => {
                                                                                                None::<(self::ml::Big,)>
                                                                                            }
                                                                                        }).clone().expect("a proven present value")).0).clone());
                                                                                        { let ml_reference_array543 = source_ml_source2_array_read_value6990.clone(); let ml_reference_position543 = source_ml_source2_array_read_index6991; let ml_reference_projection543 = self::ml_array::optional_at(&*ml_reference_array543.borrow(), self::ml_array::number_index(ml_reference_position543)); ml_reference_projection543 }
                                                                                    }
                                                                                };
                                                                                if ((((((match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }) || !(self::test_java_script_semantic_same_value((((match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, source_ml_source2_data_property_value7000, _, _) => {
                                                                                        let source_ml_source2_data_property_value7000 = *source_ml_source2_data_property_value7000;
                                                                                        Some((source_ml_source2_data_property_value7000.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::JavaScriptSemanticValue,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone(), shared_constructor_field2.clone()))) || !(self::test_java_script_semantic_same_value((((match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, source_ml_source2_data_property_value7002, _) => {
                                                                                        let source_ml_source2_data_property_value7002 = *source_ml_source2_data_property_value7002;
                                                                                        Some((source_ml_source2_data_property_value7002.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::JavaScriptSemanticValue,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone(), shared_constructor_field3.clone()))) || !(self::test_java_script_semantic_same_value((((match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value7004, _, _, _, _, _, _) => {
                                                                                        let source_ml_source2_data_property_value7004 = *source_ml_source2_data_property_value7004;
                                                                                        Some((source_ml_source2_data_property_value7004.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(self::JavaScriptSemanticValue,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone(), shared_constructor_field6.clone()))) || (({ let ml_reference_array544 = (((match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value7008) => {
                                                                                        Some((source_ml_source2_data_property_value7008.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone(); let ml_reference_projection544 = ml_reference_array544.borrow().len() as f64; ml_reference_projection544 }) != ({ let ml_reference_array545 = shared_constructor_field4.clone(); let ml_reference_projection545 = ml_reference_array545.borrow().len() as f64; ml_reference_projection545 }))) || { let ml_values546 = { let ml_reference_value547 = ((((match object.clone().expect("a proven present value") {
                                                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, source_ml_source2_data_property_value7010) => {
                                                                                        Some((source_ml_source2_data_property_value7010.clone(),))
                                                                                    }
                                                                                    _ => {
                                                                                        None::<(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,)>
                                                                                    }
                                                                                }).clone().expect("a proven present value")).0).clone()).borrow().clone(); ml_reference_value547 }; let ml_callback546 = { let state = state.clone(); std::rc::Rc::new(move |value: self::JavaScriptSemanticValue, index: f64| -> bool { !(self::test_java_script_semantic_same_value(value.clone(), ({
                                                                                    let source_ml_source2_array_read_value7012 = match state.clone() {
                                                                                        self::JavaScriptFunctionBindState::FunctionBindState(_, _, source_ml_source2_data_property_value7011, _, _, _, _, _, _) => {
                                                                                            source_ml_source2_data_property_value7011.clone()
                                                                                        }
                                                                                    };
                                                                                    {
                                                                                        let source_ml_source2_array_read_index7013 = index;
                                                                                        { let ml_reference_array548 = source_ml_source2_array_read_value7012.clone(); let ml_reference_position548 = source_ml_source2_array_read_index7013; let ml_reference_projection548 = self::ml_array::optional_at(&*ml_reference_array548.borrow(), self::ml_array::number_index(ml_reference_position548)); ml_reference_projection548 }
                                                                                    }
                                                                                }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) }) }; ml_values546.iter().cloned().enumerate().any(|(ml_index546, ml_value546)| ml_callback546(ml_value546, (ml_index546 as f64))) }) {
                                                                                    self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283Return(self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 114, 101, 115, 104, 32, 114, 101, 99, 101, 105, 118, 101, 114, 32, 97, 110, 100, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115]))
                                                                                } else {
                                                                                    self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283Next
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283Next
                                                                    };
                                                                    match source_ml_source2_s6979.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283Next => {
                                                                            {
                                                                                let source_ml_source2_s7019 = if !(match (match state.clone() {
                                                                                    self::JavaScriptFunctionBindState::FunctionBindState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value7020) => {
                                                                                        ((source_ml_source2_data_property_value7020.clone().unwrap_or_else(|| (None::<self::JavaScriptFunctionBindCall>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    if (!(self::ml_array::includes(&{ let ml_reference_value549 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![110, 97, 109, 101, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value549 }, &shared_constructor_field5.clone(), 0f64, false, |a, b| a == b)) || !(match self::test_java_script_function_bind_metadata_call((match state.clone() {
                                                                                        self::JavaScriptFunctionBindState::FunctionBindState(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value7022) => {
                                                                                            ((source_ml_source2_data_property_value7022.clone().unwrap_or_else(|| (None::<self::JavaScriptFunctionBindCall>,))).0).clone()
                                                                                        }
                                                                                    }).clone().expect("a proven present value"), shared_constructor_field2.clone()) { Some(ml_present) => ml_present, None => false })) {
                                                                                        self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284Return(self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 109, 101, 116, 97, 100, 97, 116, 97, 32, 103, 101, 116, 116, 101, 114, 32, 97, 110, 100, 32, 114, 101, 99, 101, 105, 118, 101, 114]))
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284Next
                                                                                    }
                                                                                } else {
                                                                                    if self::ml_array::includes(&{ let ml_reference_value550 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![110, 97, 109, 101, 32, 103, 101, 116, 116, 101, 114])]))).borrow().clone(); ml_reference_value550 }, &shared_constructor_field5.clone(), 0f64, false, |a, b| a == b) {
                                                                                        self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284Return(self::ml_string::Text::from_units(vec![98, 105, 110, 100, 105, 110, 103, 32, 103, 101, 116, 116, 101, 114, 32, 112, 104, 97, 115, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 105, 114, 32, 97, 99, 116, 117, 97, 108, 32, 115, 117, 115, 112, 101, 110, 100, 101, 100, 32, 99, 97, 108, 108]))
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284Next
                                                                                    }
                                                                                };
                                                                                match source_ml_source2_s7019.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284Next => {
                                                                                        self::ml_string::Text::from_units(vec![])
                                                                                    }
                                                                                    self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin284Return(source_ml_source1_temporary_result284) => {
                                                                                        source_ml_source1_temporary_result284.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorJoin283Return(source_ml_source1_temporary_result284_2) => {
                                                                            source_ml_source1_temporary_result284_2.clone()
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Result::MlSource1TemporaryReadJavaScriptFunctionBindStateErrorLoop282Return(source_ml_source1_temporary_result284_3) => {
                                                    source_ml_source1_temporary_result284_3.clone()
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
pub fn begin_java_script_function_bind(
    heap: self::JavaScriptOrdinaryHeap,
    target: self::JavaScriptSemanticValue,
    bound_this: self::JavaScriptSemanticValue,
    arguments_: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
) -> self::JavaScriptFunctionBindTransition {
    if (match target.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => false,
        _ => true,
    } || !(((match target.clone() {
        self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value7027) => {
            Some((source_ml_source2_data_property_value7027,))
        }
        _ => None::<(bool,)>,
    })
    .clone()
    .expect("a proven present value"))
    .0))
    {
        self::JavaScriptFunctionBindTransition::FunctionBindError(
            Box::new(heap.clone()),
            self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114]),
            self::ml_string::Text::from_units(vec![
                66, 105, 110, 100, 32, 109, 117, 115, 116, 32, 98, 101, 32, 99, 97, 108, 108, 101,
                100, 32, 111, 110, 32, 97, 32, 102, 117, 110, 99, 116, 105, 111, 110,
            ]),
        )
    } else {
        {
            let state = {
                let source_ml_source2_constructor_field7030 =
                    std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(
                        &{
                            let ml_reference_value551 = arguments_.borrow().clone();
                            ml_reference_value551
                        },
                        0f64,
                        f64::INFINITY,
                    )));
                {
                    let source_ml_source2_constructor_field7032 =
                        self::JavaScriptSemanticValue::Undefined;
                    {
                        let source_ml_source2_constructor_field7033 =
                            self::JavaScriptSemanticValue::Undefined;
                        {
                            let source_ml_source2_constructor_field7034 = 0f64;
                            {
                                let source_ml_source2_constructor_field7035 =
                                    self::JavaScriptSemanticValue::Text(std::rc::Rc::new(
                                        std::cell::RefCell::new(Vec::<f64>::new()),
                                    ));
                                self::JavaScriptFunctionBindState::FunctionBindState(
                                    Box::new(target.clone()),
                                    Box::new(bound_this.clone()),
                                    source_ml_source2_constructor_field7030.clone(),
                                    self::ml_string::Text::from_units(vec![
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]),
                                    Box::new(source_ml_source2_constructor_field7032.clone()),
                                    Box::new(source_ml_source2_constructor_field7033.clone()),
                                    source_ml_source2_constructor_field7034,
                                    Box::new(source_ml_source2_constructor_field7035.clone()),
                                    None::<(Option<self::JavaScriptFunctionBindCall>,)>,
                                )
                            }
                        }
                    }
                }
            };
            {
                let error =
                    self::read_java_script_function_bind_state_error(heap.clone(), state.clone());
                if (error == (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptFunctionBindTransition::FunctionBindContinue(
                        Box::new(heap.clone()),
                        Box::new(state.clone()),
                    )
                } else {
                    self::JavaScriptFunctionBindTransition::FunctionBindInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_bound_function_length(
    value: self::JavaScriptSemanticValue,
    count: f64,
) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::Number(value_value) => {
            if (value_value.is_nan() || (value_value == (f64::NEG_INFINITY))) {
                0f64
            } else {
                if (value_value == (f64::INFINITY)) {
                    f64::INFINITY
                } else {
                    self::ml_math::max(0f64, (value_value.trunc() - count))
                }
            }
        }
        _ => 0f64,
    }
}
