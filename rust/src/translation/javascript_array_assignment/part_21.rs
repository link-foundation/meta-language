// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptArrayAssignmentState;
use super::JavaScriptArrayAssignmentTransition;
use super::JavaScriptArrayCompletion;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptPrimitiveConversionCompletion;
use super::JavaScriptSemanticValue;
use super::JavaScriptSymbolRegistrationFact;
use super::continue_java_script_array_assignment_definition;
use super::continue_java_script_array_assignment_number;
use super::finish_java_script_scalar_primitive;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_string;
use super::read_java_script_array_assignment_state_error;
use super::read_java_script_semantic_text_unit;
use super::resume_java_script_scalar_protocol;
use super::step_java_script_array_property_definition;
use super::step_java_script_scalar_protocol;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_assignment(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayAssignmentState,
) -> self::JavaScriptArrayAssignmentTransition {
    {
        let error =
            self::read_java_script_array_assignment_state_error(heap.clone(), state.clone());
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                match state.clone() {
                    self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(
                        definition,
                        exotic_key,
                    ) => {
                        let definition = *definition;
                        let exotic_key = *exotic_key;
                        {
                            let transition = self::step_java_script_array_property_definition(
                                heap.clone(),
                                definition.clone(),
                            );
                            match transition.clone() {
                                self::JavaScriptArrayCompletion::ArrayNormal(_, _) => {
                                    self::continue_java_script_array_assignment_definition(
                                        exotic_key.clone(),
                                        transition.clone(),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayPropertyComplete(_, _, _) => {
                                    self::continue_java_script_array_assignment_definition(
                                        exotic_key.clone(),
                                        transition.clone(),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayNumberRequest(_, _, _) => {
                                    self::continue_java_script_array_assignment_definition(
                                        exotic_key.clone(),
                                        transition.clone(),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayDefinitionContinue(_, _) => {
                                    self::continue_java_script_array_assignment_definition(
                                        exotic_key.clone(),
                                        transition.clone(),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayInvalid(_, _) => {
                                    self::continue_java_script_array_assignment_definition(
                                        exotic_key.clone(),
                                        transition.clone(),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayError(_, _, _) => {
                                    self::continue_java_script_array_assignment_definition(
                                        exotic_key.clone(),
                                        transition.clone(),
                                    )
                                }
                                self::JavaScriptArrayCompletion::ArrayResourceLimit(
                                    heap_2,
                                    reason,
                                    _,
                                ) => {
                                    let heap_2 = *heap_2;
                                    self::JavaScriptArrayAssignmentTransition::ArrayAssignmentResourceLimit(Box::new(heap_2.clone()), Box::new(state.clone()), reason.clone())
                                }
                            }
                        }
                    }
                    self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                        definition_2,
                        exotic_key_2,
                        conversion,
                    ) => {
                        let definition_2 = *definition_2;
                        let exotic_key_2 = *exotic_key_2;
                        let conversion = *conversion;
                        self::continue_java_script_array_assignment_number(
                            heap.clone(),
                            definition_2.clone(),
                            exotic_key_2.clone(),
                            self::step_java_script_scalar_protocol(
                                heap.clone(),
                                conversion.clone(),
                                &(self::ml_string::Text::from_units(vec![
                                    99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116,
                                    105, 118, 101, 32, 110, 117, 109, 98, 101, 114,
                                ])),
                            ),
                        )
                    }
                    self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(
                        definition_3,
                        exotic_key_3,
                        value,
                    ) => {
                        let definition_3 = *definition_3;
                        let exotic_key_3 = *exotic_key_3;
                        let value = *value;
                        self::continue_java_script_array_assignment_number(
                            heap.clone(),
                            definition_3.clone(),
                            exotic_key_3.clone(),
                            self::finish_java_script_scalar_primitive(
                                &(self::ml_string::Text::from_units(vec![
                                    99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116,
                                    105, 118, 101, 32, 110, 117, 109, 98, 101, 114,
                                ])),
                                value.clone(),
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
pub fn resume_java_script_array_assignment(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayAssignmentState,
    completion: self::JavaScriptPrimitiveConversionCompletion,
) -> self::JavaScriptArrayAssignmentTransition {
    {
        let error =
            self::read_java_script_array_assignment_state_error(heap.clone(), state.clone());
        {
            let ml_library_inequality_1 = error != (self::ml_string::Text::from_units(vec![]));
            if ml_library_inequality_1 {
                self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                    Box::new(heap.clone()),
                    error.clone(),
                )
            } else {
                match state.clone() {
                    self::JavaScriptArrayAssignmentState::ArrayAssignmentDefinition(_, _) => {
                        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32,
                                65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32,
                                99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 97, 110, 32, 114, 101,
                                115, 117, 109, 101, 32, 119, 105, 116, 104, 32, 97, 32, 99, 104,
                                105, 108, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110,
                            ]),
                        )
                    }
                    self::JavaScriptArrayAssignmentState::ArrayAssignmentNumber(
                        definition,
                        exotic_key,
                        conversion,
                    ) => {
                        let definition = *definition;
                        let exotic_key = *exotic_key;
                        let conversion = *conversion;
                        self::continue_java_script_array_assignment_number(
                            heap.clone(),
                            definition.clone(),
                            exotic_key.clone(),
                            self::resume_java_script_scalar_protocol(
                                heap.clone(),
                                conversion.clone(),
                                completion.clone(),
                                &(self::ml_string::Text::from_units(vec![
                                    99, 111, 101, 114, 99, 101, 32, 112, 114, 105, 109, 105, 116,
                                    105, 118, 101, 32, 110, 117, 109, 98, 101, 114,
                                ])),
                            ),
                        )
                    }
                    self::JavaScriptArrayAssignmentState::ArrayAssignmentPrimitive(_, _, _) => {
                        self::JavaScriptArrayAssignmentTransition::ArrayAssignmentInvalid(
                            Box::new(heap.clone()),
                            self::ml_string::Text::from_units(vec![
                                111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32,
                                65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99, 32,
                                99, 97, 108, 108, 98, 97, 99, 107, 32, 99, 97, 110, 32, 114, 101,
                                115, 117, 109, 101, 32, 119, 105, 116, 104, 32, 97, 32, 99, 104,
                                105, 108, 100, 32, 99, 111, 109, 112, 108, 101, 116, 105, 111, 110,
                            ]),
                        )
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_weak_key_eligibility_loop2(
    mut value: self::JavaScriptSemanticValue,
    mut facts: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSymbolRegistrationFact>>>,
    mut low: f64,
    mut high: f64,
) -> f64 {
    loop {
        return if low < high {
            {
                let middle = ((low + high) / 2f64).floor();
                {
                    let row = {
                        let ml_reference_array229 = facts.clone();
                        let ml_reference_position229 = middle;
                        self::ml_array::optional_at(
                            &*ml_reference_array229.borrow(),
                            self::ml_array::number_index(ml_reference_position229),
                        )
                    };
                    if !(match (row.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && {
                        let source_ml_source2_comparison_operand18331 =
                            match row.clone().expect("a proven present value") {
                                self::JavaScriptSymbolRegistrationFact::SymbolRegistrationFact(
                                    source_ml_source2_data_property_value18328,
                                    _,
                                    _,
                                ) => source_ml_source2_data_property_value18328.clone(),
                            };
                        {
                            let source_ml_source2_comparison_operand18332 = match value.clone() {
                                self::JavaScriptSemanticValue::Undefined => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Null => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Boolean(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Number(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Integer(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::Text(_) => None::<self::ml::Big>,
                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                    None::<self::ml::Big>
                                }
                                self::JavaScriptSemanticValue::Symbol(
                                    source_ml_source2_data_property_value18329,
                                ) => Some(source_ml_source2_data_property_value18329.clone()),
                                self::JavaScriptSemanticValue::Object(
                                    source_ml_source2_data_property_value18330,
                                    _,
                                ) => Some(source_ml_source2_data_property_value18330.clone()),
                            };
                            if match (
                                source_ml_source2_comparison_operand18332.clone(),
                                None::<self::ml::Big>,
                            ) {
                                (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                (None, None) => true,
                                _ => false,
                            } {
                                {
                                    let source_ml_source2_comparison_order18333 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source2_comparison_operand18331.clone());
                                    (source_ml_source2_comparison_order18333 == 1f64)
                                }
                            } else {
                                source_ml_source2_comparison_operand18331
                                    < (source_ml_source2_comparison_operand18332
                                        .clone()
                                        .expect("a proven present value"))
                            }
                        }
                    } {
                        {
                            let low_2 = middle + 1f64;
                            {
                                low = low_2;
                                continue;
                            }
                        }
                    } else {
                        {
                            high = middle;
                            continue;
                        }
                    }
                }
            }
        } else {
            low
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Result {
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Done,
    MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_semantic_value_error_loop3(
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Result {
    loop {
        return if at
            < ({
                let ml_reference_array230 = units.clone();
                ml_reference_array230.borrow().len() as f64
            }) {
            {
                let unit = ({
                    let ml_reference_array231 = units.clone();
                    let ml_reference_position231 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array231.borrow(),
                        self::ml_array::number_index(ml_reference_position231),
                    )
                })
                .clone()
                .unwrap_or_else(|| -1f64);
                if (!(self::ml_math::is_integer(unit)) || (unit < 0f64)) || (unit > 65_535f64) {
                    self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Return(self::ml_string::Text::from_units(vec![115, 101, 109, 97, 110, 116, 105, 99, 32, 116, 101, 120, 116, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 99, 111, 109, 112, 108, 101, 116, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 100, 101, 32, 117, 110, 105, 116, 115]))
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Result::MlSource1TemporaryReadJavaScriptSemanticValueErrorLoop3Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Result {
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Done,
    MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Return(bool),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_test_java_script_semantic_text_equal_loop4(
    mut left: self::JavaScriptSemanticValue,
    mut right: self::JavaScriptSemanticValue,
    mut length: f64,
    mut at: f64,
) -> self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Result {
    loop {
        return if at < length {
            {
                let a = match left.clone() {
                    self::JavaScriptSemanticValue::Text(units) => ({
                        let ml_reference_array232 = units.clone();
                        let ml_reference_position232 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array232.borrow(),
                            self::ml_array::number_index(ml_reference_position232),
                        )
                    })
                    .clone()
                    .unwrap_or_else(|| -1f64),
                    self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_f18340) => {
                        let text = source_ml_source2_f18340.clone();
                        {
                            let source_ml_source2_string_unit_operand18394 = text;
                            {
                                let source_ml_source2_string_unit_operand18395 = at;
                                (std::rc::Rc::new(
                                    move |text_2: self::ml_string::Text, position: f64| -> f64 {
                                        {
                                            let index = if position.is_nan() {
                                                0f64
                                            } else {
                                                position.trunc()
                                            };
                                            if (index < 0f64) || (index >= (text_2.length())) {
                                                f64::NAN
                                            } else {
                                                ({
                                                let source_ml_source2_string_unit_operand18401 = {
                                                    let source_ml_source2_string_range_operand18396 = text_2.clone();
                                                    {
                                                        let source_ml_source2_string_range_operand18397 = index;
                                                        {
                                                            let source_ml_source2_string_range_operand18398 = index + 1f64;
                                                            {
                                                                let source_ml_source2_string_range_position18399 = source_ml_source2_string_range_operand18397;
                                                                {
                                                                    let source_ml_source2_string_range_position18400 = source_ml_source2_string_range_operand18398;
                                                                    source_ml_source2_string_range_operand18396.slice(source_ml_source2_string_range_position18399, source_ml_source2_string_range_position18400)
                                                                }
                                                            }
                                                        }
                                                    }
                                                };
                                                {
                                                    let source_ml_source2_string_unit_operand18402 = 0f64;
                                                    source_ml_source2_string_unit_operand18401.code_point_at(source_ml_source2_string_unit_operand18402)
                                                }
                                            }).clone().unwrap_or_else(|| f64::NAN)
                                            }
                                        }
                                    },
                                ))(
                                    source_ml_source2_string_unit_operand18394,
                                    source_ml_source2_string_unit_operand18395,
                                )
                            }
                        }
                    }
                    _ => -1f64,
                };
                {
                    let b = match right.clone() {
                        self::JavaScriptSemanticValue::Text(units_2) => ({
                            let ml_reference_array233 = units_2.clone();
                            let ml_reference_position233 = at;
                            self::ml_array::optional_at(
                                &*ml_reference_array233.borrow(),
                                self::ml_array::number_index(ml_reference_position233),
                            )
                        })
                        .clone()
                        .unwrap_or_else(|| -1f64),
                        self::JavaScriptSemanticValue::SourceText(_, source_ml_source2_f18423) => {
                            let text_2 = source_ml_source2_f18423.clone();
                            {
                                let source_ml_source2_string_unit_operand18477 = text_2.clone();
                                {
                                    let source_ml_source2_string_unit_operand18478 = at;
                                    (std::rc::Rc::new(
                                        move |text_3: self::ml_string::Text,
                                              position: f64|
                                              -> f64 {
                                            {
                                                let index = if position.is_nan() {
                                                    0f64
                                                } else {
                                                    position.trunc()
                                                };
                                                if (index < 0f64) || (index >= (text_3.length())) {
                                                    f64::NAN
                                                } else {
                                                    ({
                                                    let source_ml_source2_string_unit_operand18484 = {
                                                        let source_ml_source2_string_range_operand18479 = text_3.clone();
                                                        {
                                                            let source_ml_source2_string_range_operand18480 = index;
                                                            {
                                                                let source_ml_source2_string_range_operand18481 = index + 1f64;
                                                                {
                                                                    let source_ml_source2_string_range_position18482 = source_ml_source2_string_range_operand18480;
                                                                    {
                                                                        let source_ml_source2_string_range_position18483 = source_ml_source2_string_range_operand18481;
                                                                        source_ml_source2_string_range_operand18479.slice(source_ml_source2_string_range_position18482, source_ml_source2_string_range_position18483)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    };
                                                    {
                                                        let source_ml_source2_string_unit_operand18485 = 0f64;
                                                        source_ml_source2_string_unit_operand18484.code_point_at(source_ml_source2_string_unit_operand18485)
                                                    }
                                                }).clone().unwrap_or_else(|| f64::NAN)
                                                }
                                            }
                                        },
                                    ))(
                                        source_ml_source2_string_unit_operand18477,
                                        source_ml_source2_string_unit_operand18478,
                                    )
                                }
                            }
                        }
                        _ => -1f64,
                    };
                    {
                        let ml_library_inequality_1 = a != b;
                        if ml_library_inequality_1 {
                            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Return(false)
                        } else {
                            {
                                let at_2 = at + 1f64;
                                {
                                    at = at_2;
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Result::MlSource1TemporaryTestJavaScriptSemanticTextEqualLoop4Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Result {
    MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_construct_java_script_semantic_text_units_loop7(
    mut value: self::JavaScriptSemanticValue,
    mut length: f64,
    mut units: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut at: f64,
) -> self::MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Result {
    loop {
        if at < length {
            {
                let source_ml_source1_temporary_construct_java_script_semantic_text_units_ignored8 = {
                    let ml_method_owner234 = units.clone();
                    let ml_method_argument234_0 =
                        self::read_java_script_semantic_text_unit(value.clone(), at);
                    let ml_inserted = vec![ml_method_argument234_0];
                    let mut ml_method_values234 = ml_method_owner234.borrow_mut();
                    let ml_length = ml_method_values234
                        .len()
                        .checked_add(ml_inserted.len())
                        .filter(|length| *length <= 4_294_967_295usize)
                        .expect("Invalid array length");
                    ml_method_values234.extend(ml_inserted);
                    ml_length as f64
                };
                {
                    let at_2 = at + 1f64;
                    {
                        at = at_2;
                        continue;
                    }
                }
            }
        }
        return {
            self::MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Result::MlSource1TemporaryConstructJavaScriptSemanticTextUnitsLoop7Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result {
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done,
    MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_find_java_script_property_descriptor_entry_loop9(
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut key: self::JavaScriptSemanticValue,
    mut at: f64,
) -> self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result {
    loop {
        return if at
            < ({
                let ml_reference_array235 = properties.clone();
                ml_reference_array235.borrow().len() as f64
            }) {
            {
                let property = {
                    let ml_reference_array236 = properties.clone();
                    let ml_reference_position236 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array236.borrow(),
                        self::ml_array::number_index(ml_reference_position236),
                    )
                };
                if !(match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                }) && self::test_java_script_semantic_same_value(
                    match property.clone().expect("a proven present value") {
                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                            source_ml_source2_data_property_value18507,
                            _,
                        ) => {
                            let source_ml_source2_data_property_value18507 =
                                *source_ml_source2_data_property_value18507;
                            source_ml_source2_data_property_value18507.clone()
                        }
                    },
                    key.clone(),
                ) {
                    self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Return(at)
                } else {
                    {
                        let at_2 = at + 1f64;
                        {
                            at = at_2;
                            continue;
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Result::MlSource1TemporaryFindJavaScriptPropertyDescriptorEntryLoop9Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10 {
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Next,
    MlSource1TemporaryPlanJavaScriptPropertyDefinitionJoin10Return(
        std::rc::Rc<
            std::cell::RefCell<(
                std::rc::Rc<
                    std::cell::RefCell<(
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
                    )>,
                >,
                self::ml_string::Text,
                self::ml_string::Text,
                std::rc::Rc<std::cell::RefCell<Vec<self::ml_string::Text>>>,
            )>,
        >,
    ),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Result {
    MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(f64),
    MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(f64),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_literal_property_index_loop11(
    mut key: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut value: f64,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Result {
    loop {
        return if at
            < ({
                let ml_reference_array237 = key.clone();
                ml_reference_array237.borrow().len() as f64
            }) {
            {
                let digit = match {
                    let ml_reference_array238 = key.clone();
                    let ml_reference_position238 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array238.borrow(),
                        self::ml_array::number_index(ml_reference_position238),
                    )
                } {
                    Some(ml_present) => ml_present,
                    None => f64::NAN,
                } - 48f64;
                if (!(self::ml_math::is_integer(digit)) || (digit < 0f64)) || (digit > 9f64) {
                    self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Return(-1f64)
                } else {
                    {
                        let value_2 = (value * 10f64) + digit;
                        {
                            let at_2 = at + 1f64;
                            {
                                (value, at) = (value_2, at_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Result::MlSource1TemporaryReadJavaScriptLiteralPropertyIndexLoop11Done(value)
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Result {
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Done,
    MlSource1TemporaryFindJavaScriptHeapDirectoryObjectLoop12Return(f64),
}
