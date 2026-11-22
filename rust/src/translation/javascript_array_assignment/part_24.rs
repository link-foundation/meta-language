// Generated from complete js/src/translation/javascript-array-assignment.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptWeakCollectionEntry;
use super::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result;
use super::find_java_script_ordinary_property;
use super::ml;
use super::ml_array;
use super::ml_big_integer_conversion;
use super::ml_math;
use super::ml_string;
use super::read_java_script_ordinary_descriptor_value_error;
use super::read_java_script_ordinary_key_error;
use super::read_java_script_ordinary_value_error;
use super::read_java_script_semantic_text_length;
use super::read_java_script_weak_key_eligibility;
use super::test_java_script_collection_key;
use super::test_java_script_semantic_same_value;

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_weak_collection_object_error_loop60(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
    mut previous_weak_key: Option<self::ml::Big>,
    mut source_ml_source1_temporary_values57: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
    >,
    mut source_ml_source1_temporary_index57: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result {
    loop {
        return if source_ml_source1_temporary_index57
            < ({
                let ml_reference_array271 = source_ml_source1_temporary_values57.clone();
                self::ml::Big::from_u128(ml_reference_array271.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array272 = source_ml_source1_temporary_values57.clone();
                    let ml_reference_position272 = source_ml_source1_temporary_index57.clone();
                    self::ml_array::at(
                        &*ml_reference_array272.borrow(),
                        ml_reference_position272.to_index(),
                    )
                };
                if (match (entry.clone(), None::<()>) {
                    (_ml_left, Some(_ml_right)) => false,
                    _ => false,
                } || match (entry.clone(), None::<()>) {
                    (_ml_left, Some(_ml_right)) => false,
                    _ => false,
                }) || match entry.clone() {
                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(_, _) => false,
                } {
                    self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 101, 110, 115, 101, 32, 97, 99, 116, 117, 97, 108, 32, 101, 110, 116, 114, 121, 32, 114, 101, 99, 111, 114, 100, 115]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match entry.clone() {
                                self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                    source_ml_source2_data_property_value17345,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value17345 =
                                        *source_ml_source2_data_property_value17345;
                                    source_ml_source2_data_property_value17345.clone()
                                }
                            },
                        );
                        {
                            let value_error = self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                match entry.clone() {
                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                        _,
                                        source_ml_source2_data_property_value17346,
                                    ) => {
                                        let source_ml_source2_data_property_value17346 =
                                            *source_ml_source2_data_property_value17346;
                                        source_ml_source2_data_property_value17346.clone()
                                    }
                                },
                            );
                            if (key_error != (self::ml_string::Text::from_units(vec![])))
                                || (value_error != (self::ml_string::Text::from_units(vec![])))
                            {
                                self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(if key_error.0.is_empty() {
                                    value_error.clone()
                                } else {
                                    key_error.clone()
                                })
                            } else if (self::read_java_script_weak_key_eligibility(
                                heap.clone(),
                                match entry.clone() {
                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                        source_ml_source2_data_property_value17349,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value17349 =
                                            *source_ml_source2_data_property_value17349;
                                        source_ml_source2_data_property_value17349.clone()
                                    }
                                },
                            )) != (self::ml_string::Text::from_units(vec![
                                101, 108, 105, 103, 105, 98, 108, 101,
                            ])) {
                                self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 101, 108, 105, 103, 105, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 111, 114, 32, 110, 111, 110, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 115]))
                            } else {
                                if ({
                                    let source_ml_source2_s17351 = match entry.clone() {
                                            self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source2_data_property_value17361, _) => {
                                                let source_ml_source2_data_property_value17361 = *source_ml_source2_data_property_value17361;
                                                source_ml_source2_data_property_value17361.clone()
                                            }
                                        };
                                    match source_ml_source2_s17351 {
                                        self::JavaScriptSemanticValue::Object(_, _) => false,
                                        _ => true,
                                    }
                                } && {
                                    let source_ml_source2_s17363 = match entry.clone() {
                                            self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source2_data_property_value17373, _) => {
                                                let source_ml_source2_data_property_value17373 = *source_ml_source2_data_property_value17373;
                                                source_ml_source2_data_property_value17373.clone()
                                            }
                                        };
                                    match source_ml_source2_s17363 {
                                        self::JavaScriptSemanticValue::Symbol(_) => false,
                                        _ => true,
                                    }
                                }) || {
                                    let source_ml_source2_comparison_operand17377 = match match entry.clone() {
                                            self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source2_data_property_value17374, _) => {
                                                let source_ml_source2_data_property_value17374 = *source_ml_source2_data_property_value17374;
                                                source_ml_source2_data_property_value17374.clone()
                                            }
                                        } {
                                            self::JavaScriptSemanticValue::Undefined => {
                                                None::<self::ml::Big>
                                            }
                                            self::JavaScriptSemanticValue::Null => {
                                                None::<self::ml::Big>
                                            }
                                            self::JavaScriptSemanticValue::Boolean(_) => {
                                                None::<self::ml::Big>
                                            }
                                            self::JavaScriptSemanticValue::Number(_) => {
                                                None::<self::ml::Big>
                                            }
                                            self::JavaScriptSemanticValue::Integer(_) => {
                                                None::<self::ml::Big>
                                            }
                                            self::JavaScriptSemanticValue::Text(_) => {
                                                None::<self::ml::Big>
                                            }
                                            self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                None::<self::ml::Big>
                                            }
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source2_data_property_value17375) => {
                                                Some(source_ml_source2_data_property_value17375.clone())
                                            }
                                            self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value17376, _) => {
                                                Some(source_ml_source2_data_property_value17376.clone())
                                            }
                                        };
                                    if match (
                                        source_ml_source2_comparison_operand17377.clone(),
                                        None::<self::ml::Big>,
                                    ) {
                                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        if match (previous_weak_key.clone(), None::<self::ml::Big>)
                                        {
                                            (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                            (None, None) => true,
                                            _ => false,
                                        } {
                                            (f64::NAN) <= (f64::NAN)
                                        } else {
                                            {
                                                let source_ml_source2_comparison_order17380 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,previous_weak_key.clone().expect("a proven present value"));
                                                ((source_ml_source2_comparison_order17380 == -1f64)
                                                    || (source_ml_source2_comparison_order17380
                                                        == 0f64))
                                            }
                                        }
                                    } else if match (
                                        previous_weak_key.clone(),
                                        None::<self::ml::Big>,
                                    ) {
                                        (Some(ml_left), Some(ml_right)) => ml_left == ml_right,
                                        (None, None) => true,
                                        _ => false,
                                    } {
                                        {
                                            let source_ml_source2_comparison_order17379 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source2_comparison_operand17377.clone().expect("a proven present value"));
                                            ((source_ml_source2_comparison_order17379 == 1f64)
                                                || (source_ml_source2_comparison_order17379
                                                    == 0f64))
                                        }
                                    } else {
                                        (source_ml_source2_comparison_operand17377
                                            .clone()
                                            .expect("a proven present value"))
                                            <= (previous_weak_key
                                                .clone()
                                                .expect("a proven present value"))
                                    }
                                } {
                                    self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 107, 101, 121, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_weak_key_2 = match match entry.clone() {
                                                self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source2_data_property_value17381, _) => {
                                                    let source_ml_source2_data_property_value17381 = *source_ml_source2_data_property_value17381;
                                                    source_ml_source2_data_property_value17381.clone()
                                                }
                                            } {
                                                self::JavaScriptSemanticValue::Undefined => {
                                                    None::<self::ml::Big>
                                                }
                                                self::JavaScriptSemanticValue::Null => {
                                                    None::<self::ml::Big>
                                                }
                                                self::JavaScriptSemanticValue::Boolean(_) => {
                                                    None::<self::ml::Big>
                                                }
                                                self::JavaScriptSemanticValue::Number(_) => {
                                                    None::<self::ml::Big>
                                                }
                                                self::JavaScriptSemanticValue::Integer(_) => {
                                                    None::<self::ml::Big>
                                                }
                                                self::JavaScriptSemanticValue::Text(_) => {
                                                    None::<self::ml::Big>
                                                }
                                                self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                    None::<self::ml::Big>
                                                }
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source2_data_property_value17382) => {
                                                    Some(source_ml_source2_data_property_value17382.clone())
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value17383, _) => {
                                                    Some(source_ml_source2_data_property_value17383.clone())
                                                }
                                            };
                                        if match object.clone() {
                                            self::JavaScriptOrdinaryObject::WeakSetObject(
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                                _,
                                            ) => true,
                                            _ => false,
                                        } && {
                                            let source_ml_source2_s17480 = match entry.clone() {
                                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(_, source_ml_source2_data_property_value17490) => {
                                                        let source_ml_source2_data_property_value17490 = *source_ml_source2_data_property_value17490;
                                                        source_ml_source2_data_property_value17490.clone()
                                                    }
                                                };
                                            match source_ml_source2_s17480 {
                                                self::JavaScriptSemanticValue::Undefined => false,
                                                _ => true,
                                            }
                                        } {
                                            self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 32, 119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 100, 111, 32, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 77, 97, 112, 32, 118, 97, 108, 117, 101, 32, 101, 100, 103, 101, 115]))
                                        } else {
                                            {
                                                let source_ml_source1_temporary_read_java_script_weak_collection_object_error_written61 = {
                                                    let ml_field_owner273 = budget.clone();
                                                    let ml_field_previous273 =
                                                        ml_field_owner273.borrow().3;
                                                    let ml_field_value273 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                                        self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(_, source_ml_source2_data_property_value17491) => {
                                                            let source_ml_source2_data_property_value17491 = *source_ml_source2_data_property_value17491;
                                                            source_ml_source2_data_property_value17491.clone()
                                                        }
                                                    }));
                                                    ml_field_owner273.borrow_mut().3 =
                                                        ml_field_previous273 + ml_field_value273;
                                                    ()
                                                };
                                                if (({ budget.borrow().clone() }).3) > 67_108_864f64
                                                {
                                                    self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 97, 110, 100, 32, 115, 116, 114, 111, 110, 103, 32, 118, 97, 108, 117, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                } else {
                                                    {
                                                        let source_ml_source1_temporary_index57_2 =
                                                            source_ml_source1_temporary_index57
                                                                .add(&self::ml::Big::from_i128(1));
                                                        {
                                                            (previous_weak_key, source_ml_source1_temporary_index57) = (previous_weak_key_2, source_ml_source1_temporary_index57_2);
                                                            continue;
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
            self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63 {
    MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Next,
    MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result {
    MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Done,
    MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_strong_collection_object_error_loop66(
    mut ordered_keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result {
    loop {
        return if at
            < ({
                let ml_reference_array275 = ordered_keys.clone();
                ml_reference_array275.borrow().len() as f64
            }) {
            {
                let prior = {
                    let source_ml_source2_array_read_index17493 = at - 1f64;
                    {
                        let ml_reference_array276 = ordered_keys.clone();
                        let ml_reference_position276 = source_ml_source2_array_read_index17493;
                        self::ml_array::optional_at(
                            &*ml_reference_array276.borrow(),
                            self::ml_array::number_index(ml_reference_position276),
                        )
                    }
                };
                {
                    let current = {
                        let ml_reference_array277 = ordered_keys.clone();
                        let ml_reference_position277 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array277.borrow(),
                            self::ml_array::number_index(ml_reference_position277),
                        )
                    };
                    if (!(match (prior.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) && !(match (current.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    })) && self::test_java_script_collection_key(
                        prior.clone().expect("a proven present value"),
                        current.clone().expect("a proven present value"),
                    ) {
                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Return(self::ml_string::Text::from_units(vec![97, 99, 116, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 117, 110, 100, 101, 114, 32, 83, 97, 109, 101, 86, 97, 108, 117, 101, 90, 101, 114, 111]))
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
        } else {
            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result {
    MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Done,
    MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68 {
    MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next,
    MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_strong_collection_object_error_loop65(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
    mut keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_source1_temporary_values62: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
    >,
    mut source_ml_source1_temporary_index62: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result {
    loop {
        return if source_ml_source1_temporary_index62
            < ({
                let ml_reference_array278 = source_ml_source1_temporary_values62.clone();
                self::ml::Big::from_u128(ml_reference_array278.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array279 = source_ml_source1_temporary_values62.clone();
                    let ml_reference_position279 = source_ml_source1_temporary_index62.clone();
                    self::ml_array::at(
                        &*ml_reference_array279.borrow(),
                        ml_reference_position279.to_index(),
                    )
                };
                if (match (entry.clone(), None::<()>) {
                    (_ml_left, Some(_ml_right)) => false,
                    _ => false,
                } || match entry.clone() {
                    self::JavaScriptCollectionEntry::CollectionEntry(_, _, _) => false,
                }) || (({
                    let source_ml_source2_type_query_operand17504 = match entry.clone() {
                        self::JavaScriptCollectionEntry::CollectionEntry(
                            _,
                            _,
                            source_ml_source2_data_property_value17503,
                        ) => source_ml_source2_data_property_value17503,
                    };
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                }) != (self::ml_string::Text::from_units(vec![
                    98, 111, 111, 108, 101, 97, 110,
                ]))) {
                    self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 100, 101, 110, 115, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 101, 110, 116, 114, 121, 32, 114, 101, 99, 111, 114, 100, 115]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match entry.clone() {
                                self::JavaScriptCollectionEntry::CollectionEntry(
                                    source_ml_source2_data_property_value17505,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value17505 =
                                        *source_ml_source2_data_property_value17505;
                                    source_ml_source2_data_property_value17505.clone()
                                }
                            },
                        );
                        {
                            let value_error = self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                match entry.clone() {
                                    self::JavaScriptCollectionEntry::CollectionEntry(
                                        _,
                                        source_ml_source2_data_property_value17506,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value17506 =
                                            *source_ml_source2_data_property_value17506;
                                        source_ml_source2_data_property_value17506.clone()
                                    }
                                },
                            );
                            if (key_error != (self::ml_string::Text::from_units(vec![])))
                                || (value_error != (self::ml_string::Text::from_units(vec![])))
                            {
                                self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return({
let ml_library_inequality_1 = key_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                    key_error.clone()
                                } else {
                                    value_error.clone()
                                }
})
                            } else {
                                {
                                    let source_ml_source1_temporary_read_java_script_strong_collection_object_error_written67 = {
                                        let ml_field_owner280 = budget.clone();
                                        let ml_field_previous280 = ml_field_owner280.borrow().3;
                                        let ml_field_value280 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                        self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value17507, _, _) => {
                                            let source_ml_source2_data_property_value17507 = *source_ml_source2_data_property_value17507;
                                            source_ml_source2_data_property_value17507.clone()
                                        }
                                    })) + self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                        self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source2_data_property_value17508, _) => {
                                            let source_ml_source2_data_property_value17508 = *source_ml_source2_data_property_value17508;
                                            source_ml_source2_data_property_value17508.clone()
                                        }
                                    }));
                                        ml_field_owner280.borrow_mut().3 =
                                            ml_field_previous280 + ml_field_value280;
                                        ()
                                    };
                                    if (({ budget.borrow().clone() }).3) > 67_108_864f64 {
                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                    } else {
                                        {
                                            let key = match entry.clone() {
                                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value17509, _, _) => {
                                                    let source_ml_source2_data_property_value17509 = *source_ml_source2_data_property_value17509;
                                                    source_ml_source2_data_property_value17509.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source2_s17520 = match key {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Number(value) => {
                                                        if {
                                                            let source_ml_source2_same_value_operand17530 = -0f64;
                                                            (((value == source_ml_source2_same_value_operand17530) && ((value != 0f64) || ((1f64 / value) == (1f64 / source_ml_source2_same_value_operand17530)))) || ((value != value) && (source_ml_source2_same_value_operand17530 != source_ml_source2_same_value_operand17530)))
                                                        } {
                                                            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 111, 114, 101, 100, 32, 107, 101, 121, 115, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 32, 110, 101, 103, 97, 116, 105, 118, 101, 32, 78, 117, 109, 98, 101, 114, 32, 122, 101, 114, 111]))
                                                        } else {
                                                            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                };
                                                match source_ml_source2_s17520 {
                                                    self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next => {
                                                        if match object.clone() {
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        } && !(self::test_java_script_semantic_same_value(match entry.clone() {
                                                            self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value17626, _, _) => {
                                                                let source_ml_source2_data_property_value17626 = *source_ml_source2_data_property_value17626;
                                                                source_ml_source2_data_property_value17626.clone()
                                                            }
                                                        }, match entry.clone() {
                                                            self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source2_data_property_value17627, _) => {
                                                                let source_ml_source2_data_property_value17627 = *source_ml_source2_data_property_value17627;
                                                                source_ml_source2_data_property_value17627.clone()
                                                            }
                                                        })) {
                                                            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![83, 101, 116, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 107, 101, 121, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101]))
                                                        } else if match entry.clone() {
                                                                self::JavaScriptCollectionEntry::CollectionEntry(_, _, source_ml_source2_data_property_value17628) => {
                                                                    source_ml_source2_data_property_value17628
                                                                }
                                                            } {
                                                                {
                                                                    let source_ml_source1_temporary_read_java_script_strong_collection_object_error_written69 = { let ml_field_owner282 = budget.clone(); let ml_field_previous282 = ml_field_owner282.borrow().2; let ml_field_value282 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                                                        self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value17629, _, _) => {
                                                                            let source_ml_source2_data_property_value17629 = *source_ml_source2_data_property_value17629;
                                                                            source_ml_source2_data_property_value17629.clone()
                                                                        }
                                                                    })); ml_field_owner282.borrow_mut().2 = ml_field_previous282 + ml_field_value282; () };
                                                                    if (({ budget.borrow().clone() }).2) > 16_777_216f64 {
                                                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![97, 99, 116, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        {
                                                                            let source_ml_source1_temporary_read_java_script_strong_collection_object_error_ignored70 = { let ml_method_owner284 = keys.clone(); let ml_method_argument284_0 = match entry.clone() {
                                                                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source2_data_property_value17630, _, _) => {
                                                                                    let source_ml_source2_data_property_value17630 = *source_ml_source2_data_property_value17630;
                                                                                    source_ml_source2_data_property_value17630.clone()
                                                                                }
                                                                            }; let ml_inserted = vec![ml_method_argument284_0]; let mut ml_method_values284 = ml_method_owner284.borrow_mut(); let ml_length = ml_method_values284.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values284.extend(ml_inserted); ml_length as f64 };
                                                                            {
                                                                                let source_ml_source1_temporary_index62_2 = source_ml_source1_temporary_index62.add(&self::ml::Big::from_i128(1));
                                                                                {
                                                                                    source_ml_source1_temporary_index62 = source_ml_source1_temporary_index62_2;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index62_3 = source_ml_source1_temporary_index62.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_source1_temporary_index62 = source_ml_source1_temporary_index62_3;
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                    }
                                                    self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Return(source_ml_source1_temporary_result70) => {
                                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(source_ml_source1_temporary_result70.clone())
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
            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73 {
    MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Next,
    MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72 {
    MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Next,
    MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71 {
    MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Next,
    MlSource1TemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result {
    MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Done,
    MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(
        self::ml_string::Text,
    ),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_object_descriptor_error_loop75(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut at: f64,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result {
    loop {
        return if at
            < ({
                let ml_reference_array285 = properties.clone();
                ml_reference_array285.borrow().len() as f64
            }) {
            {
                let property = {
                    let ml_reference_array286 = properties.clone();
                    let ml_reference_position286 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array286.borrow(),
                        self::ml_array::number_index(ml_reference_position286),
                    )
                };
                if match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_key_error(
                            heap.clone(),
                            match property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    source_ml_source2_data_property_value17633,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value17633 =
                                        *source_ml_source2_data_property_value17633;
                                    source_ml_source2_data_property_value17633.clone()
                                }
                            },
                        );
                        {
                            let ml_library_inequality_2 =
                                key_error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_2 {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(key_error.clone())
                            } else if (self::find_java_script_ordinary_property(
                                properties.clone(),
                                match property.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        source_ml_source2_data_property_value17635,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value17635 =
                                            *source_ml_source2_data_property_value17635;
                                        source_ml_source2_data_property_value17635.clone()
                                    }
                                },
                            )) != at
                            {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                            } else {
                                {
                                    let descriptor_error = self::read_java_script_ordinary_descriptor_value_error(heap.clone(), match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value17636) => {
                                            source_ml_source2_data_property_value17636.clone()
                                        }
                                    });
                                    {
                                        let ml_library_inequality_1 = descriptor_error
                                            != (self::ml_string::Text::from_units(vec![]));
                                        if ml_library_inequality_1 {
                                            self::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(descriptor_error.clone())
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
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result {
    MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done,
    MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(self::ml_string::Text),
}
