// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptArrayCompletion;
use super::JavaScriptArrayDefinitionState;
use super::JavaScriptArrayPropertyPlan;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptSemanticValue;
use super::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result;
use super::construct_java_script_array_object;
use super::find_java_script_ordinary_object;
use super::ml;
use super::ml_array;
use super::ml_string;
use super::plan_java_script_array_length_definition;
use super::plan_java_script_array_property_definition;
use super::read_java_script_ordinary_heap_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_property_descriptor_kind;
use super::replace_java_script_ordinary_object;
use super::source_ml_source1_temporary_allocate_java_script_array_loop134;
use super::source_ml_source1_temporary_allocate_java_script_array_loop135;
use super::source_ml_source1_temporary_read_java_script_array_definition_state_error_loop137;
use super::test_java_script_array_length;
use super::test_java_script_array_length_key;
use super::test_java_script_ordinary_symbol;
use super::test_java_script_semantic_accessor;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_object_observation_operation(
    operation: &self::ml_string::Text,
) -> bool {
    self::ml_array::includes(
        &{
            let ml_reference_value280 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 98, 117, 105, 108, 116, 105, 110, 84, 97, 103,
                ]),
                self::ml_string::Text::from_units(vec![
                    83, 121, 109, 98, 111, 108, 46, 118, 97, 108, 117, 101, 79, 102,
                ]),
                self::ml_string::Text::from_units(vec![
                    78, 117, 109, 98, 101, 114, 46, 118, 97, 108, 117, 101, 79, 102,
                ]),
                self::ml_string::Text::from_units(vec![
                    66, 111, 111, 108, 101, 97, 110, 46, 118, 97, 108, 117, 101, 79, 102,
                ]),
                self::ml_string::Text::from_units(vec![
                    66, 111, 111, 108, 101, 97, 110, 46, 116, 111, 83, 116, 114, 105, 110, 103,
                ]),
                self::ml_string::Text::from_units(vec![
                    66, 105, 103, 73, 110, 116, 46, 118, 97, 108, 117, 101, 79, 102,
                ]),
                self::ml_string::Text::from_units(vec![
                    83, 116, 114, 105, 110, 103, 46, 116, 111, 83, 116, 114, 105, 110, 103,
                ]),
                self::ml_string::Text::from_units(vec![
                    83, 116, 114, 105, 110, 103, 46, 118, 97, 108, 117, 101, 79, 102,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 114, 101, 103, 117, 108, 97, 114, 69, 120, 112,
                    114, 101, 115, 115, 105, 111, 110, 83, 108, 111, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46,
                    104, 97, 115, 79, 119, 110, 80, 114, 111, 112, 101, 114, 116, 121,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 112, 114, 111, 116, 111, 116, 121, 112, 101, 46,
                    112, 114, 111, 112, 101, 114, 116, 121, 73, 115, 69, 110, 117, 109, 101, 114,
                    97, 98, 108, 101,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 104, 97, 115, 79, 119, 110,
                ]),
                self::ml_string::Text::from_units(vec![79, 98, 106, 101, 99, 116, 46, 105, 115]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 105, 115, 69, 120, 116, 101, 110, 115, 105, 98,
                    108, 101,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 105, 115, 83, 101, 97, 108, 101, 100,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 105, 115, 70, 114, 111, 122, 101, 110,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 112, 114, 101, 118, 101, 110, 116, 69, 120, 116,
                    101, 110, 115, 105, 111, 110, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 115, 101, 97, 108,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 102, 114, 101, 101, 122, 101,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 80, 114, 111, 116, 111, 116, 121,
                    112, 101, 79, 102,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 115, 101, 116, 80, 114, 111, 116, 111, 116, 121,
                    112, 101, 79, 102,
                ]),
            ])))
            .borrow()
            .clone();
            ml_reference_value280
        },
        operation,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_array_intrinsic_operation(
    operation: &self::ml_string::Text,
) -> bool {
    ((*operation)
        == (self::ml_string::Text::from_units(vec![
            65, 114, 114, 97, 121, 46, 105, 115, 65, 114, 114, 97, 121,
        ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_array_callback_operation(
    operation: &self::ml_string::Text,
) -> bool {
    self::ml_array::includes(
        &{
            let ml_reference_value281 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 102, 111, 114, 69, 97, 99, 104,
                ]),
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 115, 111, 109, 101,
                ]),
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 101, 118, 101, 114, 121,
                ]),
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 102, 105, 110, 100,
                ]),
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 73, 110, 100, 101, 120,
                ]),
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 76, 97, 115, 116,
                ]),
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 102, 105, 110, 100, 76, 97, 115, 116, 73, 110, 100,
                    101, 120,
                ]),
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 114, 101, 100, 117, 99, 101,
                ]),
                self::ml_string::Text::from_units(vec![
                    65, 114, 114, 97, 121, 46, 114, 101, 100, 117, 99, 101, 82, 105, 103, 104, 116,
                ]),
            ])))
            .borrow()
            .clone();
            ml_reference_value281
        },
        operation,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_array_transform_operation(
    operation: &self::ml_string::Text,
) -> bool {
    (((*operation)
        == (self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 46, 109, 97, 112])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                65, 114, 114, 97, 121, 46, 102, 105, 108, 116, 101, 114,
            ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_object_enumeration_operation(
    operation: &self::ml_string::Text,
) -> bool {
    self::ml_array::includes(
        &{
            let ml_reference_value282 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 107, 101, 121, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 118, 97, 108, 117, 101, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 101, 110, 116, 114, 105, 101, 115,
                ]),
            ])))
            .borrow()
            .clone();
            ml_reference_value282
        },
        operation,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_property_reflection_operation(
    operation: &self::ml_string::Text,
) -> bool {
    self::ml_array::includes(
        &{
            let ml_reference_value283 = (std::rc::Rc::new(std::cell::RefCell::new(vec![
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112,
                    101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112,
                    101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112,
                    101, 114, 116, 121, 78, 97, 109, 101, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112,
                    101, 114, 116, 121, 83, 121, 109, 98, 111, 108, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    82, 101, 102, 108, 101, 99, 116, 46, 111, 119, 110, 75, 101, 121, 115,
                ]),
                self::ml_string::Text::from_units(vec![
                    82, 101, 102, 108, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111,
                    112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114,
                ]),
            ])))
            .borrow()
            .clone();
            ml_reference_value283
        },
        operation,
        0f64,
        false,
        |a, b| a == b,
    )
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_single_descriptor_operation(
    operation: &self::ml_string::Text,
) -> bool {
    (((*operation)
        == (self::ml_string::Text::from_units(vec![
            79, 98, 106, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111, 112, 101,
            114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                82, 101, 102, 108, 101, 99, 116, 46, 103, 101, 116, 79, 119, 110, 80, 114, 111,
                112, 101, 114, 116, 121, 68, 101, 115, 99, 114, 105, 112, 116, 111, 114,
            ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_property_definitions_operation(
    operation: &self::ml_string::Text,
) -> bool {
    (((*operation)
        == (self::ml_string::Text::from_units(vec![
            79, 98, 106, 101, 99, 116, 46, 100, 101, 102, 105, 110, 101, 80, 114, 111, 112, 101,
            114, 116, 105, 101, 115,
        ])))
        || ((*operation)
            == (self::ml_string::Text::from_units(vec![
                79, 98, 106, 101, 99, 116, 46, 99, 114, 101, 97, 116, 101,
            ]))))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_standard_primitive_method_operation(
    operation: &self::ml_string::Text,
) -> bool {
    ((*operation)
        == (self::ml_string::Text::from_units(vec![
            83, 116, 114, 105, 110, 103, 46, 115, 116, 97, 114, 116, 115, 87, 105, 116, 104,
        ])))
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn test_java_script_array_intrinsic_operation(operation: &self::ml_string::Text) -> bool {
    self::test_java_script_standard_array_intrinsic_operation(operation)
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn allocate_java_script_array(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    prototype: self::JavaScriptSemanticValue,
    length: f64,
) -> self::JavaScriptArrayCompletion {
    match heap.clone() {
        self::JavaScriptOrdinaryHeap::OrdinaryHeap(
            shared_constructor_field1,
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
        ) => {
            let error = self::read_java_script_ordinary_heap_error(heap.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), error.clone())
            } else {
                if ((((identity <= (self::ml::Big::from_i128(0)))
                    || (identity > (self::ml::Big::from_i128(18446744073709551615))))
                    || ((self::find_java_script_ordinary_object(
                        shared_constructor_field1.clone(),
                        identity.clone(),
                    )) >= (0f64)))
                    || self::test_java_script_ordinary_symbol(
                        shared_constructor_field2.clone(),
                        identity.clone(),
                    ))
                {
                    self::JavaScriptArrayCompletion::ArrayInvalid(
                        Box::new(heap.clone()),
                        self::ml_string::Text::from_units(vec![
                            65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111,
                            110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 102, 114,
                            101, 115, 104, 32, 101, 120, 116, 101, 114, 110, 97, 108, 32, 117, 110,
                            115, 105, 103, 110, 101, 100, 54, 52, 32, 103, 108, 111, 98, 97, 108,
                            32, 105, 100, 101, 110, 116, 105, 116, 121,
                        ]),
                    )
                } else {
                    {
                        let actual = if (length == (0f64)) { 0f64 } else { length };
                        if !(self::test_java_script_array_length(actual)) {
                            self::JavaScriptArrayCompletion::ArrayError(
                                Box::new(heap.clone()),
                                self::ml_string::Text::from_units(vec![
                                    82, 97, 110, 103, 101, 69, 114, 114, 111, 114,
                                ]),
                                self::ml_string::Text::from_units(vec![
                                    73, 110, 118, 97, 108, 105, 100, 32, 97, 114, 114, 97, 121, 32,
                                    108, 101, 110, 103, 116, 104,
                                ]),
                            )
                        } else {
                            if (match prototype.clone() {
                                self::JavaScriptSemanticValue::Null => false,
                                _ => true,
                            } && match prototype.clone() {
                                self::JavaScriptSemanticValue::Object(_, _) => false,
                                _ => true,
                            }) {
                                self::JavaScriptArrayCompletion::ArrayInvalid(
                                    Box::new(heap.clone()),
                                    self::ml_string::Text::from_units(vec![
                                        65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116,
                                        105, 111, 110, 32, 114, 101, 116, 97, 105, 110, 115, 32,
                                        105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 110, 117,
                                        108, 108, 32, 111, 114, 32, 79, 98, 106, 101, 99, 116, 32,
                                        112, 114, 111, 116, 111, 116, 121, 112, 101,
                                    ]),
                                )
                            } else {
                                {
                                    let invalid = self::read_java_script_ordinary_value_error(
                                        heap.clone(),
                                        prototype.clone(),
                                    );
                                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                        self::JavaScriptArrayCompletion::ArrayInvalid(
                                            Box::new(heap.clone()),
                                            invalid.clone(),
                                        )
                                    } else {
                                        {
                                            let count = 0f64;
                                            {
                                                let source_ml_source1_temporary_values132 =
                                                    shared_constructor_field1.clone();
                                                {
                                                    let source_ml_source1_temporary_index132 =
                                                        self::ml::Big::from_i128(0);
                                                    {
                                                        let count_2 = self::source_ml_source1_temporary_allocate_java_script_array_loop134(count, source_ml_source1_temporary_values132.clone(), source_ml_source1_temporary_index132.clone());
                                                        if ((({
                                                            let ml_reference_array284 =
                                                                shared_constructor_field1.clone();
                                                            let ml_reference_projection284 =
                                                                ml_reference_array284.borrow().len()
                                                                    as f64;
                                                            ml_reference_projection284
                                                        }) >= (4096f64))
                                                            || (count_2 >= (16384f64)))
                                                        {
                                                            self::JavaScriptArrayCompletion::ArrayResourceLimit(Box::new(heap.clone()), self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 108, 108, 111, 99, 97, 116, 105, 111, 110, 32, 101, 120, 99, 101, 101, 100, 115, 32, 116, 104, 101, 32, 115, 104, 97, 114, 101, 100, 32, 111, 98, 106, 101, 99, 116, 32, 111, 114, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121, 32, 98, 117, 100, 103, 101, 116]), None::<(Option<self::JavaScriptArrayDefinitionState>,)>)
                                                        } else {
                                                            {
                                                                let object = self::construct_java_script_array_object(identity.clone(), prototype.clone(), actual);
                                                                {
                                                                    let at = 0f64;
                                                                    {
                                                                        let at_2 = self::source_ml_source1_temporary_allocate_java_script_array_loop135(heap.clone(), identity.clone(), at);
                                                                        {
                                                                            let next = {
                                                                                let source_ml_source2_nominal_field3229 = shared_constructor_field1.clone();
                                                                                {
                                                                                    let source_ml_source2_nominal_field3231 = shared_constructor_field2.clone();
                                                                                    {
                                                                                        let source_ml_source2_nominal_field3233 = shared_constructor_field3.clone();
                                                                                        {
                                                                                            let source_ml_source2_nominal_field3235 = shared_constructor_field4.clone();
                                                                                            {
                                                                                                let source_ml_source2_nominal_field3237 = shared_constructor_field5.clone();
                                                                                                {
                                                                                                    let source_ml_source2_nominal_field3239 = shared_constructor_field6.clone();
                                                                                                    {
                                                                                                        let source_ml_source2_nominal_override3241 = std::rc::Rc::new(std::cell::RefCell::new({
                                                                                                                let source_ml_source2_concat_value3225 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value285 = shared_constructor_field1.borrow().clone(); ml_reference_value285 }, 0f64, at_2)));
                                                                                                                {
                                                                                                                    let source_ml_source2_concat_value3226 = std::rc::Rc::new(std::cell::RefCell::new(vec![object.clone()]));
                                                                                                                    {
                                                                                                                        let source_ml_source2_concat_value3227 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::slice(&{ let ml_reference_value286 = shared_constructor_field1.borrow().clone(); ml_reference_value286 }, at_2, f64::INFINITY)));
                                                                                                                        self::ml_array::append(self::ml_array::append(self::ml_array::append(Vec::<self::JavaScriptOrdinaryObject>::new(), { let ml_reference_value287 = source_ml_source2_concat_value3225.borrow().clone(); ml_reference_value287 }), { let ml_reference_value288 = source_ml_source2_concat_value3226.borrow().clone(); ml_reference_value288 }), { let ml_reference_value289 = source_ml_source2_concat_value3227.borrow().clone(); ml_reference_value289 })
                                                                                                                    }
                                                                                                                }
                                                                                                            }));
                                                                                                        self::JavaScriptOrdinaryHeap::OrdinaryHeap(source_ml_source2_nominal_override3241.clone(), source_ml_source2_nominal_field3231.clone(), source_ml_source2_nominal_field3233.clone(), source_ml_source2_nominal_field3235.clone(), source_ml_source2_nominal_field3237.clone(), source_ml_source2_nominal_field3239.clone())
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            };
                                                                            {
                                                                                let problem = self::read_java_script_ordinary_heap_error(next.clone());
                                                                                if (problem != (self::ml_string::Text::from_units(vec![]))) {
                                                                                        self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), problem.clone())
                                                                                    } else {
                                                                                        self::JavaScriptArrayCompletion::ArrayNormal(Box::new(next.clone()), Box::new(self::JavaScriptSemanticValue::Object(identity.clone(), false)))
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
                    }
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn read_java_script_array_definition_state_error(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayDefinitionState,
) -> self::ml_string::Text {
    match state.clone() {
        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
            shared_constructor_field2,
            shared_constructor_field3,
            shared_constructor_field4,
            shared_constructor_field5,
            shared_constructor_field6,
            shared_constructor_field7,
        ) => {
            let shared_constructor_field3 = *shared_constructor_field3;
            let shared_constructor_field7 = *shared_constructor_field7;
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
                        if ((match state.clone() {
                            self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
                                _,
                                _,
                                _,
                                _,
                                _,
                                _,
                            ) => false,
                        } || !(self::ml_array::includes(
                            &{
                                let ml_reference_value290 =
                                    (std::rc::Rc::new(std::cell::RefCell::new(vec![
                                        self::ml_string::Text::from_units(vec![
                                            102, 105, 114, 115, 116, 32, 110, 117, 109, 98, 101,
                                            114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            115, 101, 99, 111, 110, 100, 32, 110, 117, 109, 98,
                                            101, 114,
                                        ]),
                                        self::ml_string::Text::from_units(vec![
                                            97, 112, 112, 108, 121,
                                        ]),
                                    ])))
                                    .borrow()
                                    .clone();
                                ml_reference_value290
                            },
                            &shared_constructor_field5.clone(),
                            0f64,
                            false,
                            |a, b| a == b,
                        ))) || !(self::test_java_script_array_length(
                            shared_constructor_field6,
                        ))) {
                            self::ml_string::Text::from_units(vec![
                                65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105,
                                111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116,
                                115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 111, 117, 110, 100,
                                101, 100, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32,
                                112, 104, 97, 115, 101,
                            ])
                        } else {
                            if {
                                let source_ml_source2_s3251 = shared_constructor_field7.clone();
                                match source_ml_source2_s3251.clone() {
                                    self::JavaScriptSemanticValue::Number(_) => false,
                                    _ => true,
                                }
                            } {
                                self::ml_string::Text::from_units(vec![
                                    65, 114, 114, 97, 121, 32, 110, 117, 109, 101, 114, 105, 99,
                                    32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 114,
                                    101, 116, 97, 105, 110, 115, 32, 105, 116, 115, 32, 101, 120,
                                    97, 99, 116, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 78,
                                    117, 109, 98, 101, 114,
                                ])
                            } else {
                                {
                                    let number_error = self::read_java_script_ordinary_value_error(
                                        heap.clone(),
                                        shared_constructor_field7.clone(),
                                    );
                                    if (number_error != (self::ml_string::Text::from_units(vec![])))
                                    {
                                        number_error.clone()
                                    } else {
                                        {
                                            let key_error =
                                                self::read_java_script_ordinary_key_error(
                                                    heap.clone(),
                                                    shared_constructor_field3.clone(),
                                                );
                                            if (key_error
                                                != (self::ml_string::Text::from_units(vec![])))
                                            {
                                                key_error.clone()
                                            } else {
                                                {
                                                    let object = {
                                                        let source_ml_source2_array_read_value3259 =
                                                            shared_constructor_field1.clone();
                                                        {
                                                            let source_ml_source2_array_read_index3260 = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), shared_constructor_field2.clone());
                                                            {
                                                                let ml_reference_array291 = source_ml_source2_array_read_value3259.clone();
                                                                let ml_reference_position291 = source_ml_source2_array_read_index3260;
                                                                let ml_reference_projection291 = self::ml_array::optional_at(&*ml_reference_array291.borrow(), self::ml_array::number_index(ml_reference_position291));
                                                                ml_reference_projection291
                                                            }
                                                        }
                                                    };
                                                    if (match (object.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } || match object.clone().expect("a proven present value") {
                                                            self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }) {
                                                            self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 98, 114, 97, 110, 100, 101, 100, 32, 103, 108, 111, 98, 97, 108, 32, 114, 101, 99, 101, 105, 118, 101, 114])
                                                        } else {
                                                            if ((self::read_java_script_property_descriptor_kind(shared_constructor_field4.clone())) == (self::ml_string::Text::from_units(vec![105, 110, 118, 97, 108, 105, 100]))) {
                                                                self::ml_string::Text::from_units(vec![97, 110, 32, 65, 114, 114, 97, 121, 32, 100, 101, 102, 105, 110, 105, 116, 105, 111, 110, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 97, 32, 118, 97, 108, 105, 100, 32, 112, 97, 114, 116, 105, 97, 108, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114])
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_values136 = std::rc::Rc::new(std::cell::RefCell::new(vec![(({ let ml_reference_value292 = shared_constructor_field4.borrow().clone(); ml_reference_value292 }).0).8, (({ let ml_reference_value293 = shared_constructor_field4.borrow().clone(); ml_reference_value293 }).0).4, (({ let ml_reference_value294 = shared_constructor_field4.borrow().clone(); ml_reference_value294 }).0).6]));
                                                                    {
                                                                        let source_ml_source1_temporary_index136 = self::ml::Big::from_i128(0);
                                                                        {
                                                                            let source_ml_source2_s3271 = self::source_ml_source1_temporary_read_java_script_array_definition_state_error_loop137(heap.clone(), source_ml_source1_temporary_values136.clone(), source_ml_source1_temporary_index136.clone());
                                                                            match source_ml_source2_s3271.clone() {
                                                                                self::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Done => {
                                                                                    if (((({ let ml_reference_value295 = shared_constructor_field4.borrow().clone(); ml_reference_value295 }).0).5 && !(self::test_java_script_semantic_accessor((({ let ml_reference_value296 = shared_constructor_field4.borrow().clone(); ml_reference_value296 }).0).4))) || ((({ let ml_reference_value297 = shared_constructor_field4.borrow().clone(); ml_reference_value297 }).0).7 && !(self::test_java_script_semantic_accessor((({ let ml_reference_value298 = shared_constructor_field4.borrow().clone(); ml_reference_value298 }).0).6)))) {
                                                                                        self::ml_string::Text::from_units(vec![65, 114, 114, 97, 121, 32, 97, 99, 99, 101, 115, 115, 111, 114, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 116, 117, 97, 108, 32, 99, 97, 108, 108, 97, 98, 108, 101, 32, 111, 114, 32, 117, 110, 100, 101, 102, 105, 110, 101, 100, 32, 115, 101, 109, 97, 110, 116, 105, 99, 32, 118, 97, 108, 117, 101, 115])
                                                                                    } else {
                                                                                        if ((shared_constructor_field5 != (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121]))) && (!(self::test_java_script_array_length_key(shared_constructor_field3.clone())) || !((({ let ml_reference_value299 = shared_constructor_field4.borrow().clone(); ml_reference_value299 }).0).9))) {
                                                                                            self::ml_string::Text::from_units(vec![111, 110, 108, 121, 32, 97, 110, 32, 97, 99, 116, 117, 97, 108, 32, 65, 114, 114, 97, 121, 32, 108, 101, 110, 103, 116, 104, 32, 118, 97, 108, 117, 101, 32, 115, 116, 97, 114, 116, 115, 32, 116, 104, 101, 32, 110, 117, 109, 101, 114, 105, 99, 32, 99, 111, 110, 118, 101, 114, 115, 105, 111, 110, 32, 112, 114, 111, 116, 111, 99, 111, 108])
                                                                                        } else {
                                                                                            self::ml_string::Text::from_units(vec![])
                                                                                        }
                                                                                    }
                                                                                }
                                                                                self::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Result::MlSource1TemporaryReadJavaScriptArrayDefinitionStateErrorLoop137Return(source_ml_source1_temporary_result137) => {
                                                                                    source_ml_source1_temporary_result137.clone()
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
                }
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn begin_java_script_array_property_definition(
    heap: self::JavaScriptOrdinaryHeap,
    identity: self::ml::Big,
    key: self::JavaScriptSemanticValue,
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
) -> self::JavaScriptArrayCompletion {
    {
        let state = self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
            identity.clone(),
            Box::new(key.clone()),
            descriptor.clone(),
            if (self::test_java_script_array_length_key(key.clone())
                && (({
                    let ml_reference_value300 = descriptor.borrow().clone();
                    ml_reference_value300
                })
                .0)
                    .9)
            {
                self::ml_string::Text::from_units(vec![
                    102, 105, 114, 115, 116, 32, 110, 117, 109, 98, 101, 114,
                ])
            } else {
                self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])
            },
            0f64,
            Box::new(self::JavaScriptSemanticValue::Number(0f64)),
        );
        {
            let error =
                self::read_java_script_array_definition_state_error(heap.clone(), state.clone());
            if (error != (self::ml_string::Text::from_units(vec![]))) {
                self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), error.clone())
            } else {
                self::JavaScriptArrayCompletion::ArrayDefinitionContinue(
                    Box::new(heap.clone()),
                    Box::new(state.clone()),
                )
            }
        }
    }
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn step_java_script_array_property_definition(
    heap: self::JavaScriptOrdinaryHeap,
    state: self::JavaScriptArrayDefinitionState,
) -> self::JavaScriptArrayCompletion {
    match state.clone() {
        self::JavaScriptArrayDefinitionState::ArrayDefinitionState(
            shared_constructor_field7,
            shared_constructor_field8,
            shared_constructor_field9,
            shared_constructor_field10,
            shared_constructor_field11,
            shared_constructor_field12,
        ) => {
            let shared_constructor_field8 = *shared_constructor_field8;
            let shared_constructor_field12 = *shared_constructor_field12;
            match heap.clone() {
                self::JavaScriptOrdinaryHeap::OrdinaryHeap(
                    shared_constructor_field6,
                    _,
                    _,
                    _,
                    _,
                    _,
                ) => {
                    let error = self::read_java_script_array_definition_state_error(
                        heap.clone(),
                        state.clone(),
                    );
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::JavaScriptArrayCompletion::ArrayInvalid(
                            Box::new(heap.clone()),
                            error.clone(),
                        )
                    } else {
                        if (shared_constructor_field10
                            != (self::ml_string::Text::from_units(vec![97, 112, 112, 108, 121])))
                        {
                            self::JavaScriptArrayCompletion::ArrayNumberRequest(
                                Box::new(heap.clone()),
                                Box::new(state.clone()),
                                Box::new(
                                    (({
                                        let ml_reference_value301 =
                                            shared_constructor_field9.borrow().clone();
                                        ml_reference_value301
                                    })
                                    .0)
                                        .8,
                                ),
                            )
                        } else {
                            {
                                let object = {
                                    let source_ml_source2_array_read_value3286 =
                                        shared_constructor_field6.clone();
                                    {
                                        let source_ml_source2_array_read_index3287 =
                                            self::find_java_script_ordinary_object(
                                                shared_constructor_field6.clone(),
                                                shared_constructor_field7.clone(),
                                            );
                                        {
                                            let ml_reference_array302 =
                                                source_ml_source2_array_read_value3286.clone();
                                            let ml_reference_position302 =
                                                source_ml_source2_array_read_index3287;
                                            let ml_reference_projection302 =
                                                self::ml_array::optional_at(
                                                    &*ml_reference_array302.borrow(),
                                                    self::ml_array::number_index(
                                                        ml_reference_position302,
                                                    ),
                                                );
                                            ml_reference_projection302
                                        }
                                    }
                                };
                                if (match (object.clone(), None::<()>) {
                                    (Some(_ml_left), Some(_ml_right)) => false,
                                    (None, None) => true,
                                    _ => false,
                                } || match object.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                        false
                                    }
                                    _ => true,
                                }) {
                                    self::JavaScriptArrayCompletion::ArrayInvalid(
                                        Box::new(heap.clone()),
                                        self::ml_string::Text::from_units(vec![
                                            65, 114, 114, 97, 121, 32, 114, 101, 99, 101, 105, 118,
                                            101, 114, 32, 109, 117, 115, 116, 32, 114, 101, 109,
                                            97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 115,
                                            104, 97, 114, 101, 100, 32, 103, 108, 111, 98, 97, 108,
                                            32, 100, 105, 114, 101, 99, 116, 111, 114, 121,
                                        ]),
                                    )
                                } else {
                                    {
                                        let number_length = shared_constructor_field12.clone();
                                        match number_length.clone() {
                                            self::JavaScriptSemanticValue::Number(value) => {
                                                let plan =
                                                    if (self::test_java_script_array_length_key(
                                                        shared_constructor_field8.clone(),
                                                    ) && (({
                                                        let ml_reference_value303 =
                                                            shared_constructor_field9
                                                                .borrow()
                                                                .clone();
                                                        ml_reference_value303
                                                    })
                                                    .0)
                                                        .9)
                                                    {
                                                        self::plan_java_script_array_length_definition(object.clone().expect("a proven present value"), shared_constructor_field9.clone(), shared_constructor_field11, value)
                                                    } else {
                                                        self::plan_java_script_array_property_definition(object.clone().expect("a proven present value"), shared_constructor_field8.clone(), shared_constructor_field9.clone())
                                                    };
                                                match plan.clone() {
                                                            self::JavaScriptArrayPropertyPlan::ArrayPropertyPlan(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3, shared_constructor_field4, shared_constructor_field5) => {
                                                                let shared_constructor_field1 = *shared_constructor_field1;
                                                                if ((shared_constructor_field4 == (self::ml_string::Text::from_units(vec![82, 97, 110, 103, 101, 69, 114, 114, 111, 114]))) || (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![84, 121, 112, 101, 69, 114, 114, 111, 114])))) {
                                                                    self::JavaScriptArrayCompletion::ArrayError(Box::new(heap.clone()), shared_constructor_field4.clone(), shared_constructor_field3.clone())
                                                                } else {
                                                                    if (shared_constructor_field4 == (self::ml_string::Text::from_units(vec![114, 101, 115, 111, 117, 114, 99, 101]))) {
                                                                        {
                                                                            let source_ml_source2_constructor_field3310 = shared_constructor_field3.clone();
                                                                            {
                                                                                let source_ml_source2_constructor_field3311 = Some(state.clone());
                                                                                self::JavaScriptArrayCompletion::ArrayResourceLimit(Box::new(heap.clone()), source_ml_source2_constructor_field3310.clone(), Some((source_ml_source2_constructor_field3311.clone(),)))
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if (shared_constructor_field4 != (self::ml_string::Text::from_units(vec![]))) {
                                                                            self::JavaScriptArrayCompletion::ArrayInvalid(Box::new(heap.clone()), shared_constructor_field3.clone())
                                                                        } else {
                                                                            {
                                                                                let updated = self::replace_java_script_ordinary_object(heap.clone(), shared_constructor_field1.clone());
                                                                                {
                                                                                    let invalid = self::read_java_script_ordinary_heap_error(updated.clone());
                                                                                    if (invalid != (self::ml_string::Text::from_units(vec![]))) {
                                                                                        {
                                                                                            let source_ml_source2_constructor_field3318 = Some(state.clone());
                                                                                            self::JavaScriptArrayCompletion::ArrayResourceLimit(Box::new(heap.clone()), invalid.clone(), Some((source_ml_source2_constructor_field3318.clone(),)))
                                                                                        }
                                                                                    } else {
                                                                                        self::JavaScriptArrayCompletion::ArrayPropertyComplete(Box::new(updated.clone()), shared_constructor_field2, shared_constructor_field5.clone())
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                            }
                                            _ => self::JavaScriptArrayCompletion::ArrayInvalid(
                                                Box::new(heap.clone()),
                                                self::ml_string::Text::from_units(vec![
                                                    65, 114, 114, 97, 121, 32, 108, 101, 110, 103,
                                                    116, 104, 32, 97, 112, 112, 108, 105, 99, 97,
                                                    116, 105, 111, 110, 32, 114, 101, 116, 97, 105,
                                                    110, 115, 32, 105, 116, 115, 32, 97, 99, 116,
                                                    117, 97, 108, 32, 115, 101, 109, 97, 110, 116,
                                                    105, 99, 32, 78, 117, 109, 98, 101, 114,
                                                ]),
                                            ),
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
