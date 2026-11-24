// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptIndexedPropertyCreation;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPropertyReflectionState;
use super::JavaScriptPropertyReflectionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276;
use super::MlSource1TemporaryStepJavaScriptPropertyReflectionLoop277Result;
use super::construct_java_script_reflected_descriptor_fields;
use super::construct_java_script_reflection_property;
use super::construct_java_script_reflection_text;
use super::create_java_script_indexed_data_property;
use super::define_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::read_java_script_ordinary_own_keys;
use super::read_java_script_property_reflection_state_error;
use super::read_java_script_reflected_own_property;
use super::read_java_script_semantic_text_length;
use super::source_ml_source1_temporary_step_java_script_property_reflection_loop277;
use super::test_java_script_reflection_property_budget;
use super::test_java_script_single_descriptor_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_property_reflection(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyReflectionState,
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
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(
                        Box::new(heap.clone()),
                        error.clone(),
                    )
                } else {
                    {
                        let single = self::test_java_script_single_descriptor_operation(
                            &(shared_constructor_field1),
                        );
                        {
                            let plural = (shared_constructor_field1
                                == (self::ml_string::Text::from_units(vec![
                                    79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80,
                                    114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105,
                                    112, 116, 111, 114, 115,
                                ])));
                            if (shared_constructor_field6
                                == (self::ml_string::Text::from_units(vec![107, 101, 121, 115])))
                            {
                                {
                                    let keys = std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                        self::JavaScriptSemanticValue,
                                    >::new(
                                    )));
                                    {
                                        let source_ml_source2_s6486 = if single {
                                            {
                                                let keys_2 =
                                                    std::rc::Rc::new(std::cell::RefCell::new(
                                                        vec![shared_constructor_field3.clone()],
                                                    ));
                                                self::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276Next(keys_2.clone())
                                            }
                                        } else {
                                            if {
                                                let source_ml_source2_s6489 =
                                                    shared_constructor_field2.clone();
                                                match source_ml_source2_s6489.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        true
                                                    }
                                                    _ => false,
                                                }
                                            } {
                                                {
                                                    let own = self::read_java_script_ordinary_own_keys(heap.clone(), (((match shared_constructor_field2.clone() {
                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value6494, _) => {
                                                            Some((source_ml_source2_data_property_value6494.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::ml::Big,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone());
                                                    if (((({
                                                        let ml_reference_value504 =
                                                            own.borrow().clone();
                                                        ml_reference_value504
                                                    })
                                                    .0)
                                                        .0)
                                                        != (self::ml_string::Text::from_units(
                                                            vec![],
                                                        )))
                                                    {
                                                        self::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276Return(Box::new(self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap.clone()), (({ let ml_reference_value505 = own.borrow().clone(); ml_reference_value505 }).0).0)))
                                                    } else {
                                                        {
                                                            let keys_3 = (({
                                                                let ml_reference_value506 =
                                                                    own.borrow().clone();
                                                                ml_reference_value506
                                                            })
                                                            .0)
                                                                .1;
                                                            self::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276Next(keys_3.clone())
                                                        }
                                                    }
                                                }
                                            } else {
                                                if ({
                                                    let source_ml_source2_s6496 =
                                                        shared_constructor_field2.clone();
                                                    match source_ml_source2_s6496.clone() {
                                                        self::JavaScriptSemanticValue::Text(_) => {
                                                            true
                                                        }
                                                        _ => false,
                                                    }
                                                } || {
                                                    let source_ml_source2_s6500 =
                                                        shared_constructor_field2.clone();
                                                    match source_ml_source2_s6500.clone() {
                                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    }
                                                }) {
                                                    {
                                                        let length = self::read_java_script_semantic_text_length(shared_constructor_field2.clone());
                                                        if (length >= (4096f64)) {
                                                            self::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276Return(Box::new(self::JavaScriptPropertyReflectionTransition::PropertyReflectionResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 107, 101, 121, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 98, 117, 100, 103, 101, 116]))))
                                                        } else {
                                                            {
                                                                let at = 0f64;
                                                                {
                                                                    let source_ml_source2_s6505 = self::source_ml_source1_temporary_step_java_script_property_reflection_loop277(keys.clone(), length, at);
                                                                    match source_ml_source2_s6505.clone() {
                                                                        self::MlSource1TemporaryStepJavaScriptPropertyReflectionLoop277Result::MlSource1TemporaryStepJavaScriptPropertyReflectionLoop277Done => {
                                                                            {
                                                                                let source_ml_source1_temporary_step_java_script_property_reflection_ignored278 = { let ml_method_owner507 = keys.clone(); let ml_method_argument507_0 = self::construct_java_script_reflection_text(&(self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]))); let ml_inserted = vec![ml_method_argument507_0]; let mut ml_method_values507 = ml_method_owner507.borrow_mut(); let ml_length = ml_method_values507.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values507.extend(ml_inserted); ml_length as f64 };
                                                                                self::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276Next(keys.clone())
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    self::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276Next(keys.clone())
                                                }
                                            }
                                        };
                                        match source_ml_source2_s6486.clone() {
                                            self::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276Next(keys_4) => {
                                                {
                                                    let keys_6 = if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 78, 97, 109, 101, 115]))) {
                                                        {
                                                            let keys_5 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values508 = { let ml_reference_value509 = keys_4.borrow().clone(); ml_reference_value509 }; let ml_callback508 = std::rc::Rc::new(move |key: self::JavaScriptSemanticValue| -> bool { match key.clone() {
                                                                self::JavaScriptSemanticValue::Symbol(_) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            } }); ml_values508.iter().cloned().enumerate().filter_map(|(ml_index508, ml_value508)| if ml_callback508(ml_value508.clone()) { Some(ml_value508) } else { None }).collect::<Vec<_>>() }));
                                                            keys_5.clone()
                                                        }
                                                    } else {
                                                        keys_4.clone()
                                                    };
                                                    {
                                                        let keys_8 = if (shared_constructor_field1 == (self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121, 83, 121, 109, 98, 111, 108, 115]))) {
                                                            {
                                                                let keys_7 = std::rc::Rc::new(std::cell::RefCell::new({ let ml_values510 = { let ml_reference_value511 = keys_6.borrow().clone(); ml_reference_value511 }; let ml_callback510 = std::rc::Rc::new(move |key: self::JavaScriptSemanticValue| -> bool { match key.clone() {
                                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                } }); ml_values510.iter().cloned().enumerate().filter_map(|(ml_index510, ml_value510)| if ml_callback510(ml_value510.clone()) { Some(ml_value510) } else { None }).collect::<Vec<_>>() }));
                                                                keys_7.clone()
                                                            }
                                                        } else {
                                                            keys_6.clone()
                                                        };
                                                        if (({ let ml_reference_array512 = keys_8.clone(); let ml_reference_projection512 = ml_reference_array512.borrow().len() as f64; ml_reference_projection512 }) > (4096f64)) {
                                                            self::JavaScriptPropertyReflectionTransition::PropertyReflectionResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 107, 101, 121, 32, 115, 110, 97, 112, 115, 104, 111, 116, 32, 98, 117, 100, 103, 101, 116]))
                                                        } else {
                                                            self::JavaScriptPropertyReflectionTransition::PropertyReflectionContinue(Box::new(heap.clone()), Box::new({
                                                                let source_ml_source2_nominal_field6513 = shared_constructor_field1.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field6515 = shared_constructor_field2.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field6517 = shared_constructor_field3.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field6519 = shared_constructor_field4.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field6521 = shared_constructor_field5.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field6523 = shared_constructor_field6.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6525 = shared_constructor_field7.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6527 = shared_constructor_field8;
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6529 = shared_constructor_field9.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6531 = shared_constructor_field10;
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6533 = shared_constructor_field11.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field6535 = shared_constructor_field12.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_override6538 = if single {
                                                                                                                    self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116])
                                                                                                                } else {
                                                                                                                    self::ml_string::Text::from_units(vec![116, 97, 114, 103, 101, 116, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110])
                                                                                                                };
                                                                                                                self::JavaScriptPropertyReflectionState::PropertyReflection(source_ml_source2_nominal_field6513.clone(), Box::new(source_ml_source2_nominal_field6515.clone()), Box::new(source_ml_source2_nominal_field6517.clone()), Box::new(source_ml_source2_nominal_field6519.clone()), Box::new(source_ml_source2_nominal_field6521.clone()), source_ml_source2_nominal_override6538.clone(), keys_8.clone(), source_ml_source2_nominal_field6527, source_ml_source2_nominal_field6529.clone(), source_ml_source2_nominal_field6531, Box::new(source_ml_source2_nominal_field6533.clone()), Box::new(source_ml_source2_nominal_field6535.clone()))
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
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276::MlSource1TemporaryStepJavaScriptPropertyReflectionJoin276Return(source_ml_source1_temporary_result276) => {
                                                let source_ml_source1_temporary_result276 = *source_ml_source1_temporary_result276;
                                                source_ml_source1_temporary_result276.clone()
                                            }
                                        }
                                    }
                                }
                            } else {
                                if ((shared_constructor_field6
                                    == (self::ml_string::Text::from_units(vec![
                                        116, 97, 114, 103, 101, 116, 32, 97, 108, 108, 111, 99, 97,
                                        116, 105, 111, 110,
                                    ])))
                                    || (shared_constructor_field6
                                        == (self::ml_string::Text::from_units(vec![
                                            100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32,
                                            97, 108, 108, 111, 99, 97, 116, 105, 111, 110,
                                        ]))))
                                {
                                    self::JavaScriptPropertyReflectionTransition::PropertyReflectionAllocate(Box::new(heap.clone()), Box::new(state.clone()), Box::new(if ((shared_constructor_field6 == (self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))) || plural) {
                                        shared_constructor_field4.clone()
                                    } else {
                                        shared_constructor_field5.clone()
                                    }))
                                } else {
                                    if (shared_constructor_field6
                                        == (self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121, 32, 119, 114, 105, 116, 101,
                                        ])))
                                    {
                                        if (shared_constructor_field8
                                            == ({
                                                let ml_reference_array513 =
                                                    shared_constructor_field7.clone();
                                                let ml_reference_projection513 =
                                                    ml_reference_array513.borrow().len() as f64;
                                                ml_reference_projection513
                                            }))
                                        {
                                            self::JavaScriptPropertyReflectionTransition::PropertyReflectionCompleted(Box::new(heap.clone()), Box::new(shared_constructor_field11.clone()))
                                        } else {
                                            {
                                                let key = {
                                                    let source_ml_source2_array_read_value6556 =
                                                        shared_constructor_field7.clone();
                                                    {
                                                        let source_ml_source2_array_read_index6557 =
                                                            shared_constructor_field8;
                                                        {
                                                            let ml_reference_array514 = source_ml_source2_array_read_value6556.clone();
                                                            let ml_reference_position514 = source_ml_source2_array_read_index6557;
                                                            let ml_reference_projection514 =
                                                                self::ml_array::optional_at(
                                                                    &*ml_reference_array514
                                                                        .borrow(),
                                                                    self::ml_array::number_index(
                                                                        ml_reference_position514,
                                                                    ),
                                                                );
                                                            ml_reference_projection514
                                                        }
                                                    }
                                                };
                                                if match (key.clone(), None::<()>) {
                                                    (Some(_ml_left), Some(_ml_right)) => false,
                                                    (None, None) => true,
                                                    _ => false,
                                                } {
                                                    self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 107, 101, 121]))
                                                } else {
                                                    {
                                                        let write = self::create_java_script_indexed_data_property(heap.clone(), shared_constructor_field11.clone(), shared_constructor_field8, key.clone().expect("a proven present value"));
                                                        match write.clone() {
                                                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyCreated(heap_2) => {
                                                                let heap_2 = *heap_2;
                                                                self::JavaScriptPropertyReflectionTransition::PropertyReflectionContinue(Box::new(heap_2.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field6562 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field6564 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field6566 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field6568 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field6570 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6572 = shared_constructor_field6.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6574 = shared_constructor_field7.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6576 = shared_constructor_field8;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6578 = shared_constructor_field9.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6580 = shared_constructor_field10;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field6582 = shared_constructor_field11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field6584 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override6586 = (shared_constructor_field8 + 1f64);
                                                                                                                    self::JavaScriptPropertyReflectionState::PropertyReflection(source_ml_source2_nominal_field6562.clone(), Box::new(source_ml_source2_nominal_field6564.clone()), Box::new(source_ml_source2_nominal_field6566.clone()), Box::new(source_ml_source2_nominal_field6568.clone()), Box::new(source_ml_source2_nominal_field6570.clone()), source_ml_source2_nominal_field6572.clone(), source_ml_source2_nominal_field6574.clone(), source_ml_source2_nominal_override6586, source_ml_source2_nominal_field6578.clone(), source_ml_source2_nominal_field6580, Box::new(source_ml_source2_nominal_field6582.clone()), Box::new(source_ml_source2_nominal_field6584.clone()))
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
                                                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyError(heap_3, reason) => {
                                                                let heap_3 = *heap_3;
                                                                self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap_3.clone()), reason.clone())
                                                            }
                                                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyInvalid(heap_4, reason_2) => {
                                                                let heap_4 = *heap_4;
                                                                self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap_4.clone()), reason_2.clone())
                                                            }
                                                            self::JavaScriptIndexedPropertyCreation::IndexedPropertyResourceLimit(heap_5, reason_3) => {
                                                                let heap_5 = *heap_5;
                                                                self::JavaScriptPropertyReflectionTransition::PropertyReflectionResourceLimit(Box::new(heap_5.clone()), Box::new(state.clone()), reason_3.clone())
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if (shared_constructor_field6
                                            == (self::ml_string::Text::from_units(vec![
                                                115, 101, 108, 101, 99, 116,
                                            ])))
                                        {
                                            if (shared_constructor_field8
                                                == ({
                                                    let ml_reference_array515 =
                                                        shared_constructor_field7.clone();
                                                    let ml_reference_projection515 =
                                                        ml_reference_array515.borrow().len() as f64;
                                                    ml_reference_projection515
                                                }))
                                            {
                                                self::JavaScriptPropertyReflectionTransition::PropertyReflectionCompleted(Box::new(heap.clone()), Box::new(if single {
                                                    self::JavaScriptSemanticValue::Undefined
                                                } else {
                                                    shared_constructor_field11.clone()
                                                }))
                                            } else {
                                                {
                                                    let key_2 = {
                                                        let source_ml_source2_array_read_value6596 =
                                                            shared_constructor_field7.clone();
                                                        {
                                                            let source_ml_source2_array_read_index6597 =
                                                                shared_constructor_field8;
                                                            {
                                                                let ml_reference_array516 = source_ml_source2_array_read_value6596.clone();
                                                                let ml_reference_position516 = source_ml_source2_array_read_index6597;
                                                                let ml_reference_projection516 = self::ml_array::optional_at(&*ml_reference_array516.borrow(), self::ml_array::number_index(ml_reference_position516));
                                                                ml_reference_projection516
                                                            }
                                                        }
                                                    };
                                                    if match (key_2.clone(), None::<()>) {
                                                        (Some(_ml_left), Some(_ml_right)) => false,
                                                        (None, None) => true,
                                                        _ => false,
                                                    } {
                                                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 111, 119, 110, 32, 107, 101, 121]))
                                                    } else {
                                                        {
                                                            let property = self::read_java_script_reflected_own_property(heap.clone(), shared_constructor_field2.clone(), key_2.clone().expect("a proven present value"));
                                                            if match (property.clone(), None::<()>)
                                                            {
                                                                (
                                                                    Some(_ml_left),
                                                                    Some(_ml_right),
                                                                ) => false,
                                                                (None, None) => true,
                                                                _ => false,
                                                            } {
                                                                self::JavaScriptPropertyReflectionTransition::PropertyReflectionContinue(Box::new(heap.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field6601 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field6603 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field6605 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field6607 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field6609 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6611 = shared_constructor_field6.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6613 = shared_constructor_field7.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6615 = shared_constructor_field8;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6617 = shared_constructor_field9.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6619 = shared_constructor_field10;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field6621 = shared_constructor_field11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field6623 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override6625 = (shared_constructor_field8 + 1f64);
                                                                                                                    self::JavaScriptPropertyReflectionState::PropertyReflection(source_ml_source2_nominal_field6601.clone(), Box::new(source_ml_source2_nominal_field6603.clone()), Box::new(source_ml_source2_nominal_field6605.clone()), Box::new(source_ml_source2_nominal_field6607.clone()), Box::new(source_ml_source2_nominal_field6609.clone()), source_ml_source2_nominal_field6611.clone(), source_ml_source2_nominal_field6613.clone(), source_ml_source2_nominal_override6625, source_ml_source2_nominal_field6617.clone(), source_ml_source2_nominal_field6619, Box::new(source_ml_source2_nominal_field6621.clone()), Box::new(source_ml_source2_nominal_field6623.clone()))
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
                                                                self::JavaScriptPropertyReflectionTransition::PropertyReflectionContinue(Box::new(heap.clone()), Box::new({
                                                                    let source_ml_source2_nominal_field6628 = shared_constructor_field1.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field6630 = shared_constructor_field2.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field6632 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field6634 = shared_constructor_field4.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field6636 = shared_constructor_field5.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6638 = shared_constructor_field6.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6640 = shared_constructor_field7.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6642 = shared_constructor_field8;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6644 = shared_constructor_field9.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6646 = shared_constructor_field10;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field6648 = shared_constructor_field11.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field6650 = shared_constructor_field12.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_override6653 = self::construct_java_script_reflected_descriptor_fields(match property.clone().expect("a proven present value") {
                                                                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value6626) => {
                                                                                                                            source_ml_source2_data_property_value6626.clone()
                                                                                                                        }
                                                                                                                    });
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_override6654 = 0f64;
                                                                                                                        self::JavaScriptPropertyReflectionState::PropertyReflection(source_ml_source2_nominal_field6628.clone(), Box::new(source_ml_source2_nominal_field6630.clone()), Box::new(source_ml_source2_nominal_field6632.clone()), Box::new(source_ml_source2_nominal_field6634.clone()), Box::new(source_ml_source2_nominal_field6636.clone()), self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]), source_ml_source2_nominal_field6640.clone(), source_ml_source2_nominal_field6642, source_ml_source2_nominal_override6653.clone(), source_ml_source2_nominal_override6654, Box::new(source_ml_source2_nominal_field6648.clone()), Box::new(source_ml_source2_nominal_field6650.clone()))
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
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if (shared_constructor_field6
                                                == (self::ml_string::Text::from_units(vec![
                                                    100, 101, 115, 99, 114, 105, 112, 116, 111,
                                                    114, 32, 119, 114, 105, 116, 101,
                                                ])))
                                            {
                                                if (shared_constructor_field10
                                                    == ({
                                                        let ml_reference_array517 =
                                                            shared_constructor_field9.clone();
                                                        let ml_reference_projection517 =
                                                            ml_reference_array517.borrow().len()
                                                                as f64;
                                                        ml_reference_projection517
                                                    }))
                                                {
                                                    if single {
                                                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionCompleted(Box::new(heap.clone()), Box::new(shared_constructor_field12.clone()))
                                                    } else {
                                                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionContinue(Box::new(heap.clone()), Box::new({
                                                            let source_ml_source2_nominal_field6663 = shared_constructor_field1.clone();
                                                            {
                                                                let source_ml_source2_nominal_field6665 = shared_constructor_field2.clone();
                                                                {
                                                                    let source_ml_source2_nominal_field6667 = shared_constructor_field3.clone();
                                                                    {
                                                                        let source_ml_source2_nominal_field6669 = shared_constructor_field4.clone();
                                                                        {
                                                                            let source_ml_source2_nominal_field6671 = shared_constructor_field5.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field6673 = shared_constructor_field6.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field6675 = shared_constructor_field7.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6677 = shared_constructor_field8;
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6679 = shared_constructor_field9.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6681 = shared_constructor_field10;
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6683 = shared_constructor_field11.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6685 = shared_constructor_field12.clone();
                                                                                                        self::JavaScriptPropertyReflectionState::PropertyReflection(source_ml_source2_nominal_field6663.clone(), Box::new(source_ml_source2_nominal_field6665.clone()), Box::new(source_ml_source2_nominal_field6667.clone()), Box::new(source_ml_source2_nominal_field6669.clone()), Box::new(source_ml_source2_nominal_field6671.clone()), self::ml_string::Text::from_units(vec![112, 117, 98, 108, 105, 115, 104]), source_ml_source2_nominal_field6675.clone(), source_ml_source2_nominal_field6677, source_ml_source2_nominal_field6679.clone(), source_ml_source2_nominal_field6681, Box::new(source_ml_source2_nominal_field6683.clone()), Box::new(source_ml_source2_nominal_field6685.clone()))
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
                                                } else {
                                                    {
                                                        let field = {
                                                            let source_ml_source2_array_read_value6690 =
                                                                shared_constructor_field9.clone();
                                                            {
                                                                let source_ml_source2_array_read_index6691 =
                                                                    shared_constructor_field10;
                                                                {
                                                                    let ml_reference_array518 = source_ml_source2_array_read_value6690.clone();
                                                                    let ml_reference_position518 = source_ml_source2_array_read_index6691;
                                                                    let ml_reference_projection518 = self::ml_array::optional_at(&*ml_reference_array518.borrow(), self::ml_array::number_index(ml_reference_position518));
                                                                    ml_reference_projection518
                                                                }
                                                            }
                                                        };
                                                        if (match (field.clone(), None::<()>) {
                                                            (Some(_ml_left), Some(_ml_right)) => {
                                                                false
                                                            }
                                                            (None, None) => true,
                                                            _ => false,
                                                        } || {
                                                            let source_ml_source2_s6693 =
                                                                shared_constructor_field12.clone();
                                                            match source_ml_source2_s6693.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }
                                                        }) {
                                                            self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 102, 105, 101, 108, 100, 32, 97, 110, 100, 32, 114, 101, 115, 117, 108, 116]))
                                                        } else {
                                                            if !(self::test_java_script_reflection_property_budget(heap.clone(), shared_constructor_field12.clone(), match field.clone().expect("a proven present value") {
                                                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value6698, _) => {
                                                                    let source_ml_source2_data_property_value6698 = *source_ml_source2_data_property_value6698;
                                                                    source_ml_source2_data_property_value6698.clone()
                                                                }
                                                            })) {
                                                                self::JavaScriptPropertyReflectionTransition::PropertyReflectionResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 102, 105, 101, 108, 100, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 104, 101, 97, 112, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                            } else {
                                                                {
                                                                    let write_2 = self::define_java_script_ordinary_property(heap.clone(), (((match shared_constructor_field12.clone() {
                                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value6700, _) => {
                                                                            Some((source_ml_source2_data_property_value6700.clone(),))
                                                                        }
                                                                        _ => {
                                                                            None::<(self::ml::Big,)>
                                                                        }
                                                                    }).clone().expect("a proven present value")).0).clone(), match field.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source2_data_property_value6701, _) => {
                                                                            let source_ml_source2_data_property_value6701 = *source_ml_source2_data_property_value6701;
                                                                            source_ml_source2_data_property_value6701.clone()
                                                                        }
                                                                    }, match field.clone().expect("a proven present value") {
                                                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value6702) => {
                                                                            source_ml_source2_data_property_value6702.clone()
                                                                        }
                                                                    });
                                                                    if ((((({ let ml_reference_value519 = write_2.borrow().clone(); ml_reference_value519 }).0).0) != (self::ml_string::Text::from_units(vec![]))) || !((({ let ml_reference_value520 = write_2.borrow().clone(); ml_reference_value520 }).0).3)) {
                                                                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new((({ let ml_reference_value521 = write_2.borrow().clone(); ml_reference_value521 }).0).1), {
                                                                            let source_ml_source2_logical6703 = (({ let ml_reference_value522 = write_2.borrow().clone(); ml_reference_value522 }).0).0;
                                                                            if !source_ml_source2_logical6703.0.is_empty() {
                                                                                source_ml_source2_logical6703.clone()
                                                                            } else {
                                                                                self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104, 32, 114, 101, 102, 108, 101, 99, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 102, 105, 101, 108, 100, 32, 109, 117, 115, 116, 32, 97, 99, 99, 101, 112, 116, 32, 105, 116, 115, 32, 100, 97, 116, 97, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110])
                                                                            }
                                                                        })
                                                                    } else {
                                                                        self::JavaScriptPropertyReflectionTransition::PropertyReflectionContinue(Box::new((({ let ml_reference_value523 = write_2.borrow().clone(); ml_reference_value523 }).0).1), Box::new({
                                                                            let source_ml_source2_nominal_field6706 = shared_constructor_field1.clone();
                                                                            {
                                                                                let source_ml_source2_nominal_field6708 = shared_constructor_field2.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field6710 = shared_constructor_field3.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6712 = shared_constructor_field4.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6714 = shared_constructor_field5.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6716 = shared_constructor_field6.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6718 = shared_constructor_field7.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6720 = shared_constructor_field8;
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field6722 = shared_constructor_field9.clone();
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field6724 = shared_constructor_field10;
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field6726 = shared_constructor_field11.clone();
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field6728 = shared_constructor_field12.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_override6730 = (shared_constructor_field10 + 1f64);
                                                                                                                            self::JavaScriptPropertyReflectionState::PropertyReflection(source_ml_source2_nominal_field6706.clone(), Box::new(source_ml_source2_nominal_field6708.clone()), Box::new(source_ml_source2_nominal_field6710.clone()), Box::new(source_ml_source2_nominal_field6712.clone()), Box::new(source_ml_source2_nominal_field6714.clone()), source_ml_source2_nominal_field6716.clone(), source_ml_source2_nominal_field6718.clone(), source_ml_source2_nominal_field6720, source_ml_source2_nominal_field6722.clone(), source_ml_source2_nominal_override6730, Box::new(source_ml_source2_nominal_field6726.clone()), Box::new(source_ml_source2_nominal_field6728.clone()))
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
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                if (shared_constructor_field6
                                                    == (self::ml_string::Text::from_units(vec![
                                                        112, 117, 98, 108, 105, 115, 104,
                                                    ])))
                                                {
                                                    {
                                                        let key_3 = {
                                                            let source_ml_source2_array_read_value6735 =
                                                                shared_constructor_field7.clone();
                                                            {
                                                                let source_ml_source2_array_read_index6736 =
                                                                    shared_constructor_field8;
                                                                {
                                                                    let ml_reference_array524 = source_ml_source2_array_read_value6735.clone();
                                                                    let ml_reference_position524 = source_ml_source2_array_read_index6736;
                                                                    let ml_reference_projection524 = self::ml_array::optional_at(&*ml_reference_array524.borrow(), self::ml_array::number_index(ml_reference_position524));
                                                                    ml_reference_projection524
                                                                }
                                                            }
                                                        };
                                                        if (match (key_3.clone(), None::<()>) {
                                                            (Some(_ml_left), Some(_ml_right)) => {
                                                                false
                                                            }
                                                            (None, None) => true,
                                                            _ => false,
                                                        } || {
                                                            let source_ml_source2_s6738 =
                                                                shared_constructor_field11.clone();
                                                            match source_ml_source2_s6738.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }
                                                        }) {
                                                            self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 112, 117, 98, 108, 105, 99, 97, 116, 105, 111, 110, 32, 107, 101, 121, 32, 97, 110, 100, 32, 114, 101, 115, 117, 108, 116]))
                                                        } else {
                                                            if !(self::test_java_script_reflection_property_budget(heap.clone(), shared_constructor_field11.clone(), key_3.clone().expect("a proven present value"))) {
                                                                self::JavaScriptPropertyReflectionTransition::PropertyReflectionResourceLimit(Box::new(heap.clone()), Box::new(state.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 101, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 112, 117, 98, 108, 105, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 104, 101, 97, 112, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 98, 117, 100, 103, 101, 116]))
                                                            } else {
                                                                {
                                                                    let field_2 = self::construct_java_script_reflection_property(key_3.clone().expect("a proven present value"), shared_constructor_field12.clone());
                                                                    {
                                                                        let write_3 = self::define_java_script_ordinary_property(heap.clone(), (((match shared_constructor_field11.clone() {
                                                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value6745, _) => {
                                                                                Some((source_ml_source2_data_property_value6745.clone(),))
                                                                            }
                                                                            _ => {
                                                                                None::<(self::ml::Big,)>
                                                                            }
                                                                        }).clone().expect("a proven present value")).0).clone(), key_3.clone().expect("a proven present value"), match field_2.clone() {
                                                                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value6746) => {
                                                                                source_ml_source2_data_property_value6746.clone()
                                                                            }
                                                                        });
                                                                        if ((((({ let ml_reference_value525 = write_3.borrow().clone(); ml_reference_value525 }).0).0) != (self::ml_string::Text::from_units(vec![]))) || !((({ let ml_reference_value526 = write_3.borrow().clone(); ml_reference_value526 }).0).3)) {
                                                                            self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new((({ let ml_reference_value527 = write_3.borrow().clone(); ml_reference_value527 }).0).1), {
                                                                                let source_ml_source2_logical6747 = (({ let ml_reference_value528 = write_3.borrow().clone(); ml_reference_value528 }).0).0;
                                                                                if !source_ml_source2_logical6747.0.is_empty() {
                                                                                    source_ml_source2_logical6747.clone()
                                                                                } else {
                                                                                    self::ml_string::Text::from_units(vec![102, 114, 101, 115, 104, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 115, 117, 108, 116, 32, 109, 117, 115, 116, 32, 97, 99, 99, 101, 112, 116, 32, 105, 116, 115, 32, 111, 119, 110, 32, 100, 97, 116, 97, 32, 112, 114, 111, 112, 101, 114, 116, 121])
                                                                                }
                                                                            })
                                                                        } else {
                                                                            self::JavaScriptPropertyReflectionTransition::PropertyReflectionContinue(Box::new((({ let ml_reference_value529 = write_3.borrow().clone(); ml_reference_value529 }).0).1), Box::new({
                                                                                let source_ml_source2_nominal_field6750 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field6752 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field6754 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field6756 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field6758 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field6760 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_field6762 = shared_constructor_field7.clone();
                                                                                                        {
                                                                                                            let source_ml_source2_nominal_field6764 = shared_constructor_field8;
                                                                                                            {
                                                                                                                let source_ml_source2_nominal_field6766 = shared_constructor_field9.clone();
                                                                                                                {
                                                                                                                    let source_ml_source2_nominal_field6768 = shared_constructor_field10;
                                                                                                                    {
                                                                                                                        let source_ml_source2_nominal_field6770 = shared_constructor_field11.clone();
                                                                                                                        {
                                                                                                                            let source_ml_source2_nominal_field6772 = shared_constructor_field12.clone();
                                                                                                                            {
                                                                                                                                let source_ml_source2_nominal_override6775 = (shared_constructor_field8 + 1f64);
                                                                                                                                {
                                                                                                                                    let source_ml_source2_nominal_override6776 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptOrdinaryProperty>::new()));
                                                                                                                                    {
                                                                                                                                        let source_ml_source2_nominal_override6777 = 0f64;
                                                                                                                                        {
                                                                                                                                            let source_ml_source2_nominal_override6778 = self::JavaScriptSemanticValue::Undefined;
                                                                                                                                            self::JavaScriptPropertyReflectionState::PropertyReflection(source_ml_source2_nominal_field6750.clone(), Box::new(source_ml_source2_nominal_field6752.clone()), Box::new(source_ml_source2_nominal_field6754.clone()), Box::new(source_ml_source2_nominal_field6756.clone()), Box::new(source_ml_source2_nominal_field6758.clone()), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116]), source_ml_source2_nominal_field6762.clone(), source_ml_source2_nominal_override6775, source_ml_source2_nominal_override6776.clone(), source_ml_source2_nominal_override6777, Box::new(source_ml_source2_nominal_field6770.clone()), Box::new(source_ml_source2_nominal_override6778.clone()))
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
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 112, 104, 97, 115, 101]))
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
