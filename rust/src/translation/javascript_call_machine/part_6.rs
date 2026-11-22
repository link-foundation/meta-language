// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCallMachine;
use super::JavaScriptIteratorThrowTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Result;
use super::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result;
use super::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Result;
use super::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Result;
use super::ml;
use super::ml_array;
use super::ml_math;
use super::ml_source_exception;
use super::ml_string;
use super::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop7;
use super::source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop9;
use super::source_ml_source1_temporary_read_java_script_semantic_text_string_loop6;
use super::source_ml_source1_temporary_read_java_script_semantic_value_error_loop3;
use super::source_ml_source1_temporary_read_java_script_weak_key_eligibility_loop2;
use super::source_ml_source1_temporary_test_java_script_semantic_text_equal_loop4;

#[derive(Clone, Debug, PartialEq)]
pub enum JavaScriptIteratorThrowResolution {
    IteratorThrowContinue(
        Box<self::JavaScriptCallMachine>,
        Box<self::JavaScriptSemanticValue>,
    ),
    IteratorThrowTransition(Box<self::JavaScriptIteratorThrowTransition>),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_validated_heap_snapshot(_heap: self::JavaScriptOrdinaryHeap) -> bool {
    false
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_weak_collection_method_owner(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    if self::ml_array::includes(
        &{
            let ml_reference_value1 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    87, 101, 97, 107, 77, 97, 112, 46, 103, 101, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    87, 101, 97, 107, 77, 97, 112, 46, 115, 101, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    87, 101, 97, 107, 77, 97, 112, 46, 104, 97, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    87, 101, 97, 107, 77, 97, 112, 46, 100, 101, 108, 101, 116, 101,
                ]),
            ])))
            .borrow()
            .clone();
            ml_reference_value1
        },
        operation,
        0f64,
        false,
        |a, b| a == b,
    ) {
        self::ml_string::Text::from_units(vec![87, 101, 97, 107, 77, 97, 112])
    } else {
        if self::ml_array::includes(
            &{
                let ml_reference_value2 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                    self::ml_string::Text::from_units(vec![
                        87, 101, 97, 107, 83, 101, 116, 46, 97, 100, 100,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        87, 101, 97, 107, 83, 101, 116, 46, 104, 97, 115,
                    ]),
                    self::ml_string::Text::from_units(vec![
                        87, 101, 97, 107, 83, 101, 116, 46, 100, 101, 108, 101, 116, 101,
                    ]),
                ])))
                .borrow()
                .clone();
                ml_reference_value2
            },
            operation,
            0f64,
            false,
            |a, b| a == b,
        ) {
            self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_weak_collection_operation_kind(
    operation: &self::ml_string::Text,
) -> self::ml_string::Text {
    self::read_java_script_weak_collection_method_owner(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_weak_key_eligibility(
    heap: self::JavaScriptOrdinaryHeap,
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Object(_, _) => {
            self::ml_string::Text::from_units(vec![101, 108, 105, 103, 105, 98, 108, 101])
        }
        _ => {
            match value.clone() {
                self::JavaScriptSemanticValue::Symbol(identity) => {
                    let facts = if {
                        let source_ml_source2_comparison_operand4 = match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                _,
                                source_ml_source2_data_property_value3,
                                _,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value3
                                .clone()
                                .unwrap_or_else(|| (None::<self::ml_string::Text>,)))
                            .0)
                                .clone(),
                        };
                        match (
                            source_ml_source2_comparison_operand4.clone(),
                            self::ml_string::Text::from_units(vec![
                                97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 83,
                                121, 109, 98, 111, 108, 32, 114, 101, 103, 105, 115, 116, 114, 121,
                                32, 99, 108, 97, 115, 115, 105, 102, 105, 99, 97, 116, 105, 111,
                                110, 32, 49,
                            ]),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        }
                    } {
                        (match heap.clone() {
                            self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                                _,
                                _,
                                _,
                                source_ml_source2_data_property_value6,
                                _,
                                _,
                            ) => ((source_ml_source2_data_property_value6
                                .clone()
                                .unwrap_or_else(|| {
                                    (None::<
                                        std::rc::Rc<
                                            std::cell::RefCell<
                                                Vec<self::JavaScriptSymbolRegistrationFact>,
                                            >,
                                        >,
                                    >,)
                                }))
                            .0)
                                .clone(),
                        })
                        .clone()
                        .unwrap_or_else(|| {
                            std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                                self::JavaScriptSymbolRegistrationFact,
                            >::new(
                            )))
                        })
                    } else {
                        std::rc::Rc::new(std::cell::RefCell::new(Vec::<
                            self::JavaScriptSymbolRegistrationFact,
                        >::new()))
                    };
                    {
                        let low = 0f64;
                        {
                            let high = {
                                let ml_reference_array3 = facts.clone();
                                let ml_reference_projection3 =
                                    ml_reference_array3.borrow().len() as f64;
                                ml_reference_projection3
                            };
                            {
                                let low_2 = self::source_ml_source1_temporary_read_java_script_weak_key_eligibility_loop2(value.clone(), facts.clone(), low, high);
                                {
                                    let candidate = {
                                        let ml_reference_array4 = facts.clone();
                                        let ml_reference_position4 = low_2;
                                        let ml_reference_projection4 = self::ml_array::optional_at(
                                            &*ml_reference_array4.borrow(),
                                            self::ml_array::number_index(ml_reference_position4),
                                        );
                                        ml_reference_projection4
                                    };
                                    {
                                        let fact = if {
                                            let source_ml_source2_comparison_operand11 = {
                                                let source_ml_source2_optional_chain_receiver9 =
                                                    (candidate.clone(),);
                                                if match (source_ml_source2_optional_chain_receiver9.clone()).0 { Some(ml_present) => { let _ = ml_present.clone(); true }, None => false } {
                                                        Some({
                                                            let source_ml_source1_temporary_optional_chain1 = ((source_ml_source2_optional_chain_receiver9.clone()).0).clone().expect("a proven present value");
                                                            match source_ml_source1_temporary_optional_chain1.clone() {
                                                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(source_ml_source2_data_property_value10, _, _) => {
                                                                    source_ml_source2_data_property_value10.clone()
                                                                }
                                                            }
                                                        })
                                                    } else {
                                                        None::<self::ml::Big>
                                                    }
                                            };
                                            match (
                                                source_ml_source2_comparison_operand11.clone(),
                                                identity.clone(),
                                            ) {
                                                (Some(ml_left), ml_right) => ml_left == ml_right,
                                                _ => false,
                                            }
                                        } {
                                            candidate.clone()
                                        } else {
                                            None::<self::JavaScriptSymbolRegistrationFact>
                                        };
                                        if match (fact.clone(), None::<()>) {
                                            (Some(_ml_left), Some(_ml_right)) => false,
                                            (None, None) => true,
                                            _ => false,
                                        } {
                                            self::ml_string::Text::from_units(vec![
                                                109, 105, 115, 115, 105, 110, 103, 32, 115, 121,
                                                109, 98, 111, 108, 32, 114, 101, 103, 105, 115,
                                                116, 114, 97, 116, 105, 111, 110, 32, 97, 117, 116,
                                                104, 111, 114, 105, 116, 121,
                                            ])
                                        } else {
                                            if match fact.clone().expect("a proven present value") {
                                                    self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(_, source_ml_source2_data_property_value13, _) => {
                                                        source_ml_source2_data_property_value13
                                                    }
                                                } {
                                                    self::ml_string::Text::from_units(vec![105, 110, 101, 108, 105, 103, 105, 98, 108, 101])
                                                } else {
                                                    self::ml_string::Text::from_units(vec![101, 108, 105, 103, 105, 98, 108, 101])
                                                }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => self::ml_string::Text::from_units(vec![
                    105, 110, 101, 108, 105, 103, 105, 98, 108, 101,
                ]),
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_source_text_profile_error(
    profile: &self::ml_string::Text,
    length: f64,
) -> self::ml_string::Text {
    if ((*profile)
        != (self::ml_string::Text::from_units(vec![
            105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 115,
            111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49,
        ])))
    {
        self::ml_string::Text::from_units(vec![
            115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114,
            101, 115, 32, 105, 116, 115, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 105, 110,
            115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32, 114, 101, 115,
            111, 117, 114, 99, 101, 32, 112, 114, 111, 102, 105, 108, 101,
        ])
    } else {
        if (((!(self::ml_math::is_safe_integer(length)) || (length < (0f64)))
            || (length > (16777216f64)))
            || ((length == (0f64)) && ((1f64 / length) < (0f64))))
        {
            self::ml_string::Text::from_units(vec![
                115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 101, 120, 99, 101, 101,
                100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 49, 54,
                55, 55, 55, 50, 49, 54, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98,
                117, 100, 103, 101, 116,
            ])
        } else {
            self::ml_string::Text::from_units(vec![])
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_value_error(
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::Text(units) => {
            if (({
                let ml_reference_array5 = units.clone();
                let ml_reference_projection5 = ml_reference_array5.borrow().len() as f64;
                ml_reference_projection5
            }) > (1048576f64))
            {
                self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 101, 120, 99,
                    101, 101, 100, 115, 32, 116, 104, 101, 32, 101, 120, 112, 108, 105, 99, 105,
                    116, 32, 85, 84, 70, 49, 54, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103,
                    101, 116,
                ])
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s18 = self::source_ml_source1_temporary_read_java_script_semantic_value_error_loop3(units.clone(), at);
                        match source_ml_source2_s18.clone() {
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Done => {
                                self::ml_string::Text::from_units(vec![])
                            }
                            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Return(source_ml_source1_temporary_result3) => {
                                source_ml_source1_temporary_result3.clone()
                            }
                        }
                    }
                }
            }
        }
        self::JavaScriptSemanticValue::SourceText(profile, text) => {
            self::read_java_script_source_text_profile_error(&(profile), text.length())
        }
        self::JavaScriptSemanticValue::Symbol(identity) => {
            if ((identity > (self::ml::Big::from_i128(0)))
                && (identity <= (self::ml::Big::from_i128(18446744073709551615))))
            {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![
                    97, 32, 115, 121, 109, 98, 111, 108, 32, 114, 101, 113, 117, 105, 114, 101,
                    115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115,
                    105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105,
                    110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121,
                ])
            }
        }
        self::JavaScriptSemanticValue::Object(identity_2, _) => {
            if ((identity_2 > (self::ml::Big::from_i128(0)))
                && (identity_2 <= (self::ml::Big::from_i128(18446744073709551615))))
            {
                self::ml_string::Text::from_units(vec![])
            } else {
                self::ml_string::Text::from_units(vec![
                    97, 110, 32, 111, 98, 106, 101, 99, 116, 32, 114, 101, 113, 117, 105, 114, 101,
                    115, 32, 97, 32, 112, 111, 115, 105, 116, 105, 118, 101, 32, 117, 110, 115,
                    105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108, 32, 108, 105,
                    110, 107, 32, 105, 100, 101, 110, 116, 105, 116, 121,
                ])
            }
        }
        _ => self::ml_string::Text::from_units(vec![]),
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_same_value(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    if (((self::read_java_script_semantic_value_error(left.clone()))
        != (self::ml_string::Text::from_units(vec![])))
        || ((self::read_java_script_semantic_value_error(right.clone()))
            != (self::ml_string::Text::from_units(vec![]))))
    {
        (self::ml_source_exception::SourceException {
            class_name: String::from("TypeError"),
            message_units: (self::ml_string::Text::from_units(vec![
                83, 97, 109, 101, 86, 97, 108, 117, 101, 32, 114, 101, 113, 117, 105, 114, 101,
                115, 32, 116, 119, 111, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 118, 97,
                108, 117, 101, 115, 32, 105, 110, 32, 116, 104, 101, 32, 100, 101, 99, 108, 97,
                114, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 109, 111, 100, 101,
                108,
            ]))
            .0,
            message_present: true,
        })
        .raise()
    } else {
        match left.clone() {
            self::JavaScriptSemanticValue::Undefined => match right.clone() {
                self::JavaScriptSemanticValue::Undefined => true,
                _ => false,
            },
            self::JavaScriptSemanticValue::Null => match right.clone() {
                self::JavaScriptSemanticValue::Null => true,
                _ => false,
            },
            self::JavaScriptSemanticValue::Boolean(left_value) => match right.clone() {
                self::JavaScriptSemanticValue::Boolean(value) => (left_value == value),
                _ => false,
            },
            self::JavaScriptSemanticValue::Number(left_value_2) => match right.clone() {
                self::JavaScriptSemanticValue::Number(value_2) => {
                    if (left_value_2 == value_2) {
                        ((left_value_2 != (0f64)) || ((1f64 / left_value_2) == (1f64 / value_2)))
                    } else {
                        (left_value_2.is_nan() && value_2.is_nan())
                    }
                }
                _ => false,
            },
            self::JavaScriptSemanticValue::Integer(left_value_3) => match right.clone() {
                self::JavaScriptSemanticValue::Integer(value_3) => (left_value_3 == value_3),
                _ => false,
            },
            self::JavaScriptSemanticValue::Symbol(left_identity) => match right.clone() {
                self::JavaScriptSemanticValue::Symbol(identity) => (left_identity == identity),
                _ => false,
            },
            self::JavaScriptSemanticValue::Object(left_identity_2, _) => match right.clone() {
                self::JavaScriptSemanticValue::Object(identity_2, _) => {
                    (left_identity_2 == identity_2)
                }
                _ => false,
            },
            _ => self::test_java_script_semantic_text_equal(left.clone(), right.clone()),
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_length(value: self::JavaScriptSemanticValue) -> f64 {
    match value.clone() {
        self::JavaScriptSemanticValue::SourceText(_, text) => text.length(),
        _ => match value.clone() {
            self::JavaScriptSemanticValue::Text(units) => {
                let ml_reference_array6 = units.clone();
                let ml_reference_projection6 = ml_reference_array6.borrow().len() as f64;
                ml_reference_projection6
            }
            _ => (-1f64),
        },
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_semantic_text_unit(
    value: self::JavaScriptSemanticValue,
    index: f64,
) -> f64 {
    if ((!(self::ml_math::is_safe_integer(index)) || (index < (0f64)))
        || (index >= (self::read_java_script_semantic_text_length(value.clone()))))
    {
        (-1f64)
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::SourceText(_, text) => {
                let source_ml_source2_string_unit_operand26 = text.clone();
                {
                    let source_ml_source2_string_unit_operand27 = index;
                    (std::rc::Rc::new(move |text_2: self::ml_string::Text, position: f64| -> f64 {
                        {
                            let index_2 = if position.is_nan() {
                                0f64
                            } else {
                                position.trunc()
                            };
                            if ((index_2 < (0f64)) || (index_2 >= (text_2.length()))) {
                                f64::NAN
                            } else {
                                ({
                                    let source_ml_source2_string_unit_operand33 = {
                                        let source_ml_source2_string_range_operand28 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand29 = index_2;
                                            {
                                                let source_ml_source2_string_range_operand30 = (index_2 + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position31 = source_ml_source2_string_range_operand29;
                                                    {
                                                        let source_ml_source2_string_range_position32 = source_ml_source2_string_range_operand30;
                                                        source_ml_source2_string_range_operand28.slice(source_ml_source2_string_range_position31, source_ml_source2_string_range_position32)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand34 = 0f64;
                                        source_ml_source2_string_unit_operand33.code_point_at(source_ml_source2_string_unit_operand34)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                            }
                        }
                    }))(
                        source_ml_source2_string_unit_operand26.clone(),
                        source_ml_source2_string_unit_operand27,
                    )
                }
            }
            _ => match value.clone() {
                self::JavaScriptSemanticValue::Text(units) => ({
                    let ml_reference_array7 = units.clone();
                    let ml_reference_position7 = index;
                    let ml_reference_projection7 = self::ml_array::optional_at(
                        &*ml_reference_array7.borrow(),
                        self::ml_array::number_index(ml_reference_position7),
                    );
                    ml_reference_projection7
                })
                .clone()
                .unwrap_or_else(|| (-1f64)),
                _ => (-1f64),
            },
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_text_equal(
    left: self::JavaScriptSemanticValue,
    right: self::JavaScriptSemanticValue,
) -> bool {
    {
        let length = self::read_java_script_semantic_text_length(left.clone());
        if ((length < (0f64))
            || (length != (self::read_java_script_semantic_text_length(right.clone()))))
        {
            false
        } else {
            if (match left.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => true,
                _ => false,
            } && match right.clone() {
                self::JavaScriptSemanticValue::SourceText(_, _) => true,
                _ => false,
            }) {
                (((((match left.clone() {
                    self::JavaScriptSemanticValue::SourceText(
                        _,
                        source_ml_source2_data_property_value43,
                    ) => Some((source_ml_source2_data_property_value43.clone(),)),
                    _ => None::<(self::ml_string::Text,)>,
                })
                .clone()
                .expect("a proven present value"))
                .0)
                    .clone())
                    == ((((match right.clone() {
                        self::JavaScriptSemanticValue::SourceText(
                            _,
                            source_ml_source2_data_property_value44,
                        ) => Some((source_ml_source2_data_property_value44.clone(),)),
                        _ => None::<(self::ml_string::Text,)>,
                    })
                    .clone()
                    .expect("a proven present value"))
                    .0)
                        .clone()))
            } else {
                {
                    let at = 0f64;
                    {
                        let source_ml_source2_s45 = self::source_ml_source1_temporary_test_java_script_semantic_text_equal_loop4(left.clone(), right.clone(), length, at);
                        match source_ml_source2_s45.clone() {
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Done => {
                                true
                            }
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Return(source_ml_source1_temporary_result4) => {
                                source_ml_source1_temporary_result4
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
pub fn read_java_script_semantic_text_string(
    value: self::JavaScriptSemanticValue,
) -> self::ml_string::Text {
    match value.clone() {
        self::JavaScriptSemanticValue::SourceText(_, text) => text.clone(),
        _ => match value.clone() {
            self::JavaScriptSemanticValue::Text(units) => {
                let text_2 = self::ml_string::Text::from_units(vec![]);
                {
                    let source_ml_source1_temporary_index5 = self::ml::Big::from_i128(0);
                    {
                        let text_3 = self::source_ml_source1_temporary_read_java_script_semantic_text_string_loop6(text_2.clone(), units.clone(), source_ml_source1_temporary_index5.clone());
                        text_3.clone()
                    }
                }
            }
            _ => (self::ml_source_exception::SourceException {
                class_name: String::from("TypeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 116, 114, 97,
                    110, 115, 112, 111, 114, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32,
                    105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 99, 111, 109, 112, 108, 101,
                    116, 101, 32, 116, 101, 120, 116, 32, 118, 97, 108, 117, 101,
                ]))
                .0,
                message_present: true,
            })
            .raise(),
        },
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn construct_java_script_semantic_text_units(
    value: self::JavaScriptSemanticValue,
    limit: f64,
) -> std::rc::Rc<std::cell::RefCell<Vec<f64>>> {
    {
        let length = self::read_java_script_semantic_text_length(value.clone());
        if ((length < (0f64)) || (length > limit)) {
            (self::ml_source_exception::SourceException {
                class_name: String::from("RangeError"),
                message_units: (self::ml_string::Text::from_units(vec![
                    115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 99, 111, 110,
                    118, 101, 114, 115, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115,
                    32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 111, 112, 101, 114, 97,
                    116, 105, 111, 110, 32, 117, 110, 105, 116, 32, 98, 117, 100, 103, 101, 116,
                ]))
                .0,
                message_present: true,
            })
            .raise()
        } else {
            match value.clone() {
                self::JavaScriptSemanticValue::Text(units) => units.clone(),
                _ => {
                    let units_2 = std::rc::Rc::new(std::cell::RefCell::new(Vec::<f64>::new()));
                    {
                        let at = 0f64;
                        {
                            let source_ml_source2_s47 = self::source_ml_source1_temporary_construct_java_script_semantic_text_units_loop7(value.clone(), length, units_2.clone(), at);
                            match source_ml_source2_s47.clone() {
                                    self::MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Done => {
                                        units_2.clone()
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
pub fn construct_java_script_source_text(
    text: &self::ml_string::Text,
) -> self::JavaScriptSemanticValue {
    {
        let error = self::read_java_script_source_text_profile_error(
            &(self::ml_string::Text::from_units(vec![
                105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32,
                115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49,
            ])),
            text.length(),
        );
        if (error != (self::ml_string::Text::from_units(vec![]))) {
            (self::ml_source_exception::SourceException {
                class_name: String::from("RangeError"),
                message_units: (error.clone()).0,
                message_present: true,
            })
            .raise()
        } else {
            self::JavaScriptSemanticValue::SourceText(
                self::ml_string::Text::from_units(vec![
                    105, 110, 115, 112, 101, 99, 116, 97, 98, 108, 101, 32, 85, 84, 70, 49, 54, 32,
                    115, 111, 117, 114, 99, 101, 32, 116, 101, 120, 116, 32, 49,
                ]),
                text.clone(),
            )
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_semantic_accessor(value: self::JavaScriptSemanticValue) -> bool {
    if ((self::read_java_script_semantic_value_error(value.clone()))
        != (self::ml_string::Text::from_units(vec![])))
    {
        false
    } else {
        match value.clone() {
            self::JavaScriptSemanticValue::Undefined => true,
            _ => match value.clone() {
                self::JavaScriptSemanticValue::Object(_, callable) => callable,
                _ => false,
            },
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn find_java_script_property_descriptor_entry(
    properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    key: self::JavaScriptSemanticValue,
) -> f64 {
    {
        let at = 0f64;
        {
            let source_ml_source2_s49 =
                self::source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop9(
                    properties.clone(),
                    key.clone(),
                    at,
                );
            match source_ml_source2_s49.clone() {
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done => {
                    (-1f64)
                }
                self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(source_ml_source1_temporary_result9) => {
                    source_ml_source1_temporary_result9
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_property_descriptor_kind(
    descriptor: std::rc::Rc<
        std::cell::RefCell<(
            (
                bool,
                bool,
                bool,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                self::JavaScriptSemanticValue,
                bool,
                bool,
                bool,
            ),
            Vec<f64>,
        )>,
    >,
) -> self::ml_string::Text {
    {
        let data = ((({
            let ml_reference_value8 = descriptor.borrow().clone();
            ml_reference_value8
        })
        .0)
            .9
            || (({
                let ml_reference_value9 = descriptor.borrow().clone();
                ml_reference_value9
            })
            .0)
                .11);
        {
            let accessor = ((({
                let ml_reference_value10 = descriptor.borrow().clone();
                ml_reference_value10
            })
            .0)
                .5
                || (({
                    let ml_reference_value11 = descriptor.borrow().clone();
                    ml_reference_value11
                })
                .0)
                    .7);
            if (data && accessor) {
                self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100])
            } else {
                if data {
                    self::ml_string::Text::from_units(vec![100, 97, 116, 97])
                } else {
                    if accessor {
                        self::ml_string::Text::from_units(vec![97, 99, 99, 101, 115, 115, 111, 114])
                    } else {
                        self::ml_string::Text::from_units(vec![103, 101, 110, 101, 114, 105, 99])
                    }
                }
            }
        }
    }
}
