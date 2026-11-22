// Generated from complete js/src/translation/javascript-collection-iteration.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptWeakCollectionEntry;
use super::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result;
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
pub fn source_ml_temporary_read_java_script_closure_object_error_loop49(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values46: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index46: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result {
    loop {
        return if source_ml_temporary_index46
            < ({
                let ml_reference_array121 = source_ml_temporary_values46.clone();
                self::ml::Big::from_u128(ml_reference_array121.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array122 = source_ml_temporary_values46.clone();
                    let ml_reference_position122 = source_ml_temporary_index46.clone();
                    self::ml_array::at(
                        &*ml_reference_array122.borrow(),
                        ml_reference_position122.to_index(),
                    )
                };
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if error == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_temporary_index46_2 =
                                source_ml_temporary_index46.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index46 = source_ml_temporary_index46_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Return(error.clone())
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop49Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptBoundObjectErrorJoin51 {
    MlTemporaryReadJavaScriptBoundObjectErrorJoin51Next,
    MlTemporaryReadJavaScriptBoundObjectErrorJoin51Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptBoundObjectErrorLoop53Result {
    MlTemporaryReadJavaScriptBoundObjectErrorLoop53Done,
    MlTemporaryReadJavaScriptBoundObjectErrorLoop53Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_bound_object_error_loop53(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values50: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index50: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Result {
    loop {
        return if source_ml_temporary_index50
            < ({
                let ml_reference_array123 = source_ml_temporary_values50.clone();
                self::ml::Big::from_u128(ml_reference_array123.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array124 = source_ml_temporary_values50.clone();
                    let ml_reference_position124 = source_ml_temporary_index50.clone();
                    self::ml_array::at(
                        &*ml_reference_array124.borrow(),
                        ml_reference_position124.to_index(),
                    )
                };
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if error == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_temporary_index50_2 =
                                source_ml_temporary_index50.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index50 = source_ml_temporary_index50_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Return(error.clone())
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop53Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54 {
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Next,
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin54Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55 {
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Next,
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin55Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58 {
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Next,
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin58Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result {
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Done,
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_weak_collection_object_error_loop60(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
    mut previous_weak_key: Option<self::ml::Big>,
    mut source_ml_temporary_values57: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
    >,
    mut source_ml_temporary_index57: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result {
    loop {
        return if source_ml_temporary_index57
            < ({
                let ml_reference_array125 = source_ml_temporary_values57.clone();
                self::ml::Big::from_u128(ml_reference_array125.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array126 = source_ml_temporary_values57.clone();
                    let ml_reference_position126 = source_ml_temporary_index57.clone();
                    self::ml_array::at(
                        &*ml_reference_array126.borrow(),
                        ml_reference_position126.to_index(),
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
                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 101, 110, 115, 101, 32, 97, 99, 116, 117, 97, 108, 32, 101, 110, 116, 114, 121, 32, 114, 101, 99, 111, 114, 100, 115]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match entry.clone() {
                                self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                    source_ml_source1_data_property_value18654,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value18654 =
                                        *source_ml_source1_data_property_value18654;
                                    source_ml_source1_data_property_value18654.clone()
                                }
                            },
                        );
                        {
                            let value_error = self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                match entry.clone() {
                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                        _,
                                        source_ml_source1_data_property_value18655,
                                    ) => {
                                        let source_ml_source1_data_property_value18655 =
                                            *source_ml_source1_data_property_value18655;
                                        source_ml_source1_data_property_value18655.clone()
                                    }
                                },
                            );
                            if (key_error != (self::ml_string::Text::from_units(vec![])))
                                || (value_error != (self::ml_string::Text::from_units(vec![])))
                            {
                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(if key_error.0.is_empty() {
                                    value_error.clone()
                                } else {
                                    key_error.clone()
                                })
                            } else if (self::read_java_script_weak_key_eligibility(
                                heap.clone(),
                                match entry.clone() {
                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                        source_ml_source1_data_property_value18658,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value18658 =
                                            *source_ml_source1_data_property_value18658;
                                        source_ml_source1_data_property_value18658.clone()
                                    }
                                },
                            )) != (self::ml_string::Text::from_units(vec![
                                101, 108, 105, 103, 105, 98, 108, 101,
                            ])) {
                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 101, 108, 105, 103, 105, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 111, 114, 32, 110, 111, 110, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 115]))
                            } else {
                                if ({
                                    let source_ml_source1_s18660 = match entry.clone() {
                                            self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source1_data_property_value18670, _) => {
                                                let source_ml_source1_data_property_value18670 = *source_ml_source1_data_property_value18670;
                                                source_ml_source1_data_property_value18670.clone()
                                            }
                                        };
                                    match source_ml_source1_s18660 {
                                        self::JavaScriptSemanticValue::Object(_, _) => false,
                                        _ => true,
                                    }
                                } && {
                                    let source_ml_source1_s18672 = match entry.clone() {
                                            self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source1_data_property_value18682, _) => {
                                                let source_ml_source1_data_property_value18682 = *source_ml_source1_data_property_value18682;
                                                source_ml_source1_data_property_value18682.clone()
                                            }
                                        };
                                    match source_ml_source1_s18672 {
                                        self::JavaScriptSemanticValue::Symbol(_) => false,
                                        _ => true,
                                    }
                                }) || {
                                    let source_ml_source1_comparison_operand18686 = match match entry.clone() {
                                            self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source1_data_property_value18683, _) => {
                                                let source_ml_source1_data_property_value18683 = *source_ml_source1_data_property_value18683;
                                                source_ml_source1_data_property_value18683.clone()
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
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_data_property_value18684) => {
                                                Some(source_ml_source1_data_property_value18684.clone())
                                            }
                                            self::JavaScriptSemanticValue::Object(source_ml_source1_data_property_value18685, _) => {
                                                Some(source_ml_source1_data_property_value18685.clone())
                                            }
                                        };
                                    if match (
                                        source_ml_source1_comparison_operand18686.clone(),
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
                                                let source_ml_source1_comparison_order18689 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,previous_weak_key.clone().expect("a proven present value"));
                                                ((source_ml_source1_comparison_order18689 == -1f64)
                                                    || (source_ml_source1_comparison_order18689
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
                                            let source_ml_source1_comparison_order18688 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source1_comparison_operand18686.clone().expect("a proven present value"));
                                            ((source_ml_source1_comparison_order18688 == 1f64)
                                                || (source_ml_source1_comparison_order18688
                                                    == 0f64))
                                        }
                                    } else {
                                        (source_ml_source1_comparison_operand18686
                                            .clone()
                                            .expect("a proven present value"))
                                            <= (previous_weak_key
                                                .clone()
                                                .expect("a proven present value"))
                                    }
                                } {
                                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 107, 101, 121, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_weak_key_2 = match match entry.clone() {
                                                self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source1_data_property_value18690, _) => {
                                                    let source_ml_source1_data_property_value18690 = *source_ml_source1_data_property_value18690;
                                                    source_ml_source1_data_property_value18690.clone()
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
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_data_property_value18691) => {
                                                    Some(source_ml_source1_data_property_value18691.clone())
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source1_data_property_value18692, _) => {
                                                    Some(source_ml_source1_data_property_value18692.clone())
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
                                            let source_ml_source1_s18789 = match entry.clone() {
                                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(_, source_ml_source1_data_property_value18799) => {
                                                        let source_ml_source1_data_property_value18799 = *source_ml_source1_data_property_value18799;
                                                        source_ml_source1_data_property_value18799.clone()
                                                    }
                                                };
                                            match source_ml_source1_s18789 {
                                                self::JavaScriptSemanticValue::Undefined => false,
                                                _ => true,
                                            }
                                        } {
                                            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 32, 119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 100, 111, 32, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 77, 97, 112, 32, 118, 97, 108, 117, 101, 32, 101, 100, 103, 101, 115]))
                                        } else {
                                            {
                                                let source_ml_temporary_read_java_script_weak_collection_object_error_written61 = {
                                                    let ml_field_owner127 = budget.clone();
                                                    let ml_field_previous127 =
                                                        ml_field_owner127.borrow().3;
                                                    let ml_field_value127 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                                        self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(_, source_ml_source1_data_property_value18800) => {
                                                            let source_ml_source1_data_property_value18800 = *source_ml_source1_data_property_value18800;
                                                            source_ml_source1_data_property_value18800.clone()
                                                        }
                                                    }));
                                                    ml_field_owner127.borrow_mut().3 =
                                                        ml_field_previous127 + ml_field_value127;
                                                    ()
                                                };
                                                if (({ budget.borrow().clone() }).3) > 67_108_864f64
                                                {
                                                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 97, 110, 100, 32, 115, 116, 114, 111, 110, 103, 32, 118, 97, 108, 117, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                } else {
                                                    {
                                                        let source_ml_temporary_index57_2 =
                                                            source_ml_temporary_index57
                                                                .add(&self::ml::Big::from_i128(1));
                                                        {
                                                            (
                                                                previous_weak_key,
                                                                source_ml_temporary_index57,
                                                            ) = (
                                                                previous_weak_key_2,
                                                                source_ml_temporary_index57_2,
                                                            );
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
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63 {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Next,
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin63Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Done,
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_strong_collection_object_error_loop66(
    mut ordered_keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result {
    loop {
        return if at
            < ({
                let ml_reference_array129 = ordered_keys.clone();
                ml_reference_array129.borrow().len() as f64
            }) {
            {
                let prior = {
                    let source_ml_source1_array_read_index18802 = at - 1f64;
                    {
                        let ml_reference_array130 = ordered_keys.clone();
                        let ml_reference_position130 = source_ml_source1_array_read_index18802;
                        self::ml_array::optional_at(
                            &*ml_reference_array130.borrow(),
                            self::ml_array::number_index(ml_reference_position130),
                        )
                    }
                };
                {
                    let current = {
                        let ml_reference_array131 = ordered_keys.clone();
                        let ml_reference_position131 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array131.borrow(),
                            self::ml_array::number_index(ml_reference_position131),
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
                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Return(self::ml_string::Text::from_units(vec![97, 99, 116, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 117, 110, 100, 101, 114, 32, 83, 97, 109, 101, 86, 97, 108, 117, 101, 90, 101, 114, 111]))
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
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Done,
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68 {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next,
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_strong_collection_object_error_loop65(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
    mut keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_temporary_values62: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
    >,
    mut source_ml_temporary_index62: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result {
    loop {
        return if source_ml_temporary_index62
            < ({
                let ml_reference_array132 = source_ml_temporary_values62.clone();
                self::ml::Big::from_u128(ml_reference_array132.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array133 = source_ml_temporary_values62.clone();
                    let ml_reference_position133 = source_ml_temporary_index62.clone();
                    self::ml_array::at(
                        &*ml_reference_array133.borrow(),
                        ml_reference_position133.to_index(),
                    )
                };
                if (match (entry.clone(), None::<()>) {
                    (_ml_left, Some(_ml_right)) => false,
                    _ => false,
                } || match entry.clone() {
                    self::JavaScriptCollectionEntry::CollectionEntry(_, _, _) => false,
                }) || (({
                    let source_ml_source1_type_query_operand18813 = match entry.clone() {
                        self::JavaScriptCollectionEntry::CollectionEntry(
                            _,
                            _,
                            source_ml_source1_data_property_value18812,
                        ) => source_ml_source1_data_property_value18812,
                    };
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                }) != (self::ml_string::Text::from_units(vec![
                    98, 111, 111, 108, 101, 97, 110,
                ]))) {
                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 100, 101, 110, 115, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 101, 110, 116, 114, 121, 32, 114, 101, 99, 111, 114, 100, 115]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match entry.clone() {
                                self::JavaScriptCollectionEntry::CollectionEntry(
                                    source_ml_source1_data_property_value18814,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value18814 =
                                        *source_ml_source1_data_property_value18814;
                                    source_ml_source1_data_property_value18814.clone()
                                }
                            },
                        );
                        {
                            let value_error = self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                match entry.clone() {
                                    self::JavaScriptCollectionEntry::CollectionEntry(
                                        _,
                                        source_ml_source1_data_property_value18815,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value18815 =
                                            *source_ml_source1_data_property_value18815;
                                        source_ml_source1_data_property_value18815.clone()
                                    }
                                },
                            );
                            if (key_error != (self::ml_string::Text::from_units(vec![])))
                                || (value_error != (self::ml_string::Text::from_units(vec![])))
                            {
                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return({
let ml_library_inequality_1 = key_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                    key_error.clone()
                                } else {
                                    value_error.clone()
                                }
})
                            } else {
                                {
                                    let source_ml_temporary_read_java_script_strong_collection_object_error_written67 = {
                                        let ml_field_owner134 = budget.clone();
                                        let ml_field_previous134 = ml_field_owner134.borrow().3;
                                        let ml_field_value134 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                        self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value18816, _, _) => {
                                            let source_ml_source1_data_property_value18816 = *source_ml_source1_data_property_value18816;
                                            source_ml_source1_data_property_value18816.clone()
                                        }
                                    })) + self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                        self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source1_data_property_value18817, _) => {
                                            let source_ml_source1_data_property_value18817 = *source_ml_source1_data_property_value18817;
                                            source_ml_source1_data_property_value18817.clone()
                                        }
                                    }));
                                        ml_field_owner134.borrow_mut().3 =
                                            ml_field_previous134 + ml_field_value134;
                                        ()
                                    };
                                    if (({ budget.borrow().clone() }).3) > 67_108_864f64 {
                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                    } else {
                                        {
                                            let key = match entry.clone() {
                                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value18818, _, _) => {
                                                    let source_ml_source1_data_property_value18818 = *source_ml_source1_data_property_value18818;
                                                    source_ml_source1_data_property_value18818.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_s18829 = match key {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Number(value) => {
                                                        if {
                                                            let source_ml_source1_same_value_operand18839 = -0f64;
                                                            (((value == source_ml_source1_same_value_operand18839) && ((value != 0f64) || ((1f64 / value) == (1f64 / source_ml_source1_same_value_operand18839)))) || ((value != value) && (source_ml_source1_same_value_operand18839 != source_ml_source1_same_value_operand18839)))
                                                        } {
                                                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 111, 114, 101, 100, 32, 107, 101, 121, 115, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 32, 110, 101, 103, 97, 116, 105, 118, 101, 32, 78, 117, 109, 98, 101, 114, 32, 122, 101, 114, 111]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                    }
                                                };
                                                match source_ml_source1_s18829 {
                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next => {
                                                        if match object.clone() {
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        } && !(self::test_java_script_semantic_same_value(match entry.clone() {
                                                            self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value18935, _, _) => {
                                                                let source_ml_source1_data_property_value18935 = *source_ml_source1_data_property_value18935;
                                                                source_ml_source1_data_property_value18935.clone()
                                                            }
                                                        }, match entry.clone() {
                                                            self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source1_data_property_value18936, _) => {
                                                                let source_ml_source1_data_property_value18936 = *source_ml_source1_data_property_value18936;
                                                                source_ml_source1_data_property_value18936.clone()
                                                            }
                                                        })) {
                                                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![83, 101, 116, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 107, 101, 121, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101]))
                                                        } else if match entry.clone() {
                                                                self::JavaScriptCollectionEntry::CollectionEntry(_, _, source_ml_source1_data_property_value18937) => {
                                                                    source_ml_source1_data_property_value18937
                                                                }
                                                            } {
                                                                {
                                                                    let source_ml_temporary_read_java_script_strong_collection_object_error_written69 = { let ml_field_owner136 = budget.clone(); let ml_field_previous136 = ml_field_owner136.borrow().2; let ml_field_value136 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                                                        self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value18938, _, _) => {
                                                                            let source_ml_source1_data_property_value18938 = *source_ml_source1_data_property_value18938;
                                                                            source_ml_source1_data_property_value18938.clone()
                                                                        }
                                                                    })); ml_field_owner136.borrow_mut().2 = ml_field_previous136 + ml_field_value136; () };
                                                                    if (({ budget.borrow().clone() }).2) > 16_777_216f64 {
                                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![97, 99, 116, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        {
                                                                            let source_ml_temporary_read_java_script_strong_collection_object_error_ignored70 = { let ml_method_owner138 = keys.clone(); let ml_method_argument138_0 = match entry.clone() {
                                                                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value18939, _, _) => {
                                                                                    let source_ml_source1_data_property_value18939 = *source_ml_source1_data_property_value18939;
                                                                                    source_ml_source1_data_property_value18939.clone()
                                                                                }
                                                                            }; let ml_inserted = vec![ml_method_argument138_0]; let mut ml_method_values138 = ml_method_owner138.borrow_mut(); let ml_length = ml_method_values138.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values138.extend(ml_inserted); ml_length as f64 };
                                                                            {
                                                                                let source_ml_temporary_index62_2 = source_ml_temporary_index62.add(&self::ml::Big::from_i128(1));
                                                                                {
                                                                                    source_ml_temporary_index62 = source_ml_temporary_index62_2;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let source_ml_temporary_index62_3 = source_ml_temporary_index62.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_temporary_index62 = source_ml_temporary_index62_3;
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                    }
                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Return(source_ml_temporary_result70) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(source_ml_temporary_result70.clone())
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
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73 {
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Next,
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin73Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72 {
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Next,
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin72Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71 {
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Next,
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin71Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result {
    MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Done,
    MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_object_descriptor_error_loop75(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result {
    loop {
        return if at
            < ({
                let ml_reference_array139 = properties.clone();
                ml_reference_array139.borrow().len() as f64
            }) {
            {
                let property = {
                    let ml_reference_array140 = properties.clone();
                    let ml_reference_position140 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array140.borrow(),
                        self::ml_array::number_index(ml_reference_position140),
                    )
                };
                if match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_key_error(
                            heap.clone(),
                            match property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    source_ml_source1_data_property_value18942,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value18942 =
                                        *source_ml_source1_data_property_value18942;
                                    source_ml_source1_data_property_value18942.clone()
                                }
                            },
                        );
                        {
                            let ml_library_inequality_2 =
                                key_error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_2 {
                                self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(key_error.clone())
                            } else if (self::find_java_script_ordinary_property(
                                properties.clone(),
                                match property.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        source_ml_source1_data_property_value18944,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value18944 =
                                            *source_ml_source1_data_property_value18944;
                                        source_ml_source1_data_property_value18944.clone()
                                    }
                                },
                            )) != at
                            {
                                self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                            } else {
                                {
                                    let descriptor_error = self::read_java_script_ordinary_descriptor_value_error(heap.clone(), match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value18945) => {
                                            source_ml_source1_data_property_value18945.clone()
                                        }
                                    });
                                    {
                                        let ml_library_inequality_1 = descriptor_error
                                            != (self::ml_string::Text::from_units(vec![]));
                                        if ml_library_inequality_1 {
                                            self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(descriptor_error.clone())
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
            self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result {
    MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done,
    MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(self::ml_string::Text),
}
