// Generated from complete js/src/translation/javascript-call-machine.js. Do not edit by hand.
use super::JavaScriptCollectionEntry;
use super::JavaScriptOrdinaryHeap;
use super::JavaScriptOrdinaryObject;
use super::JavaScriptOrdinaryProperty;
use super::JavaScriptSemanticValue;
use super::JavaScriptWeakCollectionEntry;
use super::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result;
use super::find_java_script_ordinary_object;
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
    mut budget: std::rc::Rc<std::cell::RefCell<((f64, f64, f64, f64, f64), Vec<f64>)>>,
    mut previous_weak_key: Option<self::ml::Big>,
    mut source_ml_source1_temporary_values57: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>,
    >,
    mut source_ml_source1_temporary_index57: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result {
    loop {
        return if (source_ml_source1_temporary_index57
            < ({
                let ml_reference_array2561 = source_ml_source1_temporary_values57.clone();
                let ml_reference_projection2561 =
                    self::ml::Big::from_u128(ml_reference_array2561.borrow().len() as u128);
                ml_reference_projection2561
            })) {
            {
                let entry = {
                    let ml_reference_array2562 = source_ml_source1_temporary_values57.clone();
                    let ml_reference_position2562 = source_ml_source1_temporary_index57.clone();
                    let ml_reference_projection2562 = self::ml_array::at(
                        &*ml_reference_array2562.borrow(),
                        ml_reference_position2562.to_index(),
                    );
                    ml_reference_projection2562
                };
                match entry.clone() {
                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(
                        shared_constructor_field1,
                        shared_constructor_field2,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        if ((match (entry.clone(), None::<()>) {
                            (_ml_left, Some(_ml_right)) => false,
                            _ => false,
                        } || match (entry.clone(), None::<()>) {
                            (_ml_left, Some(_ml_right)) => false,
                            _ => false,
                        }) || match entry.clone() {
                            self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(_, _) => false,
                        }) {
                            self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 101, 110, 115, 101, 32, 97, 99, 116, 117, 97, 108, 32, 101, 110, 116, 114, 121, 32, 114, 101, 99, 111, 114, 100, 115]))
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_value_error(
                                    heap.clone(),
                                    shared_constructor_field1.clone(),
                                );
                                {
                                    let value_error = self::read_java_script_ordinary_value_error(
                                        heap.clone(),
                                        shared_constructor_field2.clone(),
                                    );
                                    if ((key_error != (self::ml_string::Text::from_units(vec![])))
                                        || (value_error
                                            != (self::ml_string::Text::from_units(vec![]))))
                                    {
                                        self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(if !key_error.0.is_empty() {
                                            key_error.clone()
                                        } else {
                                            value_error.clone()
                                        })
                                    } else {
                                        if ((self::read_java_script_weak_key_eligibility(
                                            heap.clone(),
                                            shared_constructor_field1.clone(),
                                        )) != (self::ml_string::Text::from_units(vec![
                                            101, 108, 105, 103, 105, 98, 108, 101,
                                        ]))) {
                                            self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 101, 108, 105, 103, 105, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 111, 114, 32, 110, 111, 110, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 115]))
                                        } else {
                                            if (({
                                                let source_ml_source2_s39154 =
                                                    shared_constructor_field1.clone();
                                                match source_ml_source2_s39154.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => true,
                                                }
                                            } && {
                                                let source_ml_source2_s39159 =
                                                    shared_constructor_field1.clone();
                                                match source_ml_source2_s39159.clone() {
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        false
                                                    }
                                                    _ => true,
                                                }
                                            }) || {
                                                let source_ml_source2_comparison_operand39165 = (((match shared_constructor_field1.clone() {
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source2_data_property_value39163) => {
                                                        Some((source_ml_source2_data_property_value39163.clone(),))
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value39164, _) => {
                                                        Some((source_ml_source2_data_property_value39164.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml::Big,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone();
                                                if match (
                                                    previous_weak_key.clone(),
                                                    None::<self::ml::Big>,
                                                ) {
                                                    (Some(ml_left), Some(ml_right)) => {
                                                        ml_left == ml_right
                                                    }
                                                    (None, None) => true,
                                                    _ => false,
                                                } {
                                                    {
                                                        let source_ml_source2_comparison_order39167 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source2_comparison_operand39165.clone());
                                                        ((source_ml_source2_comparison_order39167 == (1f64)) || (source_ml_source2_comparison_order39167 == (0f64)))
                                                    }
                                                } else {
                                                    (source_ml_source2_comparison_operand39165
                                                        <= (previous_weak_key
                                                            .clone()
                                                            .expect("a proven present value")))
                                                }
                                            }) {
                                                self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 107, 101, 121, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                            } else {
                                                {
                                                    let previous_weak_key_2 = (((match shared_constructor_field1.clone() {
                                                        self::JavaScriptSemanticValue::Symbol(source_ml_source2_data_property_value39169) => {
                                                            Some((source_ml_source2_data_property_value39169.clone(),))
                                                        }
                                                        self::JavaScriptSemanticValue::Object(source_ml_source2_data_property_value39170, _) => {
                                                            Some((source_ml_source2_data_property_value39170.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::ml::Big,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone();
                                                    if (match object.clone() {
                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                            true
                                                        }
                                                        _ => {
                                                            false
                                                        }
                                                    } && {
                                                        let source_ml_source2_s39178 = shared_constructor_field2.clone();
                                                        match source_ml_source2_s39178.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) {
                                                        self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 32, 119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 100, 111, 32, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 77, 97, 112, 32, 118, 97, 108, 117, 101, 32, 101, 100, 103, 101, 115]))
                                                    } else {
                                                        {
                                                            let source_ml_source1_temporary_read_java_script_weak_collection_object_error_written61 = { let ml_field_owner2563 = budget.clone(); let ml_field_previous2563 = ml_field_owner2563.borrow().0.3; let ml_field_value2563 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(shared_constructor_field2.clone())); ml_field_owner2563.borrow_mut().0.3 = ml_field_previous2563 + ml_field_value2563; () };
                                                            if (((({ let ml_reference_value2564 = budget.borrow().clone(); ml_reference_value2564 }).0).3) > (67108864f64)) {
                                                                self::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Result::MlSource1TemporaryReadJavaScriptWeakCollectionObjectErrorLoop60Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 97, 110, 100, 32, 115, 116, 114, 111, 110, 103, 32, 118, 97, 108, 117, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                            } else {
                                                                {
                                                                    let source_ml_source1_temporary_index57_2 = source_ml_source1_temporary_index57.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        (previous_weak_key, source_ml_source1_temporary_index57) = (Some(previous_weak_key_2.clone()), source_ml_source1_temporary_index57_2.clone());
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
        return if (at
            < ({
                let ml_reference_array2565 = ordered_keys.clone();
                let ml_reference_projection2565 = ml_reference_array2565.borrow().len() as f64;
                ml_reference_projection2565
            })) {
            {
                let prior = {
                    let source_ml_source2_array_read_index39182 = (at - 1f64);
                    {
                        let ml_reference_array2566 = ordered_keys.clone();
                        let ml_reference_position2566 = source_ml_source2_array_read_index39182;
                        let ml_reference_projection2566 = self::ml_array::optional_at(
                            &*ml_reference_array2566.borrow(),
                            self::ml_array::number_index(ml_reference_position2566),
                        );
                        ml_reference_projection2566
                    }
                };
                {
                    let current = {
                        let ml_reference_array2567 = ordered_keys.clone();
                        let ml_reference_position2567 = at;
                        let ml_reference_projection2567 = self::ml_array::optional_at(
                            &*ml_reference_array2567.borrow(),
                            self::ml_array::number_index(ml_reference_position2567),
                        );
                        ml_reference_projection2567
                    };
                    if ((!(match (prior.clone(), None::<()>) {
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
                    )) {
                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop66Return(self::ml_string::Text::from_units(vec![97, 99, 116, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 117, 110, 100, 101, 114, 32, 83, 97, 109, 101, 86, 97, 108, 117, 101, 90, 101, 114, 111]))
                    } else {
                        {
                            let at_2 = (at + 1f64);
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
    mut budget: std::rc::Rc<std::cell::RefCell<((f64, f64, f64, f64, f64), Vec<f64>)>>,
    mut keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>,
    mut source_ml_source1_temporary_values62: std::rc::Rc<
        std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>,
    >,
    mut source_ml_source1_temporary_index62: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result {
    loop {
        return if (source_ml_source1_temporary_index62
            < ({
                let ml_reference_array2568 = source_ml_source1_temporary_values62.clone();
                let ml_reference_projection2568 =
                    self::ml::Big::from_u128(ml_reference_array2568.borrow().len() as u128);
                ml_reference_projection2568
            })) {
            {
                let entry = {
                    let ml_reference_array2569 = source_ml_source1_temporary_values62.clone();
                    let ml_reference_position2569 = source_ml_source1_temporary_index62.clone();
                    let ml_reference_projection2569 = self::ml_array::at(
                        &*ml_reference_array2569.borrow(),
                        ml_reference_position2569.to_index(),
                    );
                    ml_reference_projection2569
                };
                match entry.clone() {
                    self::JavaScriptCollectionEntry::CollectionEntry(
                        shared_constructor_field1,
                        shared_constructor_field2,
                        shared_constructor_field3,
                    ) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        if ((match (entry.clone(), None::<()>) {
                            (_ml_left, Some(_ml_right)) => false,
                            _ => false,
                        } || match entry.clone() {
                            self::JavaScriptCollectionEntry::CollectionEntry(_, _, _) => false,
                        }) || (({
                            let source_ml_source2_type_query_operand39193 =
                                shared_constructor_field3;
                            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                        }) != (self::ml_string::Text::from_units(vec![
                            98, 111, 111, 108, 101, 97, 110,
                        ])))) {
                            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 100, 101, 110, 115, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 101, 110, 116, 114, 121, 32, 114, 101, 99, 111, 114, 100, 115]))
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_value_error(
                                    heap.clone(),
                                    shared_constructor_field1.clone(),
                                );
                                {
                                    let value_error = self::read_java_script_ordinary_value_error(
                                        heap.clone(),
                                        shared_constructor_field2.clone(),
                                    );
                                    if ((key_error != (self::ml_string::Text::from_units(vec![])))
                                        || (value_error
                                            != (self::ml_string::Text::from_units(vec![]))))
                                    {
                                        self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                            key_error.clone()
                                        } else {
                                            value_error.clone()
                                        })
                                    } else {
                                        {
                                            let source_ml_source1_temporary_read_java_script_strong_collection_object_error_written67 = {
                                                let ml_field_owner2570 = budget.clone();
                                                let ml_field_previous2570 =
                                                    ml_field_owner2570.borrow().0.3;
                                                let ml_field_value2570 = (self::ml_math::max(
                                                    0f64,
                                                    self::read_java_script_semantic_text_length(
                                                        shared_constructor_field1.clone(),
                                                    ),
                                                ) + self::ml_math::max(
                                                    0f64,
                                                    self::read_java_script_semantic_text_length(
                                                        shared_constructor_field2.clone(),
                                                    ),
                                                ));
                                                ml_field_owner2570.borrow_mut().0.3 =
                                                    ml_field_previous2570 + ml_field_value2570;
                                                ()
                                            };
                                            if (((({
                                                let ml_reference_value2571 =
                                                    budget.borrow().clone();
                                                ml_reference_value2571
                                            })
                                            .0)
                                                .3)
                                                > (67108864f64))
                                            {
                                                self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                            } else {
                                                {
                                                    let key = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source2_s39201 = match key.clone() {
                                                            self::JavaScriptSemanticValue::Number(value) => {
                                                                if {
                                                                    let source_ml_source2_same_value_operand39203 = (-0f64);
                                                                    (((value == source_ml_source2_same_value_operand39203) && ((value != (0f64)) || (((1f64 / value)) == ((1f64 / source_ml_source2_same_value_operand39203))))) || ((value != value) && (source_ml_source2_same_value_operand39203 != source_ml_source2_same_value_operand39203)))
                                                                } {
                                                                    self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 111, 114, 101, 100, 32, 107, 101, 121, 115, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 32, 110, 101, 103, 97, 116, 105, 118, 101, 32, 78, 117, 109, 98, 101, 114, 32, 122, 101, 114, 111]))
                                                                } else {
                                                                    self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                                }
                                                            }
                                                            _ => {
                                                                self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next
                                                            }
                                                        };
                                                        match source_ml_source2_s39201.clone() {
                                                            self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorJoin68Next => {
                                                                if (match object.clone() {
                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                } && !(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), shared_constructor_field2.clone()))) {
                                                                    self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![83, 101, 116, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 107, 101, 121, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101]))
                                                                } else {
                                                                    if shared_constructor_field3 {
                                                                        {
                                                                            let source_ml_source1_temporary_read_java_script_strong_collection_object_error_written69 = { let ml_field_owner2572 = budget.clone(); let ml_field_previous2572 = ml_field_owner2572.borrow().0.2; let ml_field_value2572 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(shared_constructor_field1.clone())); ml_field_owner2572.borrow_mut().0.2 = ml_field_previous2572 + ml_field_value2572; () };
                                                                            if (((({ let ml_reference_value2573 = budget.borrow().clone(); ml_reference_value2573 }).0).2) > (16777216f64)) {
                                                                                self::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Result::MlSource1TemporaryReadJavaScriptStrongCollectionObjectErrorLoop65Return(self::ml_string::Text::from_units(vec![97, 99, 116, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 98, 117, 100, 103, 101, 116]))
                                                                            } else {
                                                                                {
                                                                                    let source_ml_source1_temporary_read_java_script_strong_collection_object_error_ignored70 = { let ml_method_owner2574 = keys.clone(); let ml_method_argument2574_0 = shared_constructor_field1.clone(); let ml_inserted = vec![ml_method_argument2574_0]; let mut ml_method_values2574 = ml_method_owner2574.borrow_mut(); let ml_length = ml_method_values2574.len().checked_add(ml_inserted.len()).filter(|length| *length <= 4294967295usize).expect("Invalid array length"); ml_method_values2574.extend(ml_inserted); ml_length as f64 };
                                                                                    {
                                                                                        let source_ml_source1_temporary_index62_2 = source_ml_source1_temporary_index62.add(&self::ml::Big::from_i128(1));
                                                                                        {
                                                                                            source_ml_source1_temporary_index62 = source_ml_source1_temporary_index62_2.clone();
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
                                                                                source_ml_source1_temporary_index62 = source_ml_source1_temporary_index62_3.clone();
                                                                                continue;
                                                                            }
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
        return if (at
            < ({
                let ml_reference_array2575 = properties.clone();
                let ml_reference_projection2575 = ml_reference_array2575.borrow().len() as f64;
                ml_reference_projection2575
            })) {
            {
                let property = {
                    let ml_reference_array2576 = properties.clone();
                    let ml_reference_position2576 = at;
                    let ml_reference_projection2576 = self::ml_array::optional_at(
                        &*ml_reference_array2576.borrow(),
                        self::ml_array::number_index(ml_reference_position2576),
                    );
                    ml_reference_projection2576
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
                                    source_ml_source2_data_property_value39217,
                                    _,
                                ) => {
                                    let source_ml_source2_data_property_value39217 =
                                        *source_ml_source2_data_property_value39217;
                                    source_ml_source2_data_property_value39217.clone()
                                }
                            },
                        );
                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(key_error.clone())
                        } else {
                            if ((self::find_java_script_ordinary_property(
                                properties.clone(),
                                match property.clone().expect("a proven present value") {
                                    self::JavaScriptOrdinaryProperty::OrdinaryProperty(
                                        source_ml_source2_data_property_value39219,
                                        _,
                                    ) => {
                                        let source_ml_source2_data_property_value39219 =
                                            *source_ml_source2_data_property_value39219;
                                        source_ml_source2_data_property_value39219.clone()
                                    }
                                },
                            )) != at)
                            {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                            } else {
                                {
                                    let descriptor_error = self::read_java_script_ordinary_descriptor_value_error(heap.clone(), match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source2_data_property_value39220) => {
                                            source_ml_source2_data_property_value39220.clone()
                                        }
                                    });
                                    if (descriptor_error
                                        != (self::ml_string::Text::from_units(vec![])))
                                    {
                                        self::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Result::MlSource1TemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop75Return(descriptor_error.clone())
                                    } else {
                                        {
                                            let at_2 = (at + 1f64);
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

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_prototype_chain_error_loop76(
    mut heap: self::JavaScriptOrdinaryHeap,
    mut object: self::JavaScriptOrdinaryObject,
    mut prototype: self::JavaScriptSemanticValue,
    mut depth: f64,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
                match prototype.clone() {
                    self::JavaScriptSemanticValue::Object(identity, _) => {
                        {
                            let depth_2 = (depth + 1f64);
                            if ((identity == (match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source2_data_property_value39235, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39235.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source2_data_property_value39236, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39236.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source2_data_property_value39237, _, _, _, _) => {
                                    source_ml_source2_data_property_value39237.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source2_data_property_value39238, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39238.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source2_data_property_value39239, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39239.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source2_data_property_value39240, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39240.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source2_data_property_value39241, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39241.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source2_data_property_value39242, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39242.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source2_data_property_value39243, _, _, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39243.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source2_data_property_value39244, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39244.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source2_data_property_value39245, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39245.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source2_data_property_value39246, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39246.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source2_data_property_value39247, _, _, _, _, _, _) => {
                                    source_ml_source2_data_property_value39247.clone()
                                }
                            })) || (depth_2 > ({ let ml_reference_array2577 = shared_constructor_field1.clone(); let ml_reference_projection2577 = ml_reference_array2577.borrow().len() as f64; ml_reference_projection2577 }))) {
                                self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                            } else {
                                {
                                    let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                    {
                                        let parent = {
                                            let source_ml_source2_array_read_value39252 = shared_constructor_field1.clone();
                                            { let ml_reference_array2578 = source_ml_source2_array_read_value39252.clone(); let ml_reference_position2578 = index; let ml_reference_projection2578 = self::ml_array::optional_at(&*ml_reference_array2578.borrow(), self::ml_array::number_index(ml_reference_position2578)); ml_reference_projection2578 }
                                        };
                                        if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
                                        } else {
                                            if ((((((((match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::OrdinaryObject(_, _, _, _, _, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            } && match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ClosureObject(_, _, _, _, _, _, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) && match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::BoundFunctionObject(_, _, _, _, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) && match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::MapObject(_, _, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) && match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) && match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::WeakMapObject(_, _, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) && match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::WeakSetObject(_, _, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) && match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, _, _, _, _, _, _, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) && match parent.clone().expect("a proven present value") {
                                                self::JavaScriptOrdinaryObject::ArrayObject(_, _, _, _, _) => {
                                                    false
                                                }
                                                _ => {
                                                    true
                                                }
                                            }) {
                                                self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 104, 111, 115, 116, 32, 69, 114, 114, 111, 114, 115, 32, 110, 101, 101, 100, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 108, 111, 111, 107, 117, 112]))
                                            } else {
                                                {
                                                    let prototype_2 = (((match parent.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source2_data_property_value39321, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39321 = *source_ml_source2_data_property_value39321;
                                                            Some((source_ml_source2_data_property_value39321.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source2_data_property_value39322, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39322 = *source_ml_source2_data_property_value39322;
                                                            Some((source_ml_source2_data_property_value39322.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source2_data_property_value39323, _, _, _) => {
                                                            let source_ml_source2_data_property_value39323 = *source_ml_source2_data_property_value39323;
                                                            Some((source_ml_source2_data_property_value39323.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source2_data_property_value39324, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39324 = *source_ml_source2_data_property_value39324;
                                                            Some((source_ml_source2_data_property_value39324.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source2_data_property_value39325, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39325 = *source_ml_source2_data_property_value39325;
                                                            Some((source_ml_source2_data_property_value39325.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source2_data_property_value39326, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39326 = *source_ml_source2_data_property_value39326;
                                                            Some((source_ml_source2_data_property_value39326.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source2_data_property_value39327, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39327 = *source_ml_source2_data_property_value39327;
                                                            Some((source_ml_source2_data_property_value39327.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source2_data_property_value39328, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39328 = *source_ml_source2_data_property_value39328;
                                                            Some((source_ml_source2_data_property_value39328.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source2_data_property_value39329, _, _, _, _, _, _, _) => {
                                                            let source_ml_source2_data_property_value39329 = *source_ml_source2_data_property_value39329;
                                                            Some((source_ml_source2_data_property_value39329.clone(),))
                                                        }
                                                        _ => {
                                                            None::<(self::JavaScriptSemanticValue,)>
                                                        }
                                                    }).clone().expect("a proven present value")).0).clone();
                                                    {
                                                        (prototype, depth) = (prototype_2.clone(), depth_2);
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
                    _ => {
                        self::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Result::MlSource1TemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop76Done
                    }
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop101Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done,
}

/// Execute the checked shared source algorithm.
#[must_use]
pub fn source_ml_source1_temporary_read_java_script_ordinary_heap_error_loop103(
    mut marks: std::rc::Rc<
        std::cell::RefCell<Vec<std::rc::Rc<std::cell::RefCell<((f64,), Vec<f64>)>>>>,
    >,
    mut source_ml_source1_temporary_values92: std::rc::Rc<std::cell::RefCell<Vec<f64>>>,
    mut source_ml_source1_temporary_index92: self::ml::Big,
) -> self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result {
    loop {
        return if (source_ml_source1_temporary_index92
            < ({
                let ml_reference_array2579 = source_ml_source1_temporary_values92.clone();
                let ml_reference_projection2579 =
                    self::ml::Big::from_u128(ml_reference_array2579.borrow().len() as u128);
                ml_reference_projection2579
            })) {
            {
                let selected = {
                    let ml_reference_array2580 = source_ml_source1_temporary_values92.clone();
                    let ml_reference_position2580 = source_ml_source1_temporary_index92.clone();
                    let ml_reference_projection2580 = self::ml_array::at(
                        &*ml_reference_array2580.borrow(),
                        ml_reference_position2580.to_index(),
                    );
                    ml_reference_projection2580
                };
                {
                    let mark = {
                        let ml_reference_array2581 = marks.clone();
                        let ml_reference_position2581 = selected;
                        let ml_reference_projection2581 = self::ml_array::optional_at(
                            &*ml_reference_array2581.borrow(),
                            self::ml_array::number_index(ml_reference_position2581),
                        );
                        ml_reference_projection2581
                    };
                    if !(match (mark.clone(), None::<()>) {
                        (Some(_ml_left), Some(_ml_right)) => false,
                        (None, None) => true,
                        _ => false,
                    }) {
                        {
                            let source_ml_source1_temporary_read_java_script_ordinary_heap_error_written104 = {
                                let ml_field_owner2582 =
                                    mark.clone().expect("a proven present value");
                                let ml_field_value2582 = 2f64;
                                ml_field_owner2582.borrow_mut().0.0 = ml_field_value2582;
                                ()
                            };
                            {
                                let source_ml_source1_temporary_index92_2 =
                                    source_ml_source1_temporary_index92
                                        .add(&self::ml::Big::from_i128(1));
                                {
                                    source_ml_source1_temporary_index92 =
                                        source_ml_source1_temporary_index92_2.clone();
                                    continue;
                                }
                            }
                        }
                    } else {
                        {
                            let source_ml_source1_temporary_index92_3 =
                                source_ml_source1_temporary_index92
                                    .add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_source1_temporary_index92 =
                                    source_ml_source1_temporary_index92_3.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Result::MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop103Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Result {
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Done,
    MlSource1TemporaryReadJavaScriptOrdinaryHeapErrorLoop102Return(self::ml_string::Text),
}
