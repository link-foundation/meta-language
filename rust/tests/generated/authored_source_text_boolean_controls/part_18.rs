pub fn source_ml_temporary_order_java_script_collection_keys_loop32(mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut middle: f64, mut end: f64, mut left: f64, mut right: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return if ((left < middle) || (right < end)) {
            {
                let a = { let ml_reference_array338 = source.clone(); let ml_reference_position338 = left; let ml_reference_projection338 = self::ml_array::optional_at(&*ml_reference_array338.borrow(), self::ml_array::number_index(ml_reference_position338)); ml_reference_projection338 };
                {
                    let b = { let ml_reference_array339 = source.clone(); let ml_reference_position339 = right; let ml_reference_projection339 = self::ml_array::optional_at(&*ml_reference_array339.borrow(), self::ml_array::number_index(ml_reference_position339)); ml_reference_projection339 };
                    {
                        let take_left = (right >= end);
                        {
                            let take_left_3 = if (!take_left && (left < middle)) {
                                if !(match (a.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                    if !(match (b.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        {
                                            let take_left_2 = ((self::compare_java_script_collection_keys(a.clone().expect("a proven present value"), b.clone().expect("a proven present value"))) <= (0f64));
                                            take_left_2
                                        }
                                    } else {
                                        take_left
                                    }
                                } else {
                                    take_left
                                }
                            } else {
                                take_left
                            };
                            if take_left_3 {
                                {
                                    let target_3 = if !(match (a.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        {
                                            let target_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value340 = target.borrow().clone(); ml_reference_value340 }, vec![a.clone().expect("a proven present value")])));
                                            target_2.clone()
                                        }
                                    } else {
                                        target.clone()
                                    };
                                    {
                                        let left_2 = (left + 1f64);
                                        {
                                            (target, left) = (target_3.clone(), left_2);
                                            continue;
                                        }
                                    }
                                }
                            } else {
                                {
                                    let target_5 = if !(match (b.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) {
                                        {
                                            let target_4 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value341 = target.borrow().clone(); ml_reference_value341 }, vec![b.clone().expect("a proven present value")])));
                                            target_4.clone()
                                        }
                                    } else {
                                        target.clone()
                                    };
                                    {
                                        let right_2 = (right + 1f64);
                                        {
                                            (target, right) = (target_5.clone(), right_2);
                                            continue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            target.clone()
        };
    }
}

pub fn source_ml_temporary_order_java_script_collection_keys_loop31(mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut width: f64, mut target: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut start: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return if (start < ({ let ml_reference_array342 = source.clone(); let ml_reference_projection342 = ml_reference_array342.borrow().len() as f64; ml_reference_projection342 })) {
            {
                let middle = self::ml_math::min((start + width), { let ml_reference_array343 = source.clone(); let ml_reference_projection343 = ml_reference_array343.borrow().len() as f64; ml_reference_projection343 });
                {
                    let end = self::ml_math::min((start + (width * 2f64)), { let ml_reference_array344 = source.clone(); let ml_reference_projection344 = ml_reference_array344.borrow().len() as f64; ml_reference_projection344 });
                    {
                        let left = start;
                        {
                            let target_2 = self::source_ml_temporary_order_java_script_collection_keys_loop32(source.clone(), target.clone(), middle, end, left, middle);
                            {
                                let start_2 = (start + (width * 2f64));
                                {
                                    (target, start) = (target_2.clone(), start_2);
                                    continue;
                                }
                            }
                        }
                    }
                }
            }
        } else {
            target.clone()
        };
    }
}

pub fn source_ml_temporary_order_java_script_collection_keys_loop30(mut source: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut width: f64) -> std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>> {
    loop {
        return if (width < ({ let ml_reference_array345 = source.clone(); let ml_reference_projection345 = ml_reference_array345.borrow().len() as f64; ml_reference_projection345 })) {
            {
                let target = std::rc::Rc::new(std::cell::RefCell::new(Vec::<self::JavaScriptSemanticValue>::new()));
                {
                    let start = 0f64;
                    {
                        let target_2 = self::source_ml_temporary_order_java_script_collection_keys_loop31(source.clone(), width, target.clone(), start);
                        {
                            let width_2 = (width * 2f64);
                            {
                                (source, width) = (target_2.clone(), width_2);
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            source.clone()
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34 {
    MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34Next,
    MlTemporaryReadJavaScriptOpaqueObjectErrorJoin34Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Result {
    MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Done,
    MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_opaque_object_error_loop35(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_temporary_values33: std::rc::Rc<std::cell::RefCell<Vec<Option<self::JavaScriptSemanticValue>>>>, mut source_ml_temporary_index33: self::ml::Big) -> self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Result {
    loop {
        return if (source_ml_temporary_index33 < ({ let ml_reference_array346 = source_ml_temporary_values33.clone(); let ml_reference_projection346 = self::ml::Big::from_u128(ml_reference_array346.borrow().len() as u128); ml_reference_projection346 })) {
            {
                let hook = { let ml_reference_array347 = source_ml_temporary_values33.clone(); let ml_reference_position347 = source_ml_temporary_index33.clone(); let ml_reference_projection347 = self::ml_array::at(&*ml_reference_array347.borrow(), ml_reference_position347.to_index()); ml_reference_projection347 };
                if match (hook.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 108, 108, 32, 104, 111, 111, 107, 115]))
                } else {
                    {
                        let error = self::read_java_script_ordinary_value_error(heap.clone(), hook.clone().expect("a proven present value"));
                        if (error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Return(error.clone())
                        } else {
                            if (match hook.clone().expect("a proven present value") {
                                self::JavaScriptSemanticValue::Object(_, _) => {
                                    false
                                }
                                _ => {
                                    true
                                }
                            } || !(((match hook.clone().expect("a proven present value") {
                                self::JavaScriptSemanticValue::Object(_, source_ml_source1_data_property_value16314) => {
                                    Some((source_ml_source1_data_property_value16314,))
                                }
                                _ => {
                                    None::<(bool,)>
                                }
                            }).clone().expect("a proven present value")).0)) {
                                self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 69, 114, 114, 111, 114, 32, 97, 99, 99, 101, 115, 115, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 105, 116, 115, 32, 97, 99, 116, 117, 97, 108, 32, 101, 120, 116, 101, 114, 110, 97, 108, 108, 121, 32, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 99, 97, 108, 108, 32, 104, 111, 111, 107, 115]))
                            } else {
                                {
                                    let source_ml_temporary_index33_2 = source_ml_temporary_index33.add(&self::ml::Big::from_i128(1));
                                    {
                                        source_ml_temporary_index33 = source_ml_temporary_index33_2.clone();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Result::MlTemporaryReadJavaScriptOpaqueObjectErrorLoop35Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosureObjectErrorJoin37 {
    MlTemporaryReadJavaScriptClosureObjectErrorJoin37Next,
    MlTemporaryReadJavaScriptClosureObjectErrorJoin37Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptClosureObjectErrorLoop39Result {
    MlTemporaryReadJavaScriptClosureObjectErrorLoop39Done,
    MlTemporaryReadJavaScriptClosureObjectErrorLoop39Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_closure_object_error_loop39(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_temporary_values36: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_temporary_index36: self::ml::Big) -> self::MlTemporaryReadJavaScriptClosureObjectErrorLoop39Result {
    loop {
        return if (source_ml_temporary_index36 < ({ let ml_reference_array348 = source_ml_temporary_values36.clone(); let ml_reference_projection348 = self::ml::Big::from_u128(ml_reference_array348.borrow().len() as u128); ml_reference_projection348 })) {
            {
                let value = { let ml_reference_array349 = source_ml_temporary_values36.clone(); let ml_reference_position349 = source_ml_temporary_index36.clone(); let ml_reference_projection349 = self::ml_array::at(&*ml_reference_array349.borrow(), ml_reference_position349.to_index()); ml_reference_projection349 };
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlTemporaryReadJavaScriptClosureObjectErrorLoop39Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop39Return(error.clone())
                    } else {
                        {
                            let source_ml_temporary_index36_2 = source_ml_temporary_index36.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index36 = source_ml_temporary_index36_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptClosureObjectErrorLoop39Result::MlTemporaryReadJavaScriptClosureObjectErrorLoop39Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptBoundObjectErrorJoin41 {
    MlTemporaryReadJavaScriptBoundObjectErrorJoin41Next,
    MlTemporaryReadJavaScriptBoundObjectErrorJoin41Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptBoundObjectErrorLoop43Result {
    MlTemporaryReadJavaScriptBoundObjectErrorLoop43Done,
    MlTemporaryReadJavaScriptBoundObjectErrorLoop43Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_bound_object_error_loop43(mut heap: self::JavaScriptOrdinaryHeap, mut source_ml_temporary_values40: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_temporary_index40: self::ml::Big) -> self::MlTemporaryReadJavaScriptBoundObjectErrorLoop43Result {
    loop {
        return if (source_ml_temporary_index40 < ({ let ml_reference_array350 = source_ml_temporary_values40.clone(); let ml_reference_projection350 = self::ml::Big::from_u128(ml_reference_array350.borrow().len() as u128); ml_reference_projection350 })) {
            {
                let value = { let ml_reference_array351 = source_ml_temporary_values40.clone(); let ml_reference_position351 = source_ml_temporary_index40.clone(); let ml_reference_projection351 = self::ml_array::at(&*ml_reference_array351.borrow(), ml_reference_position351.to_index()); ml_reference_projection351 };
                {
                    let error = self::read_java_script_ordinary_value_error(heap.clone(), value.clone());
                    if (error != (self::ml_string::Text::from_units(vec![]))) {
                        self::MlTemporaryReadJavaScriptBoundObjectErrorLoop43Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop43Return(error.clone())
                    } else {
                        {
                            let source_ml_temporary_index40_2 = source_ml_temporary_index40.add(&self::ml::Big::from_i128(1));
                            {
                                source_ml_temporary_index40 = source_ml_temporary_index40_2.clone();
                                continue;
                            }
                        }
                    }
                }
            }
        } else {
            self::MlTemporaryReadJavaScriptBoundObjectErrorLoop43Result::MlTemporaryReadJavaScriptBoundObjectErrorLoop43Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44 {
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Next,
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin44Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45 {
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45Next,
    MlTemporaryReadJavaScriptCollectionIteratorObjectErrorJoin45Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48 {
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48Next,
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorJoin48Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Result {
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Done,
    MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_weak_collection_object_error_loop50(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>, mut previous_weak_key: Option<self::ml::Big>, mut source_ml_temporary_values47: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptWeakCollectionEntry>>>, mut source_ml_temporary_index47: self::ml::Big) -> self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Result {
    loop {
        return if (source_ml_temporary_index47 < ({ let ml_reference_array352 = source_ml_temporary_values47.clone(); let ml_reference_projection352 = self::ml::Big::from_u128(ml_reference_array352.borrow().len() as u128); ml_reference_projection352 })) {
            {
                let entry = { let ml_reference_array353 = source_ml_temporary_values47.clone(); let ml_reference_position353 = source_ml_temporary_index47.clone(); let ml_reference_projection353 = self::ml_array::at(&*ml_reference_array353.borrow(), ml_reference_position353.to_index()); ml_reference_projection353 };
                match entry.clone() {
                    self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(shared_constructor_field1, shared_constructor_field2) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        if ((match (entry.clone(), None::<()>) { (_ml_left, Some(_ml_right)) => false, _ => false } || match (entry.clone(), None::<()>) { (_ml_left, Some(_ml_right)) => false, _ => false }) || match entry.clone() {
                            self::JavaScriptWeakCollectionEntry::WeakCollectionEntry(_, _) => {
                                false
                            }
                        }) {
                            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 105, 110, 116, 101, 114, 110, 97, 108, 32, 115, 108, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 100, 101, 110, 115, 101, 32, 97, 99, 116, 117, 97, 108, 32, 101, 110, 116, 114, 121, 32, 114, 101, 99, 111, 114, 100, 115]))
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                                {
                                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field2.clone());
                                    if ((key_error != (self::ml_string::Text::from_units(vec![]))) || (value_error != (self::ml_string::Text::from_units(vec![])))) {
                                        self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Return(if !key_error.0.is_empty() {
                                            key_error.clone()
                                        } else {
                                            value_error.clone()
                                        })
                                    } else {
                                        if ((self::read_java_script_weak_key_eligibility(heap.clone(), shared_constructor_field1.clone())) != (self::ml_string::Text::from_units(vec![101, 108, 105, 103, 105, 98, 108, 101]))) {
                                            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 114, 101, 113, 117, 105, 114, 101, 32, 97, 99, 116, 117, 97, 108, 32, 101, 108, 105, 103, 105, 98, 108, 101, 32, 79, 98, 106, 101, 99, 116, 32, 111, 114, 32, 110, 111, 110, 114, 101, 103, 105, 115, 116, 101, 114, 101, 100, 32, 83, 121, 109, 98, 111, 108, 32, 107, 101, 121, 115]))
                                        } else {
                                            if (({
                                                let source_ml_source1_s16329 = shared_constructor_field1.clone();
                                                match source_ml_source1_s16329.clone() {
                                                    self::JavaScriptSemanticValue::Object(_, _) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            } && {
                                                let source_ml_source1_s16341 = shared_constructor_field1.clone();
                                                match source_ml_source1_s16341.clone() {
                                                    self::JavaScriptSemanticValue::Symbol(_) => {
                                                        false
                                                    }
                                                    _ => {
                                                        true
                                                    }
                                                }
                                            }) || {
                                                let source_ml_source1_comparison_operand16355 = (((match shared_constructor_field1.clone() {
                                                    self::JavaScriptSemanticValue::Symbol(source_ml_source1_data_property_value16353) => {
                                                        Some((source_ml_source1_data_property_value16353.clone(),))
                                                    }
                                                    self::JavaScriptSemanticValue::Object(source_ml_source1_data_property_value16354, _) => {
                                                        Some((source_ml_source1_data_property_value16354.clone(),))
                                                    }
                                                    _ => {
                                                        None::<(self::ml::Big,)>
                                                    }
                                                }).clone().expect("a proven present value")).0).clone();
                                                if match (previous_weak_key.clone(), None::<self::ml::Big>) { (Some(ml_left), Some(ml_right)) => ml_left == ml_right, (None, None) => true, _ => false } {
                                                    {
                                                        let source_ml_source1_comparison_order16357 = self::ml_big_integer_conversion::compare_java_script_number_big_integer(f64::NAN,source_ml_source1_comparison_operand16355.clone());
                                                        ((source_ml_source1_comparison_order16357 == (1f64)) || (source_ml_source1_comparison_order16357 == (0f64)))
                                                    }
                                                } else {
                                                    (source_ml_source1_comparison_operand16355 <= (previous_weak_key.clone().expect("a proven present value")))
                                                }
                                            }) {
                                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 114, 101, 116, 97, 105, 110, 32, 117, 110, 105, 113, 117, 101, 32, 111, 114, 100, 101, 114, 101, 100, 32, 97, 99, 116, 117, 97, 108, 32, 103, 108, 111, 98, 97, 108, 32, 107, 101, 121, 32, 105, 100, 101, 110, 116, 105, 116, 105, 101, 115]))
                                            } else {
                                                {
                                                    let previous_weak_key_2 = (((match shared_constructor_field1.clone() {
                                                        self::JavaScriptSemanticValue::Symbol(source_ml_source1_data_property_value16359) => {
                                                            Some((source_ml_source1_data_property_value16359.clone(),))
                                                        }
                                                        self::JavaScriptSemanticValue::Object(source_ml_source1_data_property_value16360, _) => {
                                                            Some((source_ml_source1_data_property_value16360.clone(),))
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
                                                        let source_ml_source1_s16457 = shared_constructor_field2.clone();
                                                        match source_ml_source1_s16457.clone() {
                                                            self::JavaScriptSemanticValue::Undefined => {
                                                                false
                                                            }
                                                            _ => {
                                                                true
                                                            }
                                                        }
                                                    }) {
                                                        self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Return(self::ml_string::Text::from_units(vec![87, 101, 97, 107, 83, 101, 116, 32, 119, 101, 97, 107, 32, 115, 108, 111, 116, 115, 32, 100, 111, 32, 110, 111, 116, 32, 105, 110, 118, 101, 110, 116, 32, 77, 97, 112, 32, 118, 97, 108, 117, 101, 32, 101, 100, 103, 101, 115]))
                                                    } else {
                                                        {
                                                            let source_ml_temporary_read_java_script_weak_collection_object_error_written51 = { let ml_field_owner354 = budget.clone(); let ml_field_previous354 = ml_field_owner354.borrow().3; let ml_field_value354 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(shared_constructor_field2.clone())); ml_field_owner354.borrow_mut().3 = ml_field_previous354 + ml_field_value354; () };
                                                            if ((({ let ml_reference_value355 = budget.borrow().clone(); ml_reference_value355 }).3) > (67108864f64)) {
                                                                self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Return(self::ml_string::Text::from_units(vec![119, 101, 97, 107, 32, 97, 110, 100, 32, 115, 116, 114, 111, 110, 103, 32, 118, 97, 108, 117, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                                            } else {
                                                                {
                                                                    let source_ml_temporary_index47_2 = source_ml_temporary_index47.add(&self::ml::Big::from_i128(1));
                                                                    {
                                                                        (previous_weak_key, source_ml_temporary_index47) = (Some(previous_weak_key_2.clone()), source_ml_temporary_index47_2.clone());
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
            self::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Result::MlTemporaryReadJavaScriptWeakCollectionObjectErrorLoop50Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53 {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53Next,
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin53Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Result {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Done,
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_strong_collection_object_error_loop56(mut ordered_keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut at: f64) -> self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Result {
    loop {
        return if (at < ({ let ml_reference_array356 = ordered_keys.clone(); let ml_reference_projection356 = ml_reference_array356.borrow().len() as f64; ml_reference_projection356 })) {
            {
                let prior = {
                    let source_ml_source1_array_read_index16470 = (at - 1f64);
                    { let ml_reference_array357 = ordered_keys.clone(); let ml_reference_position357 = source_ml_source1_array_read_index16470; let ml_reference_projection357 = self::ml_array::optional_at(&*ml_reference_array357.borrow(), self::ml_array::number_index(ml_reference_position357)); ml_reference_projection357 }
                };
                {
                    let current = { let ml_reference_array358 = ordered_keys.clone(); let ml_reference_position358 = at; let ml_reference_projection358 = self::ml_array::optional_at(&*ml_reference_array358.borrow(), self::ml_array::number_index(ml_reference_position358)); ml_reference_projection358 };
                    if ((!(match (prior.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false }) && !(match (current.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false })) && self::test_java_script_collection_key(prior.clone().expect("a proven present value"), current.clone().expect("a proven present value"))) {
                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Return(self::ml_string::Text::from_units(vec![97, 99, 116, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 117, 110, 100, 101, 114, 32, 83, 97, 109, 101, 86, 97, 108, 117, 101, 90, 101, 114, 111]))
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
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop56Done
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Result {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Done(std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>),
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58 {
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58Next,
    MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_strong_collection_object_error_loop55(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut budget: std::rc::Rc<std::cell::RefCell<(f64, f64, f64, f64, f64)>>, mut keys: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptSemanticValue>>>, mut source_ml_temporary_values52: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptCollectionEntry>>>, mut source_ml_temporary_index52: self::ml::Big) -> self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Result {
    loop {
        return if (source_ml_temporary_index52 < ({ let ml_reference_array359 = source_ml_temporary_values52.clone(); let ml_reference_projection359 = self::ml::Big::from_u128(ml_reference_array359.borrow().len() as u128); ml_reference_projection359 })) {
            {
                let entry = { let ml_reference_array360 = source_ml_temporary_values52.clone(); let ml_reference_position360 = source_ml_temporary_index52.clone(); let ml_reference_projection360 = self::ml_array::at(&*ml_reference_array360.borrow(), ml_reference_position360.to_index()); ml_reference_projection360 };
                match entry.clone() {
                    self::JavaScriptCollectionEntry::CollectionEntry(shared_constructor_field1, shared_constructor_field2, shared_constructor_field3) => {
                        let shared_constructor_field1 = *shared_constructor_field1;
                        let shared_constructor_field2 = *shared_constructor_field2;
                        if ((match (entry.clone(), None::<()>) { (_ml_left, Some(_ml_right)) => false, _ => false } || match entry.clone() {
                            self::JavaScriptCollectionEntry::CollectionEntry(_, _, _) => {
                                false
                            }
                        }) || (({
                            let source_ml_source1_type_query_operand16481 = shared_constructor_field3;
                            self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])
                        }) != (self::ml_string::Text::from_units(vec![98, 111, 111, 108, 101, 97, 110])))) {
                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 105, 110, 115, 101, 114, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121, 32, 114, 101, 113, 117, 105, 114, 101, 115, 32, 100, 101, 110, 115, 101, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 101, 110, 116, 114, 121, 32, 114, 101, 99, 111, 114, 100, 115]))
                        } else {
                            {
                                let key_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field1.clone());
                                {
                                    let value_error = self::read_java_script_ordinary_value_error(heap.clone(), shared_constructor_field2.clone());
                                    if ((key_error != (self::ml_string::Text::from_units(vec![]))) || (value_error != (self::ml_string::Text::from_units(vec![])))) {
                                        self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Return(if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                                            key_error.clone()
                                        } else {
                                            value_error.clone()
                                        })
                                    } else {
                                        {
                                            let source_ml_temporary_read_java_script_strong_collection_object_error_written57 = { let ml_field_owner361 = budget.clone(); let ml_field_previous361 = ml_field_owner361.borrow().3; let ml_field_value361 = (self::ml_math::max(0f64, self::read_java_script_semantic_text_length(shared_constructor_field1.clone())) + self::ml_math::max(0f64, self::read_java_script_semantic_text_length(shared_constructor_field2.clone()))); ml_field_owner361.borrow_mut().3 = ml_field_previous361 + ml_field_value361; () };
                                            if ((({ let ml_reference_value362 = budget.borrow().clone(); ml_reference_value362 }).3) > (67108864f64)) {
                                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 101, 110, 116, 114, 105, 101, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 115, 116, 111, 114, 97, 103, 101, 32, 98, 117, 100, 103, 101, 116]))
                                            } else {
                                                {
                                                    let key = shared_constructor_field1.clone();
                                                    {
                                                        let source_ml_source1_s16497 = match key.clone() {
                                                            self::JavaScriptSemanticValue::Number(value) => {
                                                                if {
                                                                    let source_ml_source1_same_value_operand16507 = (-0f64);
                                                                    (((value == source_ml_source1_same_value_operand16507) && ((value != (0f64)) || (((1f64 / value)) == ((1f64 / source_ml_source1_same_value_operand16507))))) || ((value != value) && (source_ml_source1_same_value_operand16507 != source_ml_source1_same_value_operand16507)))
                                                                } {
                                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58Return(self::ml_string::Text::from_units(vec![99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 115, 116, 111, 114, 101, 100, 32, 107, 101, 121, 115, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 32, 110, 101, 103, 97, 116, 105, 118, 101, 32, 78, 117, 109, 98, 101, 114, 32, 122, 101, 114, 111]))
                                                                } else {
                                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58Next
                                                                }
                                                            }
                                                            _ => {
                                                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58Next
                                                            }
                                                        };
                                                        match source_ml_source1_s16497.clone() {
                                                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58Next => {
                                                                if (match object.clone() {
                                                                    self::JavaScriptOrdinaryObject::SetObject(_, _, _, _, _, _) => {
                                                                        true
                                                                    }
                                                                    _ => {
                                                                        false
                                                                    }
                                                                } && !(self::test_java_script_semantic_same_value(shared_constructor_field1.clone(), shared_constructor_field2.clone()))) {
                                                                    self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Return(self::ml_string::Text::from_units(vec![83, 101, 116, 32, 101, 110, 116, 114, 105, 101, 115, 32, 114, 101, 116, 97, 105, 110, 32, 116, 104, 101, 32, 115, 97, 109, 101, 32, 110, 111, 114, 109, 97, 108, 105, 122, 101, 100, 32, 107, 101, 121, 32, 97, 110, 100, 32, 118, 97, 108, 117, 101]))
                                                                } else {
                                                                    if shared_constructor_field3 {
                                                                        {
                                                                            let source_ml_temporary_read_java_script_strong_collection_object_error_written59 = { let ml_field_owner363 = budget.clone(); let ml_field_previous363 = ml_field_owner363.borrow().2; let ml_field_value363 = self::ml_math::max(0f64, self::read_java_script_semantic_text_length(shared_constructor_field1.clone())); ml_field_owner363.borrow_mut().2 = ml_field_previous363 + ml_field_value363; () };
                                                                            if ((({ let ml_reference_value364 = budget.borrow().clone(); ml_reference_value364 }).2) > (16777216f64)) {
                                                                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Return(self::ml_string::Text::from_units(vec![97, 99, 116, 105, 118, 101, 32, 99, 111, 108, 108, 101, 99, 116, 105, 111, 110, 32, 107, 101, 121, 115, 32, 101, 120, 99, 101, 101, 100, 32, 116, 104, 101, 105, 114, 32, 115, 104, 97, 114, 101, 100, 32, 85, 84, 70, 49, 54, 32, 99, 111, 109, 112, 97, 114, 105, 115, 111, 110, 32, 98, 117, 100, 103, 101, 116]))
                                                                            } else {
                                                                                {
                                                                                    let keys_2 = std::rc::Rc::new(std::cell::RefCell::new(self::ml_array::append({ let ml_reference_value365 = keys.borrow().clone(); ml_reference_value365 }, vec![shared_constructor_field1.clone()])));
                                                                                    {
                                                                                        let source_ml_temporary_index52_2 = source_ml_temporary_index52.add(&self::ml::Big::from_i128(1));
                                                                                        {
                                                                                            (keys, source_ml_temporary_index52) = (keys_2.clone(), source_ml_temporary_index52_2.clone());
                                                                                            continue;
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    } else {
                                                                        {
                                                                            let source_ml_temporary_index52_3 = source_ml_temporary_index52.add(&self::ml::Big::from_i128(1));
                                                                            {
                                                                                source_ml_temporary_index52 = source_ml_temporary_index52_3.clone();
                                                                                continue;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58::MlTemporaryReadJavaScriptStrongCollectionObjectErrorJoin58Return(source_ml_temporary_result59) => {
                                                                self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Return(source_ml_temporary_result59.clone())
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
            self::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Result::MlTemporaryReadJavaScriptStrongCollectionObjectErrorLoop55Done(keys.clone())
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62 {
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62Next,
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin62Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61 {
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61Next,
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin61Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60 {
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60Next,
    MlTemporaryReadJavaScriptOrdinaryDescriptorValueErrorJoin60Return(self::ml_string::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Result {
    MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Done,
    MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_ordinary_object_descriptor_error_loop64(mut heap: self::JavaScriptOrdinaryHeap, mut properties: std::rc::Rc<std::cell::RefCell<Vec<self::JavaScriptOrdinaryProperty>>>, mut at: f64) -> self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Result {
    loop {
        return if (at < ({ let ml_reference_array366 = properties.clone(); let ml_reference_projection366 = ml_reference_array366.borrow().len() as f64; ml_reference_projection366 })) {
            {
                let property = { let ml_reference_array367 = properties.clone(); let ml_reference_position367 = at; let ml_reference_projection367 = self::ml_array::optional_at(&*ml_reference_array367.borrow(), self::ml_array::number_index(ml_reference_position367)); ml_reference_projection367 };
                if match (property.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                    self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 115, 116, 111, 114, 97, 103, 101, 32, 109, 117, 115, 116, 32, 98, 101, 32, 100, 101, 110, 115, 101]))
                } else {
                    {
                        let key_error = self::read_java_script_ordinary_key_error(heap.clone(), match property.clone().expect("a proven present value") {
                            self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value16610, _) => {
                                let source_ml_source1_data_property_value16610 = *source_ml_source1_data_property_value16610;
                                source_ml_source1_data_property_value16610.clone()
                            }
                        });
                        if (key_error != (self::ml_string::Text::from_units(vec![]))) {
                            self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Return(key_error.clone())
                        } else {
                            if ((self::find_java_script_ordinary_property(properties.clone(), match property.clone().expect("a proven present value") {
                                self::JavaScriptOrdinaryProperty::OrdinaryProperty(source_ml_source1_data_property_value16612, _) => {
                                    let source_ml_source1_data_property_value16612 = *source_ml_source1_data_property_value16612;
                                    source_ml_source1_data_property_value16612.clone()
                                }
                            })) != at) {
                                self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 112, 101, 114, 116, 121, 32, 107, 101, 121, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 117, 110, 105, 113, 117, 101, 32, 119, 105, 116, 104, 111, 117, 116, 32, 114, 101, 111, 114, 100, 101, 114, 105, 110, 103, 32, 116, 104, 101, 105, 114, 32, 99, 114, 101, 97, 116, 105, 111, 110, 32, 104, 105, 115, 116, 111, 114, 121]))
                            } else {
                                {
                                    let descriptor_error = self::read_java_script_ordinary_descriptor_value_error(heap.clone(), match property.clone().expect("a proven present value") {
                                        self::JavaScriptOrdinaryProperty::OrdinaryProperty(_, source_ml_source1_data_property_value16613) => {
                                            source_ml_source1_data_property_value16613.clone()
                                        }
                                    });
                                    if (descriptor_error != (self::ml_string::Text::from_units(vec![]))) {
                                        self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Return(descriptor_error.clone())
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
            self::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Result::MlTemporaryReadJavaScriptOrdinaryObjectDescriptorErrorLoop64Done
        };
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Result {
    MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Done,
    MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Return(self::ml_string::Text),
}

pub fn source_ml_temporary_read_java_script_ordinary_prototype_chain_error_loop65(mut heap: self::JavaScriptOrdinaryHeap, mut object: self::JavaScriptOrdinaryObject, mut prototype: self::JavaScriptSemanticValue, mut depth: f64) -> self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Result {
    loop {
        return match heap.clone() {
            self::JavaScriptOrdinaryHeap::OrdinaryHeap(shared_constructor_field1, _, _, _, _, _) => {
                match prototype.clone() {
                    self::JavaScriptSemanticValue::Object(identity, _) => {
                        {
                            let depth_2 = (depth + 1f64);
                            if ((identity == (match object.clone() {
                                self::JavaScriptOrdinaryObject::ClosureObject(source_ml_source1_data_property_value16635, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16635.clone()
                                }
                                self::JavaScriptOrdinaryObject::BoundFunctionObject(source_ml_source1_data_property_value16636, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16636.clone()
                                }
                                self::JavaScriptOrdinaryObject::ArrayObject(source_ml_source1_data_property_value16637, _, _, _, _) => {
                                    source_ml_source1_data_property_value16637.clone()
                                }
                                self::JavaScriptOrdinaryObject::CollectionIteratorObject(source_ml_source1_data_property_value16638, _, _, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16638.clone()
                                }
                                self::JavaScriptOrdinaryObject::MapObject(source_ml_source1_data_property_value16639, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16639.clone()
                                }
                                self::JavaScriptOrdinaryObject::SetObject(source_ml_source1_data_property_value16640, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16640.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakMapObject(source_ml_source1_data_property_value16641, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16641.clone()
                                }
                                self::JavaScriptOrdinaryObject::WeakSetObject(source_ml_source1_data_property_value16642, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16642.clone()
                                }
                                self::JavaScriptOrdinaryObject::OrdinaryObject(source_ml_source1_data_property_value16643, _, _, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16643.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostErrorObject(source_ml_source1_data_property_value16644, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16644.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostURLObject(source_ml_source1_data_property_value16645, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16645.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostArrayObject(source_ml_source1_data_property_value16646, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16646.clone()
                                }
                                self::JavaScriptOrdinaryObject::HostByteBufferObject(source_ml_source1_data_property_value16647, _, _, _, _, _, _) => {
                                    source_ml_source1_data_property_value16647.clone()
                                }
                            })) || (depth_2 > ({ let ml_reference_array368 = shared_constructor_field1.clone(); let ml_reference_projection368 = ml_reference_array368.borrow().len() as f64; ml_reference_projection368 }))) {
                                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Return(self::ml_string::Text::from_units(vec![111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 99, 104, 97, 105, 110, 115, 32, 109, 117, 115, 116, 32, 98, 101, 32, 97, 99, 121, 99, 108, 105, 99]))
                            } else {
                                {
                                    let index = self::find_java_script_ordinary_object(shared_constructor_field1.clone(), identity.clone());
                                    {
                                        let parent = {
                                            let source_ml_source1_array_read_value16652 = shared_constructor_field1.clone();
                                            { let ml_reference_array369 = source_ml_source1_array_read_value16652.clone(); let ml_reference_position369 = index; let ml_reference_projection369 = self::ml_array::optional_at(&*ml_reference_array369.borrow(), self::ml_array::number_index(ml_reference_position369)); ml_reference_projection369 }
                                        };
                                        if match (parent.clone(), None::<()>) { (Some(_ml_left), Some(_ml_right)) => false, (None, None) => true, _ => false } {
                                            self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Return(self::ml_string::Text::from_units(vec![101, 118, 101, 114, 121, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 109, 117, 115, 116, 32, 114, 101, 109, 97, 105, 110, 32, 105, 110, 32, 116, 104, 101, 32, 103, 108, 111, 98, 97, 108, 32, 100, 105, 114, 101, 99, 116, 111, 114, 121]))
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
                                                self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Return(self::ml_string::Text::from_units(vec![111, 112, 97, 113, 117, 101, 32, 104, 111, 115, 116, 32, 69, 114, 114, 111, 114, 115, 32, 110, 101, 101, 100, 32, 101, 120, 112, 108, 105, 99, 105, 116, 32, 112, 114, 111, 116, 111, 116, 121, 112, 101, 32, 116, 114, 97, 118, 101, 114, 115, 97, 108, 32, 114, 97, 116, 104, 101, 114, 32, 116, 104, 97, 110, 32, 111, 114, 100, 105, 110, 97, 114, 121, 32, 100, 101, 115, 99, 114, 105, 112, 116, 111, 114, 32, 108, 111, 111, 107, 117, 112]))
                                            } else {
                                                {
                                                    let prototype_2 = (((match parent.clone().expect("a proven present value") {
                                                        self::JavaScriptOrdinaryObject::ClosureObject(_, source_ml_source1_data_property_value17509, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value17509 = *source_ml_source1_data_property_value17509;
                                                            Some((source_ml_source1_data_property_value17509.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::BoundFunctionObject(_, source_ml_source1_data_property_value17510, _, _, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value17510 = *source_ml_source1_data_property_value17510;
                                                            Some((source_ml_source1_data_property_value17510.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::ArrayObject(_, source_ml_source1_data_property_value17511, _, _, _) => {
                                                            let source_ml_source1_data_property_value17511 = *source_ml_source1_data_property_value17511;
                                                            Some((source_ml_source1_data_property_value17511.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::CollectionIteratorObject(_, source_ml_source1_data_property_value17512, _, _, _, _, _, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value17512 = *source_ml_source1_data_property_value17512;
                                                            Some((source_ml_source1_data_property_value17512.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::MapObject(_, source_ml_source1_data_property_value17513, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value17513 = *source_ml_source1_data_property_value17513;
                                                            Some((source_ml_source1_data_property_value17513.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::SetObject(_, source_ml_source1_data_property_value17514, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value17514 = *source_ml_source1_data_property_value17514;
                                                            Some((source_ml_source1_data_property_value17514.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakMapObject(_, source_ml_source1_data_property_value17515, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value17515 = *source_ml_source1_data_property_value17515;
                                                            Some((source_ml_source1_data_property_value17515.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::WeakSetObject(_, source_ml_source1_data_property_value17516, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value17516 = *source_ml_source1_data_property_value17516;
                                                            Some((source_ml_source1_data_property_value17516.clone(),))
                                                        }
                                                        self::JavaScriptOrdinaryObject::OrdinaryObject(_, source_ml_source1_data_property_value17517, _, _, _, _, _, _, _) => {
                                                            let source_ml_source1_data_property_value17517 = *source_ml_source1_data_property_value17517;
                                                            Some((source_ml_source1_data_property_value17517.clone(),))
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
                        self::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Result::MlTemporaryReadJavaScriptOrdinaryPrototypeChainErrorLoop65Done
                    }
                }
            }
        };
    }
}
