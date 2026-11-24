// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCreationState;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptPrimitiveWrapperState;
use super::JavaScriptSemanticValue;
use super::JavaScriptSynchronousIteratorRecord;
use super::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401;
use super::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402;
use super::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_creation_child_error;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_primitive_wrapper_state_error;
use super::source_ml_source1_temporary_read_java_script_array_creation_state_error_loop400;
use super::test_java_script_array_creation_operation;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_creation_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayCreationState,
) -> self::ml_string::Text {
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
                let error = self::read_java_script_ordinary_heap_error(heap.clone());
                if (error != (self::ml_string::Text::from_units(vec![]))) {
                    error.clone()
                } else {
                    if (((((match state.clone() {
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
                            _,
                            _,
                            _,
                            _,
                        ) => false,
                    } || !(self::test_java_script_array_creation_operation(
                        &(shared_constructor_field1),
                    ))) || (({
                        let ml_reference_array816 = shared_constructor_field3.clone();
                        let ml_reference_projection816 =
                            ml_reference_array816.borrow().len() as f64;
                        ml_reference_projection816
                    }) > (1048576f64)))
                        || (({
                            let source_ml_source2_type_query_operand13885 =
                                shared_constructor_field10;
                            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                        }) != (self::ml_string::Text::from_units(vec![
                            98, 111, 111, 108, 101, 97, 110,
                        ]))))
                        || (({
                            let source_ml_source2_type_query_operand13889 =
                                shared_constructor_field11;
                            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                        }) != (self::ml_string::Text::from_units(vec![
                            98, 111, 111, 108, 101, 97, 110,
                        ]))))
                        || !(self::ml_array::includes(
                            &{
                                let ml_reference_value817 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            109, 101, 116, 104, 111, 100,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116,
                                            101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            119, 114, 97, 112, 112, 101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            108, 101, 110, 103, 116, 104,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116,
                                            101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118,
                                            101, 114, 115, 105, 111, 110,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            99, 111, 110, 115, 116, 114, 117, 99, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113,
                                            117, 105, 115, 105, 116, 105, 111, 110,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            105, 116, 101, 114, 97, 116, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            115, 101, 108, 101, 99, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116,
                                            116, 101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![109, 97, 112]),
                                        self::ml_string::Text::from_units(vec![
                                            100, 101, 102, 105, 110, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            108, 101, 110, 103, 116, 104, 32, 115, 101, 116,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            108, 101, 110, 103, 116, 104, 32, 115, 101, 116, 116,
                                            101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            99, 108, 111, 115, 101,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            101, 114, 114, 111, 114,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value817
                            },
                            &shared_constructor_field12.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        )))
                    {
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32,
                            114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                            117, 97, 108, 32, 111, 112, 101, 114, 97, 116, 105, 111, 110, 32, 97,
                            114, 103, 117, 109, 101, 110, 116, 115, 32, 97, 110, 100, 32, 99, 111,
                            110, 116, 105, 110, 117, 97, 116, 105, 111, 110, 32, 112, 104, 97, 115,
                            101,
                        ])
                    } else {
                        if (((((((!(self::ml_math::is_safe_integer(
                            shared_constructor_field13,
                        )) || (shared_constructor_field13 < (0f64)))
                            || (shared_constructor_field13 > (9007199254740991f64)))
                            || {
                                let source_ml_source2_same_value_operand13897 =
                                    shared_constructor_field13;
                                {
                                    let source_ml_source2_same_value_operand13898 = (-0f64);
                                    (((source_ml_source2_same_value_operand13897
                                        == source_ml_source2_same_value_operand13898)
                                        && ((source_ml_source2_same_value_operand13897 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand13897)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand13898))))
                                        || ((source_ml_source2_same_value_operand13897
                                            != source_ml_source2_same_value_operand13897)
                                            && (source_ml_source2_same_value_operand13898
                                                != source_ml_source2_same_value_operand13898)))
                                }
                            })
                            || !(self::ml_math::is_safe_integer(shared_constructor_field14)))
                            || (shared_constructor_field14 < (0f64)))
                            || (shared_constructor_field14 > (9007199254740991f64)))
                            || {
                                let source_ml_source2_same_value_operand13905 =
                                    shared_constructor_field14;
                                {
                                    let source_ml_source2_same_value_operand13906 = (-0f64);
                                    (((source_ml_source2_same_value_operand13905
                                        == source_ml_source2_same_value_operand13906)
                                        && ((source_ml_source2_same_value_operand13905 != (0f64))
                                            || ((1f64
                                                / source_ml_source2_same_value_operand13905)
                                                == (1f64
                                                    / source_ml_source2_same_value_operand13906))))
                                        || ((source_ml_source2_same_value_operand13905
                                            != source_ml_source2_same_value_operand13905)
                                            && (source_ml_source2_same_value_operand13906
                                                != source_ml_source2_same_value_operand13906)))
                                }
                            })
                        {
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110,
                                32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116,
                                32, 84, 111, 76, 101, 110, 103, 116, 104, 32, 97, 110, 100, 32, 98,
                                111, 117, 110, 100, 101, 100, 32, 115, 97, 102, 101, 32, 105, 110,
                                100, 101, 120, 32, 99, 111, 111, 114, 100, 105, 110, 97, 116, 101,
                                115,
                            ])
                        } else {
                            {
                                let source_ml_source1_temporary_values399 = std::rc::Rc::new(
                                    std::cell::RefCell::new(self::ml_array::append(
                                        vec![
                                            shared_constructor_field2.clone(),
                                            shared_constructor_field4.clone(),
                                            shared_constructor_field5.clone(),
                                            shared_constructor_field6.clone(),
                                            shared_constructor_field7.clone(),
                                            shared_constructor_field8.clone(),
                                            shared_constructor_field9.clone(),
                                            shared_constructor_field15.clone(),
                                            shared_constructor_field16.clone(),
                                            shared_constructor_field17.clone(),
                                            shared_constructor_field18.clone(),
                                            shared_constructor_field19.clone(),
                                        ],
                                        {
                                            let ml_reference_value818 =
                                                shared_constructor_field3.borrow().clone();
                                            ml_reference_value818
                                        },
                                    )),
                                );
                                {
                                    let source_ml_source1_temporary_index399 =
                                        self::ml::Big::from_i128(0);
                                    {
                                        let source_ml_source2_s13920 = self::source_ml_source1_temporary_read_java_script_array_creation_state_error_loop400(heap.clone(), source_ml_source1_temporary_values399.clone(), source_ml_source1_temporary_index399.clone());
                                        match source_ml_source2_s13920.clone() {
                                            self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Result::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Done => {
                                                if ((({
                                                    let source_ml_source2_s13922 = shared_constructor_field4.clone();
                                                    match source_ml_source2_s13922.clone() {
                                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }
                                                } || ((match shared_constructor_field4.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value13927) => {
                                                        Some((source_ml_source2_data_property_value13927,))
                                                    }
                                                    _ => {
                                                        None::<(bool,)>
                                                    }
                                                }).clone().expect("a proven present value")).0) || {
                                                    let source_ml_source2_s13929 = shared_constructor_field5.clone();
                                                    match source_ml_source2_s13929.clone() {
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }
                                                }) || {
                                                    let source_ml_source2_s13933 = shared_constructor_field6.clone();
                                                    match source_ml_source2_s13933.clone() {
                                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                                            false
                                                        }
                                                        _ => {
                                                            true
                                                        }
                                                    }
                                                }) {
                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 65, 114, 114, 97, 121, 32, 97, 110, 100, 32, 119, 101, 108, 108, 45, 107, 110, 111, 119, 110, 32, 83, 121, 109, 98, 111, 108, 32, 97, 117, 116, 104, 111, 114, 105, 116, 121])
                                                } else {
                                                    if ((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 111, 102]))) && ((((((((shared_constructor_field10 || shared_constructor_field11) || (shared_constructor_field14 != ({ let ml_reference_array819 = shared_constructor_field3.clone(); let ml_reference_projection819 = ml_reference_array819.borrow().len() as f64; ml_reference_projection819 }))) || {
                                                        let source_ml_source2_s13945 = shared_constructor_field7.clone();
                                                        match source_ml_source2_s13945.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) || {
                                                        let source_ml_source2_s13948 = shared_constructor_field8.clone();
                                                        match source_ml_source2_s13948.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) || {
                                                        let source_ml_source2_s13951 = shared_constructor_field9.clone();
                                                        match source_ml_source2_s13951.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) || {
                                                        let source_ml_source2_s13954 = shared_constructor_field16.clone();
                                                        match source_ml_source2_s13954.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) || !(match (match state.clone() {
                                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value13956, _, _, _, _, _, _, _, _, _) => {
                                                            ((source_ml_source2_data_property_value13956.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                        }
                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) || (shared_constructor_field13 > shared_constructor_field14))) {
                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 111, 102, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 105, 109, 109, 117, 116, 97, 98, 108, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 32, 76, 105, 115, 116])
                                                    } else {
                                                        if ((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109]))) && ((((!(self::test_java_script_semantic_same_value(shared_constructor_field7.clone(), ({
                                                            let source_ml_source2_array_read_value13965 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_array_read_index13966 = 0f64;
                                                                { let ml_reference_array820 = source_ml_source2_array_read_value13965.clone(); let ml_reference_position820 = source_ml_source2_array_read_index13966; let ml_reference_projection820 = self::ml_array::optional_at(&*ml_reference_array820.borrow(), self::ml_array::number_index(ml_reference_position820)); ml_reference_projection820 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined))) || !(self::test_java_script_semantic_same_value(shared_constructor_field8.clone(), ({
                                                            let source_ml_source2_array_read_value13969 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_array_read_index13970 = 1f64;
                                                                { let ml_reference_array821 = source_ml_source2_array_read_value13969.clone(); let ml_reference_position821 = source_ml_source2_array_read_index13970; let ml_reference_projection821 = self::ml_array::optional_at(&*ml_reference_array821.borrow(), self::ml_array::number_index(ml_reference_position821)); ml_reference_projection821 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || !(self::test_java_script_semantic_same_value(shared_constructor_field9.clone(), ({
                                                            let source_ml_source2_array_read_value13973 = shared_constructor_field3.clone();
                                                            {
                                                                let source_ml_source2_array_read_index13974 = 2f64;
                                                                { let ml_reference_array822 = source_ml_source2_array_read_value13973.clone(); let ml_reference_position822 = source_ml_source2_array_read_index13974; let ml_reference_projection822 = self::ml_array::optional_at(&*ml_reference_array822.borrow(), self::ml_array::number_index(ml_reference_position822)); ml_reference_projection822 }
                                                            }
                                                        }).clone().unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined)))) || (shared_constructor_field10 != ({
                                                            let source_ml_source2_s13981 = shared_constructor_field8.clone();
                                                            match source_ml_source2_s13981.clone() {
                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }
                                                        }))) || (shared_constructor_field10 && ({
                                                            let source_ml_source2_s13985 = shared_constructor_field8.clone();
                                                            match source_ml_source2_s13985.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                    false
                                                                }
                                                                _ => {
                                                                    true
                                                                }
                                                            }
                                                        } || !(((match shared_constructor_field8.clone() {
                                                            self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value13990) => {
                                                                Some((source_ml_source2_data_property_value13990,))
                                                            }
                                                            _ => {
                                                                None::<(bool,)>
                                                            }
                                                        }).clone().expect("a proven present value")).0))))) {
                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 102, 105, 114, 115, 116, 32, 116, 104, 114, 101, 101, 32, 97, 114, 103, 117, 109, 101, 110, 116, 115, 32, 97, 110, 100, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 109, 97, 112, 112, 101, 114])
                                                        } else {
                                                            if (shared_constructor_field11 && (((shared_constructor_field1 != (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109]))) || {
                                                                let source_ml_source2_s13995 = shared_constructor_field15.clone();
                                                                match source_ml_source2_s13995.clone() {
                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                        false
                                                                    }
                                                                    _ => {
                                                                        true
                                                                    }
                                                                }
                                                            }) || !(((match shared_constructor_field15.clone() {
                                                                self::JavaScriptSemanticValue::Object(_, source_ml_source2_data_property_value14000) => {
                                                                    Some((source_ml_source2_data_property_value14000,))
                                                                }
                                                                _ => {
                                                                    None::<(bool,)>
                                                                }
                                                            }).clone().expect("a proven present value")).0))) {
                                                                self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 98, 108, 101, 32, 65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 114, 101, 116, 97, 105, 110, 115, 32, 116, 104, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 99, 104, 101, 100, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 109, 101, 116, 104, 111, 100])
                                                            } else {
                                                                {
                                                                    let source_ml_source2_s14014 = if !(match (match state.clone() {
                                                                        self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14015, _, _, _, _, _, _, _, _, _, _) => {
                                                                            ((source_ml_source2_data_property_value14015.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                        }
                                                                    }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                        {
                                                                            let invalid = self::read_java_script_ordinary_value_error(heap.clone(), (match state.clone() {
                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14016, _, _, _, _, _, _, _, _, _, _) => {
                                                                                    ((source_ml_source2_data_property_value14016.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                }
                                                                            }).clone().expect("a proven present value"));
                                                                            if (((invalid != (self::ml_string::Text::from_units(vec![]))) || {
                                                                                let source_ml_source2_s14018 = shared_constructor_field7.clone();
                                                                                match source_ml_source2_s14018.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                        true
                                                                                    }
                                                                                    _ => {
                                                                                        false
                                                                                    }
                                                                                }
                                                                            }) || {
                                                                                let source_ml_source2_s14023 = (match state.clone() {
                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14026, _, _, _, _, _, _, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value14026.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                    }
                                                                                }).clone().expect("a proven present value");
                                                                                match source_ml_source2_s14023.clone() {
                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                        false
                                                                                    }
                                                                                    _ => {
                                                                                        true
                                                                                    }
                                                                                }
                                                                            }) {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401Return(if !invalid.0.is_empty() {
                                                                                    invalid.clone()
                                                                                } else {
                                                                                    self::ml_string::Text::from_units(vec![112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 108, 111, 111, 107, 117, 112, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 97, 108, 109, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                                                })
                                                                            } else {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401Next
                                                                            }
                                                                        }
                                                                    } else {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401Next
                                                                    };
                                                                    match source_ml_source2_s14014.clone() {
                                                                        self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401Next => {
                                                                            {
                                                                                let source_ml_source2_s14042 = if !(match (match state.clone() {
                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14043, _, _, _, _, _) => {
                                                                                        ((source_ml_source2_data_property_value14043.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperState>,))).0).clone()
                                                                                    }
                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                                                                    {
                                                                                        let invalid_2 = self::read_java_script_primitive_wrapper_state_error(heap.clone(), (match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14044, _, _, _, _, _) => {
                                                                                                ((source_ml_source2_data_property_value14044.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperState>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value"));
                                                                                        if (((((invalid_2 != (self::ml_string::Text::from_units(vec![]))) || (shared_constructor_field12 != (self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114])))) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14047, _, _, _, _, _) => {
                                                                                                ((source_ml_source2_data_property_value14047.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperState>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(source_ml_source2_data_property_value14048, _) => {
                                                                                                let source_ml_source2_data_property_value14048 = *source_ml_source2_data_property_value14048;
                                                                                                source_ml_source2_data_property_value14048.clone()
                                                                                            }
                                                                                        }, shared_constructor_field7.clone()))) || match (match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14050, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                ((source_ml_source2_data_property_value14050.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                            }
                                                                                        }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) || !(self::test_java_script_semantic_same_value(match (match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14051, _, _, _, _, _) => {
                                                                                                ((source_ml_source2_data_property_value14051.clone().unwrap_or_else(|| (None::<self::JavaScriptPrimitiveWrapperState>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value") {
                                                                                            self::JavaScriptPrimitiveWrapperState::PrimitiveWrapperState(_, source_ml_source2_data_property_value14052) => {
                                                                                                let source_ml_source2_data_property_value14052 = *source_ml_source2_data_property_value14052;
                                                                                                source_ml_source2_data_property_value14052.clone()
                                                                                            }
                                                                                        }, (match state.clone() {
                                                                                            self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14053, _, _, _, _, _, _, _, _, _, _) => {
                                                                                                ((source_ml_source2_data_property_value14053.clone().unwrap_or_else(|| (None::<self::JavaScriptSemanticValue>,))).0).clone()
                                                                                            }
                                                                                        }).clone().expect("a proven present value")))) {
                                                                                            self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402Return(if !invalid_2.0.is_empty() {
                                                                                                invalid_2.clone()
                                                                                            } else {
                                                                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 84, 111, 79, 98, 106, 101, 99, 116, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 111, 114, 105, 103, 105, 110, 97, 108, 32, 112, 114, 105, 109, 105, 116, 105, 118, 101, 32, 97, 110, 100, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101])
                                                                                            })
                                                                                        } else {
                                                                                            self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402Next
                                                                                        }
                                                                                    }
                                                                                } else {
                                                                                    if (shared_constructor_field12 == (self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]))) {
                                                                                        self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402Return(self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 98, 111, 120, 105, 110, 103, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 99, 111, 110, 116, 105, 110, 117, 97, 116, 105, 111, 110]))
                                                                                    } else {
                                                                                        self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402Next
                                                                                    }
                                                                                };
                                                                                match source_ml_source2_s14042.clone() {
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402Next => {
                                                                                        if ((shared_constructor_field1 == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 111, 102]))) && !(self::ml_array::includes(&{ let ml_reference_value823 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116]), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 115, 101, 116]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 115, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]))).borrow().clone(); ml_reference_value823 }, &shared_constructor_field12.clone(), 0f64, false, |a, b| a == b))) {
                                                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 111, 102, 32, 117, 115, 101, 115, 32, 111, 110, 108, 121, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 67, 111, 110, 115, 116, 114, 117, 99, 116, 32, 101, 108, 101, 109, 101, 110, 116, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 97, 110, 100, 32, 102, 105, 110, 97, 108, 32, 83, 101, 116, 32, 112, 104, 97, 115, 101, 115])
                                                                                        } else {
                                                                                            if ((shared_constructor_field11 && self::ml_array::includes(&{ let ml_reference_value824 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 101]), self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]), self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])]))).borrow().clone(); ml_reference_value824 }, &shared_constructor_field12.clone(), 0f64, false, |a, b| a == b)) && (match (match state.clone() {
                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14062, _, _, _, _, _, _, _, _, _) => {
                                                                                                    ((source_ml_source2_data_property_value14062.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                }
                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match (match state.clone() {
                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14063, _, _, _, _, _, _, _, _, _) => {
                                                                                                    ((source_ml_source2_data_property_value14063.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                }
                                                                                            }).clone().expect("a proven present value") {
                                                                                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value14064) => {
                                                                                                    source_ml_source2_data_property_value14064
                                                                                                }
                                                                                            })) {
                                                                                                self::ml_string::Text::from_units(vec![108, 105, 118, 101, 32, 105, 116, 101, 114, 97, 98, 108, 101, 32, 65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 117, 110, 102, 105, 110, 105, 115, 104, 101, 100, 32, 114, 101, 99, 111, 114, 100])
                                                                                            } else {
                                                                                                if (shared_constructor_field11 && self::ml_array::includes(&{ let ml_reference_value825 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110])]))).borrow().clone(); ml_reference_value825 }, &shared_constructor_field12.clone(), 0f64, false, |a, b| a == b)) {
                                                                                                    self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 98, 108, 101, 32, 65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 99, 97, 110, 110, 111, 116, 32, 115, 117, 98, 115, 116, 105, 116, 117, 116, 101, 32, 97, 114, 114, 97, 121, 45, 108, 105, 107, 101, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108])
                                                                                                } else {
                                                                                                    if (self::ml_array::includes(&{ let ml_reference_value826 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101])]))).borrow().clone(); ml_reference_value826 }, &shared_constructor_field12.clone(), 0f64, false, |a, b| a == b) && (shared_constructor_field13 >= (if shared_constructor_field11 {
                                                                                                        9007199254740991f64
                                                                                                    } else {
                                                                                                        shared_constructor_field14
                                                                                                    }))) {
                                                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 101, 108, 101, 109, 101, 110, 116, 32, 119, 111, 114, 107, 32, 114, 101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 114, 101, 109, 97, 105, 110, 105, 110, 103, 32, 105, 110, 100, 101, 120])
                                                                                                    } else {
                                                                                                        if (({
                                                                                                            let source_ml_source2_s14075 = shared_constructor_field16.clone();
                                                                                                            match source_ml_source2_s14075.clone() {
                                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                                    false
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    true
                                                                                                                }
                                                                                                            }
                                                                                                        } && {
                                                                                                            let source_ml_source2_s14078 = shared_constructor_field16.clone();
                                                                                                            match source_ml_source2_s14078.clone() {
                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                    false
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    true
                                                                                                                }
                                                                                                            }
                                                                                                        }) || ({
                                                                                                            let source_ml_source2_s14083 = shared_constructor_field17.clone();
                                                                                                            match source_ml_source2_s14083.clone() {
                                                                                                                self::JavaScriptSemanticValue::Undefined => {
                                                                                                                    false
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    true
                                                                                                                }
                                                                                                            }
                                                                                                        } && {
                                                                                                            let source_ml_source2_s14086 = shared_constructor_field17.clone();
                                                                                                            match source_ml_source2_s14086.clone() {
                                                                                                                self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                    false
                                                                                                                }
                                                                                                                _ => {
                                                                                                                    true
                                                                                                                }
                                                                                                            }
                                                                                                        })) {
                                                                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 97, 99, 116, 117, 97, 108, 32, 97, 114, 114, 97, 121, 45, 108, 105, 107, 101, 32, 97, 110, 100, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 101, 100, 32, 79, 98, 106, 101, 99, 116, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115])
                                                                                                        } else {
                                                                                                            if (self::ml_array::includes(&{ let ml_reference_value827 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110]), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 101]), self::ml_string::Text::from_units(vec![115, 101, 108, 101, 99, 116]), self::ml_string::Text::from_units(vec![101, 108, 101, 109, 101, 110, 116, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![109, 97, 112]), self::ml_string::Text::from_units(vec![100, 101, 102, 105, 110, 101]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 115, 101, 116]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 115, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![99, 108, 111, 115, 101]), self::ml_string::Text::from_units(vec![101, 114, 114, 111, 114])]))).borrow().clone(); ml_reference_value827 }, &shared_constructor_field12.clone(), 0f64, false, |a, b| a == b) && {
                                                                                                                let source_ml_source2_s14092 = shared_constructor_field17.clone();
                                                                                                                match source_ml_source2_s14092.clone() {
                                                                                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                                                                                        false
                                                                                                                    }
                                                                                                                    _ => {
                                                                                                                        true
                                                                                                                    }
                                                                                                                }
                                                                                                            }) {
                                                                                                                self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 101, 108, 101, 109, 101, 110, 116, 32, 119, 111, 114, 107, 32, 102, 111, 108, 108, 111, 119, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 67, 111, 110, 115, 116, 114, 117, 99, 116, 32, 111, 114, 32, 65, 114, 114, 97, 121, 67, 114, 101, 97, 116, 101])
                                                                                                            } else {
                                                                                                                if (self::ml_array::includes(&{ let ml_reference_value828 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]), self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116]), self::ml_string::Text::from_units(vec![105, 116, 101, 114, 97, 116, 111, 114, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110])]))).borrow().clone(); ml_reference_value828 }, &shared_constructor_field12.clone(), 0f64, false, |a, b| a == b) && !(match (match state.clone() {
                                                                                                                    self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14097, _, _, _, _, _, _, _, _, _) => {
                                                                                                                        ((source_ml_source2_data_property_value14097.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                                    }
                                                                                                                }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) {
                                                                                                                    self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 102, 114, 111, 109, 32, 99, 97, 110, 110, 111, 116, 32, 111, 119, 110, 32, 97, 110, 32, 105, 116, 101, 114, 97, 116, 111, 114, 32, 114, 101, 99, 111, 114, 100, 32, 98, 101, 102, 111, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 110, 115, 116, 114, 117, 99, 116, 105, 111, 110, 32, 97, 110, 100, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 97, 99, 113, 117, 105, 115, 105, 116, 105, 111, 110])
                                                                                                                } else {
                                                                                                                    if (self::ml_array::includes(&{ let ml_reference_value829 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100]), self::ml_string::Text::from_units(vec![109, 101, 116, 104, 111, 100, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![119, 114, 97, 112, 112, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 103, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110]), self::ml_string::Text::from_units(vec![99, 111, 110, 115, 116, 114, 117, 99, 116])]))).borrow().clone(); ml_reference_value829 }, &shared_constructor_field12.clone(), 0f64, false, |a, b| a == b) && {
                                                                                                                        let source_ml_source2_s14100 = shared_constructor_field17.clone();
                                                                                                                        match source_ml_source2_s14100.clone() {
                                                                                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                                                                                false
                                                                                                                            }
                                                                                                                            _ => {
                                                                                                                                true
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }) {
                                                                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 99, 97, 110, 110, 111, 116, 32, 112, 117, 98, 108, 105, 115, 104, 32, 97, 32, 114, 101, 115, 117, 108, 116, 32, 98, 101, 102, 111, 114, 101, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 67, 111, 110, 115, 116, 114, 117, 99, 116, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110])
                                                                                                                    } else {
                                                                                                                        if (self::ml_array::includes(&{ let ml_reference_value830 = (std::rc::Rc::new(std::cell::RefCell::new(vec![self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 115, 101, 116]), self::ml_string::Text::from_units(vec![108, 101, 110, 103, 116, 104, 32, 115, 101, 116, 116, 101, 114]), self::ml_string::Text::from_units(vec![97, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104])]))).borrow().clone(); ml_reference_value830 }, &shared_constructor_field12.clone(), 0f64, false, |a, b| a == b) && if shared_constructor_field11 {
                                                                                                                            (match (match state.clone() {
                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14104, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                    ((source_ml_source2_data_property_value14104.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                                                }
                                                                                                                            }, None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || !(match (match state.clone() {
                                                                                                                                self::JavaScriptArrayCreationState::ArrayCreationState(_, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, _, source_ml_source2_data_property_value14105, _, _, _, _, _, _, _, _, _) => {
                                                                                                                                    ((source_ml_source2_data_property_value14105.clone().unwrap_or_else(|| (None::<self::JavaScriptSynchronousIteratorRecord>,))).0).clone()
                                                                                                                                }
                                                                                                                            }).clone().expect("a proven present value") {
                                                                                                                                self::JavaScriptSynchronousIteratorRecord::SynchronousIteratorRecord(_, _, source_ml_source2_data_property_value14106) => {
                                                                                                                                    source_ml_source2_data_property_value14106
                                                                                                                                }
                                                                                                                            }))
                                                                                                                        } else {
                                                                                                                            (shared_constructor_field13 != shared_constructor_field14)
                                                                                                                        }) {
                                                                                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 102, 105, 110, 97, 108, 32, 83, 101, 116, 32, 102, 111, 108, 108, 111, 119, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 104, 97, 117, 115, 116, 101, 100, 32, 105, 116, 101, 114, 97, 116, 105, 111, 110, 32, 111, 114, 32, 99, 111, 109, 112, 108, 101, 116, 101, 100, 32, 97, 114, 114, 97, 121, 45, 108, 105, 107, 101, 32, 105, 110, 100, 105, 99, 101, 115])
                                                                                                                        } else {
                                                                                                                            self::read_java_script_array_creation_child_error(heap.clone(), state.clone())
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
                                                                                    self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin402Return(source_ml_source1_temporary_result402) => {
                                                                                        source_ml_source1_temporary_result402.clone()
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                        self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorJoin401Return(source_ml_source1_temporary_result402_2) => {
                                                                            source_ml_source1_temporary_result402_2.clone()
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            self::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Result::MlSource1TemporaryReadJavaScriptArrayCreationStateErrorLoop400Return(source_ml_source1_temporary_result402_3) => {
                                                source_ml_source1_temporary_result402_3.clone()
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
