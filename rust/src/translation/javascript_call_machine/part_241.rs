// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptObjectEnumerationState;
use super::JavaScriptObjectEnumerationTransition;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPropertyDefinitionsTransition;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Result;
use super::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Result;
use super::construct_java_script_reflection_text;
use super::ml;
use super::ml_array;
use super::ml_number;
use super::ml_string;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_error;
use super::read_java_script_semantic_text_string;
use super::source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1038;
use super::test_java_script_enumeration_array;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1033(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1028: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>,
    >,
    mut source_ml_source1_temporary_index1028: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Result {
    loop {
        return if (source_ml_source1_temporary_index1028
            < ({
                let ml_reference_array2892 = source_ml_source1_temporary_values1028.clone();
                let ml_reference_projection2892 =
                    self::ml::Big::from_u128(ml_reference_array2892.borrow().len() as u128);
                ml_reference_projection2892
            })) {
            {
                let property = {
                    let ml_reference_array2893 = source_ml_source1_temporary_values1028.clone();
                    let ml_reference_position2893 = source_ml_source1_temporary_index1028.clone();
                    let ml_reference_projection2893 = self::ml_array::at(
                        &*ml_reference_array2893.borrow(),
                        ml_reference_position2893.to_index(),
                    );
                    ml_reference_projection2893
                };
                match property.clone() {
                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                        shared_constructor_field1,
                        shared_constructor_field2,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        {
                            let error = {
                                let source_ml_source2_logical41316 =
                                    self::read_java_script_ordinary_key_error(
                                        heap.clone(),
                                        shared_constructor_field1.clone(),
                                    );
                                if !source_ml_source2_logical41316.0.is_empty() {
                                    source_ml_source2_logical41316.clone()
                                } else {
                                    self::read_java_script_property_descriptor_error(
                                        shared_constructor_field2.clone(),
                                    )
                                }
                            };
                            if (error != (self::ml_string::Text::from_units(vec![]))) {
                                self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Return(error.clone())
                            } else {
                                {
                                    let source_ml_source1_temporary_values1029 =
                                        std::rc::Rc::new(std::cell::RefCell::new(vec![
                                            (({
                                                let ml_reference_value2894 =
                                                    shared_constructor_field2.borrow().clone();
                                                ml_reference_value2894
                                            })
                                            .0)
                                                .8,
                                            (({
                                                let ml_reference_value2895 =
                                                    shared_constructor_field2.borrow().clone();
                                                ml_reference_value2895
                                            })
                                            .0)
                                                .4,
                                            (({
                                                let ml_reference_value2896 =
                                                    shared_constructor_field2.borrow().clone();
                                                ml_reference_value2896
                                            })
                                            .0)
                                                .6,
                                        ]));
                                    {
                                        let source_ml_source1_temporary_index1029 =
                                            self::ml::Big::from_i128(0);
                                        {
                                            let source_ml_source2_s41320 = self::source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1038(heap.clone(), source_ml_source1_temporary_values1029.clone(), source_ml_source1_temporary_index1029.clone());
                                            match source_ml_source2_s41320.clone() {
                                                self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Done => {
                                                    {
                                                        let source_ml_source1_temporary_index1028_2 = source_ml_source1_temporary_index1028.add(&self::ml::Big::from_i128(1));
                                                        {
                                                            source_ml_source1_temporary_index1028 = source_ml_source1_temporary_index1028_2.clone();
                                                            continue;
                                                        }
                                                    }
                                                }
                                                self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1038Return(source_ml_source1_temporary_result1038) => {
                                                    self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Return(source_ml_source1_temporary_result1038.clone())
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
            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1033Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Result {
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Done,
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1032(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut seen: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_source1_temporary_values1027: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1027: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Result {
    loop {
        return if (source_ml_source1_temporary_index1027
            < ({
                let ml_reference_array2897 = source_ml_source1_temporary_values1027.clone();
                let ml_reference_projection2897 =
                    self::ml::Big::from_u128(ml_reference_array2897.borrow().len() as u128);
                ml_reference_projection2897
            })) {
            {
                let key = {
                    let ml_reference_array2898 = source_ml_source1_temporary_values1027.clone();
                    let ml_reference_position2898 = source_ml_source1_temporary_index1027.clone();
                    let ml_reference_projection2898 = self::ml_array::at(
                        &*ml_reference_array2898.borrow(),
                        ml_reference_position2898.to_index(),
                    );
                    ml_reference_projection2898
                };
                {
                    let error =
                        self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Return(error.clone())
                    } else {
                        if {
                            let ml_values2899 = {
                                let ml_reference_value2900 = seen.borrow().clone();
                                ml_reference_value2900
                            };
                            let ml_callback2899 = {
                                let key = key.clone();
                                std::rc::Rc::new(
                                    move |prior: self::JavaScriptSemanticValue| -> bool {
                                        self::test_java_script_semantic_same_value(
                                            prior.clone(),
                                            key.clone(),
                                        )
                                    },
                                )
                            };
                            ml_values2899
                                .iter()
                                .cloned()
                                .enumerate()
                                .any(|(ml_index2899, ml_value2899)| ml_callback2899(ml_value2899))
                        } {
                            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Return(self::ml_string::Text::from_units(vec![98, 117, 108, 107, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 100, 105, 115, 116, 105, 110, 99, 116, 32, 111, 119, 110, 45, 107, 101, 121, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115]))
                        } else {
                            {
                                let source_ml_source1_temporary_read_java_script_property_definitions_state_error_ignored1039 = {
                                    let ml_method_owner2901 = seen.clone();
                                    let ml_method_argument2901_0 = key.clone();
                                    let ml_inserted = vec![ml_method_argument2901_0];
                                    let mut ml_method_values2901 = ml_method_owner2901.borrow_mut();
                                    let ml_length = ml_method_values2901
                                        .len()
                                        .checked_add(ml_inserted.len())
                                        .filter(|length| *length <= 4294967295usize)
                                        .expect("Invalid array length");
                                    ml_method_values2901.extend(ml_inserted);
                                    ml_length as f64
                                };
                                {
                                    let source_ml_source1_temporary_index1027_2 =
                                        source_ml_source1_temporary_index1027
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index1027 =
                                            source_ml_source1_temporary_index1027_2.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1032Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Result {
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Done,
    MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_property_definitions_state_error_loop1031(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1026: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1026: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Result {
    loop {
        return if (source_ml_source1_temporary_index1026
            < ({
                let ml_reference_array2902 = source_ml_source1_temporary_values1026.clone();
                let ml_reference_projection2902 =
                    self::ml::Big::from_u128(ml_reference_array2902.borrow().len() as u128);
                ml_reference_projection2902
            })) {
            {
                let value = {
                    let ml_reference_array2903 = source_ml_source1_temporary_values1026.clone();
                    let ml_reference_position2903 = source_ml_source1_temporary_index1026.clone();
                    let ml_reference_projection2903 = self::ml_array::at(
                        &*ml_reference_array2903.borrow(),
                        ml_reference_position2903.to_index(),
                    );
                    ml_reference_projection2903
                };
                {
                    let error =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Return(error.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1026_2 =
                                source_ml_source1_temporary_index1026
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1026 =
                                    source_ml_source1_temporary_index1026_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Result::MlSource1TemporaryReadJavaScriptPropertyDefinitionsStateErrorLoop1031Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040 {
    MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040Next(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    MlSource1TemporaryStepJavaScriptPropertyDefinitionsJoin1040Return(
        Box<self::JavaScriptPropertyDefinitionsTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStepJavaScriptPropertyDefinitionsLoop1041Result {
    MlSource1TemporaryStepJavaScriptPropertyDefinitionsLoop1041Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_property_definitions_loop1041(
    mut keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut length: f64,
    mut at: f64,
) -> self::MlSource1TemporaryStepJavaScriptPropertyDefinitionsLoop1041Result {
    loop {
        return if (at < length) {
            {
                let source_ml_source1_temporary_step_java_script_property_definitions_ignored1043 = {
                    let ml_method_owner2904 = keys.clone();
                    let ml_method_argument2904_0 = self::construct_java_script_reflection_text(
                        &(self::ml_string::Text::from_text(&self::ml_number::js_number(at))),
                    );
                    let ml_inserted = vec![ml_method_argument2904_0];
                    let mut ml_method_values2904 = ml_method_owner2904.borrow_mut();
                    let ml_length = ml_method_values2904
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2904.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = (at + 1f64);
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryStepJavaScriptPropertyDefinitionsLoop1041Result::MlSource1TemporaryStepJavaScriptPropertyDefinitionsLoop1041Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052 {
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052Next,
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1052Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051 {
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051Next,
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorJoin1051Return(
        self::ml_string::Text,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Result {
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Done,
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1053(
    mut state: self::JavaScriptObjectEnumerationState,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Result {
    loop {
        return match state.clone() {
            self::JavaScriptObjectEnumerationState::ObjectEnumeration(
                _,
                _,
                _,
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
            ) => {
                if (at
                    < ({
                        let ml_reference_array2905 = shared_constructor_field1.clone();
                        let ml_reference_projection2905 =
                            ml_reference_array2905.borrow().len() as f64;
                        ml_reference_projection2905
                    }))
                {
                    if ({
                        let source_ml_source2_comparison_operand41332 = {
                            let source_ml_source2_optional_chain_receiver41331 = ({
                                let source_ml_source2_array_read_value41329 =
                                    shared_constructor_field1.clone();
                                {
                                    let ml_reference_array2906 =
                                        source_ml_source2_array_read_value41329.clone();
                                    let ml_reference_position2906 = at;
                                    let ml_reference_projection2906 = self::ml_array::optional_at(
                                        &*ml_reference_array2906.borrow(),
                                        self::ml_array::number_index(ml_reference_position2906),
                                    );
                                    ml_reference_projection2906
                                }
                            },);
                            if match (source_ml_source2_optional_chain_receiver41331.clone()).0 {
                                Some(ml_present) => {
                                    let _ = ml_present.clone();
                                    true
                                }
                                None => false,
                            } {
                                Some({
                                    let source_ml_source1_temporary_optional_chain1047 =
                                        ((source_ml_source2_optional_chain_receiver41331.clone())
                                            .0)
                                            .clone()
                                            .expect("a proven present value");
                                    match source_ml_source1_temporary_optional_chain1047.clone() {
                                        self::JavaScriptSemanticValue::Undefined => {
                                            self::ml_string::Text::from_units(vec![
                                                117, 110, 100, 101, 102, 105, 110, 101, 100,
                                            ])
                                        }
                                        self::JavaScriptSemanticValue::Null => {
                                            self::ml_string::Text::from_units(vec![
                                                110, 117, 108, 108,
                                            ])
                                        }
                                        self::JavaScriptSemanticValue::Boolean(_) => {
                                            self::ml_string::Text::from_units(vec![
                                                98, 111, 111, 108, 101, 97, 110,
                                            ])
                                        }
                                        self::JavaScriptSemanticValue::Number(_) => {
                                            self::ml_string::Text::from_units(vec![
                                                110, 117, 109, 98, 101, 114,
                                            ])
                                        }
                                        self::JavaScriptSemanticValue::Integer(_) => {
                                            self::ml_string::Text::from_units(vec![
                                                105, 110, 116, 101, 103, 101, 114,
                                            ])
                                        }
                                        self::JavaScriptSemanticValue::Text(_) => {
                                            self::ml_string::Text::from_units(vec![
                                                116, 101, 120, 116,
                                            ])
                                        }
                                        self::JavaScriptSemanticValue::SourceText(_, _) => {
                                            self::ml_string::Text::from_units(vec![
                                                115, 111, 117, 114, 99, 101, 84, 101, 120, 116,
                                            ])
                                        }
                                        self::JavaScriptSemanticValue::Symbol(_) => {
                                            self::ml_string::Text::from_units(vec![
                                                115, 121, 109, 98, 111, 108,
                                            ])
                                        }
                                        self::JavaScriptSemanticValue::Object(_, _) => {
                                            self::ml_string::Text::from_units(vec![
                                                111, 98, 106, 101, 99, 116,
                                            ])
                                        }
                                    }
                                })
                            } else {
                                None::<self::ml_string::Text>
                            }
                        };
                        !(match (
                            source_ml_source2_comparison_operand41332.clone(),
                            self::ml_string::Text::from_units(vec![116, 101, 120, 116]),
                        ) {
                            (Some(ml_left), ml_right) => ml_left == ml_right,
                            _ => false,
                        })
                    } || ((self::read_java_script_semantic_text_string(
                        ({
                            let source_ml_source2_array_read_value41339 =
                                shared_constructor_field1.clone();
                            {
                                let ml_reference_array2907 =
                                    source_ml_source2_array_read_value41339.clone();
                                let ml_reference_position2907 = at;
                                let ml_reference_projection2907 = self::ml_array::optional_at(
                                    &*ml_reference_array2907.borrow(),
                                    self::ml_array::number_index(ml_reference_position2907),
                                );
                                ml_reference_projection2907
                            }
                        })
                        .clone()
                        .unwrap_or_else(|| self::JavaScriptSemanticValue::Undefined),
                    )) != (self::ml_string::Text::from_text(
                        &self::ml_number::js_number(at),
                    )))) {
                        self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Return(self::ml_string::Text::from_units(vec![83, 116, 114, 105, 110, 103, 32, 111, 119, 110, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32, 101, 120, 97, 99, 116, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 105, 110, 100, 101, 120, 32, 107, 101, 121, 115]))
                    } else {
                        {
                            let at_2 = (at + 1f64);
                            {
                                at = at_2;
                                continue;
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1053Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Result {
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Done,
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1050(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut state: self::JavaScriptObjectEnumerationState,
    mut source_ml_source1_temporary_values1046: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1046: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Result {
    loop {
        return match state.clone() {
            self::JavaScriptObjectEnumerationState::ObjectEnumeration(
                shared_constructor_field1,
                _,
                shared_constructor_field2,
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
                if (source_ml_source1_temporary_index1046
                    < ({
                        let ml_reference_array2908 = source_ml_source1_temporary_values1046.clone();
                        let ml_reference_projection2908 =
                            self::ml::Big::from_u128(ml_reference_array2908.borrow().len() as u128);
                        ml_reference_projection2908
                    }))
                {
                    {
                        let value = {
                            let ml_reference_array2909 =
                                source_ml_source1_temporary_values1046.clone();
                            let ml_reference_position2909 =
                                source_ml_source1_temporary_index1046.clone();
                            let ml_reference_projection2909 = self::ml_array::at(
                                &*ml_reference_array2909.borrow(),
                                ml_reference_position2909.to_index(),
                            );
                            ml_reference_projection2909
                        };
                        if (((shared_constructor_field1
                            == (self::ml_string::Text::from_units(vec![
                                79, 98, 106, 101, 99, 116, 46, 107, 101, 121, 115,
                            ])))
                            && match value.clone() {
                                self::JavaScriptSemanticValue::Text(_) => false,
                                _ => true,
                            })
                            && match value.clone() {
                                self::JavaScriptSemanticValue::SourceText(_, _) => false,
                                _ => true,
                            })
                        {
                            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 107, 101, 121, 115, 32, 114, 101, 115, 117, 108, 116, 32, 76, 105, 115, 116, 115, 32, 99, 111, 110, 116, 97, 105, 110, 32, 111, 110, 108, 121, 32, 97, 99, 116, 117, 97, 108, 32, 116, 101, 120, 116, 32, 107, 101, 121, 115]))
                        } else {
                            if ((shared_constructor_field1
                                == (self::ml_string::Text::from_units(vec![
                                    79, 98, 106, 101, 99, 116, 46, 101, 110, 116, 114, 105, 101,
                                    115,
                                ])))
                                && !(self::test_java_script_enumeration_array(
                                    heap.clone(),
                                    value.clone(),
                                    shared_constructor_field2.clone(),
                                    2f64,
                                )))
                            {
                                self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 115, 117, 108, 116, 32, 76, 105, 115, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 97, 99, 116, 117, 97, 108, 32, 108, 101, 110, 103, 116, 104, 45, 116, 119, 111, 32, 101, 110, 116, 114, 121, 32, 65, 114, 114, 97, 121, 115]))
                            } else {
                                {
                                    let source_ml_source1_temporary_index1046_2 =
                                        source_ml_source1_temporary_index1046
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index1046 =
                                            source_ml_source1_temporary_index1046_2.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                } else {
                    self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1050Done
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Result {
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Done,
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1049(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut seen: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_source1_temporary_values1045: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1045: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Result {
    loop {
        return if (source_ml_source1_temporary_index1045
            < ({
                let ml_reference_array2910 = source_ml_source1_temporary_values1045.clone();
                let ml_reference_projection2910 =
                    self::ml::Big::from_u128(ml_reference_array2910.borrow().len() as u128);
                ml_reference_projection2910
            })) {
            {
                let key = {
                    let ml_reference_array2911 = source_ml_source1_temporary_values1045.clone();
                    let ml_reference_position2911 = source_ml_source1_temporary_index1045.clone();
                    let ml_reference_projection2911 = self::ml_array::at(
                        &*ml_reference_array2911.borrow(),
                        ml_reference_position2911.to_index(),
                    );
                    ml_reference_projection2911
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_key_error(heap.clone(), key.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Return(invalid.clone())
                    } else {
                        if {
                            let ml_values2912 = {
                                let ml_reference_value2913 = seen.borrow().clone();
                                ml_reference_value2913
                            };
                            let ml_callback2912 = {
                                let key = key.clone();
                                std::rc::Rc::new(
                                    move |prior: self::JavaScriptSemanticValue| -> bool {
                                        self::test_java_script_semantic_same_value(
                                            prior.clone(),
                                            key.clone(),
                                        )
                                    },
                                )
                            };
                            ml_values2912
                                .iter()
                                .cloned()
                                .enumerate()
                                .any(|(ml_index2912, ml_value2912)| ml_callback2912(ml_value2912))
                        } {
                            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Return(self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 32, 101, 110, 117, 109, 101, 114, 97, 116, 105, 111, 110, 32, 107, 101, 121, 32, 115, 110, 97, 112, 115, 104, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 97, 99, 116, 117, 97, 108, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115]))
                        } else {
                            {
                                let source_ml_source1_temporary_read_java_script_object_enumeration_state_error_ignored1054 = {
                                    let ml_method_owner2914 = seen.clone();
                                    let ml_method_argument2914_0 = key.clone();
                                    let ml_inserted = vec![ml_method_argument2914_0];
                                    let mut ml_method_values2914 = ml_method_owner2914.borrow_mut();
                                    let ml_length = ml_method_values2914
                                        .len()
                                        .checked_add(ml_inserted.len())
                                        .filter(|length| *length <= 4294967295usize)
                                        .expect("Invalid array length");
                                    ml_method_values2914.extend(ml_inserted);
                                    ml_length as f64
                                };
                                {
                                    let source_ml_source1_temporary_index1045_2 =
                                        source_ml_source1_temporary_index1045
                                            .add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_source1_temporary_index1045 =
                                            source_ml_source1_temporary_index1045_2.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1049Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Result {
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Done,
    MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_object_enumeration_state_error_loop1048(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_source1_temporary_values1044: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_source1_temporary_index1044: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Result {
    loop {
        return if (source_ml_source1_temporary_index1044
            < ({
                let ml_reference_array2915 = source_ml_source1_temporary_values1044.clone();
                let ml_reference_projection2915 =
                    self::ml::Big::from_u128(ml_reference_array2915.borrow().len() as u128);
                ml_reference_projection2915
            })) {
            {
                let value = {
                    let ml_reference_array2916 = source_ml_source1_temporary_values1044.clone();
                    let ml_reference_position2916 = source_ml_source1_temporary_index1044.clone();
                    let ml_reference_projection2916 = self::ml_array::at(
                        &*ml_reference_array2916.borrow(),
                        ml_reference_position2916.to_index(),
                    );
                    ml_reference_projection2916
                };
                {
                    let invalid =
                        self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Return(invalid.clone())
                    } else {
                        {
                            let source_ml_source1_temporary_index1044_2 =
                                source_ml_source1_temporary_index1044
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index1044 =
                                    source_ml_source1_temporary_index1044_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Result::MlSource1TemporaryReadJavaScriptObjectEnumerationStateErrorLoop1048Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055 {
    MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055Next(
        std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    ),
    MlSource1TemporaryStepJavaScriptObjectEnumerationJoin1055Return(
        Box<self::JavaScriptObjectEnumerationTransition>,
    ),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1056Result {
    MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1056Done,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1057Result {
    MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1057Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_step_java_script_object_enumeration_loop1057(
    text: &self::ml_string::Text,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut part: f64,
) -> self::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1057Result {
    loop {
        return if (part < (text.length())) {
            {
                let source_ml_source1_temporary_step_java_script_object_enumeration_ignored1059 = {
                    let ml_method_owner2917 = units.clone();
                    let ml_method_argument2917_0 = {
                        let source_ml_source2_string_unit_operand41356 = text.clone();
                        {
                            let source_ml_source2_string_unit_operand41357 = part;
                            (std::rc::Rc::new(
                                move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                    {
                                        let index = if position.is_nan() {
                                            0f64
                                        } else {
                                            position.trunc()
                                        };
                                        if ((index < (0f64)) || (index >= (text_2.length()))) {
                                            f64::NAN
                                        } else {
                                            ({
                                    let source_ml_source2_string_unit_operand41363 = {
                                        let source_ml_source2_string_range_operand41358 = text_2.clone();
                                        {
                                            let source_ml_source2_string_range_operand41359 = index;
                                            {
                                                let source_ml_source2_string_range_operand41360 = (index + 1f64);
                                                {
                                                    let source_ml_source2_string_range_position41361 = source_ml_source2_string_range_operand41359;
                                                    {
                                                        let source_ml_source2_string_range_position41362 = source_ml_source2_string_range_operand41360;
                                                        source_ml_source2_string_range_operand41358.slice(source_ml_source2_string_range_position41361, source_ml_source2_string_range_position41362)
                                                    }
                                                }
                                            }
                                        }
                                    };
                                    {
                                        let source_ml_source2_string_unit_operand41364 = 0f64;
                                        source_ml_source2_string_unit_operand41363.code_point_at(source_ml_source2_string_unit_operand41364)
                                    }
                                }).clone().unwrap_or_else(|| f64::NAN)
                                        }
                                    }
                                },
                            ))(
                                source_ml_source2_string_unit_operand41356.clone(),
                                source_ml_source2_string_unit_operand41357,
                            )
                        }
                    };
                    let ml_inserted = vec![ml_method_argument2917_0];
                    let mut ml_method_values2917 = ml_method_owner2917.borrow_mut();
                    let ml_length = ml_method_values2917
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4294967295usize)
                        .expect("Invalid array length");
                    ml_method_values2917.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let part_2 = (part + 1f64);
                    {
                        part = part_2;
                        continue;
                    }
                }
            }
        } else {
            self::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1057Result::MlSource1TemporaryStepJavaScriptObjectEnumerationLoop1057Done
        };
    }
}
