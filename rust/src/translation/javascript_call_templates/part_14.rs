// Generated from complete js/src/translation/javascript-call-templates.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptWeakCollectionEntry;
use super::MlTemporaryReadJavaScriptClosureObjectErrorLoop60Result;
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
pub fn source_ml_temporary_read_java_script_closure_object_error_loop60(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values57: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index57: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptClosureObjectErrorLoop60Result {
    loop {
        return if source_ml_temporary_index57
            < ({
                let ml_reference_array123 = source_ml_temporary_values57.clone();
                self::ml::Big::from_u128(ml_reference_array123.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array124 = source_ml_temporary_values57.clone();
                    let ml_reference_position124 = source_ml_temporary_index57.clone();
                    self::ml_array::at(
                        &*ml_reference_array124.borrow(),
                        ml_reference_position124.to_index(),
                    )
                };
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if error == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_temporary_index57_2 =
                                source_ml_temporary_index57.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index57 = source_ml_temporary_index57_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadJavaScriptClosureObjectErrorLoop60Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop60Return(error.clone())
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosureObjectErrorLoop60Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop60Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptBoundObjectErrorJoin62 {
    MlTemporaryReadJavaScriptBoundObjectErrorJoin62Next,
    MlTemporaryReadJavaScriptBoundObjectErrorJoin62Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptBoundObjectErrorLoop64Result {
    MlTemporaryReadJavaScriptBoundObjectErrorLoop64Done,
    MlTemporaryReadJavaScriptBoundObjectErrorLoop64Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_bound_object_error_loop64(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut source_ml_temporary_values61: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>,
    >,
    mut source_ml_temporary_index61: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptBoundObjectErrorLoop64Result {
    loop {
        return if source_ml_temporary_index61
            < ({
                let ml_reference_array125 = source_ml_temporary_values61.clone();
                self::ml::Big::from_u128(ml_reference_array125.borrow().len() as u128)
            }) {
            {
                let value = {
                    let ml_reference_array126 = source_ml_temporary_values61.clone();
                    let ml_reference_position126 = source_ml_temporary_index61.clone();
                    self::ml_array::at(
                        &*ml_reference_array126.borrow(),
                        ml_reference_position126.to_index(),
                    )
                };
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), value);
                    if error == (self::ml_string::Text::from_units(vec![])) {
                        {
                            let source_ml_temporary_index61_2 =
                                source_ml_temporary_index61.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index61 = source_ml_temporary_index61_2;
                                continue;
                            }
                        }
                    }
                    {
                        self::MlTemporaryReadJavaScriptBoundObjectErrorLoop64Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop64Return(error.clone())
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptBoundObjectErrorLoop64Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop64Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65 {
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Next,
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin65Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66 {
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Next,
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin66Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69 {
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69Next,
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin69Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result {
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Done,
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_weak_collection_object_error_loop71(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
    mut previous_weak_key: Option<self::ml::Big>,
    mut source_ml_temporary_values68: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
    >,
    mut source_ml_temporary_index68: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result {
    loop {
        return if source_ml_temporary_index68
            < ({
                let ml_reference_array127 = source_ml_temporary_values68.clone();
                self::ml::Big::from_u128(ml_reference_array127.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array128 = source_ml_temporary_values68.clone();
                    let ml_reference_position128 = source_ml_temporary_index68.clone();
                    self::ml_array::at(
                        &*ml_reference_array128.borrow(),
                        ml_reference_position128.to_index(),
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
                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 101, 110, 115, 101, 32, 97, 99, 116, 117, 97, 108, 32, 101, 110, 116, 114, 121, 32, 114, 101, 99, 111, 114, 100, 115]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match entry.clone() {
                                self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                    source_ml_source1_data_property_value17946,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value17946 =
                                        *source_ml_source1_data_property_value17946;
                                    source_ml_source1_data_property_value17946.clone()
                                }
                            },
                        );
                        {
                            let value_error = self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                match entry.clone() {
                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                        _,
                                        source_ml_source1_data_property_value17947,
                                    ) => {
                                        let source_ml_source1_data_property_value17947 =
                                            *source_ml_source1_data_property_value17947;
                                        source_ml_source1_data_property_value17947.clone()
                                    }
                                },
                            );
                            if (key_error != (self::ml_string::Text::from_units(vec![])))
                                || (value_error != (self::ml_string::Text::from_units(vec![])))
                            {
                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Return(if key_error.0.is_empty() {
                                    value_error.clone()
                                } else {
                                    key_error.clone()
                                })
                            } else if (self::read_java_script_weak_key_eligibility(
                                heap.clone(),
                                match entry.clone() {
                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                                        source_ml_source1_data_property_value17950,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value17950 =
                                            *source_ml_source1_data_property_value17950;
                                        source_ml_source1_data_property_value17950.clone()
                                    }
                                },
                            )) != (self::ml_string::Text::from_units(vec![
                                101, 108, 105, 103, 105, 98, 108, 101,
                            ])) {
                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 101, 108, 105, 103, 105, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 111, 114, 32, 110, 111, 110, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 115]))
                            } else {
                                if ({
                                    let source_ml_source1_s17952 = match entry.clone() {
                                            self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source1_data_property_value17962, _) => {
                                                let source_ml_source1_data_property_value17962 = *source_ml_source1_data_property_value17962;
                                                source_ml_source1_data_property_value17962.clone()
                                            }
                                        };
                                    match source_ml_source1_s17952 {
                                        self::JavaScriptSemanticValue::Object(_, _) => false,
                                        _ => true,
                                    }
                                } && {
                                    let source_ml_source1_s17964 = match entry.clone() {
                                            self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source1_data_property_value17974, _) => {
                                                let source_ml_source1_data_property_value17974 = *source_ml_source1_data_property_value17974;
                                                source_ml_source1_data_property_value17974.clone()
                                            }
                                        };
                                    match source_ml_source1_s17964 {
                                        self::JavaScriptSemanticValue::Symbol(_) => false,
                                        _ => true,
                                    }
                                }) || {
                                    let source_ml_source1_comparison_operand17978 = match match entry.clone() {
                                            self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source1_data_property_value17975, _) => {
                                                let source_ml_source1_data_property_value17975 = *source_ml_source1_data_property_value17975;
                                                source_ml_source1_data_property_value17975.clone()
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
                                            self::JavaScriptSemanticValue::Symbol(source_ml_source1_data_property_value17976) => {
                                                Some(source_ml_source1_data_property_value17976.clone())
                                            }
                                            self::JavaScriptSemanticValue::Object(source_ml_source1_data_property_value17977, _) => {
                                                Some(source_ml_source1_data_property_value17977.clone())
                                            }
                                        };
                                    if match (
                                        source_ml_source1_comparison_operand17978.clone(),
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
                                                let source_ml_source1_comparison_order17981 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,previous_weak_key.clone().expect("a proven present value"));
                                                ((source_ml_source1_comparison_order17981 == -1f64)
                                                    || (source_ml_source1_comparison_order17981
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
                                            let source_ml_source1_comparison_order17980 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source1_comparison_operand17978.clone().expect("a proven present value"));
                                            ((source_ml_source1_comparison_order17980 == 1f64)
                                                || (source_ml_source1_comparison_order17980
                                                    == 0f64))
                                        }
                                    } else {
                                        (source_ml_source1_comparison_operand17978
                                            .clone()
                                            .expect("a proven present value"))
                                            <= (previous_weak_key
                                                .clone()
                                                .expect("a proven present value"))
                                    }
                                } {
                                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 107, 101, 121, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                } else {
                                    {
                                        let previous_weak_key_2 = match match entry.clone() {
                                                self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(source_ml_source1_data_property_value17982, _) => {
                                                    let source_ml_source1_data_property_value17982 = *source_ml_source1_data_property_value17982;
                                                    source_ml_source1_data_property_value17982.clone()
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
                                                self::JavaScriptSemanticValue::Symbol(source_ml_source1_data_property_value17983) => {
                                                    Some(source_ml_source1_data_property_value17983.clone())
                                                }
                                                self::JavaScriptSemanticValue::Object(source_ml_source1_data_property_value17984, _) => {
                                                    Some(source_ml_source1_data_property_value17984.clone())
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
                                            let source_ml_source1_s18081 = match entry.clone() {
                                                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(_, source_ml_source1_data_property_value18091) => {
                                                        let source_ml_source1_data_property_value18091 = *source_ml_source1_data_property_value18091;
                                                        source_ml_source1_data_property_value18091.clone()
                                                    }
                                                };
                                            match source_ml_source1_s18081 {
                                                self::JavaScriptSemanticValue::Undefined => false,
                                                _ => true,
                                            }
                                        } {
                                            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Return(self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 32, 119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 100, 111, 32, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 77, 97, 112, 32, 118, 97, 108, 117, 101, 32, 101, 100, 103, 101, 115]))
                                        } else {
                                            {
                                                let source_ml_temporary_read_java_script_weak_collection_object_error_written72 = {
                                                    let ml_field_owner129 = budget.clone();
                                                    let ml_field_previous129 =
                                                        ml_field_owner129.borrow().3;
                                                    let ml_field_value129 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                                        self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(_, source_ml_source1_data_property_value18092) => {
                                                            let source_ml_source1_data_property_value18092 = *source_ml_source1_data_property_value18092;
                                                            source_ml_source1_data_property_value18092.clone()
                                                        }
                                                    }));
                                                    ml_field_owner129.borrow_mut().3 =
                                                        ml_field_previous129 + ml_field_value129;
                                                    ()
                                                };
                                                if (({ budget.borrow().clone() }).3) > 67_108_864f64
                                                {
                                                    self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 97, 110, 100, 32, 115, 116, 114, 111, 110, 103, 32, 118, 97, 108, 117, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                } else {
                                                    {
                                                        let source_ml_temporary_index68_2 =
                                                            source_ml_temporary_index68
                                                                .add(&self::ml::Big::from_i128(1));
                                                        {
                                                            (
                                                                previous_weak_key,
                                                                source_ml_temporary_index68,
                                                            ) = (
                                                                previous_weak_key_2,
                                                                source_ml_temporary_index68_2,
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
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop71Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74 {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74Next,
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin74Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Result {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Done,
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_strong_collection_object_error_loop77(
    mut ordered_keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Result {
    loop {
        return if at
            < ({
                let ml_reference_array131 = ordered_keys.clone();
                ml_reference_array131.borrow().len() as f64
            }) {
            {
                let prior = {
                    let source_ml_source1_array_read_index18094 = at - 1f64;
                    {
                        let ml_reference_array132 = ordered_keys.clone();
                        let ml_reference_position132 = source_ml_source1_array_read_index18094;
                        self::ml_array::optional_at(
                            &*ml_reference_array132.borrow(),
                            self::ml_array::number_index(ml_reference_position132),
                        )
                    }
                };
                {
                    let current = {
                        let ml_reference_array133 = ordered_keys.clone();
                        let ml_reference_position133 = at;
                        self::ml_array::optional_at(
                            &*ml_reference_array133.borrow(),
                            self::ml_array::number_index(ml_reference_position133),
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
                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Return(self::ml_string::Text::from_units(vec![97, 99, 116, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 117, 110, 100, 101, 114, 32, 83, 97, 109, 101, 86, 97, 108, 117, 101, 90, 101, 114, 111]))
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
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop77Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Done,
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79 {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Next,
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_strong_collection_object_error_loop76(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>,
    mut keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_temporary_values73: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
    >,
    mut source_ml_temporary_index73: self::ml::Big,
) -> self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result {
    loop {
        return if source_ml_temporary_index73
            < ({
                let ml_reference_array134 = source_ml_temporary_values73.clone();
                self::ml::Big::from_u128(ml_reference_array134.borrow().len() as u128)
            }) {
            {
                let entry = {
                    let ml_reference_array135 = source_ml_temporary_values73.clone();
                    let ml_reference_position135 = source_ml_temporary_index73.clone();
                    self::ml_array::at(
                        &*ml_reference_array135.borrow(),
                        ml_reference_position135.to_index(),
                    )
                };
                if (match (entry.clone(), None::<()>) {
                    (_ml_left, Some(_ml_right)) => false,
                    _ => false,
                } || match entry.clone() {
                    self::JavaScriptCollectionEntry::CollectionEntry(_, _, _) => false,
                }) || (({
                    let source_ml_source1_type_query_operand18105 = match entry.clone() {
                        self::JavaScriptCollectionEntry::CollectionEntry(
                            _,
                            _,
                            source_ml_source1_data_property_value18104,
                        ) => source_ml_source1_data_property_value18104,
                    };
                    self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                }) != (self::ml_string::Text::from_units(vec![
                    98, 111, 111, 108, 101, 97, 110,
                ]))) {
                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 100, 101, 110, 115, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 101, 110, 116, 114, 121, 32, 114, 101, 99, 111, 114, 100, 115]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_value_error(
                            heap.clone(),
                            match entry.clone() {
                                self::JavaScriptCollectionEntry::CollectionEntry(
                                    source_ml_source1_data_property_value18106,
                                    _,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value18106 =
                                        *source_ml_source1_data_property_value18106;
                                    source_ml_source1_data_property_value18106.clone()
                                }
                            },
                        );
                        {
                            let value_error = self::read_java_script_ordinary_value_error(
                                heap.clone(),
                                match entry.clone() {
                                    self::JavaScriptCollectionEntry::CollectionEntry(
                                        _,
                                        source_ml_source1_data_property_value18107,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value18107 =
                                            *source_ml_source1_data_property_value18107;
                                        source_ml_source1_data_property_value18107.clone()
                                    }
                                },
                            );
                            if (key_error != (self::ml_string::Text::from_units(vec![])))
                                || (value_error != (self::ml_string::Text::from_units(vec![])))
                            {
                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Return({
let ml_library_inequality_1 = key_error != (self::ml_string::Text::from_units(vec![])) ;
if ml_library_inequality_1 {
                                    key_error.clone()
                                } else {
                                    value_error.clone()
                                }
})
                            } else {
                                {
                                    let source_ml_temporary_read_java_script_strong_collection_object_error_written78 = {
                                        let ml_field_owner136 = budget.clone();
                                        let ml_field_previous136 = ml_field_owner136.borrow().3;
                                        let ml_field_value136 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                        self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value18108, _, _) => {
                                            let source_ml_source1_data_property_value18108 = *source_ml_source1_data_property_value18108;
                                            source_ml_source1_data_property_value18108.clone()
                                        }
                                    })) + self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                        self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source1_data_property_value18109, _) => {
                                            let source_ml_source1_data_property_value18109 = *source_ml_source1_data_property_value18109;
                                            source_ml_source1_data_property_value18109.clone()
                                        }
                                    }));
                                        ml_field_owner136.borrow_mut().3 =
                                            ml_field_previous136 + ml_field_value136;
                                        ()
                                    };
                                    if (({ budget.borrow().clone() }).3) > 67_108_864f64 {
                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                    } else {
                                        {
                                            let key = match entry.clone() {
                                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value18110, _, _) => {
                                                    let source_ml_source1_data_property_value18110 = *source_ml_source1_data_property_value18110;
                                                    source_ml_source1_data_property_value18110.clone()
                                                }
                                            };
                                            {
                                                let source_ml_source1_s18121 = match key {
                                                    self::JavaScriptSemanticValue::Undefined => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Next
                                                    }
                                                    self::JavaScriptSemanticValue::Null => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Next
                                                    }
                                                    self::JavaScriptSemanticValue::Boolean(_) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Next
                                                    }
                                                    self::JavaScriptSemanticValue::Number(value) => {
                                                        if {
                                                            let source_ml_source1_same_value_operand18131 = -0f64;
                                                            (((value == source_ml_source1_same_value_operand18131) && ((value != 0f64) || ((1f64 / value) == (1f64 / source_ml_source1_same_value_operand18131)))) || ((value != value) && (source_ml_source1_same_value_operand18131 != source_ml_source1_same_value_operand18131)))
                                                        } {
                                                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 111, 114, 101, 100, 32, 107, 101, 121, 115, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 32, 110, 101, 103, 97, 116, 105, 118, 101, 32, 78, 117, 109, 98, 101, 114, 32, 122, 101, 114, 111]))
                                                        } else {
                                                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Next
                                                        }
                                                    }
                                                    self::JavaScriptSemanticValue::Integer(_) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Next
                                                    }
                                                    self::JavaScriptSemanticValue::Text(_) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Next
                                                    }
                                                    self::JavaScriptSemanticValue::SourceText(_, _) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Next
                                                    }
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Next
                                                    }
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Next
                                                    }
                                                };
                                                match source_ml_source1_s18121 {
                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Next => {
                                                        if match object.clone() {
                                                            self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                true
                                                            }
                                                            _ => {
                                                                false
                                                            }
                                                        } && !(self::test_java_script_semantic_same_value(match entry.clone() {
                                                            self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value18227, _, _) => {
                                                                let source_ml_source1_data_property_value18227 = *source_ml_source1_data_property_value18227;
                                                                source_ml_source1_data_property_value18227.clone()
                                                            }
                                                        }, match entry.clone() {
                                                            self::JavaScriptCollectionEntry::CollectionEntry(_, source_ml_source1_data_property_value18228, _) => {
                                                                let source_ml_source1_data_property_value18228 = *source_ml_source1_data_property_value18228;
                                                                source_ml_source1_data_property_value18228.clone()
                                                            }
                                                        })) {
                                                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Return(self::ml_string::Text::from_units(vec![83, 101, 116, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 107, 101, 121, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101]))
                                                        } else if match entry.clone() {
                                                                self::JavaScriptCollectionEntry::CollectionEntry(_, _, source_ml_source1_data_property_value18229) => {
                                                                    source_ml_source1_data_property_value18229
                                                                }
                                                            } {
                                                                {
                                                                    let source_ml_temporary_read_java_script_strong_collection_object_error_written80 = { let ml_field_owner138 = budget.clone(); let ml_field_previous138 = ml_field_owner138.borrow().2; let ml_field_value138 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(match entry.clone() {
                                                                        self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value18230, _, _) => {
                                                                            let source_ml_source1_data_property_value18230 = *source_ml_source1_data_property_value18230;
                                                                            source_ml_source1_data_property_value18230.clone()
                                                                        }
                                                                    })); ml_field_owner138.borrow_mut().2 = ml_field_previous138 + ml_field_value138; () };
                                                                    if (({ budget.borrow().clone() }).2) > 16_777_216f64 {
                                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Return(self::ml_string::Text::from_units(vec![97, 99, 116, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 98, 117, 100, 103, 101, 116]))
                                                                    } else {
                                                                        {
                                                                            let source_ml_temporary_read_java_script_strong_collection_object_error_ignored81 = { let ml_method_owner140 = keys.clone(); let ml_method_argument140_0 = match entry.clone() {
                                                                                self::JavaScriptCollectionEntry::CollectionEntry(source_ml_source1_data_property_value18231, _, _) => {
                                                                                    let source_ml_source1_data_property_value18231 = *source_ml_source1_data_property_value18231;
                                                                                    source_ml_source1_data_property_value18231.clone()
                                                                                }
                                                                            }; let ml_inserted = vec![ml_method_argument140_0]; let mut ml_method_values140 = ml_method_owner140.borrow_mut(); let ml_length = ml_method_values140.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4_294_967_295usize).expect("Invalid array length"); ml_method_values140.extend(ml_inserted); ml_length as f64 };
                                                                            {
                                                                                let source_ml_temporary_index73_2 = source_ml_temporary_index73.add(&self::ml::Big::from_i128(1));
                                                                                {
                                                                                    source_ml_temporary_index73 = source_ml_temporary_index73_2;
                                                                                    continue;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                {
                                                                    let source_ml_temporary_index73_3 = source_ml_temporary_index73.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        source_ml_temporary_index73 = source_ml_temporary_index73_3;
                                                                        continue;
                                                                    }
                                                                }
                                                            }
                                                    }
                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin79Return(source_ml_temporary_result81) => {
                                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Return(source_ml_temporary_result81.clone())
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
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop76Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84 {
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84Next,
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin84Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83 {
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83Next,
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin83Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82 {
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82Next,
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin82Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Result {
    MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Done,
    MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Return(self::ml_string::Text),
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_temporary_read_java_script_ordinary_object_descriptor_error_loop86(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>,
    mut at: f64,
) -> self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Result {
    loop {
        return if at
            < ({
                let ml_reference_array141 = properties.clone();
                ml_reference_array141.borrow().len() as f64
            }) {
            {
                let property = {
                    let ml_reference_array142 = properties.clone();
                    let ml_reference_position142 = at;
                    self::ml_array::optional_at(
                        &*ml_reference_array142.borrow(),
                        self::ml_array::number_index(ml_reference_position142),
                    )
                };
                if match (property.clone(), None::<()>) {
                    (Some(_ml_left), Some(_ml_right)) => false,
                    (None, None) => true,
                    _ => false,
                } {
                    self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_key_error(
                            heap.clone(),
                            match property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                    source_ml_source1_data_property_value18234,
                                    _,
                                ) => {
                                    let source_ml_source1_data_property_value18234 =
                                        *source_ml_source1_data_property_value18234;
                                    source_ml_source1_data_property_value18234.clone()
                                }
                            },
                        );
                        {
                            let ml_library_inequality_2 =
                                key_error != (self::ml_string::Text::from_units(vec![]));
                            if ml_library_inequality_2 {
                                self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Return(key_error.clone())
                            } else if (self::find_java_script_ordinary_property(
                                properties.clone(),
                                match property.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        source_ml_source1_data_property_value18236,
                                        _,
                                    ) => {
                                        let source_ml_source1_data_property_value18236 =
                                            *source_ml_source1_data_property_value18236;
                                        source_ml_source1_data_property_value18236.clone()
                                    }
                                },
                            )) != at
                            {
                                self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                            } else {
                                {
                                    let descriptor_error = self::read_java_script_ordinary_descriptor_value_error(heap.clone(), match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value18237) => {
                                            source_ml_source1_data_property_value18237.clone()
                                        }
                                    });
                                    {
                                        let ml_library_inequality_1 = descriptor_error
                                            != (self::ml_string::Text::from_units(vec![]));
                                        if ml_library_inequality_1 {
                                            self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Return(descriptor_error.clone())
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
            self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop86Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Result {
    MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Done,
    MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop87Return(self::ml_string::Text),
}
