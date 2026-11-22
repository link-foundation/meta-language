// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPropertyReflectionState;
use super::JavaScriptPropertyReflectionTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270;
use super::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272;
use super::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Result;
use super::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Result;
use super::find_java_script_ordinary_object;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::source_ml_source1_temporary_read_java_script_property_reflection_state_error_loop269;
use super::source_ml_source1_temporary_read_java_script_property_reflection_state_error_loop271;
use super::source_ml_source1_temporary_test_java_script_reflection_property_budget_loop275;
use super::test_java_script_property_reflection_operation;
use super::test_java_script_reflection_ordinary_kind;
use super::test_java_script_single_descriptor_operation;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_reflection_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptPropertyReflectionState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptPropertyReflectionState::PropertyReflection(
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
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field4 = *shared_constructor_field4;
            let shared_constructor_field5 = *shared_constructor_field5;
            let shared_constructor_field6 = *shared_constructor_field6;
            let shared_constructor_field12 = *shared_constructor_field12;
            let shared_constructor_field13 = *shared_constructor_field13;
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
                        if (((((((((((!(self::test_java_script_property_reflection_operation(
                            &(shared_constructor_field2),
                        ))
                            || !(self::ml_array::includes(
                                &{
                                    let ml_reference_value491 =
                                        (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            self::ml_string::Text::from_units(vec![
                                                107, 101, 121, 115,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                116, 97, 114, 103, 101, 116, 32, 97, 108, 108, 111,
                                                99, 97, 116, 105, 111, 110,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                115, 101, 108, 101, 99, 116,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                100, 101, 115, 99, 114, 105, 112, 116, 111, 114,
                                                32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                100, 101, 115, 99, 114, 105, 112, 116, 111, 114,
                                                32, 119, 114, 105, 116, 101,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                112, 117, 98, 108, 105, 115, 104,
                                            ]),
                                            self::ml_string::Text::from_units(vec![
                                                97, 114, 114, 97, 121, 32, 119, 114, 105, 116, 101,
                                            ]),
                                        ])))
                                        .borrow()
                                        .clone();
                                    ml_reference_value491
                                },
                                &shared_constructor_field7.clone(),
                                0f64,
                                false,
                                |a, b| a == b,
                            )))
                            || (({
                                let ml_reference_array492 = shared_constructor_field8.clone();
                                let ml_reference_projection492 =
                                    ml_reference_array492.borrow().len() as f64;
                                ml_reference_projection492
                            }) > (4096f64)))
                            || !(self::ml_math::is_safe_integer(shared_constructor_field9)))
                            || {
                                let source_ml_source2_same_value_operand6291 =
                                    shared_constructor_field9;
                                {
                                    let source_ml_source2_same_value_operand6292 = (-0f64);
                                    (((source_ml_source2_same_value_operand6291
                                        == source_ml_source2_same_value_operand6292)
                                        && ((source_ml_source2_same_value_operand6291 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand6291)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand6292))))
                                        || ((source_ml_source2_same_value_operand6291
                                            != source_ml_source2_same_value_operand6291)
                                            && (source_ml_source2_same_value_operand6292
                                                != source_ml_source2_same_value_operand6292)))
                                }
                            })
                            || (shared_constructor_field9 < (0f64)))
                            || (shared_constructor_field9
                                > ({
                                    let ml_reference_array493 = shared_constructor_field8.clone();
                                    let ml_reference_projection493 =
                                        ml_reference_array493.borrow().len() as f64;
                                    ml_reference_projection493
                                })))
                            || !(self::ml_math::is_safe_integer(shared_constructor_field11)))
                            || {
                                let source_ml_source2_same_value_operand6301 =
                                    shared_constructor_field11;
                                {
                                    let source_ml_source2_same_value_operand6302 = (-0f64);
                                    (((source_ml_source2_same_value_operand6301
                                        == source_ml_source2_same_value_operand6302)
                                        && ((source_ml_source2_same_value_operand6301 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand6301)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand6302))))
                                        || ((source_ml_source2_same_value_operand6301
                                            != source_ml_source2_same_value_operand6301)
                                            && (source_ml_source2_same_value_operand6302
                                                != source_ml_source2_same_value_operand6302)))
                                }
                            })
                            || (shared_constructor_field11 < (0f64)))
                            || (shared_constructor_field11
                                > ({
                                    let ml_reference_array494 = shared_constructor_field10.clone();
                                    let ml_reference_projection494 =
                                        ml_reference_array494.borrow().len() as f64;
                                    ml_reference_projection494
                                })))
                            || (({
                                let ml_reference_array495 = shared_constructor_field10.clone();
                                let ml_reference_projection495 =
                                    ml_reference_array495.borrow().len() as f64;
                                ml_reference_projection495
                            }) > (4f64)))
                        {
                            self::ml_string::Text::from_units(vec![
                                112, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 102, 108,
                                101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115,
                                32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112,
                                101, 114, 97, 116, 105, 111, 110, 32, 112, 104, 97, 115, 101, 32,
                                97, 110, 100, 32, 98, 111, 117, 110, 100, 101, 100, 32, 119, 114,
                                105, 116, 101, 32, 112, 111, 115, 105, 116, 105, 111, 110, 115,
                            ])
                        } else {
                            {
                                let source_ml_source1_temporary_values267 = std::rc::Rc::new(
                                    std::cell::RefCell::new(self::ml_array::append(
                                        vec![
                                            shared_constructor_field3.clone(),
                                            shared_constructor_field4.clone(),
                                            shared_constructor_field5.clone(),
                                            shared_constructor_field6.clone(),
                                            shared_constructor_field12.clone(),
                                            shared_constructor_field13.clone(),
                                        ],
                                        {
                                            let ml_reference_value496 = (std::rc::Rc::new(std::cell::RefCell::new({ let ml_values497 = { let ml_reference_value498 = shared_constructor_field10.borrow().clone(); ml_reference_value498 }; let ml_callback497 = std::rc::Rc::new(move |field: self::JavaScriptOrdinaryProperty| -> self::JavaScriptSemanticValue { (({ let ml_reference_value499 = (match field.clone() {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value6318) => {
                                            source_ml_source2_data_property_value6318.clone()
                                        }
                                    }).borrow().clone(); ml_reference_value499 }).0).8 }); ml_values497.iter().cloned().enumerate().map(|(ml_index497, ml_value497)| ml_callback497(ml_value497)).collect::<Vec<_>>() }))).borrow().clone();
                                            ml_reference_value496
                                        },
                                    )),
                                );
                                {
                                    let source_ml_source1_temporary_index267 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s6319 = self::source_ml_source1_temporary_read_java_script_property_reflection_state_error_loop269(heap.clone(), source_ml_source1_temporary_values267.clone(), source_ml_source1_temporary_index267.clone());
                                        match source_ml_source2_s6319.clone() {
                                                self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Result::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Done => {
                                                    if (((({
                                                        let source_ml_source2_s6321 = shared_constructor_field5.clone();
                                                        match source_ml_source2_s6321.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    } || {
                                                        let source_ml_source2_s6326 = shared_constructor_field6.clone();
                                                        match source_ml_source2_s6326.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) || {
                                                        let source_ml_source2_s6331 = shared_constructor_field3.clone();
                                                        match source_ml_source2_s6331.clone() {
                                                            self::JavaScriptSemanticValue::Null => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        }
                                                    }) || {
                                                        let source_ml_source2_s6334 = shared_constructor_field3.clone();
                                                        match source_ml_source2_s6334.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        }
                                                    }) || (shared_constructor_field2.starts_with(&self::ml_string::Text::from_units(vec![82, 101, 102, 108, 101, 99, 116, 46])) && {
                                                        let source_ml_source2_s6338 = shared_constructor_field3.clone();
                                                        match source_ml_source2_s6338.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    })) {
                                                        self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 101, 114, 99, 101, 100, 32, 105, 110, 112, 117, 116, 32, 97, 110, 100, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 114, 101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 115])
                                                    } else {
                                                        {
                                                            let source_ml_source2_s6353 = if {
                                                                let source_ml_source2_s6355 = shared_constructor_field3.clone();
                                                                match source_ml_source2_s6355.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                }
                                                            } {
                                                                {
                                                                    let owner = {
                                                                        let source_ml_source2_array_read_value6363 = shared_constructor_field1.clone();
                                                                        {
                                                                            let source_ml_source2_array_read_index6364 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), (((match shared_constructor_field3.clone() {
                                                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value6362, _) => {
                                                                                    Some((source_ml_source2_data_property_value6362.clone(),))
                                                                                }
                                                                                _ => {
                                                                                    None::<(self::ml::Big,)>
                                                                                }
                                                                            }).clone().expect("a proven present value")).0).clone());
                                                                            { let ml_reference_array500 = source_ml_source2_array_read_value6363.clone(); let ml_reference_position500 = source_ml_source2_array_read_index6364; let ml_reference_projection500 = self::ml_array::optional_at(&*ml_reference_array500.borrow(), self::ml_array::number_index(ml_reference_position500)); ml_reference_projection500 }
                                                                        }
                                                                    };
                                                                    if (match (owner.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(self::test_java_script_reflection_ordinary_kind(&(match owner.clone().expect("a proven present value") {
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
                                                                    })))) {
                                                                        self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 119, 110, 45, 107, 101, 121, 32, 97, 110, 100, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 112, 114, 111, 116, 111, 99, 111, 108, 115]))
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270Next
                                                                    }
                                                                }
                                                            } else {
                                                                self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270Next
                                                            };
                                                            match source_ml_source2_s6353.clone() {
                                                                self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270Next => {
                                                                    {
                                                                        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                                                                        {
                                                                            let source_ml_source1_temporary_values268 = shared_constructor_field8.clone();
                                                                            {
                                                                                let source_ml_source1_temporary_index268 = self::ml::Big::from_i128(0);
                                                                                {
                                                                                    let source_ml_source2_s6366 = self::source_ml_source1_temporary_read_java_script_property_reflection_state_error_loop271(heap.clone(), seen.clone(), source_ml_source1_temporary_values268.clone(), source_ml_source1_temporary_index268.clone());
                                                                                    match source_ml_source2_s6366.clone() {
                                                                                        self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Result::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Done => {
                                                                                            {
                                                                                                let source_ml_source2_s6369 = if self::test_java_script_single_descriptor_operation(&(shared_constructor_field2)) {
                                                                                                    {
                                                                                                        let invalid = self::read_java_script_ordinary_key_error(heap.clone(), shared_constructor_field4.clone());
                                                                                                        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                                                            self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272Return(invalid.clone())
                                                                                                        } else {
                                                                                                            self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272Next
                                                                                                        }
                                                                                                    }
                                                                                                } else {
                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272Next
                                                                                                };
                                                                                                match source_ml_source2_s6369.clone() {
                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272Next => {
                                                                                                        if ((((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110]))) || (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 119, 114, 105, 116, 101])))) || (shared_constructor_field7 == (self::ml_string::Text::from_units(vec![112, 117, 98, 108, 105, 115, 104])))) && (({ let ml_reference_array501 = shared_constructor_field10.clone(); let ml_reference_projection501 = ml_reference_array501.borrow().len() as f64; ml_reference_projection501 }) != (4f64))) {
                                                                                                            self::ml_string::Text::from_units(vec![70, 114, 111, 109, 80, 114, 111, 112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 102, 111, 117, 114, 32, 115, 101, 108, 101, 99, 116, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 102, 105, 101, 108, 100, 115])
                                                                                                        } else {
                                                                                                            if ((((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 119, 114, 105, 116, 101]))) && {
                                                                                                                let source_ml_source2_s6383 = shared_constructor_field13.clone();
                                                                                                                match source_ml_source2_s6383.clone() {
                                                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                        false
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        true
                                                                                                                    }
                                                                                                                }
                                                                                                            }) || ((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![112, 117, 98, 108, 105, 115, 104]))) && {
                                                                                                                let source_ml_source2_s6390 = shared_constructor_field13.clone();
                                                                                                                match source_ml_source2_s6390.clone() {
                                                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                        false
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        true
                                                                                                                    }
                                                                                                                }
                                                                                                            })) || ((shared_constructor_field7 == (self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 119, 114, 105, 116, 101]))) && {
                                                                                                                let source_ml_source2_s6397 = shared_constructor_field12.clone();
                                                                                                                match source_ml_source2_s6397.clone() {
                                                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                        false
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        true
                                                                                                                    }
                                                                                                                }
                                                                                                            })) {
                                                                                                                self::ml_string::Text::from_units(vec![114, 101, 102, 108, 101, 99, 116, 105, 111, 110, 32, 119, 114, 105, 116, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 101, 100, 32, 114, 101, 115, 117, 108, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115])
                                                                                                            } else {
                                                                                                                self::ml_string::Text::from_units(vec![])
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                    self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin272Return(source_ml_source1_temporary_result272) => {
                                                                                                        source_ml_source1_temporary_result272.clone()
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                        self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Result::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop271Return(source_ml_source1_temporary_result272_2) => {
                                                                                            source_ml_source1_temporary_result272_2.clone()
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorJoin270Return(source_ml_source1_temporary_result273) => {
                                                                    source_ml_source1_temporary_result273.clone()
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Result::MlSource1TemporaryReadJavaScriptPropertyReflectionStateErrorLoop269Return(source_ml_source1_temporary_result273_2) => {
                                                    source_ml_source1_temporary_result273_2.clone()
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
pub fn begin_java_script_property_reflection(
    heap: self::JavaScriptOrdinaryHeap,
    operation: &self::ml_string::Text,
    input: self::JavaScriptSemanticValue,
    key: self::JavaScriptSemanticValue,
    object_prototype: self::JavaScriptSemanticValue,
    array_prototype: self::JavaScriptSemanticValue,
) -> self::JavaScriptPropertyReflectionTransition {
    {
        let invalid = {
            let source_ml_source2_logical6401 =
                self::read_java_script_ordinary_heap_error(heap.clone());
            if !source_ml_source2_logical6401.0.is_empty() {
                source_ml_source2_logical6401.clone()
            } else {
                self::read_java_script_ordinary_value_error(heap.clone(), input.clone())
            }
        };
        if (invalid != (self::ml_string::Text::from_units(vec![]))) {
            self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(
                Box::new(heap.clone()),
                invalid.clone(),
            )
        } else {
            if ((match input.clone() {
                self::JavaScriptSemanticValue::Null => true,
                _ => false,
            } || match input.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                _ => false,
            }) || (operation.starts_with(&self::ml_string::Text::from_units(vec![
                82, 101, 102, 108, 101, 99, 116, 46,
            ])) && match input.clone() {
                self::JavaScriptSemanticValue::Object(_, _) => false,
                _ => true,
            })) {
                self::JavaScriptPropertyReflectionTransition::PropertyReflectionError(
                    Box::new(heap.clone()),
                    self::ml_string::Text::from_units(vec![
                        84, 121, 112, 101, 69, 114, 114, 111, 114,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        80, 114, 111, 112, 101, 114, 116, 121, 32, 114, 101, 102, 108, 101, 99,
                        116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105,
                        116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 111, 110, 110, 117, 108,
                        108, 32, 79, 98, 106, 101, 99, 116, 32, 105, 110, 112, 117, 116,
                    ]),
                )
            } else {
                {
                    let state = self::JavaScriptPropertyReflectionState::PropertyReflection(
                        operation.clone(),
                        Box::new(input.clone()),
                        Box::new(key.clone()),
                        Box::new(object_prototype.clone()),
                        Box::new(array_prototype.clone()),
                        self::ml_string::Text::from_units(vec![107, 101, 121, 115]),
                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                            self::JavaScriptSemanticValue,
                        >::new())),
                        0f64,
                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                            self::JavaScriptOrdinaryProperty,
                        >::new())),
                        0f64,
                        Box::new(self::JavaScriptSemanticValue::Undefined),
                        Box::new(self::JavaScriptSemanticValue::Undefined),
                    );
                    {
                        let error = self::read_java_script_property_reflection_state_error(
                            heap.clone(),
                            state.clone(),
                        );
                        if (error == (self::ml_string::Text::from_units(vec![]))) {
                            self::JavaScriptPropertyReflectionTransition::PropertyReflectionContinue(
                                Box::new(heap.clone()),
                                Box::new(state.clone()),
                            )
                        } else {
                            self::JavaScriptPropertyReflectionTransition::PropertyReflectionInvalid(
                                Box::new(heap.clone()),
                                error.clone(),
                            )
                        }
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_reflection_property_budget(
    heap: self::JavaScriptOrdinaryHeap,
    target: self::JavaScriptSemanticValue,
    key: self::JavaScriptSemanticValue,
) -> bool {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
            match target.clone() {
                self::JavaScriptSemanticValue::Object(identity, _) => {
                    let object = {
                        let source_ml_source2_array_read_value6407 =
                            shared_constructor_field1.clone();
                        {
                            let source_ml_source2_array_read_index6408 =
                                self::find_java_script_ordinary_object(
                                    shared_constructor_field1.clone(),
                                    identity.clone(),
                                );
                            {
                                let ml_reference_array502 =
                                    source_ml_source2_array_read_value6407.clone();
                                let ml_reference_position502 =
                                    source_ml_source2_array_read_index6408;
                                let ml_reference_projection502 = self::ml_array::optional_at(
                                    &*ml_reference_array502.borrow(),
                                    self::ml_array::number_index(ml_reference_position502),
                                );
                                ml_reference_projection502
                            }
                        }
                    };
                    if match (object.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    } {
                        false
                    } else {
                        if ((self::find_java_script_ordinary_property(
                            match object.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryObject::ClosureObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6422,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value6422.clone(),
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6423,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value6423.clone(),
                                self::JavaScriptOrdinaryObject::ArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6424,
                                ) => source_ml_source2_data_property_value6424.clone(),
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6425,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value6425.clone(),
                                self::JavaScriptOrdinaryObject::MapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6426,
                                    _,
                                ) => source_ml_source2_data_property_value6426.clone(),
                                self::JavaScriptOrdinaryObject::SetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6427,
                                    _,
                                ) => source_ml_source2_data_property_value6427.clone(),
                                self::JavaScriptOrdinaryObject::WeakMapObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6428,
                                    _,
                                ) => source_ml_source2_data_property_value6428.clone(),
                                self::JavaScriptOrdinaryObject::WeakSetObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6429,
                                    _,
                                ) => source_ml_source2_data_property_value6429.clone(),
                                self::JavaScriptOrdinaryObject::OrdinaryObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6430,
                                ) => source_ml_source2_data_property_value6430.clone(),
                                self::JavaScriptOrdinaryObject::HostErrorObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6431,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value6431.clone(),
                                self::JavaScriptOrdinaryObject::HostURLObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6432,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value6432.clone(),
                                self::JavaScriptOrdinaryObject::HostArrayObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6433,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value6433.clone(),
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(
                                    _,
                                    _,
                                    _,
                                    _,
                                    source_ml_source2_data_property_value6434,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value6434.clone(),
                            },
                            key.clone(),
                        )) >= (0f64))
                        {
                            true
                        } else {
                            {
                                let total = 0f64;
                                {
                                    let source_ml_source1_temporary_values274 =
                                        shared_constructor_field1.clone();
                                    {
                                        let source_ml_source1_temporary_index274 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let total_2 = self::source_ml_source1_temporary_test_java_script_reflection_property_budget_loop275(total, source_ml_source1_temporary_values274.clone(), source_ml_source1_temporary_index274.clone());
                                            ((({
                                                let ml_reference_array503 = match object.clone().expect("a proven present value") {
                                                    self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, source_ml_source2_data_property_value6449, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value6449.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, source_ml_source2_data_property_value6450, _, _, _) => {
                                                        source_ml_source2_data_property_value6450.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, source_ml_source2_data_property_value6451) => {
                                                        source_ml_source2_data_property_value6451.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, source_ml_source2_data_property_value6452, _, _, _, _, _, _) => {
                                                        source_ml_source2_data_property_value6452.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, source_ml_source2_data_property_value6453, _) => {
                                                        source_ml_source2_data_property_value6453.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, source_ml_source2_data_property_value6454, _) => {
                                                        source_ml_source2_data_property_value6454.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, source_ml_source2_data_property_value6455, _) => {
                                                        source_ml_source2_data_property_value6455.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, source_ml_source2_data_property_value6456, _) => {
                                                        source_ml_source2_data_property_value6456.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, source_ml_source2_data_property_value6457) => {
                                                        source_ml_source2_data_property_value6457.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostErrorObject(_, _, _, _, source_ml_source2_data_property_value6458, _, _) => {
                                                        source_ml_source2_data_property_value6458.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostURLObject(_, _, _, _, source_ml_source2_data_property_value6459, _, _) => {
                                                        source_ml_source2_data_property_value6459.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostArrayObject(_, _, _, _, source_ml_source2_data_property_value6460, _, _) => {
                                                        source_ml_source2_data_property_value6460.clone()
                                                    }
                                                    self::JavaScriptOrdinaryObject::HostByteBufferObject(_, _, _, _, source_ml_source2_data_property_value6461, _, _) => {
                                                        source_ml_source2_data_property_value6461.clone()
                                                    }
                                                };
                                                let ml_reference_projection503 =
                                                    ml_reference_array503.borrow().len() as f64;
                                                ml_reference_projection503
                                            }) < (4096f64))
                                                && (total_2 < (16384f64)))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => false,
            }
        }
    }
}
